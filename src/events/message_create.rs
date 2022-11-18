use std::sync::Arc;

use twilight_http::Client;
use twilight_model::gateway::payload::incoming::MessageCreate;
use twilight_util::builder::embed::{EmbedBuilder};
use twilight_model::id::Id;

use crate::config::BotConfig;

use regex::Regex;

const PLAYGROUND_REGEX: &str = r"^\s*https:\/\/roblox-ts\.com\/playground\/#code\/[A-Za-z0-9\-\+]+\s*$";

pub async fn handle(event: MessageCreate, config: &BotConfig, http: Arc<Client>) {
	// test
	if event.author.id != Id::new(120766508702892033) {
		return;
	}

	println!("New message");

	let trimmed = event.content.trim();

	let re = Regex::new(PLAYGROUND_REGEX).unwrap();
	if !re.is_match(&trimmed) {
		return;
	}

	println!("Message matched");

	let embeds = [
		EmbedBuilder::new()
			.title("Playground link")
			.url(trimmed)
			.color(0xE2_24_1A)
			.build(),
	];

	let msg = http.create_message(event.channel_id);

	let msg = match msg.embeds(&embeds) {
		Ok(value) => value,
		Err(_) => return,
	};

	println!("Embed successful");

	if !msg.await.is_ok() {
		return;
	}

	println!("Safe to delete message?");

	// only delete the original message if we successfully sent an embed
	// http.delete_message(event.channel_id, event.id).await.ok();
}
