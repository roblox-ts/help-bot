use std::{future::IntoFuture, sync::Arc};

use twilight_http::Client;
use twilight_model::gateway::payload::incoming::ThreadUpdate;

use crate::config::BotConfig;

pub async fn handle(event: ThreadUpdate, config: &BotConfig, http: Arc<Client>) {
    if Some(config.help_channel_id) != event.parent_id {
        return;
    }

    let unsolved_tag_id = config.unsolved_tag_id;
    let solved_tag_id = config.solved_tag_id;

    let mut applied_tags = event.applied_tags.clone().unwrap_or_default();
    let has_solved = applied_tags.iter().any(|&v| v == solved_tag_id);
    let has_unsolved = applied_tags.iter().any(|&v| v == unsolved_tag_id);

    let thread_name = event.name.clone().unwrap_or_default();

    if has_solved && has_unsolved {
        println!("Removing unsolved tag from existing thread: \"{thread_name}\"");
        applied_tags.retain(|&v| v != unsolved_tag_id);
    } else if !has_solved && !has_unsolved {
        println!("Adding unsolved tag to existing thread: \"{thread_name}\"");
        applied_tags.push(unsolved_tag_id);
    } else {
        return;
    }

    http.update_thread(event.id)
        .applied_tags(Some(&applied_tags))
        .into_future()
        .await
        .ok();
}
