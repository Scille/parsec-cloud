// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![cfg(not(target_family = "wasm"))]

#[cfg(target_family = "unix")]
mod unix;
#[cfg(target_os = "windows")]
mod windows;
// Testbed integration is tested in the `libparsec_tests_fixture` crate.
#[cfg(feature = "test-with-testbed")]
mod testbed;

use std::path::{Path, PathBuf};

#[cfg(target_family = "unix")]
pub(crate) use unix as platform;
#[cfg(target_os = "windows")]
pub(crate) use windows as platform;

use libparsec_types::prelude::*;

const IN_USE_DEVICES_DIR_NAME: &str = "in_use_devices";

#[derive(Debug, thiserror::Error)]
pub enum TryLockDeviceForUseError {
    #[error("The device is already in use")]
    AlreadyInUse,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[cfg(feature = "test-with-testbed")]
pub use testbed::InUseDeviceLockGuard;

#[cfg(not(feature = "test-with-testbed"))]
pub use platform::InUseDeviceLockGuard;

pub fn try_lock_device_for_use(
    config_dir: &Path,
    device_id: DeviceID,
) -> Result<InUseDeviceLockGuard, TryLockDeviceForUseError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_try_lock_device_for_use(config_dir, device_id) {
        return result;
    }

    let outcome = platform::try_lock_device_for_use(config_dir, device_id);
    #[cfg(feature = "test-with-testbed")]
    let outcome = outcome.map(|lock| lock.into());

    outcome
}

pub async fn lock_device_for_use(
    config_dir: &Path,
    device_id: DeviceID,
) -> Result<InUseDeviceLockGuard, anyhow::Error> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_lock_device_for_use(config_dir, device_id).await {
        return Ok(result);
    }

    let outcome = platform::lock_device_for_use(config_dir, device_id).await;
    #[cfg(feature = "test-with-testbed")]
    let outcome = outcome.map(|lock| lock.into());

    outcome
}

fn get_lock_file_path(config_dir: &Path, device_id: DeviceID) -> PathBuf {
    let mut path = config_dir.to_owned();
    path.push(IN_USE_DEVICES_DIR_NAME);
    path.push(device_id.hex());
    path
}

#[cfg(test)]
#[path = "../tests/unit/base.rs"]
#[allow(clippy::unwrap_used)]
mod operations;
