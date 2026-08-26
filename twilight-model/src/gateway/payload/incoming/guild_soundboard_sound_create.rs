use crate::guild::SoundboardSound;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// A guild soundboard sound has been created.
///
/// Requires [`Intents::GUILD_EXPRESSIONS`].
///
/// [`Intents::GUILD_EXPRESSIONS`]: crate::gateway::Intents::GUILD_EXPRESSIONS
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GuildSoundboardSoundCreate(pub SoundboardSound);

impl Deref for GuildSoundboardSoundCreate {
    type Target = SoundboardSound;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GuildSoundboardSoundCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
