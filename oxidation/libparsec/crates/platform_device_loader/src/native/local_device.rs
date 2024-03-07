// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use keyring::Entry as KeyringEntry;
use std::path::Path;

use libparsec_types::prelude::*;

use crate::{
    derive_key_from_biometrics, load_device_file, load_device_with_biometrics_core,
    load_device_with_keyring_core, load_device_with_password_core,
};

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

pub fn load_device_with_keyring_from_path(key_file: &Path) -> LocalDeviceResult<LocalDevice> {
    let device_file = load_device_file(key_file)?;

    match device_file {
        DeviceFile::Keyring(device_file) => {
            let entry = KeyringEntry::new("parsec", device_file.slughash().as_str())
                .map_err(|x| LocalDeviceError::Keyring(x.to_string()))?;
            let passphrase = entry
                .get_password()
                .map_err(|x| LocalDeviceError::Keyring(x.to_string()))?;
            load_device_with_keyring_core(&device_file, passphrase.as_str())
        }
        _ => {
            unreachable!("Tried to load non-keyring device with `load_device_with_keyring`")
        }
    }
}

pub fn load_device_with_keyring(key: &str) -> LocalDeviceResult<LocalDevice> {
    load_device_with_keyring_from_path(Path::new(key))
}

pub fn load_device_with_biometrics_from_path(key_file: &Path) -> LocalDeviceResult<LocalDevice> {
    let device_file = load_device_file(key_file)?;

    match device_file {
        DeviceFile::Biometrics(device_file) => {
            let password = derive_key_from_biometrics(&device_file.slughash())
                .map_err(LocalDeviceError::Biometrics)?;
            load_device_with_biometrics_core(&device_file, password.as_str())
        }
        _ => {
            unreachable!("Tried to load non-biometrics device with `load_device_with_biometrics`")
        }
    }
}

pub fn load_device_with_biometrics(key: &str) -> LocalDeviceResult<LocalDevice> {
    load_device_with_biometrics_from_path(Path::new(key))
}
