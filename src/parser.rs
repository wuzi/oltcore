use crate::{
    models::{OntAutofindEntry, OntInfo, OpticalInfo, ServicePort},
    Fsp,
};
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
                fsp: Fsp::default(),
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
            "F/S/P" => {
                if let Some(fsp) = Fsp::parse(value) {
                    entry.fsp = fsp;
                }
            }
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
        fsp: Fsp::default(),
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

    let mut found_data = false;

    for line in output.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "F/S/P" => {
                    info.fsp = Fsp::parse(value)?;
                    found_data = true;
                }
                "ONT-ID" => {
                    info.id = value.parse().unwrap_or(0);
                    found_data = true;
                }
                "Control flag" => {
                    info.control_flag = value.to_string();
                    found_data = true;
                }
                "Run state" => {
                    info.run_state = value.to_string();
                    found_data = true;
                }
                "Config state" => {
                    info.config_state = value.to_string();
                    found_data = true;
                }
                "Match state" => {
                    info.match_state = value.to_string();
                    found_data = true;
                }
                "DBA type" => {
                    info.dba_type = value.to_string();
                    found_data = true;
                }
                "ONT distance(m)" => {
                    info.distance = value.parse().unwrap_or(0);
                    found_data = true;
                }
                "ONT last distance(m)" => {
                    info.last_distance = value.parse().unwrap_or(0);
                    found_data = true;
                }
                "Memory occupation" => {
                    info.memory_occupation = value.to_string();
                    found_data = true;
                }
                "CPU occupation" => {
                    info.cpu_occupation = value.to_string();
                    found_data = true;
                }
                "Temperature" => {
                    if let Some(temp) = value.split('(').next() {
                        info.temperature = temp.trim().parse().unwrap_or(0);
                    }
                    found_data = true;
                }
                "Authentic type" => {
                    info.authentic_type = value.to_string();
                    found_data = true;
                }
                "SN" => {
                    if let Some((sn, readable)) = value.split_once('(') {
                        info.sn = sn.trim().to_string();
                        info.sn_readable = readable.trim_end_matches(')').to_string();
                    } else {
                        info.sn = value.to_string();
                    }
                    found_data = true;
                }
                "Management mode" => {
                    info.management_mode = value.to_string();
                    found_data = true;
                }
                "Description" => {
                    info.description = value.to_string();
                    found_data = true;
                }
                "Last down cause" => {
                    info.last_down_cause = value.to_string();
                    found_data = true;
                }
                "Last up time" => {
                    info.last_up_time = value.to_string();
                    found_data = true;
                }
                "Last down time" => {
                    info.last_down_time = value.to_string();
                    found_data = true;
                }
                "ONT online duration" => {
                    info.online_duration = value.to_string();
                    found_data = true;
                }
                "Line profile ID" => {
                    info.line_profile_id = value.parse().unwrap_or(0);
                    found_data = true;
                }
                "Line profile name" => {
                    info.line_profile_name = value.to_string();
                    found_data = true;
                }
                "Service profile ID" => {
                    info.service_profile_id = value.parse().unwrap_or(0);
                    found_data = true;
                }
                "Service profile name" => {
                    info.service_profile_name = value.to_string();
                    found_data = true;
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

    if output.contains("No service virtual port can be operated") {
        return ports;
    }

    // Regex to match service port lines
    // Example real output: "     68 1063 common   gpon 0/9 /2  0    20    vlan  20         10   10   up"
    // Captures: INDEX (68), VLAN ID (1063)
    let Ok(re) = Regex::new(r"(?m)^\s+(\d+)\s+(\d+)\s+\w+\s+gpon") else {
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
        if line.starts_with("Failure: ")
            && !line.contains("No service virtual port can be operated")
        {
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
        assert_eq!(
            entries[0].fsp,
            Fsp {
                frame: 0,
                slot: 6,
                port: 1
            }
        );
        assert_eq!(entries[0].serial_number_readable, "DD72-ABCD");
    }

    #[test]
    fn parse_ont_autofind_multiple_entries() {
        let output = "Number: 1\nF/S/P: 0/1/2\nOnt SN: ABCD (ABCD-1234)\nPassword: pass1\n\nNumber: 2\nF/S/P: 0/1/3\nOnt SN: EFGH\n";
        let entries = parse_ont_autofind(output);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].number, 1);
        assert_eq!(
            entries[0].fsp,
            Fsp {
                frame: 0,
                slot: 1,
                port: 2
            }
        );
        assert_eq!(entries[0].serial_number, "ABCD");
        assert_eq!(entries[0].serial_number_readable, "ABCD-1234");
        assert_eq!(entries[0].password, "pass1");
        assert_eq!(entries[1].number, 2);
        assert_eq!(
            entries[1].fsp,
            Fsp {
                frame: 0,
                slot: 1,
                port: 3
            }
        );
        assert_eq!(entries[1].serial_number, "EFGH");
        assert_eq!(entries[1].serial_number_readable, "");
    }

    #[test]
    fn parse_ont_autofind_skips_zero_number() {
        let output = "Number: 0\nF/S/P: 0/1/2\nOnt SN: ABCD\n";
        let entries = parse_ont_autofind(output);
        assert!(entries.is_empty());
    }

    #[test]
    fn parse_ont_autofind_invalid_fsp_keeps_default() {
        let output = "Number: 1\nF/S/P: invalid\nOnt SN: ABCD\n";
        let entries = parse_ont_autofind(output);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].fsp, Fsp::default());
        assert_eq!(entries[0].serial_number, "ABCD");
    }

    #[test]
    fn parse_ont_info_full() {
        let output = "F/S/P: 0/2/3\nONT-ID: 42\nControl flag: active\nRun state: online\nConfig state: normal\nMatch state: match\nDBA type: DBA1\nONT distance(m): 120\nONT last distance(m): 115\nMemory occupation: 15%\nCPU occupation: 5%\nTemperature: 45(C)\nAuthentic type: sn\nSN: 1234 (ABCD-1234)\nManagement mode: OMCI\nDescription: test ont\nLast down cause: loss\nLast up time: 2024-01-01 00:00:00\nLast down time: 2024-01-01 01:00:00\nONT online duration: 1h\nLine profile ID: 10\nLine profile name: line10\nService profile ID: 20\nService profile name: svc20\n";
        let info = parse_ont_info(output).expect("expected info");
        assert_eq!(
            info.fsp,
            Fsp {
                frame: 0,
                slot: 2,
                port: 3
            }
        );
        assert_eq!(info.id, 42);
        assert_eq!(info.control_flag, "active");
        assert_eq!(info.run_state, "online");
        assert_eq!(info.config_state, "normal");
        assert_eq!(info.match_state, "match");
        assert_eq!(info.dba_type, "DBA1");
        assert_eq!(info.distance, 120);
        assert_eq!(info.last_distance, 115);
        assert_eq!(info.memory_occupation, "15%");
        assert_eq!(info.cpu_occupation, "5%");
        assert_eq!(info.temperature, 45);
        assert_eq!(info.authentic_type, "sn");
        assert_eq!(info.sn, "1234");
        assert_eq!(info.sn_readable, "ABCD-1234");
        assert_eq!(info.management_mode, "OMCI");
        assert_eq!(info.description, "test ont");
        assert_eq!(info.last_down_cause, "loss");
        assert_eq!(info.last_up_time, "2024-01-01 00:00:00");
        assert_eq!(info.last_down_time, "2024-01-01 01:00:00");
        assert_eq!(info.online_duration, "1h");
        assert_eq!(info.line_profile_id, 10);
        assert_eq!(info.line_profile_name, "line10");
        assert_eq!(info.service_profile_id, 20);
        assert_eq!(info.service_profile_name, "svc20");
    }

    #[test]
    fn parse_ont_info_unknown_only_returns_none() {
        let output = "Foo: bar\nBaz: qux\n";
        let info = parse_ont_info(output);
        assert!(info.is_none());
    }

    #[test]
    fn parse_ont_info_invalid_fsp_returns_none() {
        let output = "F/S/P: invalid\nONT-ID: 1\n";
        let info = parse_ont_info(output);
        assert!(info.is_none());
    }

    #[test]
    fn parse_optical_info_empty() {
        let output = "";
        let info = parse_optical_info(output);
        assert!(info.is_none());
    }

    #[test]
    fn parse_optical_info_minimal() {
        let output = "ONU NNI port ID: 1/1/1\nVendor name: VendorX\nRx optical power(dBm): -12.3\n";
        let info = parse_optical_info(output).expect("expected optical info");
        assert_eq!(info.onu_nni_port_id, "1/1/1");
        assert_eq!(info.vendor_name, "VendorX");
        assert_eq!(info.rx_optical_power, "-12.3");
    }

    #[test]
    fn parse_service_ports_parses_entries() {
        let output = "     68 1063 common   gpon 0/9 /2  0    20    vlan  20         10   10   up\n     69 1064 common   gpon 0/9 /3  0    20    vlan  20         10   10   up\n";
        let ports = parse_service_ports(output);
        assert_eq!(ports.len(), 2);
        assert_eq!(
            ports[0],
            ServicePort {
                index: 68,
                vlan: 1063
            }
        );
        assert_eq!(
            ports[1],
            ServicePort {
                index: 69,
                vlan: 1064
            }
        );
    }

    #[test]
    fn parse_service_ports_failure_is_empty() {
        let output = "Failure: No service virtual port can be operated\n  1234  100  gpon  0/4/0  1  20  100  translate  inbound  10  10  flow\n";
        let ports = parse_service_ports(output);
        assert!(ports.is_empty());
    }

    #[test]
    fn extract_ont_id_parses_value() {
        let output = "ONTID :123\n";
        let id = extract_ont_id(output);
        assert_eq!(id, Some(123));
    }

    #[test]
    fn extract_ont_id_missing_returns_none() {
        let output = "ONT-ID: 1\n";
        let id = extract_ont_id(output);
        assert!(id.is_none());
    }

    #[test]
    fn check_for_failure_ok() {
        let output = "OK\n";
        let result = check_for_failure(output);
        assert!(result.is_ok());
    }

    #[test]
    fn check_for_failure_command_failed() {
        let output = "Failure: device busy\n";
        let result = check_for_failure(output);
        match result {
            Err(crate::error::Error::CommandFailed(msg)) => {
                assert_eq!(msg, "device busy");
            }
            _ => panic!("expected command failed error"),
        }
    }

    #[test]
    fn check_for_failure_invalid_serial() {
        let output = "Parameter error\n";
        let result = check_for_failure(output);
        match result {
            Err(crate::error::Error::InvalidSerialNumber) => {}
            _ => panic!("expected invalid serial number error"),
        }
    }
}
