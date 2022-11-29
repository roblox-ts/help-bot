use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::MessageCreate;
use twilight_util::builder::embed::EmbedBuilder;

lazy_static! {
    static ref PLAYGROUND_REGEX: Regex =
        Regex::new(r"^\s*https://roblox-ts\.com/playground/#code/[A-Za-z0-9\-\+]+\s*$").unwrap();
}

pub async fn handle(client: Arc<Client>, event: MessageCreate) -> Result<()> {
    let content = &event.content;

    if !PLAYGROUND_REGEX.is_match(content) {
        return Ok(());
    }

    let embeds = [EmbedBuilder::new()
        .title("Playground link")
        .url(content.trim())
        .description(format!("Posted by <@{}>", event.author.id))
        .color(0xE2_24_1A)
        .build()];

    let mut message = client
        .create_message(event.channel_id)
        .embeds(&embeds)?;

    if let Some(referenced_message) = &event.referenced_message {
        message = message.reply(referenced_message.id);
    }

    message.await?;

    // only delete the original message if we successfully sent an embed
    client.delete_message(event.channel_id, event.id).await?;

    println!(
        "Created embedded playground link for {}",
        &event.author.name
    );

    Ok(())
}
