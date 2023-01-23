// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod web;
// Testbed integration is tested in the `libarsec_tests_fixture` crate.
#[cfg(feature = "test-testbed-support")]
mod testbed;

use libparsec_client_types::{
    DeviceFilePassword, DeviceFileType, LocalDevice, LocalDeviceError, LocalDeviceResult,
};
use libparsec_crypto::SecretKey;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
#[cfg(target_arch = "wasm32")]
pub use web::*;

pub(crate) fn load_device_with_password_core(
    device: &DeviceFilePassword,
    password: &str,
) -> LocalDeviceResult<LocalDevice> {
    let key = SecretKey::from_password(password, &device.salt);
    let data = key.decrypt(&device.ciphertext)?;

    LocalDevice::load(&data).map_err(|_| LocalDeviceError::Validation {
        ty: DeviceFileType::Password,
    })
}
