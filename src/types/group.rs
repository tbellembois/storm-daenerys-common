use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Group {
    pub cn: String,
    pub description: String,
    pub member: Option<Vec<String>>, // "uid=thbellem,ou=people,dc=uca,dc=fr"
    pub owner: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AddDelUserToGroup {
    #[serde(rename(serialize = "group", deserialize = "group"))]
    pub group_cn: String,
    #[serde(rename(serialize = "username", deserialize = "username"))]
    pub user_cn: String,
    #[serde(serialize_with = "emit_directory")]
    pub directory: String,
    #[serde(serialize_with = "emit_manager")]
    pub manager: bool,
}

fn emit_directory<S: Serializer>(_: &String, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str("universite|ldap_uca")
}
fn emit_manager<S: Serializer>(_: &bool, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_bool(false)
}
