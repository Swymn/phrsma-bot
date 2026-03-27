use poise::{
    CreateReply,
    serenity_prelude::{Colour, CreateEmbed, CreateEmbedFooter, Timestamp},
};

use crate::{
    application::agent_service::pick_random_agent,
    domain::{agent::Agent, role::RoleName},
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

impl RoleName {
    fn get_color(&self) -> Colour {
        match self {
            RoleName::Duelist => Colour::ROSEWATER,
            RoleName::Controller => Colour::DARK_GOLD,
            RoleName::Initiator => Colour::DARK_BLUE,
            RoleName::Sentinel => Colour::DARK_TEAL,
        }
    }
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

fn create_embed(agent: &Agent) -> CreateReply {
    let fields = agent
        .get_abilities()
        .iter()
        .map(|ability| (ability.get_name(), ability.get_slot(), true));

    let embed = CreateEmbed::new()
        .title(format!("Tu dois jouer {}", agent.get_name()))
        .color(agent.get_role_name().get_color())
        .image(agent.get_icon())
        .fields(fields)
        .footer(CreateEmbedFooter::new("Amuse toi bien !"))
        .timestamp(Timestamp::now());

    CreateReply::default().embed(embed)
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
        let agents = fetch_agents().await;
        match agents {
            Ok(agents) => pick_random_agent(&agents, role.map(|role| role.into())),
            Err(e) => {
                eprintln!("Erreur lors de la recuperation des agents: {e}");
                None
            }
        }
    };

    match agent {
        Some(agent) => {
            let reply = create_embed(&agent);
            ctx.send(reply).await?;
        }
        None => {
            ctx.say("Je n'arrive pas à récupérer la liste des agents, réessaye plus tard.")
                .await?;
        }
    };

    Ok(())
}
