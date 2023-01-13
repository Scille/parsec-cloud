// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use base64::{engine::general_purpose::STANDARD, Engine};

use libparsec_client_types::{DeviceFile, LocalDevice, LocalDeviceError, LocalDeviceResult};

use crate::load_device_with_password_core;

pub fn load_device_with_password(slug: &str, password: &str) -> LocalDeviceResult<LocalDevice> {
    let window = web_sys::window().unwrap();
    if let Ok(Some(storage)) = window.local_storage() {
        if let Ok(Some(devices)) = storage.get_item("devices") {
            let device_file = devices
                .split(':')
                .filter_map(|x| STANDARD.decode(x).ok())
                .filter_map(|x| DeviceFile::load(&x).ok())
                .filter_map(|x| match x {
                    DeviceFile::Password(x) => Some(x),
                    _ => None,
                })
                .find(|x| x.slug.as_ref().map(|x| x.as_str()) == Some(slug))
                .ok_or_else(|| LocalDeviceError::NotFound { slug: slug.into() })?;

            return load_device_with_password_core(password, &device_file);
        }

        return Err(LocalDeviceError::NotFound { slug: slug.into() });
    }

    Err(LocalDeviceError::LocalStorageNotAvailable)
}
