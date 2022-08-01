// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use serde_with::*;

use libparsec_types::data_macros::new_data_struct_type;
use libparsec_types::*;

/*
 *  for legacy device files where the filename contains complementary information.
 */

new_data_struct_type!(
    LegacyDeviceFile,
    type: "password",

    salt: Vec<u8>,
    ciphertext: Vec<u8>,

    // Added in Parsec v1.14
    // Since human_handle/device_label has been introduced, device_id is
    // redacted (i.e. user_id and device_name are 2 random uuids), hence
    // those fields have been added to the device file so the login page in
    // the GUI can use them to provide useful information.
    #[serde(
        default,
        deserialize_with = "maybe_field::deserialize_some",
    )]
    human_handle: Option<Option<HumanHandle>>,
    // Added in Parsec v1.14
    #[serde(
        default,
        deserialize_with = "maybe_field::deserialize_some",
    )]
    device_label: Option<Option<DeviceLabel>>,
);

/*
 * s for device files that does not rely on the filename for complementary information.
 */

new_data_struct_type!(
    PasswordDeviceFile,
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
    RecoveryDeviceFile,
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
    SmartcardDeviceFile,
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

///  for device files that does not rely on the filename for complementary information.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DeviceFile {
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
        rmp_serde::from_slice(serialized).map_err(|_| "Invalid serialization")
    }

    pub fn dump(&self) -> Vec<u8> {
        rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!())
    }
}
