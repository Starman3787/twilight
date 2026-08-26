use crate::{
    guild::SoundboardSound,
    id::{Id, marker::GuildMarker},
};
use serde::{Deserialize, Serialize};

/// Several of a guild's soundboard sounds have been updated at once.
///
/// Requires [`Intents::GUILD_EXPRESSIONS`].
///
/// [`Intents::GUILD_EXPRESSIONS`]: crate::gateway::Intents::GUILD_EXPRESSIONS
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GuildSoundboardSoundsUpdate {
    /// ID of the guild whose sounds were updated.
    pub guild_id: Id<GuildMarker>,
    /// The guild's soundboard sounds.
    pub soundboard_sounds: Vec<SoundboardSound>,
}
