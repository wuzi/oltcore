use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OntAutofindEntry {
    /// ONT entry number
    pub number: u32,
    /// Frame/Slot/Port
    pub fsp: String,
    /// ONT serial number (raw)
    pub serial_number: String,
    /// ONT serial number (human-readable format)
    pub serial_number_readable: String,
    /// ONT password
    pub password: String,
    /// Logical ONU Identifier
    pub lo_id: String,
    pub check_code: String,
    pub vendor_id: String,
    pub version: String,
    pub software_version: String,
    pub equipment_id: String,
    pub customized_info: String,
    pub auto_find_time: String,
}

impl OntAutofindEntry {
    #[must_use]
    pub fn fsp(&self) -> Option<(u32, u32, u32)> {
        let parts: Vec<&str> = self.fsp.split('/').collect();
        if parts.len() != 3 {
            return None;
        }

        let frame = parts[0].parse().ok()?;
        let slot = parts[1].parse().ok()?;
        let port = parts[2].parse().ok()?;

        Some((frame, slot, port))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OntInfo {
    /// Frame/Slot/Port
    pub fsp: String,
    pub id: u32,
    pub control_flag: String,
    pub run_state: String,
    pub config_state: String,
    pub match_state: String,
    pub dba_type: String,
    /// ONT distance in meters
    pub distance: u32,
    /// ONT last distance in meters
    pub last_distance: u32,
    /// Memory occupation percentage
    pub memory_occupation: String,
    /// CPU occupation percentage
    pub cpu_occupation: String,
    /// Temperature in Celsius
    pub temperature: i32,
    pub authentic_type: String,
    /// Serial number (raw)
    pub sn: String,
    /// Serial number (human-readable)
    pub sn_readable: String,
    pub management_mode: String,
    pub description: String,
    pub last_down_cause: String,
    pub last_up_time: String,
    pub last_down_time: String,
    pub online_duration: String,
    pub line_profile_id: u32,
    pub line_profile_name: String,
    pub service_profile_id: u32,
    pub service_profile_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OpticalInfo {
    pub onu_nni_port_id: String,
    pub module_type: String,
    pub module_sub_type: String,
    pub used_type: String,
    pub encapsulation_type: String,
    pub optical_power_precision: String,
    pub vendor_name: String,
    pub vendor_rev: String,
    pub vendor_pn: String,
    pub vendor_sn: String,
    pub date_code: String,
    pub rx_optical_power: String,
    pub rx_power_current_warning_threshold: String,
    pub rx_power_current_alarm_threshold: String,
    pub tx_optical_power: String,
    pub tx_power_current_warning_threshold: String,
    pub tx_power_current_alarm_threshold: String,
    pub laser_bias_current: String,
    pub tx_bias_current_warning_threshold: String,
    pub tx_bias_current_alarm_threshold: String,
    pub temperature: String,
    pub temperature_warning_threshold: String,
    pub temperature_alarm_threshold: String,
    pub voltage: String,
    pub supply_voltage_warning_threshold: String,
    pub supply_voltage_alarm_threshold: String,
    pub olt_rx_ont_optical_power: String,
    pub catv_rx_optical_power: String,
    pub catv_rx_power_alarm_threshold: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServicePort {
    pub index: u32,
    pub vlan: u32,
}

/// Frame/Slot/Port representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fsp {
    pub frame: u32,
    pub slot: u32,
    pub port: u32,
}

impl Fsp {
    #[must_use]
    pub fn parse(fsp: &str) -> Option<Self> {
        let parts: Vec<&str> = fsp.split('/').collect();
        if parts.len() != 3 {
            return None;
        }

        Some(Self {
            frame: parts[0].parse().ok()?,
            slot: parts[1].parse().ok()?,
            port: parts[2].parse().ok()?,
        })
    }
}

impl std::fmt::Display for Fsp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.frame, self.slot, self.port)
    }
}
