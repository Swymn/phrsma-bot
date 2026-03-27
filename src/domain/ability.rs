use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    slot: String,
    display_name: String,
    display_icon: Option<String>,
}

impl Ability {
    pub fn new(slot: String, name: String, icon: Option<String>) -> Self {
        Ability {
            slot,
            display_name: name,
            display_icon: icon,
        }
    }

    pub fn get_slot(&self) -> &str {
        &self.slot
    }

    pub fn get_name(&self) -> &str {
        &self.display_name
    }

    pub fn get_icon(&self) -> &Option<String> {
        &self.display_icon
    }
}
