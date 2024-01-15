use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct User {
    pub id: String,
    pub display: String,
}
