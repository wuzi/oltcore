use crate::models::Fsp;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct ActiveAlarms {
    pub alarms: Vec<ActiveAlarm>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct ActiveAlarm {
    pub serial_number: u64,
    pub timestamp: String,
    pub alarm_name: String,
    pub fsp: Option<Fsp>,
    pub ont_id: Option<u32>,
    pub equipment_id: Option<String>,
    pub description: Option<String>,
    pub cause: Option<String>,
    pub advice: Option<String>,
    pub severity: Option<String>,
    pub service_effect: Option<String>,
}

#[must_use]
pub fn parse_active_alarms_list(output: &str) -> ActiveAlarms {
    let mut alarms = ActiveAlarms::default();
    let mut current_alarm: Option<ActiveAlarm> = None;

    for line in output.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('-') || line.starts_with("Command") {
            continue;
        }

        if line.starts_with("AlarmSN") && line.contains("Date&Time") {
            continue;
        }

        if let Some(first_char) = line.chars().next() {
            if first_char.is_ascii_digit() {
                if let Some(alarm) = current_alarm.take() {
                    alarms.alarms.push(alarm);
                }

                current_alarm = parse_alarm_list_header(line);
            } else if let Some(alarm) = current_alarm.as_mut() {
                alarm.alarm_name.push(' ');
                alarm.alarm_name.push_str(line);
            }
        }
    }

    if let Some(alarm) = current_alarm.take() {
        alarms.alarms.push(alarm);
    }

    alarms
}

#[must_use]
pub fn parse_active_alarms_detail(output: &str) -> ActiveAlarms {
    let mut alarms = ActiveAlarms::default();
    let mut current_alarm: Option<ActiveAlarm> = None;

    for line in output.lines() {
        let line_trimmed = line.trim();

        if line_trimmed.is_empty() {
            continue;
        }

        if line_trimmed.starts_with("ALARM ") && line_trimmed.contains("FAULT") {
            if let Some(alarm) = current_alarm.take() {
                alarms.alarms.push(alarm);
            }

            current_alarm = parse_alarm_detail_header(line_trimmed);
            continue;
        }

        let Some(alarm) = current_alarm.as_mut() else {
            continue;
        };

        if line_trimmed.starts_with("ALARM NAME") {
            if let Some((_, name)) = line_trimmed.split_once(':') {
                alarm.alarm_name = name.trim().to_string();
            }
        } else if line_trimmed.starts_with("PARAMETERS") {
            if let Some((_, params)) = line_trimmed.split_once(':') {
                parse_parameters(params.trim(), alarm);
            }
        } else if line_trimmed.starts_with("DESCRIPTION") {
            if let Some((_, desc)) = line_trimmed.split_once(':') {
                alarm.description = Some(desc.trim().to_string());
            }
        } else if line_trimmed.starts_with("SRVEFF") {
            if let Some((_, effect)) = line_trimmed.split_once(':') {
                alarm.service_effect = Some(effect.trim().to_string());
            }
        } else if line_trimmed.starts_with("CAUSE") {
            if let Some((_, cause)) = line_trimmed.split_once(':') {
                alarm.cause = Some(cause.trim().to_string());
            }
        } else if line_trimmed.starts_with("ADVICE") {
            if let Some((_, advice)) = line_trimmed.split_once(':') {
                alarm.advice = Some(advice.trim().to_string());
            }
        } else if line_trimmed == "--- END" {
            if let Some(alarm) = current_alarm.take() {
                alarms.alarms.push(alarm);
            }
        }
    }

    if let Some(alarm) = current_alarm {
        alarms.alarms.push(alarm);
    }

    alarms
}

fn parse_alarm_list_header(line: &str) -> Option<ActiveAlarm> {
    let parts: Vec<&str> = line.splitn(2, char::is_whitespace).collect();
    if parts.is_empty() {
        return None;
    }

    let serial_number = parts[0].parse().ok()?;

    let Ok(date_re) = Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}[+\-]\d{2}:\d{2})") else {
        return None;
    };

    let timestamp = date_re
        .captures(line)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_default();

    let alarm_name = line.find(&timestamp).map_or_else(String::new, |pos| {
        let after_timestamp = &line[pos + timestamp.len()..];
        after_timestamp.trim().to_string()
    });

    Some(ActiveAlarm {
        serial_number,
        timestamp,
        alarm_name,
        ..ActiveAlarm::default()
    })
}

fn parse_alarm_detail_header(line: &str) -> Option<ActiveAlarm> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 6 {
        return None;
    }

    let serial_number = parts[1].parse().ok()?;

    let severity = parts
        .iter()
        .find(|&&p| p == "FAULT" || p == "INFORMATION")
        .map(|&s| s.to_string());

    Some(ActiveAlarm {
        serial_number,
        severity,
        ..ActiveAlarm::default()
    })
}

fn parse_parameters(params: &str, alarm: &mut ActiveAlarm) {
    let Ok(fsp_re) = Regex::new(r"FrameID: (\d+), SlotID: (\d+), PortID: (\d+)") else {
        return;
    };

    if let Some(caps) = fsp_re.captures(params) {
        if let (Some(frame), Some(slot), Some(port)) = (
            caps.get(1).and_then(|m| m.as_str().parse().ok()),
            caps.get(2).and_then(|m| m.as_str().parse().ok()),
            caps.get(3).and_then(|m| m.as_str().parse().ok()),
        ) {
            alarm.fsp = Some(Fsp { frame, slot, port });
        }
    }

    let Ok(ont_re) = Regex::new(r"ONT ID: (\d+)") else {
        return;
    };

    if let Some(caps) = ont_re.captures(params) {
        alarm.ont_id = caps.get(1).and_then(|m| m.as_str().parse().ok());
    }

    let Ok(eq_re) = Regex::new(r"Equipment ID: ([^\s,]+)") else {
        return;
    };

    if let Some(caps) = eq_re.captures(params) {
        alarm.equipment_id = Some(caps.get(1).map_or("", |m| m.as_str()).to_string());
    }
}
