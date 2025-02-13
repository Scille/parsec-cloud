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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceHistoryEntryStat {
    File {
        id: VlobID,
        parent: VlobID,
        created: DateTime,
        updated: DateTime,
        version: VersionInt,
        size: SizeInt,
    },
    // Here Folder can also be the root of the workspace (i.e. WorkspaceManifest)
    Folder {
        id: VlobID,
        parent: VlobID,
        created: DateTime,
        updated: DateTime,
        version: VersionInt,
    },
}

impl WorkspaceHistoryEntryStat {
    pub fn id(&self) -> VlobID {
        match self {
            WorkspaceHistoryEntryStat::File { id, .. } => *id,
            WorkspaceHistoryEntryStat::Folder { id, .. } => *id,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistoryStatEntryError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
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

pub(crate) async fn stat_entry_by_id(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    entry_id: VlobID,
) -> Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError> {
    let manifest = ops
        .store
        .get_entry(at, entry_id)
        .await
        .map_err(|err| match err {
            WorkspaceHistoryStoreGetEntryError::Offline(e) => {
                WorkspaceHistoryStatEntryError::Offline(e)
            }
            WorkspaceHistoryStoreGetEntryError::Stopped => WorkspaceHistoryStatEntryError::Stopped,
            WorkspaceHistoryStoreGetEntryError::EntryNotFound => {
                WorkspaceHistoryStatEntryError::EntryNotFound
            }
            WorkspaceHistoryStoreGetEntryError::NoRealmAccess => {
                WorkspaceHistoryStatEntryError::NoRealmAccess
            }
            WorkspaceHistoryStoreGetEntryError::InvalidKeysBundle(err) => {
                WorkspaceHistoryStatEntryError::InvalidKeysBundle(err)
            }
            WorkspaceHistoryStoreGetEntryError::InvalidCertificate(err) => {
                WorkspaceHistoryStatEntryError::InvalidCertificate(err)
            }
            WorkspaceHistoryStoreGetEntryError::InvalidManifest(err) => {
                WorkspaceHistoryStatEntryError::InvalidManifest(err)
            }
            WorkspaceHistoryStoreGetEntryError::InvalidHistory(err) => {
                WorkspaceHistoryStatEntryError::InvalidHistory(err)
            }
            WorkspaceHistoryStoreGetEntryError::Internal(err) => {
                err.context("cannot resolve path").into()
            }
        })?;

    let info = match manifest {
        ArcChildManifest::Folder(manifest) => WorkspaceHistoryEntryStat::Folder {
            id: manifest.id,
            parent: manifest.parent,
            created: manifest.created,
            updated: manifest.updated,
            version: manifest.version,
        },
        ArcChildManifest::File(manifest) => WorkspaceHistoryEntryStat::File {
            id: manifest.id,
            parent: manifest.parent,
            created: manifest.created,
            updated: manifest.updated,
            version: manifest.version,
            size: manifest.size,
        },
    };

    Ok(info)
}

pub(crate) async fn stat_entry(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    path: &FsPath,
) -> Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError> {
    let manifest = ops
        .store
        .resolve_path(at, path)
        .await
        .map_err(|err| match err {
            WorkspaceHistoryStoreResolvePathError::Offline(e) => {
                WorkspaceHistoryStatEntryError::Offline(e)
            }
            WorkspaceHistoryStoreResolvePathError::Stopped => {
                WorkspaceHistoryStatEntryError::Stopped
            }
            WorkspaceHistoryStoreResolvePathError::EntryNotFound => {
                WorkspaceHistoryStatEntryError::EntryNotFound
            }
            WorkspaceHistoryStoreResolvePathError::NoRealmAccess => {
                WorkspaceHistoryStatEntryError::NoRealmAccess
            }
            WorkspaceHistoryStoreResolvePathError::InvalidKeysBundle(err) => {
                WorkspaceHistoryStatEntryError::InvalidKeysBundle(err)
            }
            WorkspaceHistoryStoreResolvePathError::InvalidCertificate(err) => {
                WorkspaceHistoryStatEntryError::InvalidCertificate(err)
            }
            WorkspaceHistoryStoreResolvePathError::InvalidManifest(err) => {
                WorkspaceHistoryStatEntryError::InvalidManifest(err)
            }
            WorkspaceHistoryStoreResolvePathError::InvalidHistory(err) => {
                WorkspaceHistoryStatEntryError::InvalidHistory(err)
            }
            WorkspaceHistoryStoreResolvePathError::Internal(err) => {
                err.context("cannot resolve path").into()
            }
        })?;

    let info = match manifest {
        ArcChildManifest::Folder(manifest) => WorkspaceHistoryEntryStat::Folder {
            id: manifest.id,
            parent: manifest.parent,
            created: manifest.created,
            updated: manifest.updated,
            version: manifest.version,
        },
        ArcChildManifest::File(manifest) => WorkspaceHistoryEntryStat::File {
            id: manifest.id,
            parent: manifest.parent,
            created: manifest.created,
            updated: manifest.updated,
            version: manifest.version,
            size: manifest.size,
        },
    };
    Ok(info)
}
