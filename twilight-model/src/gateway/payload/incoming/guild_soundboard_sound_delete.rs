use crate::id::{
    Id,
    marker::{GuildMarker, SoundMarker},
};
use serde::{Deserialize, Serialize};

/// A guild soundboard sound has been deleted.
///
/// Discord sends only the two IDs, so anything the deletion should be reported
/// against has to have been recorded beforehand.
///
/// Requires [`Intents::GUILD_EXPRESSIONS`].
///
/// [`Intents::GUILD_EXPRESSIONS`]: crate::gateway::Intents::GUILD_EXPRESSIONS
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildSoundboardSoundDelete {
    /// ID of the guild the sound belonged to.
    pub guild_id: Id<GuildMarker>,
    /// ID of the deleted sound.
    pub sound_id: Id<SoundMarker>,
}
