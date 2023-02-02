// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::path::Path;

use libparsec_client_types::{DeviceFile, LocalDevice, LocalDeviceResult};

use crate::{load_device_file, load_device_with_password_core};

pub fn load_device_with_password_from_path(
    key_file: &Path,
    password: &str,
) -> LocalDeviceResult<LocalDevice> {
    let device_file = load_device_file(key_file)?;

    match device_file {
        DeviceFile::Password(device_file) => load_device_with_password_core(&device_file, password),
        _ => {
            unreachable!("Tried to load recovery/smartcard device with `load_device_with_password`")
        }
    }
}

pub fn load_device_with_password(key: &str, password: &str) -> LocalDeviceResult<LocalDevice> {
    load_device_with_password_from_path(Path::new(key), password)
}
