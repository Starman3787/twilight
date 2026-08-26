use crate::{
    id::{
        Id,
        marker::{EmojiMarker, GuildMarker, SoundMarker},
    },
    user::User,
};
use serde::{Deserialize, Serialize};

/// A sound that members of a guild can play in a voice channel.
///
/// Default sounds available to every guild have no [`guild_id`] and are not
/// tied to a creator.
///
/// [`guild_id`]: Self::guild_id
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SoundboardSound {
    /// Whether the sound can be used.
    ///
    /// May be false when the guild loses a server boost level that the sound
    /// slot depended on.
    pub available: bool,
    /// ID of the sound's custom emoji, if it has one.
    pub emoji_id: Option<Id<EmojiMarker>>,
    /// Unicode character of the sound's standard emoji, if it has one.
    pub emoji_name: Option<String>,
    /// ID of the guild the sound belongs to.
    ///
    /// Absent for Discord's default sounds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Name of the sound.
    pub name: String,
    /// ID of the sound.
    pub sound_id: Id<SoundMarker>,
    /// User who created the sound.
    ///
    /// Only present when the current user has the `CREATE_GUILD_EXPRESSIONS`
    /// or `MANAGE_GUILD_EXPRESSIONS` permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// Volume of the sound, from 0 to 1.
    pub volume: f64,
}

#[cfg(test)]
mod tests {
    use super::SoundboardSound;
    use crate::id::Id;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(
        SoundboardSound: available,
        emoji_id,
        emoji_name,
        guild_id,
        name,
        sound_id,
        user,
        volume
    );
    assert_impl_all!(
        SoundboardSound: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Serialize,
        Send,
        Sync
    );

    #[test]
    fn soundboard_sound() {
        let value = SoundboardSound {
            available: true,
            emoji_id: None,
            emoji_name: Some("🔊".to_owned()),
            guild_id: Some(Id::new(1)),
            name: "airhorn".to_owned(),
            sound_id: Id::new(2),
            user: None,
            volume: 0.5,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "SoundboardSound",
                    len: 7,
                },
                Token::Str("available"),
                Token::Bool(true),
                Token::Str("emoji_id"),
                Token::None,
                Token::Str("emoji_name"),
                Token::Some,
                Token::Str("🔊"),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("airhorn"),
                Token::Str("sound_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("volume"),
                Token::F64(0.5),
                Token::StructEnd,
            ],
        );
    }
}
