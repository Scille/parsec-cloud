// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::{WorkspaceHistoryGetEntryError, WorkspaceHistoryOps, WorkspaceHistoryResolvePathError};
use crate::certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistoryOpenFileError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
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
    Internal(#[from] anyhow::Error),
}

pub async fn open_file(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    path: FsPath,
) -> Result<(FileDescriptor, VlobID), WorkspaceHistoryOpenFileError> {
    let manifest = ops.resolve_path(at, &path).await.map_err(|err| match err {
        WorkspaceHistoryResolvePathError::Offline(e) => WorkspaceHistoryOpenFileError::Offline(e),
        WorkspaceHistoryResolvePathError::Stopped => WorkspaceHistoryOpenFileError::Stopped,
        WorkspaceHistoryResolvePathError::EntryNotFound => {
            WorkspaceHistoryOpenFileError::EntryNotFound
        }
        WorkspaceHistoryResolvePathError::NoRealmAccess => {
            WorkspaceHistoryOpenFileError::NoRealmAccess
        }
        WorkspaceHistoryResolvePathError::InvalidKeysBundle(invalid_keys_bundle_error) => {
            WorkspaceHistoryOpenFileError::InvalidKeysBundle(invalid_keys_bundle_error)
        }
        WorkspaceHistoryResolvePathError::InvalidCertificate(invalid_certificate_error) => {
            WorkspaceHistoryOpenFileError::InvalidCertificate(invalid_certificate_error)
        }
        WorkspaceHistoryResolvePathError::InvalidManifest(invalid_manifest_error) => {
            WorkspaceHistoryOpenFileError::InvalidManifest(invalid_manifest_error)
        }
        WorkspaceHistoryResolvePathError::Internal(err) => err.into(),
    })?;

    let manifest = match manifest {
        ArcLocalChildManifest::File(manifest) => manifest,
        ArcLocalChildManifest::Folder(manifest) => {
            return Err(WorkspaceHistoryOpenFileError::EntryNotAFile {
                entry_id: manifest.base.id,
            });
        }
    };
    let entry_id = manifest.base.id;

    let mut cache = ops.cache.lock().expect("Mutex is poisoned");
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
    let manifest = ops.get_entry(at, entry_id).await.map_err(|err| match err {
        WorkspaceHistoryGetEntryError::Offline(e) => WorkspaceHistoryOpenFileError::Offline(e),
        WorkspaceHistoryGetEntryError::Stopped => WorkspaceHistoryOpenFileError::Stopped,
        WorkspaceHistoryGetEntryError::EntryNotFound => {
            WorkspaceHistoryOpenFileError::EntryNotFound
        }
        WorkspaceHistoryGetEntryError::NoRealmAccess => {
            WorkspaceHistoryOpenFileError::NoRealmAccess
        }
        WorkspaceHistoryGetEntryError::InvalidKeysBundle(invalid_keys_bundle_error) => {
            WorkspaceHistoryOpenFileError::InvalidKeysBundle(invalid_keys_bundle_error)
        }
        WorkspaceHistoryGetEntryError::InvalidCertificate(invalid_certificate_error) => {
            WorkspaceHistoryOpenFileError::InvalidCertificate(invalid_certificate_error)
        }
        WorkspaceHistoryGetEntryError::InvalidManifest(invalid_manifest_error) => {
            WorkspaceHistoryOpenFileError::InvalidManifest(invalid_manifest_error)
        }
        WorkspaceHistoryGetEntryError::Internal(err) => err.into(),
    })?;

    let manifest = match manifest {
        ArcLocalChildManifest::File(manifest) => manifest,
        ArcLocalChildManifest::Folder(manifest) => {
            return Err(WorkspaceHistoryOpenFileError::EntryNotAFile {
                entry_id: manifest.base.id,
            });
        }
    };

    let mut cache = ops.cache.lock().expect("Mutex is poisoned");
    let fd = cache.next_file_descriptor;
    cache.next_file_descriptor.0 += 1;
    cache.opened_files.insert(fd, manifest);

    Ok(fd)
}
