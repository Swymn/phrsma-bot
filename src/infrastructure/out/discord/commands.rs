use crate::{
    application::agent_service::pick_random_agent, infrastructure::valorant::client::fetch_agents,
};

#[derive(Default, Debug)]
pub struct Data {}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Select a random agent to play on Valorant.
#[poise::command(
    slash_command,
    description_localized("fr", "Choisi un agent aléatoire à jouer sur Valorant."),
    description_localized("en-US", "Select random agent to play on Valorant."),
    aliases("agent", "agents")
)]
pub async fn random_agent(ctx: Context<'_>) -> Result<(), Error> {
    let agent = {
        let agents = fetch_agents().await?;
        pick_random_agent(&agents)
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
