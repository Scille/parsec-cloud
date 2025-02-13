// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::{WorkspaceHistoryGetEntryError, WorkspaceHistoryOps, WorkspaceHistoryResolvePathError};
use crate::certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError};

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
    Internal(#[from] anyhow::Error),
}

pub(crate) async fn stat_entry_by_id(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    entry_id: VlobID,
) -> Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError> {
    let manifest = ops.get_entry(at, entry_id).await.map_err(|err| match err {
        WorkspaceHistoryGetEntryError::Offline(e) => WorkspaceHistoryStatEntryError::Offline(e),
        WorkspaceHistoryGetEntryError::Stopped => WorkspaceHistoryStatEntryError::Stopped,
        WorkspaceHistoryGetEntryError::EntryNotFound => {
            WorkspaceHistoryStatEntryError::EntryNotFound
        }
        WorkspaceHistoryGetEntryError::NoRealmAccess => {
            WorkspaceHistoryStatEntryError::NoRealmAccess
        }
        WorkspaceHistoryGetEntryError::InvalidKeysBundle(err) => {
            WorkspaceHistoryStatEntryError::InvalidKeysBundle(err)
        }
        WorkspaceHistoryGetEntryError::InvalidCertificate(err) => {
            WorkspaceHistoryStatEntryError::InvalidCertificate(err)
        }
        WorkspaceHistoryGetEntryError::InvalidManifest(err) => {
            WorkspaceHistoryStatEntryError::InvalidManifest(err)
        }
        WorkspaceHistoryGetEntryError::Internal(err) => err.context("cannot resolve path").into(),
    })?;

    let info = match manifest {
        ArcLocalChildManifest::Folder(manifest) => WorkspaceHistoryEntryStat::Folder {
            id: manifest.base.id,
            parent: manifest.parent,
            created: manifest.base.created,
            updated: manifest.updated,
            version: manifest.base.version,
        },
        ArcLocalChildManifest::File(manifest) => WorkspaceHistoryEntryStat::File {
            id: manifest.base.id,
            parent: manifest.parent,
            created: manifest.base.created,
            updated: manifest.updated,
            version: manifest.base.version,
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
    let manifest = ops.resolve_path(at, path).await.map_err(|err| match err {
        WorkspaceHistoryResolvePathError::Offline(e) => WorkspaceHistoryStatEntryError::Offline(e),
        WorkspaceHistoryResolvePathError::Stopped => WorkspaceHistoryStatEntryError::Stopped,
        WorkspaceHistoryResolvePathError::EntryNotFound => {
            WorkspaceHistoryStatEntryError::EntryNotFound
        }
        WorkspaceHistoryResolvePathError::NoRealmAccess => {
            WorkspaceHistoryStatEntryError::NoRealmAccess
        }
        WorkspaceHistoryResolvePathError::InvalidKeysBundle(err) => {
            WorkspaceHistoryStatEntryError::InvalidKeysBundle(err)
        }
        WorkspaceHistoryResolvePathError::InvalidCertificate(err) => {
            WorkspaceHistoryStatEntryError::InvalidCertificate(err)
        }
        WorkspaceHistoryResolvePathError::InvalidManifest(err) => {
            WorkspaceHistoryStatEntryError::InvalidManifest(err)
        }
        WorkspaceHistoryResolvePathError::Internal(err) => {
            err.context("cannot resolve path").into()
        }
    })?;

    let info = match manifest {
        ArcLocalChildManifest::Folder(manifest) => WorkspaceHistoryEntryStat::Folder {
            id: manifest.base.id,
            parent: manifest.parent,
            created: manifest.base.created,
            updated: manifest.updated,
            version: manifest.base.version,
        },
        ArcLocalChildManifest::File(manifest) => WorkspaceHistoryEntryStat::File {
            id: manifest.base.id,
            parent: manifest.parent,
            created: manifest.base.created,
            updated: manifest.updated,
            version: manifest.base.version,
            size: manifest.size,
        },
    };
    Ok(info)
}
