// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// We don't want to have this crate depend on `libparsec_types` for it test
// given `libparsec_types` itself uses this crate... hence in case of bug in
// this crate we wouldn't be able to use the tests !
// So here we simulate `libparsec_types` by implementing the bare minimum that
// our macros need (only mock types that are NOT in `libparsec_serialization_format_types`).

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub struct DeviceID(pub String);

impl libparsec_serialization_format_types::rmp_serialize::Serialize for DeviceID {
    fn serialize(
        &self,
        writer: &mut Vec<u8>,
    ) -> Result<(), libparsec_serialization_format_types::rmp_serialize::SerializeError> {
        libparsec_serialization_format_types::rmp_serialize::Serialize::serialize(
            self.0.as_str(),
            writer,
        )
    }
}

impl libparsec_serialization_format_types::rmp_serialize::Deserialize for DeviceID {
    fn deserialize(
        value: libparsec_serialization_format_types::rmp_serialize::ValueRef<'_>,
    ) -> Result<Self, libparsec_serialization_format_types::rmp_serialize::DeserializeError> {
        Ok(Self(
            libparsec_serialization_format_types::rmp_serialize::Deserialize::deserialize(value)?,
        ))
    }
}
