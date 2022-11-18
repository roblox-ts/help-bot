use twilight_model::id::{
    marker::{ChannelMarker, TagMarker},
    Id,
};

#[derive(Copy, Clone)]
pub struct BotConfig {
    pub help_channel_id: Id<ChannelMarker>,
    pub unsolved_tag_id: Id<TagMarker>,
    pub solved_tag_id: Id<TagMarker>,
}
