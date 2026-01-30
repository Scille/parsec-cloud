// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod device;
mod load;
mod strategy;
pub use device::*;
use libparsec_platform_async::stream::StreamExt;
use libparsec_platform_filesystem::{
    list_files, load_file, remove_file, save_content, ListFilesError, LoadFileError,
    RemoveFileError, SaveContentError,
};
pub use strategy::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native/mod.rs"]
mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "web/mod.rs"]
mod platform;
// Testbed integration is tested in the `libparsec_tests_fixture` crate.
#[cfg(feature = "test-with-testbed")]
mod testbed;

#[path = "../tests/units/mod.rs"]
#[cfg(test)]
mod tests;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use libparsec_types::prelude::*;

const LOCAL_PENDING_EXT: &str = "pending";

pub(crate) const DEVICE_FILE_EXT: &str = "keys";
pub(crate) const ARCHIVE_DEVICE_EXT: &str = "archived";
pub(crate) const PENDING_ASYNC_ENROLLMENT_EXT: &str = "pending";

pub const PARSEC_BASE_CONFIG_DIR: &str = "PARSEC_BASE_CONFIG_DIR";
pub const PARSEC_BASE_DATA_DIR: &str = "PARSEC_BASE_DATA_DIR";
pub const PARSEC_BASE_HOME_DIR: &str = "PARSEC_BASE_HOME_DIR";

#[derive(Debug, thiserror::Error)]
enum LoadCiphertextKeyError {
    /// Typically returned if the access strategy doesn't match the device type
    #[error("Invalid data")]
    InvalidData,
    #[cfg_attr(target_arch = "wasm32", expect(dead_code))]
    #[error("Decryption failed")]
    DecryptionFailed,
    /// Note only a subset of load strategies requires server access to
    /// fetch an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("No response from {server} server: {error}")]
    // We don't use `ConnectionError` here since this type only corresponds to
    // an answer from the Parsec server and here any arbitrary server may have
    // been (unsuccessfully) requested (e.g. OpenBao server).
    RemoteOpaqueKeyFetchOffline {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    /// Note only a subset of load strategies requires server access to
    /// fetch an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("{server} server opaque key fetch failed: {error}")]
    RemoteOpaqueKeyFetchFailed {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    #[error(transparent)]
    Internal(anyhow::Error),
}

pub fn get_default_data_base_dir() -> PathBuf {
    platform::get_default_data_base_dir()
}

pub fn get_default_config_dir() -> PathBuf {
    platform::get_default_config_dir()
}

pub fn get_default_mountpoint_base_dir() -> PathBuf {
    platform::get_default_mountpoint_base_dir()
}

fn get_devices_dir(config_dir: &Path) -> PathBuf {
    config_dir.join("devices")
}

/// Return the default keyfile path for a given device.
///
/// Note that the filename does not carry any intrinsic meaning.
/// Here, we simply use the device ID (as it is a UUID) to avoid name collision.
pub fn get_default_key_file(config_dir: &Path, device_id: DeviceID) -> PathBuf {
    let mut device_path = get_devices_dir(config_dir);
    device_path.push(format!("{}.{DEVICE_FILE_EXT}", device_id.hex()));
    device_path
}

pub fn get_default_local_pending_file(
    config_dir: &Path,
    enrollment_id: PKIEnrollmentID,
) -> PathBuf {
    let mut local_pending_path = get_local_pending_dir(config_dir);
    local_pending_path.push(format!("{}.{LOCAL_PENDING_EXT}", enrollment_id.hex()));
    local_pending_path
}

fn get_local_pending_dir(config_dir: &Path) -> PathBuf {
    config_dir.join("pending_requests")
}

pub fn get_default_pending_async_enrollment_file(
    config_dir: &Path,
    enrollment_id: AsyncEnrollmentID,
) -> PathBuf {
    let mut enrollments_dir = get_pending_async_enrollment_dir(config_dir);

    enrollments_dir.push(format!(
        "{}.{PENDING_ASYNC_ENROLLMENT_EXT}",
        enrollment_id.hex()
    ));

    enrollments_dir
}

fn get_pending_async_enrollment_dir(config_dir: &Path) -> PathBuf {
    config_dir.join("async_enrollments")
}

#[derive(Debug, thiserror::Error)]
pub enum LoadAvailableDeviceError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error("Invalid path: {}", .0)]
    InvalidPath(anyhow::Error),
    #[error("Invalid data")]
    InvalidData,
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl From<LoadFileError> for LoadAvailableDeviceError {
    fn from(value: LoadFileError) -> Self {
        match value {
            LoadFileError::StorageNotAvailable => LoadAvailableDeviceError::StorageNotAvailable,
            LoadFileError::NotAFile
            | LoadFileError::InvalidParent
            | LoadFileError::InvalidPath
            | LoadFileError::NotFound => LoadAvailableDeviceError::InvalidPath(value.into()),
            LoadFileError::Internal(error) => LoadAvailableDeviceError::Internal(error),
        }
    }
}

/// Similar than `load_device`, but without the decryption part.
///
/// This is only needed for device file vault access that needs its
/// organization & device IDs to determine which account vault item
/// contains its decryption key.
///
/// Note `config_dir` is only used as discriminant for the testbed here
pub async fn load_available_device(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    device_file: PathBuf,
) -> Result<AvailableDevice, LoadAvailableDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(all_available_devices) = testbed::maybe_list_available_devices(config_dir) {
        if let Some(result) = all_available_devices
            .into_iter()
            .find(|c_access| c_access.key_file_path == device_file)
        {
            return Ok(result);
        }
    }

    let file_content = load_file(&device_file).await?;

    load_available_device_from_blob(device_file, &file_content)
        .map_err(|_| LoadAvailableDeviceError::InvalidData)
}

#[derive(Debug, thiserror::Error)]
pub enum LoadDeviceError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error("Invalid path: {}", .0)]
    InvalidPath(anyhow::Error),
    #[error("Invalid data")]
    InvalidData,
    #[error("Decryption failed")]
    DecryptionFailed,
    /// Note only a subset of load strategies requires server access to
    /// fetch an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("No response from {server} server: {error}")]
    // We don't use `ConnectionError` here since this type only corresponds to
    // an answer from the Parsec server and here any arbitrary server may have
    // been (unsuccessfully) requested (e.g. OpenBao server).
    RemoteOpaqueKeyFetchOffline {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    /// Note only a subset of load strategies requires server access to
    /// fetch an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("{server} server opaque key fetch failed: {error}")]
    RemoteOpaqueKeyFetchFailed {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl From<LoadFileError> for LoadDeviceError {
    fn from(value: LoadFileError) -> Self {
        match value {
            LoadFileError::StorageNotAvailable => LoadDeviceError::StorageNotAvailable,
            LoadFileError::NotAFile
            | LoadFileError::InvalidParent
            | LoadFileError::InvalidPath
            | LoadFileError::NotFound => LoadDeviceError::InvalidPath(value.into()),
            LoadFileError::Internal(error) => LoadDeviceError::Internal(error),
        }
    }
}

/// Note `config_dir` is only used as discriminant for the testbed here
pub async fn load_device(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    access: &DeviceAccessStrategy,
) -> Result<Arc<LocalDevice>, LoadDeviceError> {
    log::debug!("Loading device at {}", access.key_file().display());
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_load_device(config_dir, access) {
        return result;
    }

    let file_content = load_file(access.key_file()).await?;
    let device_file = DeviceFile::load(&file_content).map_err(|_| LoadDeviceError::InvalidData)?;
    let ciphertext_key = load::load_ciphertext_key(access, &device_file)
        .await
        .map_err(|err| match err {
            LoadCiphertextKeyError::InvalidData => LoadDeviceError::InvalidData,
            LoadCiphertextKeyError::DecryptionFailed => LoadDeviceError::DecryptionFailed,
            LoadCiphertextKeyError::Internal(err) => LoadDeviceError::Internal(err),
            LoadCiphertextKeyError::RemoteOpaqueKeyFetchOffline { server, error } => {
                LoadDeviceError::RemoteOpaqueKeyFetchOffline { server, error }
            }
            LoadCiphertextKeyError::RemoteOpaqueKeyFetchFailed { server, error } => {
                LoadDeviceError::RemoteOpaqueKeyFetchFailed { server, error }
            }
        })?;
    let device = decrypt_device_file(&device_file, &ciphertext_key).map_err(|err| match err {
        DecryptDeviceFileError::Decrypt(_) => LoadDeviceError::DecryptionFailed,
        DecryptDeviceFileError::Load(_) => LoadDeviceError::InvalidData,
    })?;

    Ok(Arc::new(device))
}

/// Note `config_dir` is only used as discriminant for the testbed here
pub async fn save_device(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    strategy: &DeviceSaveStrategy,
    device: &LocalDevice,
    key_file: PathBuf,
) -> Result<AvailableDevice, SaveDeviceError> {
    log::debug!("Saving device at {}", key_file.display());
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_save_device(config_dir, strategy, device, key_file.clone())
    {
        return result;
    }

    device::save_device(strategy, device, device.now(), key_file).await
}

pub fn is_keyring_available() -> bool {
    platform::is_keyring_available()
}

#[derive(Debug, thiserror::Error)]
pub enum RemoveDeviceError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error("Device not found")]
    NotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<RemoveFileError> for RemoveDeviceError {
    fn from(value: RemoveFileError) -> Self {
        match value {
            RemoveFileError::NotFound => RemoveDeviceError::NotFound,
            RemoveFileError::StorageNotAvailable => RemoveDeviceError::StorageNotAvailable,
            RemoveFileError::InvalidParent
            | RemoveFileError::InvalidPath
            | RemoveFileError::Internal(_) => RemoveDeviceError::Internal(value.into()),
        }
    }
}

pub async fn remove_device(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    device_path: &Path,
) -> Result<(), RemoveDeviceError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_remove_device(config_dir, device_path) {
        return result;
    }

    Ok(remove_file(device_path).await?)
}

fn load_available_device_from_blob(
    path: PathBuf,
    blob: &[u8],
) -> Result<AvailableDevice, libparsec_types::RmpDecodeError> {
    let device_file = DeviceFile::load(blob)?;

    let (
        ty,
        created_on,
        protected_on,
        server_addr,
        organization_id,
        user_id,
        device_id,
        human_handle,
        device_label,
    ) = match device_file {
        DeviceFile::Keyring(device) => (
            AvailableDeviceType::Keyring,
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
        ),
        DeviceFile::Password(device) => (
            AvailableDeviceType::Password,
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
        ),
        DeviceFile::Recovery(device) => (
            AvailableDeviceType::Recovery,
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
        ),
        DeviceFile::PKI(device) => (
            AvailableDeviceType::PKI {
                certificate_ref: device.certificate_ref,
            },
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
        ),
        DeviceFile::AccountVault(device) => (
            AvailableDeviceType::AccountVault,
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
        ),
        DeviceFile::OpenBao(device) => (
            AvailableDeviceType::OpenBao {
                openbao_entity_id: device.openbao_entity_id,
                openbao_preferred_auth_id: device.openbao_preferred_auth_id,
            },
            device.created_on,
            device.protected_on,
            device.server_url,
            device.organization_id,
            device.user_id,
            device.device_id,
            device.human_handle,
            device.device_label,
        ),
    };

    Ok(AvailableDevice {
        key_file_path: path,
        created_on,
        protected_on,
        server_addr,
        organization_id,
        user_id,
        device_id,
        human_handle,
        device_label,
        ty,
    })
}

fn encrypt_device(device: &LocalDevice, key: &SecretKey) -> Bytes {
    let cleartext = zeroize::Zeroizing::new(device.dump());
    key.encrypt(&cleartext).into()
}

#[derive(Debug, thiserror::Error)]
pub enum DecryptDeviceFileError {
    #[error("Failed to decrypt device file: {0}")]
    Decrypt(CryptoError),
    #[error("Failed to load device: {0}")]
    Load(&'static str),
}

fn decrypt_device_file(
    device_file: &DeviceFile,
    ciphertext_key: &SecretKey,
) -> Result<LocalDevice, DecryptDeviceFileError> {
    let cleartext = ciphertext_key
        .decrypt(device_file.ciphertext())
        .map_err(DecryptDeviceFileError::Decrypt)
        .map(zeroize::Zeroizing::new)?;
    LocalDevice::load(&cleartext).map_err(DecryptDeviceFileError::Load)
}

#[derive(Debug, thiserror::Error)]
pub enum SavePkiLocalPendingError {
    #[error(transparent)]
    SaveContentError(#[from] SaveContentError),
    #[error(transparent)]
    Internal(anyhow::Error),
}

pub async fn save_pki_local_pending(
    local_pending: PKILocalPendingEnrollment,
    local_file: PathBuf,
) -> Result<(), SavePkiLocalPendingError> {
    log::debug!(
        "Saving pki enrollment local pending file at {}",
        local_file.display()
    );
    let data = local_pending.dump();
    Ok(save_content(&local_file, &data).await?)
}

#[derive(Debug, thiserror::Error)]
enum LoadContentError {
    #[error(transparent)]
    InvalidPath(anyhow::Error),
    #[error("Cannot deserialize file content")]
    InvalidData,
    #[error("storage not available")]
    StorageNotAvailable,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<LoadFileError> for LoadContentError {
    fn from(value: LoadFileError) -> Self {
        match value {
            LoadFileError::StorageNotAvailable => LoadContentError::StorageNotAvailable,
            LoadFileError::NotAFile
            | LoadFileError::InvalidParent
            | LoadFileError::InvalidPath
            | LoadFileError::NotFound => LoadContentError::InvalidPath(value.into()),
            LoadFileError::Internal(error) => LoadContentError::Internal(error),
        }
    }
}

async fn load_pki_pending_file(path: &Path) -> Result<PKILocalPendingEnrollment, LoadContentError> {
    let content = load_file(path).await?;
    PKILocalPendingEnrollment::load(&content)
        .inspect_err(|err| log::debug!("Failed to load pki pending file {}: {err}", path.display()))
        .map_err(|_| LoadContentError::InvalidData)
}

#[derive(Debug, thiserror::Error)]
pub enum ListPkiLocalPendingError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error(transparent)]
    Internal(anyhow::Error),
}
impl From<ListFilesError> for ListPkiLocalPendingError {
    fn from(value: ListFilesError) -> Self {
        match value {
            ListFilesError::StorageNotAvailable => ListPkiLocalPendingError::StorageNotAvailable,
            ListFilesError::InvalidParent | ListFilesError::Internal(_) => {
                ListPkiLocalPendingError::Internal(value.into())
            }
        }
    }
}

pub async fn list_pki_local_pending(
    config_dir: &Path,
) -> Result<Vec<PKILocalPendingEnrollment>, ListPkiLocalPendingError> {
    let pending_dir = get_local_pending_dir(config_dir);
    let mut files = list_files(&pending_dir, LOCAL_PENDING_EXT).await?;

    // Sort entries so result is deterministic
    files.sort();
    log::trace!("Found pending request files: {files:?}");

    Ok(libparsec_platform_async::stream::iter(files)
        .filter_map(async |path| load_pki_pending_file(&path).await.ok())
        .collect::<Vec<_>>()
        .await)
}

#[derive(Debug, thiserror::Error)]
pub enum SaveAsyncEnrollmentLocalPendingError {
    #[error("No space left")]
    NoSpaceLeft,
    #[error("Path is invalid")]
    InvalidPath,
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl From<SaveContentError> for SaveAsyncEnrollmentLocalPendingError {
    fn from(value: SaveContentError) -> Self {
        match value {
            SaveContentError::StorageNotAvailable | SaveContentError::NoSpaceLeft => {
                SaveAsyncEnrollmentLocalPendingError::NoSpaceLeft
            }
            SaveContentError::NotAFile
            | SaveContentError::InvalidParent
            | SaveContentError::InvalidPath
            | SaveContentError::ParentNotFound
            | SaveContentError::CannotEdit => SaveAsyncEnrollmentLocalPendingError::InvalidPath,
            SaveContentError::Internal(error) => {
                SaveAsyncEnrollmentLocalPendingError::Internal(error)
            }
        }
    }
}

pub async fn save_pending_async_enrollment(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    cleartext_content: AsyncEnrollmentLocalPendingCleartextContent,
    to_become_device_signing_key: &SigningKey,
    to_become_user_private_key: &PrivateKey,
    ciphertext_key: &SecretKey,
    file_path: PathBuf,
) -> Result<AvailablePendingAsyncEnrollment, SaveAsyncEnrollmentLocalPendingError> {
    log::debug!(
        "Saving async enrollment local pending file at {}",
        file_path.display()
    );

    let raw_cleartext_content: Bytes = cleartext_content.dump().into();
    let ciphertext_cleartext_content_digest = ciphertext_key
        .encrypt(HashDigest::from_data(&raw_cleartext_content).as_ref())
        .into();
    let ciphertext_signing_key = ciphertext_key
        .encrypt(to_become_device_signing_key.to_bytes().as_ref())
        .into();
    let ciphertext_private_key = ciphertext_key
        .encrypt(to_become_user_private_key.to_bytes().as_ref())
        .into();
    let content = AsyncEnrollmentLocalPending {
        cleartext_content: raw_cleartext_content,
        ciphertext_cleartext_content_digest,
        ciphertext_signing_key,
        ciphertext_private_key,
    };
    let raw_content = content.dump();

    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_mut))]
    let mut saved = false;

    #[cfg(feature = "test-with-testbed")]
    if let Some(result) =
        testbed::maybe_save_pending_async_enrollment(config_dir, &raw_content, &file_path)
    {
        result?;
        saved = true;
    }

    if !saved {
        save_content(&file_path, &raw_content).await?;
    }

    let identity_system = match cleartext_content.identity_system {
        AsyncEnrollmentLocalPendingIdentitySystem::PKI {
            certificate_ref, ..
        } => AvailablePendingAsyncEnrollmentIdentitySystem::PKI { certificate_ref },
        AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
            openbao_entity_id,
            openbao_preferred_auth_id,
            ..
        } => AvailablePendingAsyncEnrollmentIdentitySystem::OpenBao {
            openbao_entity_id,
            openbao_preferred_auth_id,
        },
    };
    Ok(AvailablePendingAsyncEnrollment {
        file_path,
        submitted_on: cleartext_content.submitted_on,
        addr: ParsecAsyncEnrollmentAddr::new(
            cleartext_content.server_url,
            cleartext_content.organization_id,
        ),
        enrollment_id: cleartext_content.enrollment_id,
        requested_device_label: cleartext_content.requested_device_label,
        requested_human_handle: cleartext_content.requested_human_handle,
        identity_system,
    })
}

#[derive(Debug, thiserror::Error)]
pub enum LoadPendingAsyncEnrollmentError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error("Invalid path: {}", .0)]
    InvalidPath(anyhow::Error),
    #[error("Invalid data")]
    InvalidData,
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl From<LoadFileError> for LoadPendingAsyncEnrollmentError {
    fn from(value: LoadFileError) -> Self {
        match value {
            LoadFileError::StorageNotAvailable => {
                LoadPendingAsyncEnrollmentError::StorageNotAvailable
            }
            LoadFileError::NotAFile
            | LoadFileError::InvalidParent
            | LoadFileError::InvalidPath
            | LoadFileError::NotFound => LoadPendingAsyncEnrollmentError::InvalidPath(value.into()),
            LoadFileError::Internal(..) => LoadPendingAsyncEnrollmentError::Internal(value.into()),
        }
    }
}
/// Note `config_dir` is only used as discriminant for the testbed here
pub async fn load_pending_async_enrollment(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    file_path: &Path,
) -> Result<
    (
        AsyncEnrollmentLocalPending,
        AsyncEnrollmentLocalPendingCleartextContent,
    ),
    LoadPendingAsyncEnrollmentError,
> {
    log::debug!(
        "Loading pending async enrollment file at {}",
        file_path.display()
    );

    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_load_pending_async_enrollment(config_dir, file_path) {
        return result;
    }

    let raw = load_file(file_path).await?;
    load_pending_async_enrollment_frow_raw(file_path, &raw)
        .map_err(|_| LoadPendingAsyncEnrollmentError::InvalidData)
}

#[derive(Debug, thiserror::Error)]
pub enum ListPendingAsyncEnrollmentsError {
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl From<LoadFileError> for ListPendingAsyncEnrollmentsError {
    fn from(value: LoadFileError) -> Self {
        match value {
            LoadFileError::StorageNotAvailable => {
                ListPendingAsyncEnrollmentsError::StorageNotAvailable
            }
            LoadFileError::NotAFile
            | LoadFileError::InvalidParent
            | LoadFileError::InvalidPath
            | LoadFileError::NotFound
            | LoadFileError::Internal(_) => {
                ListPendingAsyncEnrollmentsError::Internal(value.into())
            }
        }
    }
}

impl From<ListFilesError> for ListPendingAsyncEnrollmentsError {
    fn from(value: ListFilesError) -> Self {
        match value {
            ListFilesError::StorageNotAvailable => {
                ListPendingAsyncEnrollmentsError::StorageNotAvailable
            }
            ListFilesError::InvalidParent | ListFilesError::Internal(_) => {
                ListPendingAsyncEnrollmentsError::Internal(value.into())
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AvailablePendingAsyncEnrollment {
    pub file_path: PathBuf,
    pub submitted_on: DateTime,
    pub addr: ParsecAsyncEnrollmentAddr,
    pub enrollment_id: AsyncEnrollmentID,
    pub requested_device_label: DeviceLabel,
    pub requested_human_handle: HumanHandle,
    pub identity_system: AvailablePendingAsyncEnrollmentIdentitySystem,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AvailablePendingAsyncEnrollmentIdentitySystem {
    OpenBao {
        openbao_entity_id: String,
        openbao_preferred_auth_id: String,
    },
    PKI {
        certificate_ref: X509CertificateReference,
    },
}

pub async fn list_pending_async_enrollments(
    config_dir: &Path,
) -> Result<Vec<AvailablePendingAsyncEnrollment>, ListPendingAsyncEnrollmentsError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_list_pending_async_enrollments(config_dir) {
        return result;
    }
    let pending_async_enrollments_dir = get_pending_async_enrollment_dir(config_dir);
    let mut files = list_files(&pending_async_enrollments_dir, LOCAL_PENDING_EXT).await?;

    // Sort entries so result is deterministic
    files.sort();

    Ok(libparsec_platform_async::stream::iter(files)
        .filter_map(async |path| {
            let raw = load_file(&path).await.ok()?;
            load_pending_async_enrollment_as_available_frow_raw(path, &raw).ok()
        })
        .collect::<Vec<_>>()
        .await)
}

fn load_pending_async_enrollment_frow_raw(
    path: &Path,
    raw: &[u8],
) -> Result<
    (
        AsyncEnrollmentLocalPending,
        AsyncEnrollmentLocalPendingCleartextContent,
    ),
    (),
> {
    let cooked = AsyncEnrollmentLocalPending::load(raw)
        .inspect_err(|err| {
            log::warn!(
                "Failed to load async enrollment local pending file {}: {err}",
                path.display()
            )
        })
        .map_err(|_| ())?;

    let cooked_cleartext_content =
        AsyncEnrollmentLocalPendingCleartextContent::load(&cooked.cleartext_content)
            .inspect_err(|err| {
                log::warn!(
            "Failed to load cleartext part of async enrollment local pending file {}: {err}",
            path.display()
        )
            })
            .map_err(|_| ())?;

    Ok((cooked, cooked_cleartext_content))
}

fn load_pending_async_enrollment_as_available_frow_raw(
    path: PathBuf,
    raw: &[u8],
) -> Result<AvailablePendingAsyncEnrollment, ()> {
    let (_, content) = load_pending_async_enrollment_frow_raw(&path, raw)?;

    let identity_system = match content.identity_system {
        AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
            openbao_entity_id,
            openbao_preferred_auth_id,
            ..
        } => AvailablePendingAsyncEnrollmentIdentitySystem::OpenBao {
            openbao_entity_id,
            openbao_preferred_auth_id,
        },
        AsyncEnrollmentLocalPendingIdentitySystem::PKI {
            certificate_ref, ..
        } => AvailablePendingAsyncEnrollmentIdentitySystem::PKI { certificate_ref },
    };

    Ok(AvailablePendingAsyncEnrollment {
        file_path: path,
        submitted_on: content.submitted_on,
        addr: ParsecAsyncEnrollmentAddr::new(content.server_url, content.organization_id),
        enrollment_id: content.enrollment_id,
        requested_device_label: content.requested_device_label,
        requested_human_handle: content.requested_human_handle,
        identity_system,
    })
}

pub type RemovePendingAsyncEnrollmentError = RemoveDeviceError;

pub async fn remove_pending_async_enrollment(
    #[cfg_attr(not(feature = "test-with-testbed"), expect(unused_variables))] config_dir: &Path,
    file_path: &Path,
) -> Result<(), RemovePendingAsyncEnrollmentError> {
    #[cfg(feature = "test-with-testbed")]
    if let Some(result) = testbed::maybe_remove_pending_async_enrollment(config_dir, file_path) {
        return result;
    }

    Ok(remove_file(file_path).await?)
}
