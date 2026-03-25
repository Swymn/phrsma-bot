use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum RoleName {
    Initiator,
    Duelist,
    Controller,
    Sentinel,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    uuid: String,
    display_name: RoleName,
}

impl Role {
    pub fn new(uuid: String, name: RoleName) -> Self {
        Role {
            uuid,
            display_name: name,
        }
    }

    pub fn get_role_type(&self) -> &RoleName {
        &self.display_name
    }
}
