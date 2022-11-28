use crate::config::CONFIG;
use anyhow::Result;
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::channel::Channel;

pub async fn process_thread(client: Arc<Client>, channel: Channel) -> Result<()> {
    if Some(CONFIG.help_channel_id) != channel.parent_id {
        return Ok(());
    }

    let mut applied_tags = channel.applied_tags.clone().unwrap_or_default();
    let has_solved = applied_tags.iter().any(|&v| v == CONFIG.solved_tag_id);
    let has_unsolved = applied_tags.iter().any(|&v| v == CONFIG.unsolved_tag_id);

    let thread_name = channel.name.clone().unwrap_or_default();

    if has_solved && has_unsolved {
        println!("Removing unsolved tag from existing thread: \"{thread_name}\"");
        applied_tags.retain(|&v| v != CONFIG.unsolved_tag_id);
    } else if !has_solved && !has_unsolved {
        println!("Adding unsolved tag to existing thread: \"{thread_name}\"");
        applied_tags.push(CONFIG.unsolved_tag_id);
    } else {
        return Ok(());
    }

    client
        .update_thread(channel.id)
        .applied_tags(Some(&applied_tags))
        .await?;

    Ok(())
}
