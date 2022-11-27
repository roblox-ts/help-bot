use anyhow::Result;
use std::{future::IntoFuture, sync::Arc};
use twilight_http::Client;
use twilight_model::channel::Channel;

use crate::config;

pub async fn process_thread(client: Arc<Client>, channel: Channel) -> Result<()> {
    if Some(*config::HELP_CHANNEL_ID) != channel.parent_id {
        return Ok(());
    }

    let mut applied_tags = channel.applied_tags.clone().unwrap_or_default();
    let has_solved = applied_tags.iter().any(|&v| v == *config::SOLVED_TAG_ID);
    let has_unsolved = applied_tags.iter().any(|&v| v == *config::UNSOLVED_TAG_ID);

    let thread_name = channel.name.clone().unwrap_or_default();

    if has_solved && has_unsolved {
        println!("Removing unsolved tag from existing thread: \"{thread_name}\"");
        applied_tags.retain(|&v| v != *config::UNSOLVED_TAG_ID);
    } else if !has_solved && !has_unsolved {
        println!("Adding unsolved tag to existing thread: \"{thread_name}\"");
        applied_tags.push(*config::UNSOLVED_TAG_ID);
    } else {
        return Ok(());
    }

    client
        .update_thread(channel.id)
        .applied_tags(Some(&applied_tags))
        .into_future()
        .await?;

    Ok(())
}
