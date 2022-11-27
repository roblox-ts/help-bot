use crate::jobs::process_thread::process_thread;
use anyhow::Result;
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::ThreadUpdate;

pub async fn handle(client: Arc<Client>, event: ThreadUpdate) -> Result<()> {
    process_thread(client, event.0).await
}
