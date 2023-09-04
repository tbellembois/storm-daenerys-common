use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Group {
    pub cn: String,
    pub description: String,
    pub member: Option<Vec<String>>, // "uid=thbellem,ou=people,dc=uca,dc=fr"
    pub owner: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateGroup {
    pub cn: String,
    pub description: String,
    pub owner: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddDelUserToGroup {
    pub group_cn: String,
    pub user_cn: String,
}
