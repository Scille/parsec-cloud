// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use libparsec_types::prelude::*;

use crate::{
    ChangeAuthentificationError, LoadDeviceError, LoadRecoveryDeviceError, SaveDeviceError,
    SaveRecoveryDeviceError,
};

/*
 * List available devices
 */

pub async fn list_available_devices(_config_dir: &Path) -> Vec<AvailableDevice> {
    todo!()
}

/*
 * Save & load
 */

pub async fn load_device(
    _access: &DeviceAccessStrategy,
) -> Result<(Arc<LocalDevice>, DateTime), LoadDeviceError> {
    todo!()
}

pub async fn save_device(
    _access: &DeviceAccessStrategy,
    _device: &LocalDevice,
    _created_on: DateTime,
) -> Result<AvailableDevice, SaveDeviceError> {
    todo!()
}

pub async fn change_authentication(
    _current_access: &DeviceAccessStrategy,
    _new_access: &DeviceAccessStrategy,
) -> Result<AvailableDevice, ChangeAuthentificationError> {
    todo!()
}

pub async fn archive_device(_device_path: &Path) -> Result<(), crate::ArchiveDeviceError> {
    todo!()
}

pub async fn remove_device(_device_path: &Path) -> Result<(), crate::RemoveDeviceError> {
    todo!()
}

/*
 * Recovery
 */

pub async fn load_recovery_device(
    _key_file: &Path,
    _passphrase: SecretKeyPassphrase,
) -> Result<LocalDevice, LoadRecoveryDeviceError> {
    todo!()
}

pub async fn save_recovery_device(
    _key_file: &Path,
    _device: &LocalDevice,
) -> Result<SecretKeyPassphrase, SaveRecoveryDeviceError> {
    todo!()
}
