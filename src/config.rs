use anyhow::Context;
use lazy_static::lazy_static;
use twilight_model::id::{
    marker::{ChannelMarker, TagMarker},
    Id,
};

fn get_env(name: &str) -> String {
    std::env::var(name)
        .context(format!(
            "Unable to find environment variable named \"{name}\"!"
        ))
        .unwrap()
}

fn get_env_id<T>(name: &str) -> Id<T> {
    Id::new(get_env(name).parse::<u64>().unwrap())
}

lazy_static! {
    pub static ref TOKEN: String = get_env("TOKEN");
    pub static ref HELP_CHANNEL_ID: Id<ChannelMarker> = get_env_id("HELP_CHANNEL_ID");
    pub static ref UNSOLVED_TAG_ID: Id<TagMarker> = get_env_id("UNSOLVED_TAG_ID");
    pub static ref SOLVED_TAG_ID: Id<TagMarker> = get_env_id("SOLVED_TAG_ID");
}
