use std::env;

use poise::serenity_prelude as serenity;

use crate::interfaces::discord::commands;

mod application;
mod domain;
mod infrastructure;
mod interfaces;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Missing env variable DISCORD_TOKEN");

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::random_agent()],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                println!("Connecté en tant que {}", ready.user.name);
                Ok(commands::Data::default())
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
