// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_core_fs as core_fs;
#[cfg(all(not(target_arch = "wasm32"), feature = "test-utils"))]
pub mod local_db {
    pub use libparsec_platform_storage::sqlite::{
        test_clear_local_db_in_memory_mock, test_toggle_local_db_in_memory_mock,
    };
}

#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_protocol as protocol;
#[cfg(feature = "test-utils")]
pub use libparsec_testbed as testbed;
#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_types as types;

pub use libparsec_client_connection as client_connection;
pub use libparsec_core as core;
pub use libparsec_crypto as crypto;
pub use libparsec_platform_device_loader as platform_device_loader;

pub use libparsec_client_high_level_api::*;
