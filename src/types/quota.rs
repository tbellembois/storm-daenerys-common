use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SetQuota {
    pub name: String,
    pub quota: u64,
}

#[derive(PartialEq)]
pub enum QuotaUnit {
    Megabyte,
    Gigabyte,
    Terabyte,
}

impl fmt::Display for QuotaUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuotaUnit::Megabyte => write!(f, "MiB"),
            QuotaUnit::Gigabyte => write!(f, "GiB"),
            QuotaUnit::Terabyte => write!(f, "TiB"),
        }
    }
}
