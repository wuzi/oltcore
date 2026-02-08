use crate::models::{OntAutofindEntry, OntInfo, OpticalInfo, ServicePort};
use regex::Regex;

#[must_use]
pub fn parse_ont_autofind(output: &str) -> Vec<OntAutofindEntry> {
    let mut entries = Vec::new();
    let output = output.replace('\r', "\n");
    let mut current: Option<OntAutofindEntry> = None;

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let Some((key, value)) = line.split_once(':') else {
            continue;
        };

        let key = key.trim();
        let value = value.trim();

        if key == "Number" {
            if let Some(entry) = current.take() {
                if entry.number > 0 {
                    entries.push(entry);
                }
            }

            let mut entry = OntAutofindEntry {
                number: 0,
                fsp: String::new(),
                serial_number: String::new(),
                serial_number_readable: String::new(),
                password: String::new(),
                lo_id: String::new(),
                check_code: String::new(),
                vendor_id: String::new(),
                version: String::new(),
                software_version: String::new(),
                equipment_id: String::new(),
                customized_info: String::new(),
                auto_find_time: String::new(),
            };
            entry.number = value.parse().unwrap_or(0);
            current = Some(entry);
            continue;
        }

        let Some(entry) = current.as_mut() else {
            continue;
        };

        match key {
            "F/S/P" => entry.fsp = value.to_string(),
            "Ont SN" => {
                if let Some((sn, readable)) = value.split_once('(') {
                    entry.serial_number = sn.trim().to_string();
                    entry.serial_number_readable = readable.trim_end_matches(')').to_string();
                } else {
                    entry.serial_number = value.to_string();
                }
            }
            "Password" => entry.password = value.to_string(),
            "Loid" => entry.lo_id = value.to_string(),
            "Checkcode" => entry.check_code = value.to_string(),
            "VendorID" => entry.vendor_id = value.to_string(),
            "Ont Version" => entry.version = value.to_string(),
            "Ont SoftwareVersion" => entry.software_version = value.to_string(),
            "Ont EquipmentID" => entry.equipment_id = value.to_string(),
            "Ont Customized Info" => entry.customized_info = value.to_string(),
            "Ont autofind time" => entry.auto_find_time = value.to_string(),
            _ => {}
        }
    }

    if let Some(entry) = current {
        if entry.number > 0 {
            entries.push(entry);
        }
    }

    entries
}

#[must_use]
pub fn parse_ont_info(output: &str) -> Option<OntInfo> {
    if output.contains("The required ONT does not exist") {
        return None;
    }

    let mut info = OntInfo {
        fsp: String::new(),
        id: 0,
        control_flag: String::new(),
        run_state: String::new(),
        config_state: String::new(),
        match_state: String::new(),
        dba_type: String::new(),
        distance: 0,
        last_distance: 0,
        memory_occupation: String::new(),
        cpu_occupation: String::new(),
        temperature: 0,
        authentic_type: String::new(),
        sn: String::new(),
        sn_readable: String::new(),
        management_mode: String::new(),
        description: String::new(),
        last_down_cause: String::new(),
        last_up_time: String::new(),
        last_down_time: String::new(),
        online_duration: String::new(),
        line_profile_id: 0,
        line_profile_name: String::new(),
        service_profile_id: 0,
        service_profile_name: String::new(),
    };

    for line in output.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "F/S/P" => info.fsp = value.to_string(),
                "ONT-ID" => info.id = value.parse().unwrap_or(0),
                "Control flag" => info.control_flag = value.to_string(),
                "Run state" => info.run_state = value.to_string(),
                "Config state" => info.config_state = value.to_string(),
                "Match state" => info.match_state = value.to_string(),
                "DBA type" => info.dba_type = value.to_string(),
                "ONT distance(m)" => info.distance = value.parse().unwrap_or(0),
                "ONT last distance(m)" => info.last_distance = value.parse().unwrap_or(0),
                "Memory occupation" => info.memory_occupation = value.to_string(),
                "CPU occupation" => info.cpu_occupation = value.to_string(),
                "Temperature" => {
                    if let Some(temp) = value.split('(').next() {
                        info.temperature = temp.trim().parse().unwrap_or(0);
                    }
                }
                "Authentic type" => info.authentic_type = value.to_string(),
                "SN" => {
                    if let Some((sn, readable)) = value.split_once('(') {
                        info.sn = sn.trim().to_string();
                        info.sn_readable = readable.trim_end_matches(')').to_string();
                    } else {
                        info.sn = value.to_string();
                    }
                }
                "Management mode" => info.management_mode = value.to_string(),
                "Description" => info.description = value.to_string(),
                "Last down cause" => info.last_down_cause = value.to_string(),
                "Last up time" => info.last_up_time = value.to_string(),
                "Last down time" => info.last_down_time = value.to_string(),
                "ONT online duration" => info.online_duration = value.to_string(),
                "Line profile ID" => info.line_profile_id = value.parse().unwrap_or(0),
                "Line profile name" => info.line_profile_name = value.to_string(),
                "Service profile ID" => info.service_profile_id = value.parse().unwrap_or(0),
                "Service profile name" => info.service_profile_name = value.to_string(),
                _ => {}
            }
        }
    }

    if info.fsp.is_empty() {
        None
    } else {
        Some(info)
    }
}

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn parse_optical_info(output: &str) -> Option<OpticalInfo> {
    let mut info = OpticalInfo {
        onu_nni_port_id: String::new(),
        module_type: String::new(),
        module_sub_type: String::new(),
        used_type: String::new(),
        encapsulation_type: String::new(),
        optical_power_precision: String::new(),
        vendor_name: String::new(),
        vendor_rev: String::new(),
        vendor_pn: String::new(),
        vendor_sn: String::new(),
        date_code: String::new(),
        rx_optical_power: String::new(),
        rx_power_current_warning_threshold: String::new(),
        rx_power_current_alarm_threshold: String::new(),
        tx_optical_power: String::new(),
        tx_power_current_warning_threshold: String::new(),
        tx_power_current_alarm_threshold: String::new(),
        laser_bias_current: String::new(),
        tx_bias_current_warning_threshold: String::new(),
        tx_bias_current_alarm_threshold: String::new(),
        temperature: String::new(),
        temperature_warning_threshold: String::new(),
        temperature_alarm_threshold: String::new(),
        voltage: String::new(),
        supply_voltage_warning_threshold: String::new(),
        supply_voltage_alarm_threshold: String::new(),
        olt_rx_ont_optical_power: String::new(),
        catv_rx_optical_power: String::new(),
        catv_rx_power_alarm_threshold: String::new(),
    };

    let mut found_data = false;

    for line in output.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "ONU NNI port ID" => {
                    info.onu_nni_port_id = value.to_string();
                    found_data = true;
                }
                "Module type" => info.module_type = value.to_string(),
                "Module sub-type" => info.module_sub_type = value.to_string(),
                "Used type" => info.used_type = value.to_string(),
                "Encapsulation Type" => info.encapsulation_type = value.to_string(),
                "Optical power precision(dBm)" => info.optical_power_precision = value.to_string(),
                "Vendor name" => info.vendor_name = value.to_string(),
                "Vendor rev" => info.vendor_rev = value.to_string(),
                "Vendor PN" => info.vendor_pn = value.to_string(),
                "Vendor SN" => info.vendor_sn = value.to_string(),
                "Date Code" => info.date_code = value.to_string(),
                "Rx optical power(dBm)" => info.rx_optical_power = value.to_string(),
                "Rx power current warning threshold(dBm)" => {
                    info.rx_power_current_warning_threshold = value.to_string();
                }
                "Rx power current alarm threshold(dBm)" => {
                    info.rx_power_current_alarm_threshold = value.to_string();
                }
                "Tx optical power(dBm)" => info.tx_optical_power = value.to_string(),
                "Tx power current warning threshold(dBm)" => {
                    info.tx_power_current_warning_threshold = value.to_string();
                }
                "Tx power current alarm threshold(dBm)" => {
                    info.tx_power_current_alarm_threshold = value.to_string();
                }
                "Laser bias current(mA)" => info.laser_bias_current = value.to_string(),
                "Tx bias current warning threshold(mA)" => {
                    info.tx_bias_current_warning_threshold = value.to_string();
                }
                "Tx bias current alarm threshold(mA)" => {
                    info.tx_bias_current_alarm_threshold = value.to_string();
                }
                "Temperature(C)" => info.temperature = value.to_string(),
                "Temperature warning threshold(C)" => {
                    info.temperature_warning_threshold = value.to_string();
                }
                "Temperature alarm threshold(C)" => {
                    info.temperature_alarm_threshold = value.to_string();
                }
                "Voltage(V)" => info.voltage = value.to_string(),
                "Supply voltage warning threshold(V)" => {
                    info.supply_voltage_warning_threshold = value.to_string();
                }
                "Supply voltage alarm threshold(V)" => {
                    info.supply_voltage_alarm_threshold = value.to_string();
                }
                "OLT Rx ONT optical power(dBm)" => {
                    info.olt_rx_ont_optical_power = value.to_string();
                }
                "CATV Rx optical power(dBm)" => info.catv_rx_optical_power = value.to_string(),
                "CATV Rx power alarm threshold(dBm)" => {
                    info.catv_rx_power_alarm_threshold = value.to_string();
                }
                _ => {}
            }
        }
    }

    if found_data {
        Some(info)
    } else {
        None
    }
}

#[must_use]
pub fn parse_service_ports(output: &str) -> Vec<ServicePort> {
    let mut ports = Vec::new();

    if output.contains("Failure: No service virtual port can be operated") {
        return ports;
    }

    // Regex to match service port lines
    // Example: "  1234  100  gpon  0/4/0  1  20  100  translate  inbound  10  10  flow"
    let Ok(re) = Regex::new(
        r"\s+(\d+)\s+(\d+)\s+\w+\s+\d+/\d+/\d+\s+\d+\s+\d+\s+\d+\s+\w+\s+\w+\s+\d+\s+\d+\s+\w+",
    ) else {
        return ports;
    };

    for cap in re.captures_iter(output) {
        if let (Some(index), Some(vlan)) = (cap.get(1), cap.get(2)) {
            if let (Ok(index), Ok(vlan)) = (index.as_str().parse(), vlan.as_str().parse()) {
                ports.push(ServicePort { index, vlan });
            }
        }
    }

    ports
}

#[must_use]
pub fn extract_ont_id(output: &str) -> Option<u32> {
    let re = Regex::new(r"ONTID\s*:(\d+)").ok()?;
    re.captures(output)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse().ok())
}

pub fn check_for_failure(output: &str) -> crate::error::Result<()> {
    for line in output.lines() {
        let line = line.trim();
        if line.starts_with("Failure: ") {
            let msg = line.trim_start_matches("Failure: ");
            return Err(crate::error::Error::CommandFailed(msg.to_string()));
        }
        if line.contains("Parameter error") {
            return Err(crate::error::Error::InvalidSerialNumber);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ont_autofind_empty() {
        let output = "";
        let entries = parse_ont_autofind(output);
        assert!(entries.is_empty());
    }

    #[test]
    fn parse_ont_info_empty() {
        let output = "";
        let info = parse_ont_info(output);
        assert!(info.is_none());
    }

    #[test]
    fn parse_ont_info_not_found() {
        let output = "The required ONT does not exist";
        let info = parse_ont_info(output);
        assert!(info.is_none());
    }

    #[test]
    fn parse_ont_autofind_with_carriage_returns() {
        let output = "Number              : 1\r\nF/S/P               : 0/6/1\r\nOnt SN              : 4444 (DD72-ABCD)\r\n";
        let entries = parse_ont_autofind(output);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].number, 1);
        assert_eq!(entries[0].fsp, "0/6/1");
        assert_eq!(entries[0].serial_number_readable, "DD72-ABCD");
    }
}
