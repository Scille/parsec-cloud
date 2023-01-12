// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::path::Path;

use libparsec_client_types::{
    DeviceFile, DeviceFileType, LocalDevice, LocalDeviceError, LocalDeviceResult,
};
use libparsec_crypto::SecretKey;

use crate::load_device_file;

pub fn load_device_with_password(
    key_file: &Path,
    password: &str,
) -> LocalDeviceResult<LocalDevice> {
    let device_file = load_device_file(key_file)?;

    match device_file {
        DeviceFile::Password(p) => {
            let key = SecretKey::from_password(password, &p.salt);
            let data = key
                .decrypt(&p.ciphertext)
                .map_err(LocalDeviceError::CryptoError)?;

            LocalDevice::load(&data)
                .map_err(|_| LocalDeviceError::Validation(DeviceFileType::Password))
        }
        _ => {
            unreachable!("Tried to load recovery/smartcard device with `load_device_with_password`")
        }
    }
}
