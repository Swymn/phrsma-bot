#[derive(Default, Debug)]
pub struct Data {}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn random_agent(ctx: Context<'_>) -> Result<(), Error> {
    todo!();
}
