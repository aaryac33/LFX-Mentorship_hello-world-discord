use serenity::{
    async_trait,
    model::gateway::Ready,
    prelude::*,
};
use anyhow::Result;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let discord_token = std::env::var("DISCORD_TOKEN")?;

    let mut client = Client::new(&discord_token)
        .event_handler(Handler)
        .await?;

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }

    Ok(())
}
