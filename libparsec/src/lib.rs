// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(feature = "python-bindings-support")]
pub mod low_level {
    // LowLevel stuff only exposed for Python bindings
    pub use libparsec_crypto as crypto;
    pub use libparsec_protocol as protocol;
    pub use libparsec_serialization_format as serialization_format;
    #[cfg(feature = "test-utils")]
    pub use libparsec_testbed as testbed;
    pub use libparsec_types as types;
}

mod addr;
mod cancel;
mod client;
mod config;
mod events;
mod handle;
mod invite;
mod os;
mod testbed;

pub use addr::*;
pub use cancel::*;
pub use client::*;
pub use config::*;
pub use events::*;
pub use invite::*;
pub use os::*;
pub use testbed::*;

pub async fn list_available_devices(config_dir: &std::path::Path) -> Vec<AvailableDevice> {
    libparsec_platform_device_loader::list_available_devices(config_dir).await
}
