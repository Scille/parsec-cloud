// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use serde_bytes::ByteBuf;

pub const UUID_EXT_ID: i8 = 2;

pub struct UuidExtVisitor;

impl<'de> serde::de::Visitor<'de> for UuidExtVisitor {
    type Value = uuid::Uuid;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("an UUID as extension 2 format")
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, self)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let tag: i8 = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let data: ByteBuf = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        if tag == UUID_EXT_ID {
            uuid::Uuid::from_slice(&data)
                .map_err(|_| serde::de::Error::custom("invalid size of data extension"))
        } else {
            let unexpected = serde::de::Unexpected::Signed(tag as i64);
            Err(serde::de::Error::invalid_value(unexpected, &self))
        }
    }
}
