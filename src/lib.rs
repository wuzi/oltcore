//! OLT Core Library
//!
//! A Rust library for interacting with Huawei MA5800 OLT (Optical Line Terminal) and similar devices.

pub mod alarm;
pub mod error;
pub mod models;
pub mod ont_info_summary;
pub mod parser;
pub mod r2d2;
pub mod ssh;

pub use alarm::{parse_active_alarms_detail, parse_active_alarms_list, ActiveAlarm, ActiveAlarms};
pub use error::{Error, Result};
pub use models::{Fsp, OntAutofindEntry, OntInfo, OpticalInfo, ServicePort};
pub use ont_info_summary::{
    parse_ont_info_summary, OntInfoSummary, OntInfoSummaryOnt, OntInfoSummaryPort,
};
pub use parser::{parse_ont_autofind, parse_ont_info, parse_optical_info, parse_service_ports};
pub use r2d2::ConnectionManager;
pub use ssh::Connection;
