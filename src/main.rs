use crate::events::handle_event;
use anyhow::Result;
use dotenv::dotenv;
use futures::StreamExt;
use std::sync::Arc;
use twilight_gateway::{Intents, Shard};
use twilight_http::Client;

mod config;
mod events;
mod jobs;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    lazy_static::initialize(&config::TOKEN);
    lazy_static::initialize(&config::HELP_CHANNEL_ID);
    lazy_static::initialize(&config::UNSOLVED_TAG_ID);
    lazy_static::initialize(&config::SOLVED_TAG_ID);

    println!("Starting server..");
    tokio::spawn(server::start_server());

    let client = Arc::new(Client::new(config::TOKEN.to_string()));

    let (shard, mut events) = Shard::new(
        config::TOKEN.to_string(),
        Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
    );

    println!("Starting shard..");

    shard.start().await?;

    println!("Shard connected!");

    while let Some(event) = events.next().await {
        tokio::spawn(handle_event(client.clone(), event));
    }

    println!("Shard disconnected :(");

    Ok(())
}
