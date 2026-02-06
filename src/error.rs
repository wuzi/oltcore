use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SshError(String),
    IoError(std::io::Error),
    ParseError(String),
    AuthenticationFailed,
    ConnectionFailed(String),
    DeviceError(String),
    NotFound,
    InvalidSerialNumber,
    InvalidMacAddress,
    InvalidContext(String),
    CommandFailed(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SshError(msg) => write!(f, "SSH error: {msg}"),
            Self::IoError(err) => write!(f, "IO error: {err}"),
            Self::ParseError(msg) => write!(f, "Parse error: {msg}"),
            Self::AuthenticationFailed => write!(f, "SSH authentication failed"),
            Self::ConnectionFailed(msg) => write!(f, "Connection failed: {msg}"),
            Self::DeviceError(msg) => write!(f, "Device error: {msg}"),
            Self::NotFound => write!(f, "ONT not found"),
            Self::InvalidSerialNumber => write!(f, "Invalid serial number"),
            Self::InvalidMacAddress => write!(f, "Invalid MAC address"),
            Self::InvalidContext(msg) => write!(f, "Invalid context: {msg}"),
            Self::CommandFailed(msg) => write!(f, "Command failed: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<ssh2::Error> for Error {
    fn from(err: ssh2::Error) -> Self {
        Self::SshError(err.to_string())
    }
}
