use crate::config::BotConfig;
use anyhow::Result;
use std::{future::IntoFuture, sync::Arc};
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::ThreadUpdate;

pub async fn handle(client: Arc<Client>, config: &BotConfig, event: ThreadUpdate) -> Result<()> {
    if Some(config.help_channel_id) != event.parent_id {
        return Ok(());
    }

    let mut applied_tags = event.applied_tags.clone().unwrap_or_default();
    let has_solved = applied_tags.iter().any(|&v| v == config.solved_tag_id);
    let has_unsolved = applied_tags.iter().any(|&v| v == config.unsolved_tag_id);

    let thread_name = event.name.clone().unwrap_or_default();

    if has_solved && has_unsolved {
        println!("Removing unsolved tag from existing thread: \"{thread_name}\"");
        applied_tags.retain(|&v| v != config.unsolved_tag_id);
    } else if !has_solved && !has_unsolved {
        println!("Adding unsolved tag to existing thread: \"{thread_name}\"");
        applied_tags.push(config.unsolved_tag_id);
    } else {
        return Ok(());
    }

    client
        .update_thread(event.id)
        .applied_tags(Some(&applied_tags))
        .into_future()
        .await?;

    Ok(())
}
