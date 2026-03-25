use crate::{
    application::agent_service::pick_random_agent, domain::role::RoleName,
    infrastructure::valorant::client::fetch_agents,
};

#[derive(Default, Debug)]
pub struct Data {}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

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

/// Select a random agent to play on Valorant.
#[poise::command(
    slash_command,
    description_localized("fr", "Choisi un agent aléatoire à jouer sur Valorant."),
    description_localized("en-US", "Select random agent to play on Valorant."),
    aliases("agent", "agents")
)]
pub async fn random_agent(
    ctx: Context<'_>,
    #[description = "Un role en particulier ?"] role: Option<RoleParameter>,
) -> Result<(), Error> {
    let agent = {
        let agents = fetch_agents().await?;
        pick_random_agent(&agents, role.map(|role| role.into()))
    };

    match agent {
        Some(agent) => {
            ctx.say(format!("Tu dois jouer {}.", agent.get_name()))
                .await?
        }
        None => {
            ctx.say("Je n'arrive pas à récupérer la liste des agents, réessaye plus tard.")
                .await?
        }
    };

    Ok(())
}
