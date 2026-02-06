use ssh2::Session;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::error::{Error, Result};
use crate::models::{Fsp, OntInfo, OpticalInfo, ServicePort};
use crate::parser::{
    check_for_failure, extract_ont_id, parse_ont_autofind, parse_ont_info, parse_optical_info,
    parse_service_ports,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionLevel {
    /// Root level (>)
    Root = 0,
    /// Enable level (#)
    Enable = 1,
    /// Config level ((config)#)
    Config = 2,
    /// Interface GPON level ((config-if-gpon-F/S)#)
    InterfaceGpon = 3,
}

#[derive(Debug, Clone, Copy)]
pub struct SessionContext {
    pub level: SessionLevel,
    pub frame: Option<u32>,
    pub slot: Option<u32>,
}

impl SessionContext {
    const fn new() -> Self {
        Self {
            level: SessionLevel::Root,
            frame: None,
            slot: None,
        }
    }
}

pub struct ServicePortConfig {
    pub vlan: u32,
    pub fsp: Fsp,
    pub ont_id: u32,
    pub gemport: u32,
    pub user_vlan: u32,
    pub inbound_traffic_table: u32,
    pub outbound_traffic_table: u32,
}

pub struct Connection {
    #[allow(dead_code)]
    session: Session,
    channel: ssh2::Channel,
    context: SessionContext,
}

impl Connection {
    pub fn connect(host: &str, port: i32, username: &str, password: &str) -> Result<Self> {
        let addr = format!("{host}:{port}");
        let tcp = TcpStream::connect(&addr).map_err(|e| Error::ConnectionFailed(e.to_string()))?;

        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;

        session.userauth_password(username, password)?;

        if !session.authenticated() {
            return Err(Error::AuthenticationFailed);
        }

        let mut channel = session.channel_session()?;
        channel.shell()?;

        let mut conn = Self {
            session,
            channel,
            context: SessionContext::new(),
        };

        conn.read_until_prompt(">")?;
        conn.enable()?;
        conn.config()?;

        Ok(conn)
    }

    pub fn enable(&mut self) -> Result<()> {
        if self.context.level != SessionLevel::Root {
            return Err(Error::InvalidContext(
                "Not in root mode, cannot enable".to_string(),
            ));
        }

        self.channel.write_all(b"enable\n")?;
        self.channel.flush()?;
        self.read_until_prompt("#")?;
        self.context.level = SessionLevel::Enable;

        Ok(())
    }

    pub fn config(&mut self) -> Result<()> {
        if self.context.level != SessionLevel::Enable {
            return Err(Error::InvalidContext(
                "Not in enable mode, cannot enter config".to_string(),
            ));
        }

        self.channel.write_all(b"config\n")?;
        self.channel.flush()?;
        self.read_until_prompt("(config)#")?;
        self.context.level = SessionLevel::Config;

        Ok(())
    }

    pub fn ensure_config(&mut self) -> Result<()> {
        match self.context.level {
            SessionLevel::Config => Ok(()),
            SessionLevel::InterfaceGpon => self.quit(),
            SessionLevel::Enable => self.config(),
            SessionLevel::Root => {
                self.enable()?;
                self.config()
            }
        }
    }

    pub fn ping(&mut self) -> Result<()> {
        let _ = self.execute("")?;
        Ok(())
    }

    pub fn interface_gpon(&mut self, frame: u32, slot: u32) -> Result<()> {
        if self.context.level != SessionLevel::Config {
            return Err(Error::InvalidContext(
                "Not in config mode, cannot enter interface gpon".to_string(),
            ));
        }

        let cmd = format!("interface gpon {frame}/{slot}\n");
        self.channel.write_all(cmd.as_bytes())?;
        self.channel.flush()?;

        let prompt = format!("(config-if-gpon-{frame}/{slot})#");
        self.read_until_prompt(&prompt)?;

        self.context.level = SessionLevel::InterfaceGpon;
        self.context.frame = Some(frame);
        self.context.slot = Some(slot);

        Ok(())
    }

    pub fn quit(&mut self) -> Result<()> {
        match self.context.level {
            SessionLevel::Root => {
                return Err(Error::InvalidContext("Already at root level".to_string()));
            }
            SessionLevel::InterfaceGpon => {
                self.channel.write_all(b"quit\n")?;
                self.channel.flush()?;
                self.read_until_prompt("(config)#")?;
                self.context.level = SessionLevel::Config;
                self.context.frame = None;
                self.context.slot = None;
            }
            SessionLevel::Config => {
                self.channel.write_all(b"quit\n")?;
                self.channel.flush()?;
                self.read_until_prompt("#")?;
                self.context.level = SessionLevel::Enable;
            }
            SessionLevel::Enable => {
                self.channel.write_all(b"quit\n")?;
                self.channel.flush()?;
                self.read_until_prompt("before logout")?;
                self.channel.write_all(b"y\n")?;
                self.channel.flush()?;
                self.context.level = SessionLevel::Root;
            }
        }

        Ok(())
    }

    pub fn logout(&mut self) -> Result<()> {
        while self.context.level != SessionLevel::Root {
            self.quit()?;
        }
        Ok(())
    }

    pub fn display_ont_autofind_all(&mut self) -> Result<Vec<crate::models::OntAutofindEntry>> {
        if self.context.level != SessionLevel::Config {
            return Err(Error::InvalidContext("Must be in config mode".to_string()));
        }

        let output = self.execute_command("display ont autofind all", "(config)#")?;
        let entries = parse_ont_autofind(&output);
        if entries.is_empty() && std::env::var("OLTCORE_DEBUG_OUTPUT").ok().as_deref() == Some("1")
        {
            let escaped = output.escape_default().to_string();
            eprintln!("OLTCORE_DEBUG_OUTPUT: empty parse for display ont autofind all\n{escaped}");
        }
        Ok(entries)
    }

    pub fn display_ont_info_by_sn(&mut self, serial_number: &str) -> Result<Option<OntInfo>> {
        if self.context.level != SessionLevel::Config {
            return Err(Error::InvalidContext("Must be in config mode".to_string()));
        }

        let sn = serial_number
            .split_whitespace()
            .next()
            .unwrap_or(serial_number);
        let cmd = format!("display ont info by-sn {sn}");
        let output = self.execute_command(&cmd, "(config)#")?;

        if output.contains("The required ONT does not exist") {
            return Ok(None);
        }
        if output.contains("Parameter error") {
            return Err(Error::InvalidSerialNumber);
        }

        Ok(parse_ont_info(&output))
    }

    pub fn display_ont_optical_info(
        &mut self,
        port: u32,
        ont_id: u32,
    ) -> Result<Option<OpticalInfo>> {
        if self.context.level != SessionLevel::InterfaceGpon {
            return Err(Error::InvalidContext(
                "Must be in interface gpon mode".to_string(),
            ));
        }

        let frame = self
            .context
            .frame
            .ok_or_else(|| Error::InvalidContext("Frame not set in context".to_string()))?;
        let slot = self
            .context
            .slot
            .ok_or_else(|| Error::InvalidContext("Slot not set in context".to_string()))?;

        let cmd = format!("display ont optical-info {port} {ont_id}");
        let prompt = format!("(config-if-gpon-{frame}/{slot})#");
        let output = self.execute_command(&cmd, &prompt)?;

        check_for_failure(&output)?;
        Ok(parse_optical_info(&output))
    }

    pub fn display_service_port(&mut self, fsp: Fsp, ont_id: u32) -> Result<Vec<ServicePort>> {
        if self.context.level != SessionLevel::Config {
            return Err(Error::InvalidContext("Must be in config mode".to_string()));
        }

        let cmd = format!(
            "display service-port port {}/{}/{} ont {}",
            fsp.frame, fsp.slot, fsp.port, ont_id
        );
        let output = self.execute_command(&cmd, "(config)#")?;

        check_for_failure(&output)?;
        Ok(parse_service_ports(&output))
    }

    pub fn ont_add(
        &mut self,
        port: u32,
        sn: &str,
        description: &str,
        line_profile_id: u32,
        service_profile_id: u32,
    ) -> Result<u32> {
        if self.context.level != SessionLevel::InterfaceGpon {
            return Err(Error::InvalidContext(
                "Must be in interface gpon mode".to_string(),
            ));
        }

        let frame = self
            .context
            .frame
            .ok_or_else(|| Error::InvalidContext("Frame not set in context".to_string()))?;
        let slot = self
            .context
            .slot
            .ok_or_else(|| Error::InvalidContext("Slot not set in context".to_string()))?;

        let sn = sn.split_whitespace().next().unwrap_or(sn);
        let cmd = format!(
            "ont add {port} sn-auth {sn} omci ont-lineprofile-id {line_profile_id} ont-srvprofile-id {service_profile_id} desc {description}"
        );

        let prompt = format!("(config-if-gpon-{frame}/{slot})#");
        let output = self.execute_command(&cmd, &prompt)?;

        check_for_failure(&output)?;

        extract_ont_id(&output)
            .ok_or_else(|| Error::ParseError("Failed to extract ONT ID from output".to_string()))
    }

    pub fn ont_delete_all(&mut self, port: u32) -> Result<()> {
        if self.context.level != SessionLevel::InterfaceGpon {
            return Err(Error::InvalidContext(
                "Must be in interface gpon mode".to_string(),
            ));
        }

        let frame = self
            .context
            .frame
            .ok_or_else(|| Error::InvalidContext("Frame not set in context".to_string()))?;
        let slot = self
            .context
            .slot
            .ok_or_else(|| Error::InvalidContext("Slot not set in context".to_string()))?;

        let cmd = format!("ont delete {port} all");
        let output = self.execute_command(&cmd, "(y/n)[n]:")?;

        check_for_failure(&output)?;

        let prompt = format!("(config-if-gpon-{frame}/{slot})#");
        self.execute_command("y", &prompt)?;

        Ok(())
    }

    pub fn ont_port_native_vlan(
        &mut self,
        port: u32,
        ont_id: u32,
        ont_type: &str,
        vlan: u32,
        priority: u32,
    ) -> Result<()> {
        if self.context.level != SessionLevel::InterfaceGpon {
            return Err(Error::InvalidContext(
                "Must be in interface gpon mode".to_string(),
            ));
        }

        let frame = self
            .context
            .frame
            .ok_or_else(|| Error::InvalidContext("Frame not set in context".to_string()))?;
        let slot = self
            .context
            .slot
            .ok_or_else(|| Error::InvalidContext("Slot not set in context".to_string()))?;

        let cmd = format!(
            "ont port native-vlan {port} {ont_id} {ont_type} vlan {vlan} priority {priority}"
        );

        let prompt = format!("(config-if-gpon-{frame}/{slot})#");
        let output = self.execute_command(&cmd, &prompt)?;

        check_for_failure(&output)?;
        Ok(())
    }

    pub fn service_port_add(&mut self, config: &ServicePortConfig) -> Result<()> {
        if self.context.level != SessionLevel::Config {
            return Err(Error::InvalidContext("Must be in config mode".to_string()));
        }

        let cmd = format!(
            "service-port vlan {} gpon {}/{}/{} ont {} gemport {} multi-service user-vlan {} tag-transform translate inbound traffic-table index {} outbound traffic-table index {}",
            config.vlan, config.fsp.frame, config.fsp.slot, config.fsp.port, config.ont_id, config.gemport, config.user_vlan, config.inbound_traffic_table, config.outbound_traffic_table
        );

        let output = self.execute_command(&cmd, "(config)#")?;
        check_for_failure(&output)?;
        Ok(())
    }

    pub fn service_port_undo(&mut self, service_port_id: u32) -> Result<()> {
        if self.context.level != SessionLevel::Config {
            return Err(Error::InvalidContext("Must be in config mode".to_string()));
        }

        let cmd = format!("undo service-port {service_port_id}");
        let output = self.execute_command(&cmd, "(config)#")?;
        check_for_failure(&output)?;
        Ok(())
    }

    pub fn execute(&mut self, command: &str) -> Result<String> {
        let prompt = match self.context.level {
            SessionLevel::Root => ">",
            SessionLevel::Enable => "#",
            SessionLevel::Config => "(config)#",
            SessionLevel::InterfaceGpon => {
                let frame = self
                    .context
                    .frame
                    .ok_or_else(|| Error::InvalidContext("Frame not set in context".to_string()))?;
                let slot = self
                    .context
                    .slot
                    .ok_or_else(|| Error::InvalidContext("Slot not set in context".to_string()))?;
                return self.execute_command(command, &format!("(config-if-gpon-{frame}/{slot})#"));
            }
        };

        self.execute_command(command, prompt)
    }

    fn execute_command(&mut self, command: &str, expected_prompt: &str) -> Result<String> {
        let max_retries = 3;
        let retry_delay = Duration::from_secs(1);
        let mut attempt = 0;

        loop {
            self.channel.write_all(command.as_bytes())?;
            self.channel.write_all(b"\n")?;
            self.channel.flush()?;

            let output = self.read_until_prompt(expected_prompt)?;
            if output.contains("Failure: System is busy") {
                let retry_all = std::env::var("OLTCORE_RETRY_ALL").ok().as_deref() == Some("1");
                let retry_allowed = should_retry_on_busy(command) || retry_all;
                if retry_allowed && attempt < max_retries {
                    attempt += 1;
                    if std::env::var("OLTCORE_DEBUG_OUTPUT").ok().as_deref() == Some("1") {
                        eprintln!("OLTCORE_DEBUG_OUTPUT: System is busy, retrying...");
                    }
                    sleep(retry_delay);
                    continue;
                }
                return Err(Error::CommandFailed(
                    "System is busy, please retry after a while".to_string(),
                ));
            }

            return Ok(output);
        }
    }

    fn read_until_prompt(&mut self, prompt: &str) -> Result<String> {
        let mut buffer = vec![0; 4096];
        let mut output = String::new();

        loop {
            match self.channel.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    let text = String::from_utf8_lossy(&buffer[..n]);
                    output.push_str(&text);

                    if output.contains("---- More") || output.contains("Press 'Q' to break") {
                        self.channel.write_all(b"\n")?;
                        self.channel.flush()?;
                        if let Some(pos) = output.rfind("----") {
                            output.truncate(pos);
                        }
                        continue;
                    }

                    if output.contains(" }:") {
                        self.channel.write_all(b"\n")?;
                        self.channel.flush()?;
                        continue;
                    }

                    let clean_output = output.replace("\x1b[37D", "");
                    let trimmed = clean_output.trim_end();
                    let suffix_match =
                        prompt.ends_with('#') || prompt.ends_with('>') || prompt.ends_with(':');

                    if (suffix_match && trimmed.ends_with(prompt))
                        || (!suffix_match && clean_output.contains(prompt))
                    {
                        output = clean_output;
                        self.session.set_blocking(false);
                        let quiet_timeout = Duration::from_millis(300);
                        let poll_sleep = Duration::from_millis(10);
                        let mut last_read = Instant::now();
                        loop {
                            match self.channel.read(&mut buffer) {
                                Ok(0) => {
                                    if last_read.elapsed() >= quiet_timeout {
                                        break;
                                    }
                                    sleep(poll_sleep);
                                }
                                Ok(n) => {
                                    let text = String::from_utf8_lossy(&buffer[..n]);
                                    output.push_str(&text);
                                    last_read = Instant::now();
                                }
                                Err(e) => {
                                    if e.kind() == std::io::ErrorKind::WouldBlock {
                                        if last_read.elapsed() >= quiet_timeout {
                                            break;
                                        }
                                        sleep(poll_sleep);
                                        continue;
                                    }
                                    self.session.set_blocking(true);
                                    return Err(Error::IoError(e));
                                }
                            }
                        }
                        self.session.set_blocking(true);
                        let final_output = output.replace("\x1b[37D", "");
                        return Ok(final_output);
                    }

                    output = clean_output;
                }
                Err(e) => return Err(Error::IoError(e)),
            }
        }

        Ok(output)
    }
}

fn should_retry_on_busy(command: &str) -> bool {
    let cmd = command.trim_start().to_ascii_lowercase();
    cmd.starts_with("display ") || cmd.starts_with("show ") || cmd.starts_with("list ")
}

impl Drop for Connection {
    fn drop(&mut self) {
        let _ = self.channel.close();
    }
}
