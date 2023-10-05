use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Ord, Eq, PartialEq, PartialOrd, Clone, Debug)]
pub enum Qualifier {
    /// Unrecognized/corrupt entries
    Undefined,
    /// Permissions for owner of the file
    UserObj,
    /// Permissions for owning group of the file
    GroupObj,
    /// Permissions for everyone else not covered by the ACL
    Other,
    /// Permissions for user with UID `u32` value
    User(u32),
    /// Permissions for group with GID `u32` value
    Group(u32),
    /// Auto-generated entry
    Mask,
}

impl fmt::Display for Qualifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Qualifier::Undefined => write!(f, "Undefined"),
            Qualifier::UserObj => write!(f, "UserObj"),
            Qualifier::GroupObj => write!(f, "GroupObj"),
            Qualifier::Other => write!(f, "Other"),
            Qualifier::User(u) => write!(f, "User:{}", u),
            Qualifier::Group(g) => write!(f, "Group:{}", g),
            Qualifier::Mask => write!(f, "Mask"),
        }
    }
}

#[derive(Serialize, Deserialize, Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct AclEntry {
    pub qualifier: Qualifier,         // the subject of a permission grant
    pub qualifier_cn: Option<String>, // optionnal user or group name when qualifier is User(u32) or Group(u32)
    pub perm: u32,
}

impl fmt::Debug for AclEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}: {:?} {}",
            self.qualifier, self.qualifier_cn, self.perm
        )
    }
}

impl fmt::Display for AclEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let qualifier: String = match self.qualifier {
            Qualifier::Undefined => "".to_string(),
            Qualifier::UserObj => "owner_user".to_string(),
            Qualifier::GroupObj => "owner_group".to_string(),
            Qualifier::Other => "other".to_string(),
            Qualifier::User(_) => "user".to_string(),
            Qualifier::Group(_) => "group".to_string(),
            Qualifier::Mask => "mask".to_string(),
        };

        let qualifier_cn: String = match &self.qualifier_cn {
            Some(qualifier_cn) => qualifier_cn.to_string(),
            None => "".to_string(),
        };

        let perm: String = match self.perm {
            0 => "---".to_string(),
            1 => "--x".to_string(),
            2 => "-w-".to_string(),
            3 => "-wx".to_string(),
            4 => "r--".to_string(),
            5 => "r-x".to_string(),
            6 => "rw-".to_string(),
            7 => "rwx".to_string(),
            _ => "_".to_string(),
        };

        write!(f, "{} {}: {}", qualifier, qualifier_cn, perm)
    }
}

#[derive(Deserialize, Serialize)]
pub struct SetAcl {
    pub name: String,
    pub acls: Vec<AclEntry>,
}
