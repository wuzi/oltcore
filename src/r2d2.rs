use std::fmt;

use r2d2::ManageConnection;

use crate::error::Error;
use crate::error::Result;
use crate::ssh::Connection;

#[derive(Clone)]
pub struct ConnectionManager {
    host: String,
    port: i32,
    user: String,
    pass: String,
}

impl ConnectionManager {
    #[must_use]
    pub fn new<S: Into<String>>(host: S, port: i32, user: S, pass: S) -> Self {
        Self {
            host: host.into(),
            port,
            user: user.into(),
            pass: pass.into(),
        }
    }
}

impl fmt::Debug for ConnectionManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConnectionManager<{}@{}>", self.user, self.host)
    }
}

impl ManageConnection for ConnectionManager {
    type Connection = Connection;
    type Error = Error;

    fn connect(&self) -> Result<Self::Connection> {
        Connection::connect(&self.host, self.port, &self.user, &self.pass)
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<()> {
        conn.ping()
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        conn.ensure_config().is_err()
    }
}
