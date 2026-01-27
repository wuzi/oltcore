use ssh2::Session;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::error::{Error, Result};
use crate::models::OntInfo;
use crate::parser::{parse_ont_autofind, parse_ont_info};

pub struct Connection {
    #[allow(dead_code)]
    session: Session,
    channel: ssh2::Channel,
}

impl Connection {
    pub fn connect(host: &str, port: u16, username: &str, password: &str) -> Result<Self> {
        let addr = format!("{host}:{port}");
        let tcp = TcpStream::connect(&addr).map_err(|e| Error::ConnectionFailed(e.to_string()))?;

        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;

        session.userauth_password(username, password)?;

        if !session.authenticated() {
            return Err(Error::AuthenticationFailed);
        }

        let channel = session.channel_session()?;

        Ok(Self { session, channel })
    }

    pub fn display_ont_autofind_all(&mut self) -> Result<Vec<crate::models::OntAutofindEntry>> {
        let output = self.execute_command("display ont autofind all")?;
        Ok(parse_ont_autofind(&output))
    }

    pub fn display_ont_info_by_sn(&mut self, serial_number: &str) -> Result<Option<OntInfo>> {
        let output = self.execute_command(&format!("display ont info by-sn {serial_number}"))?;
        Ok(parse_ont_info(&output))
    }

    pub fn execute(&mut self, command: &str) -> Result<String> {
        self.execute_command(command)
    }

    pub fn init_shell(&mut self) -> Result<()> {
        self.channel.shell()?;

        self.read_until_prompt(">")?;

        self.channel.write_all(b"enable\n")?;
        self.channel.flush()?;
        self.read_until_prompt("#")?;

        self.channel.write_all(b"config\n")?;
        self.channel.flush()?;
        self.read_until_prompt("(config)#")?;

        Ok(())
    }

    fn execute_command(&mut self, command: &str) -> Result<String> {
        self.channel.write_all(command.as_bytes())?;
        self.channel.write_all(b"\n")?;
        self.channel.flush()?;

        self.read_until_prompt("(config)#")
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

                    if output.contains("-- More") || output.contains("Press space") {
                        self.channel.write_all(b" ")?;
                        self.channel.flush()?;
                        if let Some(pos) = output.rfind("-- More --") {
                            output.truncate(pos);
                        }
                        if let Some(pos) = output.rfind("Press space") {
                            output.truncate(pos);
                        }
                    }

                    if output.contains(prompt) {
                        break;
                    }
                }
                Err(e) => return Err(Error::IoError(e)),
            }
        }

        Ok(output)
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        let _ = self.channel.close();
    }
}
