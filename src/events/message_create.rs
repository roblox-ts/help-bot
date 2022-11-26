use anyhow::Result;
use regex::Regex;
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::MessageCreate;
use twilight_util::builder::embed::EmbedBuilder;

const PLAYGROUND_REGEX: &str = r"^\s*https://roblox-ts\.com/playground/#code/[A-Za-z0-9\-\+]+\s*$";

pub async fn handle(client: Arc<Client>, event: MessageCreate) -> Result<()> {
    let content = &event.content;

    let re = Regex::new(PLAYGROUND_REGEX).unwrap();
    if !re.is_match(content) {
        return Ok(());
    }

    let embeds = [EmbedBuilder::new()
        .title("Playground link")
        .url(content.trim())
        .description(format!("Posted by <@{}>", event.author.id))
        .color(0xE2_24_1A)
        .build()];

    client
        .create_message(event.channel_id)
        .embeds(&embeds)?
        .await?;

    // only delete the original message if we successfully sent an embed
    client.delete_message(event.channel_id, event.id).await?;

    println!(
        "Created embedded playground link for {}",
        &event.author.name
    );

    Ok(())
}
