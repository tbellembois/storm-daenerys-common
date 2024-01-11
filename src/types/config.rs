use std::collections::HashMap;

use log::debug;
use serde::{Deserialize, Serialize};

use super::directory::Quota;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub admin: String,
    pub current_admin_restriction: Option<String>,
    pub users_dsi_api_group_prefix: String,
    pub root_groups: Option<Vec<String>>,
    pub quota: Quota,
}

// Returns the restriction for the connected user.
pub fn get_current_admin_restriction(
    x_remote_user: String,
    admin_restriction: Option<HashMap<String, String>>,
) -> Option<String> {
    debug!("get_admin_restriction");

    // Check if there is a restrict view configuration for the connected user.
    let mut maybe_restrict_group = None;
    if let Some(ref view_restrict) = admin_restriction {
        maybe_restrict_group = view_restrict.get_key_value(&x_remote_user);
    }

    // Extract the group.
    let mut maybe_group: Option<String> = None;
    if let Some(user_group) = maybe_restrict_group {
        let (_, grp) = user_group;
        maybe_group = Some(grp.to_string())
    }

    debug!("group: {:?}", maybe_group);

    maybe_group
}
