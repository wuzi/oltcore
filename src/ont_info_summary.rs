use crate::models::Fsp;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct OntInfoSummary {
    pub ports: Vec<OntInfoSummaryPort>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OntInfoSummaryPort {
    pub fsp: Fsp,
    pub total_onts: u32,
    pub online_onts: u32,
    pub onts: Vec<OntInfoSummaryOnt>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct OntInfoSummaryOnt {
    pub id: u32,
    pub run_state: String,
    pub last_up_time: String,
    pub last_down_time: String,
    pub last_down_cause: String,
    pub sn: String,
    pub ont_type: String,
    pub distance_m: Option<u32>,
    pub rx_power: Option<f32>,
    pub tx_power: Option<f32>,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SummarySection {
    None,
    States,
    Details,
}

struct PortBuilder {
    port: OntInfoSummaryPort,
    ont_map: HashMap<u32, OntInfoSummaryOnt>,
    ont_order: Vec<u32>,
}

#[must_use]
pub fn parse_ont_info_summary(output: &str) -> OntInfoSummary {
    let Ok(port_re) =
        Regex::new(r"^In port (\d+/\d+/\d+), the total of ONTs are: (\d+), online: (\d+)")
    else {
        return OntInfoSummary::default();
    };

    let mut summary = OntInfoSummary::default();
    let mut current_port: Option<PortBuilder> = None;
    let mut section = SummarySection::None;

    let cleaned = output.replace('\r', "\n");
    for line in cleaned.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(caps) = port_re.captures(line) {
            if let Some(port) = current_port.take() {
                summary.ports.push(finalize_port(port));
            }

            let fsp = caps
                .get(1)
                .and_then(|m| Fsp::parse(m.as_str()))
                .unwrap_or_default();
            let total_onts = caps
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let online_onts = caps
                .get(3)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            current_port = Some(PortBuilder {
                port: OntInfoSummaryPort {
                    fsp,
                    total_onts,
                    online_onts,
                    onts: Vec::new(),
                },
                ont_map: HashMap::new(),
                ont_order: Vec::new(),
            });
            section = SummarySection::None;
            continue;
        }

        if line.starts_with("ONT  Run") {
            section = SummarySection::States;
            continue;
        }

        if line.starts_with("ONT        SN") {
            section = SummarySection::Details;
            continue;
        }

        if line.starts_with("---") || line.starts_with("Command") {
            continue;
        }

        let Some(port) = current_port.as_mut() else {
            continue;
        };

        match section {
            SummarySection::States => {
                if let Some(state) = parse_state_line(line) {
                    let entry = upsert_ont(&mut port.ont_map, &mut port.ont_order, state.id);
                    entry.run_state = state.run_state;
                    entry.last_up_time = state.last_up_time;
                    entry.last_down_time = state.last_down_time;
                    entry.last_down_cause = state.last_down_cause;
                }
            }
            SummarySection::Details => {
                if let Some(ont) = parse_ont_line(line) {
                    let entry = upsert_ont(&mut port.ont_map, &mut port.ont_order, ont.id);
                    entry.sn = ont.sn;
                    entry.ont_type = ont.ont_type;
                    entry.distance_m = ont.distance_m;
                    entry.rx_power = ont.rx_power;
                    entry.tx_power = ont.tx_power;
                    entry.description = ont.description;
                }
            }
            SummarySection::None => {}
        }
    }

    if let Some(port) = current_port.take() {
        summary.ports.push(finalize_port(port));
    }

    summary
}

struct StateFields {
    id: u32,
    run_state: String,
    last_up_time: String,
    last_down_time: String,
    last_down_cause: String,
}

struct OntFields {
    id: u32,
    sn: String,
    ont_type: String,
    distance_m: Option<u32>,
    rx_power: Option<f32>,
    tx_power: Option<f32>,
    description: String,
}

fn parse_state_line(line: &str) -> Option<StateFields> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 6 {
        return None;
    }

    let id = parts[0].parse().ok()?;
    let run_state = parts[1].to_string();
    let last_up_time = format!("{} {}", parts[2], parts[3]);
    let last_down_time = format!("{} {}", parts[4], parts[5]);
    let last_down_cause = if parts.len() > 6 {
        parts[6..].join(" ")
    } else {
        String::new()
    };

    Some(StateFields {
        id,
        run_state,
        last_up_time,
        last_down_time,
        last_down_cause,
    })
}

fn parse_ont_line(line: &str) -> Option<OntFields> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 6 {
        return None;
    }

    let id = parts[0].parse().ok()?;
    let sn = parts[1].to_string();
    let ont_type = parts[2].to_string();
    let distance_m = parse_optional_u32(parts[3]);
    let (rx_power, tx_power) = parse_rx_tx_power(parts[4]);
    let description = if parts.len() > 5 {
        parts[5..].join(" ")
    } else {
        String::new()
    };

    Some(OntFields {
        id,
        sn,
        ont_type,
        distance_m,
        rx_power,
        tx_power,
        description,
    })
}

fn upsert_ont<'a>(
    ont_map: &'a mut HashMap<u32, OntInfoSummaryOnt>,
    ont_order: &mut Vec<u32>,
    id: u32,
) -> &'a mut OntInfoSummaryOnt {
    if !ont_map.contains_key(&id) {
        ont_map.insert(
            id,
            OntInfoSummaryOnt {
                id,
                ..OntInfoSummaryOnt::default()
            },
        );
        ont_order.push(id);
    }

    ont_map.get_mut(&id).expect("ONT entry must exist")
}

fn finalize_port(mut builder: PortBuilder) -> OntInfoSummaryPort {
    let mut port = builder.port;
    port.onts = builder
        .ont_order
        .into_iter()
        .filter_map(|id| builder.ont_map.remove(&id))
        .collect();
    port
}

fn parse_optional_u32(value: &str) -> Option<u32> {
    if value == "-" {
        None
    } else {
        value.parse().ok()
    }
}

fn parse_rx_tx_power(value: &str) -> (Option<f32>, Option<f32>) {
    if value == "-/-" {
        return (None, None);
    }

    let Some((rx, tx)) = value.split_once('/') else {
        return (None, None);
    };

    let rx = if rx == "-" { None } else { rx.parse().ok() };
    let tx = if tx == "-" { None } else { tx.parse().ok() };

    (rx, tx)
}
