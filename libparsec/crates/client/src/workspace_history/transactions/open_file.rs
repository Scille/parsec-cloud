// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace_history::{
        store::{WorkspaceHistoryStoreGetEntryError, WorkspaceHistoryStoreResolvePathError},
        InvalidManifestHistoryError, WorkspaceHistoryOps,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistoryOpenFileError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Path points to an entry (ID: `{}`) that is not a file", .entry_id)]
    EntryNotAFile { entry_id: VlobID },
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    InvalidHistory(#[from] Box<InvalidManifestHistoryError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for WorkspaceHistoryOpenFileError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub async fn open_file(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    path: FsPath,
) -> Result<(FileDescriptor, VlobID), WorkspaceHistoryOpenFileError> {
    let manifest = ops
        .store
        .resolve_path(at, &path)
        .await
        .map_err(|err| match err {
            WorkspaceHistoryStoreResolvePathError::Offline => {
                WorkspaceHistoryOpenFileError::Offline
            }
            WorkspaceHistoryStoreResolvePathError::Stopped => {
                WorkspaceHistoryOpenFileError::Stopped
            }
            WorkspaceHistoryStoreResolvePathError::EntryNotFound => {
                WorkspaceHistoryOpenFileError::EntryNotFound
            }
            WorkspaceHistoryStoreResolvePathError::NoRealmAccess => {
                WorkspaceHistoryOpenFileError::NoRealmAccess
            }
            WorkspaceHistoryStoreResolvePathError::InvalidKeysBundle(invalid_keys_bundle_error) => {
                WorkspaceHistoryOpenFileError::InvalidKeysBundle(invalid_keys_bundle_error)
            }
            WorkspaceHistoryStoreResolvePathError::InvalidCertificate(
                invalid_certificate_error,
            ) => WorkspaceHistoryOpenFileError::InvalidCertificate(invalid_certificate_error),
            WorkspaceHistoryStoreResolvePathError::InvalidManifest(invalid_manifest_error) => {
                WorkspaceHistoryOpenFileError::InvalidManifest(invalid_manifest_error)
            }
            WorkspaceHistoryStoreResolvePathError::InvalidHistory(invalid_manifest_error) => {
                WorkspaceHistoryOpenFileError::InvalidHistory(invalid_manifest_error)
            }
            WorkspaceHistoryStoreResolvePathError::Internal(err) => err.into(),
        })?;

    let manifest = match manifest {
        ArcChildManifest::File(manifest) => manifest,
        ArcChildManifest::Folder(manifest) => {
            return Err(WorkspaceHistoryOpenFileError::EntryNotAFile {
                entry_id: manifest.id,
            });
        }
    };
    let entry_id = manifest.id;

    let mut cache = ops.rw.lock().expect("Mutex is poisoned");
    let fd = cache.next_file_descriptor;
    cache.next_file_descriptor.0 += 1;
    cache.opened_files.insert(fd, manifest);

    Ok((fd, entry_id))
}

pub async fn open_file_by_id(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    entry_id: VlobID,
) -> Result<FileDescriptor, WorkspaceHistoryOpenFileError> {
    let manifest = ops
        .store
        .get_entry(at, entry_id)
        .await
        .map_err(|err| match err {
            WorkspaceHistoryStoreGetEntryError::Offline => WorkspaceHistoryOpenFileError::Offline,
            WorkspaceHistoryStoreGetEntryError::Stopped => WorkspaceHistoryOpenFileError::Stopped,
            WorkspaceHistoryStoreGetEntryError::EntryNotFound => {
                WorkspaceHistoryOpenFileError::EntryNotFound
            }
            WorkspaceHistoryStoreGetEntryError::NoRealmAccess => {
                WorkspaceHistoryOpenFileError::NoRealmAccess
            }
            WorkspaceHistoryStoreGetEntryError::InvalidKeysBundle(invalid_keys_bundle_error) => {
                WorkspaceHistoryOpenFileError::InvalidKeysBundle(invalid_keys_bundle_error)
            }
            WorkspaceHistoryStoreGetEntryError::InvalidCertificate(invalid_certificate_error) => {
                WorkspaceHistoryOpenFileError::InvalidCertificate(invalid_certificate_error)
            }
            WorkspaceHistoryStoreGetEntryError::InvalidManifest(invalid_manifest_error) => {
                WorkspaceHistoryOpenFileError::InvalidManifest(invalid_manifest_error)
            }
            WorkspaceHistoryStoreGetEntryError::InvalidHistory(invalid_manifest_error) => {
                WorkspaceHistoryOpenFileError::InvalidHistory(invalid_manifest_error)
            }
            WorkspaceHistoryStoreGetEntryError::Internal(err) => err.into(),
        })?;

    let manifest = match manifest {
        ArcChildManifest::File(manifest) => manifest,
        ArcChildManifest::Folder(manifest) => {
            return Err(WorkspaceHistoryOpenFileError::EntryNotAFile {
                entry_id: manifest.id,
            });
        }
    };

    let mut cache = ops.rw.lock().expect("Mutex is poisoned");
    let fd = cache.next_file_descriptor;
    cache.next_file_descriptor.0 += 1;
    cache.opened_files.insert(fd, manifest);

    Ok(fd)
}
