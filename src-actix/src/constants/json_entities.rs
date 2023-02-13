use serde::{Serialize, Deserialize};

pub struct ApplicationJsonEntity {
    uid: String,
    name: String,
    application_type: String,
}

impl ApplicationJsonEntity {
    pub fn UID() -> &'static str {
        return "uid";
    }
    pub fn NAME() -> &'static str {
        return "name";
    }
    pub fn TYPE() -> &'static str {
        return "type";
    }
}

#[derive(Serialize, Deserialize)]
pub struct OrganizationJsonEntity {
    #[serde(default)]
    name: String,

    #[serde(default)]
    uid: String,

    #[serde(rename = "type", default)]
    application_type: String,

}