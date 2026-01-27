use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntAutofindEntry {
    /// ONT entry number
    pub number: u32,
    /// Frame/Slot/Port
    pub fsp: String,
    /// ONT serial number (raw)
    pub ont_sn: String,
    /// ONT serial number (human-readable format)
    pub ont_sn_readable: String,
    /// ONT password
    pub password: String,
    /// Logical ONU Identifier
    pub loid: String,
    pub checkcode: String,
    pub vendor_id: String,
    pub ont_version: String,
    pub ont_software_version: String,
    pub ont_equipment_id: String,
    pub ont_customized_info: String,
    pub ont_autofind_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntInfo {
    /// Frame/Slot/Port
    pub fsp: String,
    pub ont_id: u32,
    pub control_flag: String,
    pub run_state: String,
    pub config_state: String,
    pub match_state: String,
    pub dba_type: String,
    /// ONT distance in meters
    pub ont_distance: u32,
    /// ONT last distance in meters
    pub ont_last_distance: u32,
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
    pub ont_online_duration: String,
    pub line_profile_id: u32,
    pub line_profile_name: String,
    pub service_profile_id: u32,
    pub service_profile_name: String,
}
