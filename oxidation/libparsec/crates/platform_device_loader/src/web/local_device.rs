// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_client_types::{DeviceFile, LocalDevice, LocalDeviceError, LocalDeviceResult};

use crate::load_device_with_password_core;

pub async fn load_device_with_password(
    slug: &str,
    password: &str,
) -> LocalDeviceResult<LocalDevice> {
    let window = web_sys::window().unwrap();
    if let Ok(Some(storage)) = window.local_storage() {
        if let Ok(Some(devices)) = storage.get_item("devices") {
            let devices = serde_json::from_str::<Vec<DeviceFile>>(&devices).unwrap_or_default();
            let device_file = devices
                .into_iter()
                .filter_map(|x| match x {
                    DeviceFile::Password(x) => Some(x),
                    _ => None,
                })
                .find(|x| x.slug.as_str() == slug)
                .ok_or_else(|| LocalDeviceError::NotFound { slug: slug.into() })?;

            return load_device_with_password_core(&device_file, password);
        }

        return Err(LocalDeviceError::NotFound { slug: slug.into() });
    }

    Err(LocalDeviceError::LocalStorageNotAvailable)
}
