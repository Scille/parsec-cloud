// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// This module provides ID constants used for testing purposes.
// The full fixture builders (Device, Organization, etc.) remain in `libparsec_types`
// since they depend on schema-generated types.
#![allow(clippy::unwrap_used)]

use crate::{DeviceID, UserID};

pub const ALICE_USER_ID: UserID = UserID(uuid::uuid!("A11CEC00100000000000000000000000"));
pub const BOB_USER_ID: UserID = UserID(uuid::uuid!("808C0010000000000000000000000000"));
pub const CARL_USER_ID: UserID = UserID(uuid::uuid!("CA470010000000000000000000000000"));
pub const DIANA_USER_ID: UserID = UserID(uuid::uuid!("D1444010000000000000000000000000"));
pub const MALLORY_USER_ID: UserID = UserID(uuid::uuid!("3A11031C001000000000000000000000"));
pub const MIKE_USER_ID: UserID = UserID(uuid::uuid!("31CEC001000000000000000000000000"));
pub const PHILIP_USER_ID: UserID = UserID(uuid::uuid!("91119EC0010000000000000000000000"));
pub const ALICE_DEV1_DEVICE_ID: DeviceID =
    DeviceID(uuid::uuid!("DE10A11CEC0010000000000000000000"));
pub const ALICE_DEV2_DEVICE_ID: DeviceID =
    DeviceID(uuid::uuid!("DE20A11CEC0010000000000000000000"));
pub const ALICE_DEV3_DEVICE_ID: DeviceID =
    DeviceID(uuid::uuid!("DE30A11CEC0010000000000000000000"));
pub const BOB_DEV1_DEVICE_ID: DeviceID = DeviceID(uuid::uuid!("DE10808C001000000000000000000000"));
pub const BOB_DEV2_DEVICE_ID: DeviceID = DeviceID(uuid::uuid!("DE20808C001000000000000000000000"));
pub const MALLORY_DEV1_DEVICE_ID: DeviceID =
    DeviceID(uuid::uuid!("DE103A11031C00100000000000000000"));
pub const MALLORY_DEV2_DEVICE_ID: DeviceID =
    DeviceID(uuid::uuid!("DE203A11031C00100000000000000000"));
pub const MIKE_DEV1_DEVICE_ID: DeviceID = DeviceID(uuid::uuid!("DE1031CEC00100000000000000000000"));
pub const PHILIP_DEV1_DEVICE_ID: DeviceID =
    DeviceID(uuid::uuid!("DE1091119EC001000000000000000000"));
