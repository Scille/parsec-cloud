// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    collections::HashSet,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use libparsec_client_types::{
    AvailableDevice, DeviceFile, DeviceFilePassword, DeviceFileRecovery, DeviceFileType,
    LegacyDeviceFile, LocalDevice, LocalDeviceError, LocalDeviceResult,
};
use libparsec_crypto::SecretKey;

use crate::load_device_with_password_from_path;

pub(crate) const DEVICE_FILE_EXT: &str = "keys";

pub async fn list_available_devices_core(
    config_dir: &Path,
) -> LocalDeviceResult<Vec<AvailableDevice>> {
    let mut list = vec![];
    // Set of seen slugs
    let mut seen = HashSet::new();

    let key_file_paths = config_dir.join("devices");

    // Consider `.keys` files in devices directory
    let mut key_file_paths = read_key_file_paths(key_file_paths)?;

    // Sort paths so the discovery order is deterministic
    // In the case of duplicate files, that means only the first discovered device is considered
    key_file_paths.sort();

    for key_file_path in key_file_paths {
        let device = match load_available_device(key_file_path).await {
            // Load the device file
            Ok(device) => device,
            // Ignore invalid files
            Err(_) => continue,
        };

        // Ignore duplicate files
        if seen.contains(&device.slug) {
            continue;
        }

        seen.insert(device.slug.clone());

        list.push(device);
    }

    Ok(list)
}

pub async fn list_available_devices(config_dir: &Path) -> Vec<AvailableDevice> {
    list_available_devices_core(config_dir)
        .await
        .unwrap_or_default()
}

pub async fn save_recovery_device(
    key_file: &Path,
    device: LocalDevice,
    force: bool,
) -> LocalDeviceResult<String> {
    if !force && check_device_already_exist(key_file).await {
        return Err(LocalDeviceError::AlreadyExists(key_file.to_path_buf()));
    }

    let (passphrase, key) = SecretKey::generate_recovery_passphrase();

    let ciphertext = key.encrypt(&device.dump());

    let key_file_content = DeviceFile::Recovery(DeviceFileRecovery {
        ciphertext,
        organization_id: device.organization_id().clone(),
        slug: device.slug(),
        human_handle: device.human_handle,
        device_label: device.device_label,
        device_id: device.device_id,
    });

    save_device_file(key_file, &key_file_content).await?;

    Ok(passphrase)
}

/// Check if the provided [Path] don't correspond to an already exist file.
async fn check_device_already_exist(key_file: &Path) -> bool {
    if let Ok(metadata) = tokio::fs::metadata(key_file).await {
        metadata.is_file()
    } else {
        false
    }
}

/// TODO: need test (backend_cmds required)
pub async fn load_recovery_device(
    key_file: &Path,
    passphrase: &str,
) -> LocalDeviceResult<LocalDevice> {
    let ciphertext = tokio::fs::read(key_file)
        .await
        .map_err(|_| LocalDeviceError::Access(key_file.to_path_buf()))?;
    let data = DeviceFile::load(&ciphertext)
        .map_err(|_| LocalDeviceError::Deserialization(key_file.to_path_buf()))?;

    let device = match data {
        DeviceFile::Recovery(device) => device,
        _ => {
            return Err(LocalDeviceError::Validation {
                ty: DeviceFileType::Recovery,
            })
        }
    };

    let key = SecretKey::from_recovery_passphrase(passphrase)?;
    let plaintext = key.decrypt(&device.ciphertext)?;
    LocalDevice::load(&plaintext)
        .map_err(|_| LocalDeviceError::Deserialization(key_file.to_path_buf()))
}

pub async fn save_device_file(
    key_file_path: &Path,
    device_file: &DeviceFile,
) -> LocalDeviceResult<()> {
    if let Some(parent) = key_file_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|_| LocalDeviceError::Access(key_file_path.to_path_buf()))?;
    }

    let data = device_file.dump();

    tokio::fs::write(key_file_path, data)
        .await
        .map_err(|_| LocalDeviceError::Access(key_file_path.to_path_buf()))
}

fn load_legacy_device_file(key_file: &Path, ciphertext: &[u8]) -> LocalDeviceResult<DeviceFile> {
    let LegacyDeviceFile::Password(legacy_device) = LegacyDeviceFile::load(ciphertext)
        .map_err(|_| LocalDeviceError::Deserialization(key_file.to_path_buf()))?;

    // Legacy device slug is their stem
    let slug = key_file.file_stem().unwrap().to_str().unwrap();
    let (organization_id, device_id) =
        LocalDevice::load_slug(slug).map_err(|_| LocalDeviceError::InvalidSlug)?;

    Ok(DeviceFile::Password(DeviceFilePassword {
        salt: legacy_device.salt,
        ciphertext: legacy_device.ciphertext,
        human_handle: legacy_device.human_handle,
        device_label: legacy_device.device_label,
        device_id,
        organization_id,
        slug: slug.to_string(),
    }))
}

pub async fn load_device_file(key_file_path: &Path) -> LocalDeviceResult<DeviceFile> {
    let data = tokio::fs::read(key_file_path)
        .await
        .map_err(|_| LocalDeviceError::Access(key_file_path.to_path_buf()))?;

    // In case of failure try to load a legacy_device and convert it to a
    // regular device file
    DeviceFile::load(&data)
        .map_err(|_| LocalDeviceError::Deserialization(key_file_path.to_path_buf()))
        .or_else(|_| load_legacy_device_file(key_file_path, &data))
}

pub async fn load_available_device(key_file_path: PathBuf) -> LocalDeviceResult<AvailableDevice> {
    let (ty, organization_id, device_id, human_handle, device_label, slug) =
        match load_device_file(&key_file_path).await? {
            DeviceFile::Password(device) => (
                DeviceFileType::Password,
                device.organization_id,
                device.device_id,
                device.human_handle,
                device.device_label,
                device.slug,
            ),
            DeviceFile::Recovery(device) => (
                DeviceFileType::Recovery,
                device.organization_id,
                device.device_id,
                device.human_handle,
                device.device_label,
                device.slug,
            ),
            DeviceFile::Smartcard(device) => (
                DeviceFileType::Smartcard,
                device.organization_id,
                device.device_id,
                device.human_handle,
                device.device_label,
                device.slug,
            ),
        };

    Ok(AvailableDevice {
        key_file_path,
        organization_id,
        device_id,
        human_handle,
        device_label,
        slug,
        ty,
    })
}

/// Return the default keyfile path for a given device.
///
/// Note that the filename does not carry any intrinsic meaning.
/// Here, we simply use the slughash to avoid name collision.
pub fn get_default_key_file(config_dir: &Path, device: &LocalDevice) -> PathBuf {
    let mut device_path = config_dir.to_path_buf();

    device_path.push("devices");

    device_path.push(format!("{}.{DEVICE_FILE_EXT}", device.slughash()));

    device_path
}

fn read_key_file_paths(path: PathBuf) -> LocalDeviceResult<Vec<PathBuf>> {
    let mut key_file_paths = vec![];

    if !path.exists() {
        return Ok(key_file_paths);
    }

    for path in std::fs::read_dir(&path)
        .map_err(|_| LocalDeviceError::Access(path))?
        .filter_map(|path| path.ok())
        .map(|entry| entry.path())
    {
        if path.extension() == Some(OsStr::new(DEVICE_FILE_EXT)) {
            key_file_paths.push(path)
        } else if path.is_dir() {
            key_file_paths.append(&mut read_key_file_paths(path)?)
        }
    }

    Ok(key_file_paths)
}

pub async fn save_device_with_password(
    key_file: &Path,
    device: &LocalDevice,
    password: &str,
    force: bool,
) -> Result<(), LocalDeviceError> {
    if key_file.exists() && !force {
        return Err(LocalDeviceError::AlreadyExists(key_file.to_path_buf()));
    }

    let cleartext = device.dump();
    let salt = SecretKey::generate_salt();
    let key = SecretKey::from_password(password, &salt);

    let ciphertext = key.encrypt(&cleartext);
    let key_file_content = DeviceFile::Password(DeviceFilePassword {
        salt,
        ciphertext,
        // TODO: Can we avoid these copy?
        human_handle: device.human_handle.clone(),
        device_label: device.device_label.clone(),
        device_id: device.device_id.clone(),
        organization_id: device.organization_id().clone(),
        slug: device.slug(),
    });
    save_device_file(key_file, &key_file_content).await?;

    Ok(())
}

pub async fn save_device_with_password_in_config(
    config_dir: &Path,
    device: &LocalDevice,
    password: &str,
) -> Result<PathBuf, LocalDeviceError> {
    let key_file = get_default_key_file(config_dir, device);

    create_key_file_subdir(&key_file).await;

    // Why do we use `force=True` here ?
    // Key file name is per-device unique (given it contains the device slughash),
    // hence there is no risk to overwrite another device.
    // So if we are overwriting a key file it could be by:
    // - the same device object, hence overwriting has no effect
    // - a device object with same slughash but different device/user keys
    //   This would mean the device enrollment has been replayed (which is
    //   not possible in theory, but could occur in case of a rollback in the
    //   Parsec server), in this case the old device object is now invalid
    //   and it's a good thing to replace it.
    save_device_with_password(&key_file, device, password, true).await?;

    Ok(key_file)
}

/// Create the sub-directories where the `key file` will be stored.
async fn create_key_file_subdir(key_file: &Path) {
    tokio::fs::create_dir_all(&key_file.parent().expect("`key_file` MUST have parent"))
        .await
        .expect("Failed to create some directory");
}

pub async fn change_device_password(
    key_file: &Path,
    old_password: &str,
    new_password: &str,
) -> Result<(), LocalDeviceError> {
    let device = load_device_with_password_from_path(key_file, old_password).await?;
    save_device_with_password(key_file, &device, new_password, true).await
}

pub async fn get_available_device(
    config_dir: &Path,
    slug: &str,
) -> LocalDeviceResult<AvailableDevice> {
    let devices = list_available_devices_core(config_dir).await?;

    devices
        .iter()
        .find(|d| d.slug == slug)
        .ok_or_else(|| LocalDeviceError::Access(config_dir.to_path_buf()))
        .cloned()
}
