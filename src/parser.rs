use crate::models::{OntAutofindEntry, OntInfo};

#[must_use]
pub fn parse_ont_autofind(output: &str) -> Vec<OntAutofindEntry> {
    let mut entries = Vec::new();
    let blocks: Vec<&str> = output
        .split("----------------------------------------------------------------------------")
        .filter(|s| s.contains("Number"))
        .collect();

    for block in blocks {
        let mut entry = OntAutofindEntry {
            number: 0,
            fsp: String::new(),
            ont_sn: String::new(),
            ont_sn_readable: String::new(),
            password: String::new(),
            loid: String::new(),
            checkcode: String::new(),
            vendor_id: String::new(),
            ont_version: String::new(),
            ont_software_version: String::new(),
            ont_equipment_id: String::new(),
            ont_customized_info: String::new(),
            ont_autofind_time: String::new(),
        };

        for line in block.lines() {
            let line = line.trim();
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "Number" => entry.number = value.parse().unwrap_or(0),
                    "F/S/P" => entry.fsp = value.to_string(),
                    "Ont SN" => {
                        if let Some((sn, readable)) = value.split_once('(') {
                            entry.ont_sn = sn.trim().to_string();
                            entry.ont_sn_readable = readable.trim_end_matches(')').to_string();
                        }
                    }
                    "Password" => entry.password = value.to_string(),
                    "Loid" => entry.loid = value.to_string(),
                    "Checkcode" => entry.checkcode = value.to_string(),
                    "VendorID" => entry.vendor_id = value.to_string(),
                    "Ont Version" => entry.ont_version = value.to_string(),
                    "Ont SoftwareVersion" => entry.ont_software_version = value.to_string(),
                    "Ont EquipmentID" => entry.ont_equipment_id = value.to_string(),
                    "Ont Customized Info" => entry.ont_customized_info = value.to_string(),
                    "Ont autofind time" => entry.ont_autofind_time = value.to_string(),
                    _ => {}
                }
            }
        }

        if entry.number > 0 {
            entries.push(entry);
        }
    }

    entries
}

#[must_use]
pub fn parse_ont_info(output: &str) -> Option<OntInfo> {
    let mut info = OntInfo {
        fsp: String::new(),
        ont_id: 0,
        control_flag: String::new(),
        run_state: String::new(),
        config_state: String::new(),
        match_state: String::new(),
        dba_type: String::new(),
        ont_distance: 0,
        ont_last_distance: 0,
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
        ont_online_duration: String::new(),
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
                "ONT-ID" => info.ont_id = value.parse().unwrap_or(0),
                "Control flag" => info.control_flag = value.to_string(),
                "Run state" => info.run_state = value.to_string(),
                "Config state" => info.config_state = value.to_string(),
                "Match state" => info.match_state = value.to_string(),
                "DBA type" => info.dba_type = value.to_string(),
                "ONT distance(m)" => info.ont_distance = value.parse().unwrap_or(0),
                "ONT last distance(m)" => info.ont_last_distance = value.parse().unwrap_or(0),
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
                "ONT online duration" => info.ont_online_duration = value.to_string(),
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
}
