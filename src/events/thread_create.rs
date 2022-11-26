use anyhow::Result;
use std::{future::IntoFuture, sync::Arc};
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::ThreadCreate;

use crate::config::BotConfig;

pub async fn handle(client: Arc<Client>, config: &BotConfig, event: ThreadCreate) -> Result<()> {
    if Some(config.help_channel_id) != event.parent_id {
        return Ok(());
    }

    let mut applied_tags = event.applied_tags.clone().unwrap_or_default();
    let has_unsolved = applied_tags.iter().any(|&v| v == config.unsolved_tag_id);

    if !has_unsolved {
        applied_tags.push(config.unsolved_tag_id);
    }

    let thread_name = event.name.clone().unwrap_or_default();
    println!("Adding unsolved tag to new thread: \"{thread_name}\"");

    client
        .update_thread(event.id)
        .applied_tags(Some(&applied_tags))
        .into_future()
        .await?;

    Ok(())
}
