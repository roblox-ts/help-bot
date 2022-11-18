mod message_create;
mod thread_create;
mod thread_update;

use std::sync::Arc;

use twilight_gateway::Event;
use twilight_http::Client;

use crate::config::BotConfig;

pub async fn process_event(event: Event, config: BotConfig, http: Arc<Client>) {
    match event {
        Event::MessageCreate(message) => message_create::handle(*message, &config, http).await,
        Event::ThreadCreate(thread) => thread_create::handle(*thread, &config, http).await,
        Event::ThreadUpdate(thread) => thread_update::handle(*thread, &config, http).await,
        _ => {}
    }
}
