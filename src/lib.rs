//! OLT Core Library
//!
//! A Rust library for interacting with Huawei MA5800 OLT (Optical Line Terminal) and similar devices.

pub mod error;
pub mod models;
pub mod parser;
pub mod ssh;

pub use error::{Error, Result};
pub use models::{OntAutofindEntry, OntInfo};
pub use parser::{parse_ont_autofind, parse_ont_info};
pub use ssh::Connection;
