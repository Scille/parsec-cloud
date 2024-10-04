// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::{WorkspaceHistoryGetEntryError, WorkspaceHistoryOps, WorkspaceHistoryResolvePathError};
use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::EntryStat,
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistoryStatEntryError {
    #[error("Cannot reach the server")]
    Offline,
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

impl From<ConnectionError> for WorkspaceHistoryStatEntryError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub(crate) async fn stat_entry_by_id(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    entry_id: VlobID,
) -> Result<EntryStat, WorkspaceHistoryStatEntryError> {
    let manifest = ops.get_entry(at, entry_id).await.map_err(|err| match err {
        WorkspaceHistoryGetEntryError::Offline => WorkspaceHistoryStatEntryError::Offline,
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
        ArcLocalChildManifest::Folder(manifest) => EntryStat::Folder {
            confinement_point: None,
            id: manifest.base.id,
            parent: manifest.parent,
            created: manifest.base.created,
            updated: manifest.updated,
            base_version: manifest.base.version,
            is_placeholder: manifest.base.version == 0,
            need_sync: manifest.need_sync,
        },
        ArcLocalChildManifest::File(manifest) => EntryStat::File {
            confinement_point: None,
            id: manifest.base.id,
            parent: manifest.parent,
            created: manifest.base.created,
            updated: manifest.updated,
            base_version: manifest.base.version,
            is_placeholder: manifest.base.version == 0,
            need_sync: manifest.need_sync,
            size: manifest.size,
        },
    };

    Ok(info)
}

pub(crate) async fn stat_entry(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    path: &FsPath,
) -> Result<EntryStat, WorkspaceHistoryStatEntryError> {
    let manifest = ops.resolve_path(at, path).await.map_err(|err| match err {
        WorkspaceHistoryResolvePathError::Offline => WorkspaceHistoryStatEntryError::Offline,
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
        ArcLocalChildManifest::Folder(manifest) => EntryStat::Folder {
            confinement_point: None,
            id: manifest.base.id,
            parent: manifest.parent,
            created: manifest.base.created,
            updated: manifest.updated,
            base_version: manifest.base.version,
            is_placeholder: manifest.base.version == 0,
            need_sync: manifest.need_sync,
        },
        ArcLocalChildManifest::File(manifest) => EntryStat::File {
            confinement_point: None,
            id: manifest.base.id,
            parent: manifest.parent,
            created: manifest.base.created,
            updated: manifest.updated,
            base_version: manifest.base.version,
            is_placeholder: manifest.base.version == 0,
            need_sync: manifest.need_sync,
            size: manifest.size,
        },
    };
    Ok(info)
}
