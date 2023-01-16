// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_client_types as client_types;
#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_core as core;
#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_core_fs as core_fs;
#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_protocol as protocol;
#[cfg(not(target_arch = "wasm32"))]
pub use libparsec_types as types;

pub use libparsec_crypto as crypto;

// TODO: replace me by the high-level API here ;-)
pub use libparsec_client_types::{AvailableDevice, DeviceFileType, StrPath};

pub fn list_available_devices(config_dir: StrPath) -> Vec<AvailableDevice> {
    libparsec_client_types::list_available_devices(&std::path::PathBuf::from(config_dir))
        .unwrap_or_default()
}
