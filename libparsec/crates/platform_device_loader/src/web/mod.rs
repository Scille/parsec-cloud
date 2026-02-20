// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{LoadCiphertextKeyError, SaveDeviceError};
use libparsec_types::prelude::*;
use std::path::{Path, PathBuf};

pub(crate) async fn save_device_keyring(
    _: &LocalDevice,
    _: &DateTime,
    _: &Path,
    _: &ParsecAddr,
    _: &DateTime,
    _: Option<TOTPOpaqueKeyID>,
    _: Option<&SecretKey>,
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
    _: Option<TOTPOpaqueKeyID>,
    _: Option<&SecretKey>,
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
    false
}

pub(super) fn get_default_data_base_dir() -> PathBuf {
    PathBuf::from("/")
}

pub(super) fn get_default_config_dir() -> PathBuf {
    PathBuf::from("/")
}

pub(super) fn get_default_mountpoint_base_dir() -> PathBuf {
    PathBuf::from("/")
}
