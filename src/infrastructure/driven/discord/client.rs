use poise::serenity_prelude::{self as serenity, Client, Error};
use std::env;

use crate::infrastructure::discord;

pub async fn build_client() -> Result<Client, Error> {
    let token = env::var("DISCORD_TOKEN").expect("Missing env variable DISCORD_TOKEN");

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![discord::commands::random_agent()],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                println!("Connecté en tant que {}", ready.user.name);
                Ok(discord::context::Data::default())
            })
        })
        .build();

    serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
}
