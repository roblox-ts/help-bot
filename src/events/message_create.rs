use std::sync::Arc;
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::MessageCreate;
use twilight_util::builder::embed::EmbedBuilder;
use regex::Regex;

const PLAYGROUND_REGEX: &str = r"^\s*https://roblox-ts\.com/playground/#code/[A-Za-z0-9\-\+]+\s*$";

pub async fn handle(event: MessageCreate, http: Arc<Client>) {
    let content = &event.content;

    let re = Regex::new(PLAYGROUND_REGEX).unwrap();
    if !re.is_match(&content) {
        return;
    }

    let embeds = [
        EmbedBuilder::new()
            .title("Playground link")
            .url(content.trim())
            .description(format!("<@{}>", event.author.id))
            .color(0xE2_24_1A)
            .build(),
    ];

    let msg = http.create_message(event.channel_id);

    let msg = match msg.embeds(&embeds) {
        Ok(value) => value,
        Err(_) => return,
    };

    if !msg.await.is_ok() {
        return;
    }

    // only delete the original message if we successfully sent an embed
    http.delete_message(event.channel_id, event.id).await.ok();

    println!("Created embeded playground link for {}", &event.author.name);
}
