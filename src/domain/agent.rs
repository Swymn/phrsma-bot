use serde::Deserialize;

use crate::domain::{
    ability::Ability,
    role::{Role, RoleName},
};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    uuid: String,
    display_name: String,
    display_icon: String,
    abilities: Vec<Ability>,
    role: Role,
}

impl Agent {
    pub fn new(
        uuid: String,
        name: String,
        icon: String,
        role: Role,
        abilities: Vec<Ability>,
    ) -> Self {
        Agent {
            uuid,
            display_name: name,
            display_icon: icon,
            abilities,
            role,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.uuid
    }

    pub fn get_icon(&self) -> &str {
        &self.display_icon
    }

    pub fn get_name(&self) -> &str {
        &self.display_name
    }

    pub fn get_role_name(&self) -> &RoleName {
        self.role.get_role_type()
    }

    pub fn get_abilities(&self) -> &[Ability] {
        &self.abilities
    }
}
