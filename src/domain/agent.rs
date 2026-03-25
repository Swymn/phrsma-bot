use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    uuid: String,
    display_name: String,
}

impl Agent {
    pub fn new(uuid: String, name: String) -> Self {
        Agent {
            uuid,
            display_name: name,
        }
    }
    pub fn get_id(&self) -> &str {
        &self.uuid
    }

    pub fn get_name(&self) -> &str {
        &self.display_name
    }
}
