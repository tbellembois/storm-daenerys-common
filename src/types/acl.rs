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

#[derive(Deserialize, Serialize)]
pub struct SetAcl {
    pub name: String,
    pub acls: Vec<AclEntry>,
}
