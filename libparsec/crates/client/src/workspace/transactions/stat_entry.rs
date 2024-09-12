// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::{
        store::{GetManifestError, PathConfinementPoint, ResolvePathError},
        WorkspaceOps,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryStat {
    File {
        /// The confinement point corresponds to the entry id of the parent folderish
        /// manifest that contains a child with a confined name in the path leading
        /// to our entry.
        confinement_point: Option<VlobID>,
        id: VlobID,
        parent: VlobID,
        created: DateTime,
        updated: DateTime,
        base_version: VersionInt,
        is_placeholder: bool,
        need_sync: bool,
        size: SizeInt,
    },
    // Here Folder can also be the root of the workspace (i.e. WorkspaceManifest)
    Folder {
        /// The confinement point corresponds to the entry id of the parent folderish
        /// manifest that contains a child with a confined name in the path leading
        /// to our entry.
        confinement_point: Option<VlobID>,
        id: VlobID,
        parent: VlobID,
        created: DateTime,
        updated: DateTime,
        base_version: VersionInt,
        is_placeholder: bool,
        need_sync: bool,
    },
}

impl EntryStat {
    pub fn id(&self) -> VlobID {
        match self {
            EntryStat::File { id, .. } => *id,
            EntryStat::Folder { id, .. } => *id,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceStatEntryError {
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

impl From<ConnectionError> for WorkspaceStatEntryError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub(crate) async fn stat_entry_by_id(
    ops: &WorkspaceOps,
    entry_id: VlobID,
    precomputed_confinement_point: Option<PathConfinementPoint>,
) -> Result<EntryStat, WorkspaceStatEntryError> {
    let manifest = ops
        .store
        .get_manifest(entry_id)
        .await
        .map_err(|err| match err {
            GetManifestError::Offline => WorkspaceStatEntryError::Offline,
            GetManifestError::Stopped => WorkspaceStatEntryError::Stopped,
            GetManifestError::EntryNotFound => WorkspaceStatEntryError::EntryNotFound,
            GetManifestError::NoRealmAccess => WorkspaceStatEntryError::NoRealmAccess,
            GetManifestError::InvalidKeysBundle(err) => {
                WorkspaceStatEntryError::InvalidKeysBundle(err)
            }
            GetManifestError::InvalidCertificate(err) => {
                WorkspaceStatEntryError::InvalidCertificate(err)
            }
            GetManifestError::InvalidManifest(err) => WorkspaceStatEntryError::InvalidManifest(err),
            GetManifestError::Internal(err) => err.context("cannot resolve path").into(),
        })?;

    let confinement_point = match precomputed_confinement_point {
        None => {
            ops.store
                .retrieve_path_from_id(entry_id)
                .await
                .map_err(|err| match err {
                    ResolvePathError::Offline => WorkspaceStatEntryError::Offline,
                    ResolvePathError::Stopped => WorkspaceStatEntryError::Stopped,
                    ResolvePathError::EntryNotFound => WorkspaceStatEntryError::EntryNotFound,
                    ResolvePathError::NoRealmAccess => WorkspaceStatEntryError::NoRealmAccess,
                    ResolvePathError::InvalidKeysBundle(err) => {
                        WorkspaceStatEntryError::InvalidKeysBundle(err)
                    }
                    ResolvePathError::InvalidCertificate(err) => {
                        WorkspaceStatEntryError::InvalidCertificate(err)
                    }
                    ResolvePathError::InvalidManifest(err) => {
                        WorkspaceStatEntryError::InvalidManifest(err)
                    }
                    ResolvePathError::Internal(err) => err.context("cannot retrieve path").into(),
                })?
                .1
        }
        Some(confinement_point) => confinement_point,
    };

    let info = match manifest {
        ArcLocalChildManifest::Folder(manifest) => EntryStat::Folder {
            confinement_point: confinement_point.into(),
            id: manifest.base.id,
            parent: manifest.parent,
            created: manifest.base.created,
            updated: manifest.updated,
            base_version: manifest.base.version,
            is_placeholder: manifest.base.version == 0,
            need_sync: manifest.need_sync,
        },
        ArcLocalChildManifest::File(manifest) => EntryStat::File {
            confinement_point: confinement_point.into(),
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
    ops: &WorkspaceOps,
    path: &FsPath,
) -> Result<EntryStat, WorkspaceStatEntryError> {
    let (manifest, confinement_point) =
        ops.store
            .resolve_path(path)
            .await
            .map_err(|err| match err {
                ResolvePathError::Offline => WorkspaceStatEntryError::Offline,
                ResolvePathError::Stopped => WorkspaceStatEntryError::Stopped,
                ResolvePathError::EntryNotFound => WorkspaceStatEntryError::EntryNotFound,
                ResolvePathError::NoRealmAccess => WorkspaceStatEntryError::NoRealmAccess,
                ResolvePathError::InvalidKeysBundle(err) => {
                    WorkspaceStatEntryError::InvalidKeysBundle(err)
                }
                ResolvePathError::InvalidCertificate(err) => {
                    WorkspaceStatEntryError::InvalidCertificate(err)
                }
                ResolvePathError::InvalidManifest(err) => {
                    WorkspaceStatEntryError::InvalidManifest(err)
                }
                ResolvePathError::Internal(err) => err.context("cannot resolve path").into(),
            })?;

    let info = match manifest {
        ArcLocalChildManifest::Folder(manifest) => EntryStat::Folder {
            confinement_point: confinement_point.into(),
            id: manifest.base.id,
            parent: manifest.parent,
            created: manifest.base.created,
            updated: manifest.updated,
            base_version: manifest.base.version,
            is_placeholder: manifest.base.version == 0,
            need_sync: manifest.need_sync,
        },
        ArcLocalChildManifest::File(manifest) => EntryStat::File {
            confinement_point: confinement_point.into(),
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
