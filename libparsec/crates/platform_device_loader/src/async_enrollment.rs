// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_async::stream::StreamExt;
use libparsec_platform_filesystem::{
    list_files, load_file, remove_file, save_content, ListFilesError, LoadFileError,
    SaveContentError,
};
use libparsec_types::prelude::*;

use std::path::{Path, PathBuf};

#[cfg(feature = "test-with-testbed")]
use crate::testbed;
use crate::{device::RemoveDeviceError, get_pending_async_enrollment_dir, LOCAL_PENDING_EXT};

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
            load_pending_async_enrollment_as_available_frow_raw(path.to_path_buf(), &raw).ok()
        })
        .collect::<Vec<_>>()
        .await)
}

pub(crate) fn load_pending_async_enrollment_frow_raw(
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

pub(crate) fn load_pending_async_enrollment_as_available_frow_raw(
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
