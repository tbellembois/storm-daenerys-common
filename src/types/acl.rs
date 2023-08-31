use std::fmt;

use posix_acl::ACLEntry;
use serde::{Deserialize, Serialize};
use users::get_group_by_gid;
use users::get_group_by_name;
use users::get_user_by_name;
use users::get_user_by_uid;

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

impl AclEntry {
    pub fn from_posix_acl_entry(entry: &ACLEntry) -> Self {
        let qualifier = match entry.qual {
            posix_acl::Qualifier::Undefined => Qualifier::Undefined,
            posix_acl::Qualifier::UserObj => Qualifier::UserObj,
            posix_acl::Qualifier::GroupObj => Qualifier::GroupObj,
            posix_acl::Qualifier::Other => Qualifier::Other,
            posix_acl::Qualifier::User(u) => Qualifier::User(u),
            posix_acl::Qualifier::Group(g) => Qualifier::Group(g),
            posix_acl::Qualifier::Mask => Qualifier::Mask,
        };

        let perm = entry.perm;

        // FIXME
        let qualifier_cn: Option<String> = match qualifier {
            Qualifier::Undefined => None,
            Qualifier::UserObj => None,
            Qualifier::GroupObj => None,
            Qualifier::Other => None,
            Qualifier::User(u) => match get_user_by_uid(u) {
                Some(user) => Some(user.name().to_string_lossy().to_string()),
                None => Some("can not find user name".to_string()), // FIXME
            },
            Qualifier::Group(g) => match get_group_by_gid(g) {
                Some(group) => Some(group.name().to_string_lossy().to_string()),
                None => Some("can not find group name".to_string()), // FIXME
            },
            Qualifier::Mask => None,
        };

        AclEntry {
            qualifier,
            perm,
            qualifier_cn,
        }
    }

    pub fn to_posix_acl_entry(&self) -> ACLEntry {
        let qual = match self.qualifier {
            Qualifier::Undefined => posix_acl::Qualifier::Undefined,
            Qualifier::UserObj => posix_acl::Qualifier::UserObj,
            Qualifier::GroupObj => posix_acl::Qualifier::GroupObj,
            Qualifier::Other => posix_acl::Qualifier::Other,
            Qualifier::User(_) => match get_user_by_name(&self.qualifier_cn.as_ref().unwrap()) {
                Some(user) => posix_acl::Qualifier::User(user.uid()),
                None => posix_acl::Qualifier::User(65535), // FIXME
            },
            Qualifier::Group(_) => match get_group_by_name(&self.qualifier_cn.as_ref().unwrap()) {
                Some(group) => posix_acl::Qualifier::Group(group.gid()),
                None => posix_acl::Qualifier::Group(65535), // FIXME
            },
            Qualifier::Mask => posix_acl::Qualifier::Mask,
        };

        let perm = self.perm;

        posix_acl::ACLEntry { qual, perm }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SetAcl {
    pub name: String,
    pub acls: Vec<AclEntry>,
}
