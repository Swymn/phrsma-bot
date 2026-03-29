use crate::domain::role::RoleName;

#[derive(poise::ChoiceParameter)]
pub enum RoleParameter {
    Duelist,
    Controller,
    Sentinel,
    Initiator,
}

impl From<RoleParameter> for RoleName {
    fn from(value: RoleParameter) -> Self {
        match value {
            RoleParameter::Initiator => RoleName::Initiator,
            RoleParameter::Duelist => RoleName::Duelist,
            RoleParameter::Controller => RoleName::Controller,
            RoleParameter::Sentinel => RoleName::Sentinel,
        }
    }
}
