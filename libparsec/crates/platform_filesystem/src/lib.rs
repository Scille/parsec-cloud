// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::anyhow;

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#[cfg(not(target_arch = "wasm32"))]
pub mod native;
#[path = "../tests/units/mod.rs"]
#[cfg(test)]
mod tests;
#[cfg(target_arch = "wasm32")]
pub mod web;

#[derive(Debug, thiserror::Error)]
pub enum SaveContentError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error(transparent)]
    InvalidPath(anyhow::Error),
    #[error(transparent)]
    Internal(anyhow::Error),
}
