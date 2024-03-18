use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SetQuota {
    pub name: String,
    pub quota: u32,
}
