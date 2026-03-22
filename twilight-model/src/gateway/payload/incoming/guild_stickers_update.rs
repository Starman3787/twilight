use crate::{
    channel::message::Sticker,
    id::{Id, marker::GuildMarker},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuildStickersUpdate {
    pub guild_id: Id<GuildMarker>,
    pub stickers: Vec<Sticker>,
}

#[cfg(test)]
mod tests {
    use super::GuildStickersUpdate;
    use crate::{
        channel::message::{
            Sticker,
            sticker::{StickerFormatType, StickerType},
        },
        id::Id,
    };
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(GuildStickersUpdate: guild_id, stickers);

    assert_impl_all!(
        GuildStickersUpdate: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );

    /// Test that `GuildStickersUpdate` deserializes and serializes correctly
    /// using the full [`Sticker`] model with all fields.
    #[test]
    fn guild_stickers_update_full_sticker() {
        let value = GuildStickersUpdate {
            guild_id: Id::new(1),
            stickers: Vec::from([Sticker {
                available: true,
                description: Some("a cool sticker".into()),
                format_type: StickerFormatType::Png,
                guild_id: Some(Id::new(1)),
                id: Id::new(2),
                kind: StickerType::Guild,
                name: "cool".into(),
                pack_id: None,
                sort_value: None,
                tags: "cool,neat".into(),
                user: None,
            }]),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GuildStickersUpdate",
                    len: 2,
                },
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("stickers"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Sticker",
                    len: 8,
                },
                Token::Str("available"),
                Token::Bool(true),
                Token::Str("description"),
                Token::Some,
                Token::Str("a cool sticker"),
                Token::Str("format_type"),
                Token::U8(StickerFormatType::Png.into()),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(2),
                Token::Str("name"),
                Token::Str("cool"),
                Token::Str("tags"),
                Token::Str("cool,neat"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}

