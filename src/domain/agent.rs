use serde::Deserialize;

use crate::domain::role::{Role, RoleName};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    uuid: String,
    display_name: String,
    role: Role,
}

impl Agent {
    pub fn new(uuid: String, name: String, role: Role) -> Self {
        Agent {
            uuid,
            display_name: name,
            role,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.uuid
    }

    pub fn get_name(&self) -> &str {
        &self.display_name
    }

    pub fn get_role_name(&self) -> &RoleName {
        self.role.get_role_type()
    }
}
