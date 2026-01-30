// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;
use zeroize::Zeroizing;

use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum LoadRecoveryDeviceError {
    #[error("Cannot deserialize file content")]
    InvalidData,
    #[error("Passphrase format is invalid")]
    InvalidPassphrase,
    #[error("Failed to decrypt file content")]
    DecryptionFailed,
}

/// Decrypt and deserialize the local device to use as recovery device (i.e. the device
/// creating a new device) during a recovery device import operation.
pub fn load_recovery_device(
    recovery_device: &[u8],
    passphrase: SecretKeyPassphrase,
) -> Result<Arc<LocalDevice>, LoadRecoveryDeviceError> {
    let key = SecretKey::from_recovery_passphrase(passphrase)
        .map_err(|_| LoadRecoveryDeviceError::InvalidPassphrase)?;

    // Regular load
    let device_file =
        DeviceFile::load(recovery_device).map_err(|_| LoadRecoveryDeviceError::InvalidData)?;

    let recovery_device = match device_file {
        DeviceFile::Recovery(x) => {
            let cleartext = key
                .decrypt(&x.ciphertext)
                .map(Zeroizing::new)
                .map_err(|_| LoadRecoveryDeviceError::DecryptionFailed)?;

            LocalDevice::load(&cleartext).map_err(|_| LoadRecoveryDeviceError::InvalidData)?
        }
        // We are not expecting other type of device file
        _ => return Err(LoadRecoveryDeviceError::InvalidData),
    };

    Ok(Arc::new(recovery_device))
}

/// Serialize the provided local device into a package that can be exported as
/// recovery device (i.e. a buffer containing the encrypted local device and
/// its corresponding passphrase to be used for decryption).
pub fn dump_recovery_device(recovery_device: &LocalDevice) -> (SecretKeyPassphrase, Vec<u8>) {
    let created_on = recovery_device.now();
    let server_addr: ParsecAddr = recovery_device.organization_addr.clone().into();

    let (passphrase, key) = SecretKey::generate_recovery_passphrase();

    let ciphertext = {
        let cleartext = Zeroizing::new(recovery_device.dump());
        let ciphertext = key.encrypt(&cleartext);
        ciphertext.into()
    };

    let file_content = DeviceFile::Recovery(DeviceFileRecovery {
        created_on,
        // Note recovery device is not supposed to change its protection
        protected_on: created_on,
        server_url: server_addr,
        organization_id: recovery_device.organization_id().to_owned(),
        user_id: recovery_device.user_id,
        device_id: recovery_device.device_id,
        human_handle: recovery_device.human_handle.to_owned(),
        device_label: recovery_device.device_label.to_owned(),
        ciphertext,
    })
    .dump();

    (passphrase, file_content)
}
