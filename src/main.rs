use std::sync::Arc;

use anyhow::Result;
use config::BotConfig;
use futures::StreamExt;
use twilight_gateway::{Intents, Shard};
use twilight_http::Client;

use twilight_model::id::Id;

use crate::events::process_event;

mod config;
mod events;

fn get_env_id<T>(name: &str) -> Result<Id<T>> {
    let env_value = std::env::var(name)?;
    let parsed_value = env_value.parse::<u64>()?;
    return Ok(Id::new(parsed_value));
}

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TOKEN")?;
    let help_channel_id = get_env_id("HELP_CHANNEL_ID")?;
    let unsolved_tag_id = get_env_id("UNSOLVED_TAG_ID")?;
    let solved_tag_id = get_env_id("SOLVED_TAG_ID")?;

    let config = BotConfig {
        help_channel_id,
        unsolved_tag_id,
        solved_tag_id,
    };

    let client = Arc::new(Client::new(token.to_string()));

    let (shard, mut events) = Shard::new(
        token.to_string(),
        Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
    );

    println!("Starting..");

    shard.start().await?;

    println!("Connected!");

    while let Some(event) = events.next().await {
        tokio::spawn(process_event(event, config.clone(), client.clone()));
    }

    println!("Disconnected :(");

    Ok(())
}
