// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use serde_with::*;

use parsec_api_types::data_macros::new_data_struct_type;
use parsec_api_types::*;

// TODO: move this somewhere more generic
mod maybe_field {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Any value that is present is considered Some value, including null.
    pub fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(Some)
    }

    /// Any value that is present is considered Some value, including null.
    pub fn serialize_some<T, S>(x: &Option<T>, s: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        x.serialize(s)
    }
}

/*
 * Schema for legacy device files where the filename contains complementary information.
 */

new_data_struct_type!(
    LegacyDeviceFile,
    type: "password",

    salt: Vec<u8>,
    ciphertext: Vec<u8>,

    // Since human_handle/device_label has been introduced, device_id is
    // redacted (i.e. user_id and device_name are 2 random uuids), hence
    // those fields have been added to the device file so the login page in
    // the GUI can use them to provide useful information.
    #[serde(
        default,
        deserialize_with = "maybe_field::deserialize_some",
        serialize_with = "maybe_field::serialize_some"
    )]
    human_handle: Option<Option<HumanHandle>>,
    #[serde(
        default,
        deserialize_with = "maybe_field::deserialize_some",
        serialize_with = "maybe_field::serialize_some"
    )]
    device_label: Option<Option<DeviceLabel>>,
);

/*
 * Schemas for device files that does not rely on the filename for complementary information.
 */

new_data_struct_type!(
    PasswordDeviceFileSchema,
    type: "password",

    ciphertext: Vec<u8>,

    // Override those fields to make them required (although `None` is still valid)
    human_handle: Option<HumanHandle>,
    device_label: Option<DeviceLabel>,

    // Store device ID, organization ID and slug in the device file
    // For legacy versions, this information is available in the file name
    device_id: DeviceID,
    organization_id: OrganizationID,
    slug: String,

    // Custom fields
    salt: Vec<u8>,
);

new_data_struct_type!(
    RecoveryDeviceFileSchema,
    type: "recovery",

    ciphertext: Vec<u8>,

    // Override those fields to make them required (although `None` is still valid)
    human_handle: Option<HumanHandle>,
    device_label: Option<DeviceLabel>,

    // Store device ID, organization ID and slug in the device file
    // For legacy versions, this information is available in the file name
    device_id: DeviceID,
    organization_id: OrganizationID,
    slug: String,
    // Custom fields
    // *nothing*
);

new_data_struct_type!(
    SmartcardDeviceFileSchema,
    type: "smartcard",

    ciphertext: Vec<u8>,

    // Override those fields to make them required (although `None` is still valid)
    human_handle: Option<HumanHandle>,
    device_label: Option<DeviceLabel>,

    // Store device ID, organization ID and slug in the device file
    // For legacy versions, this information is available in the file name
    device_id: DeviceID,
    organization_id: OrganizationID,
    slug: String,

    // Custom fields
    encrypted_key: Vec<u8>,
    certificate_id: String,
    certificate_sha1: Option<Vec<u8>>,
);

/// Schema for device files that does not rely on the filename for complementary information.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum DeviceFile {
    #[serde(rename = "password")]
    Password {
        ciphertext: ByteBuf,

        human_handle: Option<HumanHandle>,
        device_label: Option<DeviceLabel>,

        // Store device ID, organization ID and slug in the device file
        // For legacy versions, this information is available in the file name
        device_id: DeviceID,
        organization_id: OrganizationID,
        slug: String,

        // Custom fields
        salt: ByteBuf,
    },

    #[serde(rename = "recovery")]
    Recovery {
        ciphertext: ByteBuf,

        human_handle: Option<HumanHandle>,
        device_label: Option<DeviceLabel>,

        // Store device ID, organization ID and slug in the device file
        // For legacy versions, this information is available in the file name
        device_id: DeviceID,
        organization_id: OrganizationID,
        slug: String,
        // Custom fields
        // *nothing*
    },

    #[serde(rename = "smartcard")]
    Smartcard {
        ciphertext: ByteBuf,

        human_handle: Option<HumanHandle>,
        device_label: Option<DeviceLabel>,

        // Store device ID, organization ID and slug in the device file
        // For legacy versions, this information is available in the file name
        device_id: DeviceID,
        organization_id: OrganizationID,
        slug: String,

        // Custom fields
        encrypted_key: ByteBuf,
        certificate_id: String,
        certificate_sha1: Option<ByteBuf>,
    },
}

impl DeviceFile {
    pub fn load(serialized: &[u8]) -> Result<Self, &'static str> {
        rmp_serde::from_read_ref(&serialized).map_err(|_| "Invalid serialization")
    }

    pub fn dump(&self) -> Vec<u8> {
        rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!())
    }
}
