// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{LoadCiphertextKeyError, SaveDeviceError};
use libparsec_types::prelude::*;
use std::path::Path;

pub(crate) async fn save_device_keyring(
    _: &LocalDevice,
    _: &DateTime,
    _: &Path,
    _: &ParsecAddr,
    _: &DateTime,
) -> Result<(), SaveDeviceError> {
    panic!("Keyring not supported on Web")
}

pub(crate) async fn save_device_pki(
    _: &LocalDevice,
    _: &DateTime,
    _: &Path,
    _: &ParsecAddr,
    _: &DateTime,
    _: &X509CertificateReference,
) -> Result<(), SaveDeviceError> {
    panic!("PKI not supported on Web")
}

pub(super) async fn load_ciphertext_key_keyring(
    _: &DeviceFile,
) -> Result<SecretKey, LoadCiphertextKeyError> {
    panic!("Keyring not supported on Web")
}

pub(super) async fn load_ciphertext_key_pki(
    _: &DeviceFile,
) -> Result<SecretKey, LoadCiphertextKeyError> {
    panic!("PKI not supported on Web")
}

pub(super) fn is_keyring_available() -> bool {
    panic!("Keyring not supported on Web")
}
