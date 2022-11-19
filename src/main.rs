use std::sync::Arc;

use anyhow::{Result, Context};
use config::BotConfig;
use dotenv::dotenv;
use futures::StreamExt;
use twilight_gateway::{Intents, Shard};
use twilight_http::Client;

use twilight_model::id::Id;

use crate::events::process_event;

mod config;
mod events;
mod server;

fn get_env(name: &str) -> Result<String> {
    std::env::var(name).context(format!("Unable to find environment variable named \"{name}\"!"))
}

fn get_env_id<T>(name: &str) -> Result<Id<T>> {
    Ok(Id::new(get_env(name)?.parse::<u64>()?))
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let token = get_env("TOKEN")?;

    let config = BotConfig {
        help_channel_id: get_env_id("HELP_CHANNEL_ID")?,
        unsolved_tag_id: get_env_id("UNSOLVED_TAG_ID")?,
        solved_tag_id: get_env_id("SOLVED_TAG_ID")?,
    };

    tokio::spawn(server::start_server());

    let client = Arc::new(Client::new(token.to_string()));

    let (shard, mut events) = Shard::new(
        token.to_string(),
        Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
    );

    println!("Starting shard..");

    shard.start().await?;

    println!("Shard connected!");

    while let Some(event) = events.next().await {
        tokio::spawn(process_event(event, config, client.clone()));
    }

    println!("Shard disconnected :(");

    Ok(())
}
