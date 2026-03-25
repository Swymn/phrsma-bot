use dotenv::dotenv;

use crate::infrastructure::discord::client::build_client;

mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    // Initiliaze env variables with dotenv
    dotenv().ok();

    // Initialize bot
    let client = build_client().await;

    // Run the bot
    match client {
        Ok(mut client) => {
            client.start().await.unwrap();
        }

        Err(e) => {
            eprintln!("Failed to start the bot: {e}");
        }
    };
}
