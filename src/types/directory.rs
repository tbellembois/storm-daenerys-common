use serde::{Deserialize, Serialize};

use super::acl::AclEntry;

#[derive(Deserialize, Serialize, Ord, Eq, PartialEq, PartialOrd, Clone, Debug)]
pub struct Directory {
    pub name: String,
    pub path: String,
    pub quota: Option<u64>,
    pub acls: Vec<AclEntry>,
    pub valid: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateDirectory {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RenameDirectory {
    pub name: String,
    pub new_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Quota {
    pub available_space: u64,
    pub total_space: u64,
}
