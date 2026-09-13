//! Deserialise an optional ID where Discord may send a literal `0`.
//!
//! [`Id`] is backed by `NonZeroU64`, so `"target_id": 0` — which Discord sends
//! on `GUILD_AUDIT_LOG_ENTRY_CREATE` for invite deletes (action type 42) — and
//! `"application_id": 0` inside a presence activity fail to deserialise and
//! take the whole gateway frame with them. A zero ID carries no information,
//! so it is read as `None`.

use crate::id::Id;
use serde::{
    Deserializer,
    de::{Error as DeError, Unexpected, Visitor},
};
use std::{fmt, marker::PhantomData};

struct ZeroIdVisitor<T>(PhantomData<T>);

impl<'de, T> Visitor<'de> for ZeroIdVisitor<T> {
    type Value = Option<Id<T>>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("an optional ID as an integer or string, where 0 means none")
    }

    fn visit_none<E: DeError>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_unit<E: DeError>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(self)
    }

    fn visit_newtype_struct<D: Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(self)
    }

    fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
        Ok(Id::new_checked(value))
    }

    fn visit_i64<E: DeError>(self, value: i64) -> Result<Self::Value, E> {
        u64::try_from(value)
            .map(Id::new_checked)
            .map_err(|_| E::invalid_value(Unexpected::Signed(value), &self))
    }

    fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
        value
            .parse::<u64>()
            .map(Id::new_checked)
            .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))
    }
}

pub(crate) fn zero_id_as_none<'de, D, T>(deserializer: D) -> Result<Option<Id<T>>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(ZeroIdVisitor(PhantomData))
}

#[cfg(test)]
mod tests {
    use super::zero_id_as_none;
    use crate::id::{Id, marker::GenericMarker};
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Holder {
        #[serde(default, deserialize_with = "zero_id_as_none")]
        id: Option<Id<GenericMarker>>,
    }

    #[test]
    fn zero_is_none() {
        let holder: Holder = serde_json::from_str(r#"{"id":0}"#).unwrap();
        assert_eq!(holder.id, None);
        let holder: Holder = serde_json::from_str(r#"{"id":"0"}"#).unwrap();
        assert_eq!(holder.id, None);
    }

    #[test]
    fn null_and_missing_are_none() {
        let holder: Holder = serde_json::from_str(r#"{"id":null}"#).unwrap();
        assert_eq!(holder.id, None);
        let holder: Holder = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(holder.id, None);
    }

    #[test]
    fn nonzero_is_some() {
        let holder: Holder = serde_json::from_str(r#"{"id":"123"}"#).unwrap();
        assert_eq!(holder.id, Some(Id::new(123)));
        let holder: Holder = serde_json::from_str(r#"{"id":123}"#).unwrap();
        assert_eq!(holder.id, Some(Id::new(123)));
    }
}
