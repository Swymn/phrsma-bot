mod embed;
mod parameter;

use crate::{
    application::agent_service::pick_random_agent,
    infrastructure::{
        discord::{
            commands::random_agents::parameter::RoleParameter,
            context::{Context, Error},
        },
        valorant::client::fetch_agents,
    },
};

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
            let reply = embed::create_embed(&agent);
            ctx.send(reply).await?;
        }
        None => {
            ctx.say("Je n'arrive pas à récupérer la liste des agents, réessaye plus tard.")
                .await?;
        }
    };

    Ok(())
}
