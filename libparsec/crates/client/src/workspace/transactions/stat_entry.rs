// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::{
        store::{FsPathResolutionAndManifest, GetEntryError},
        WorkspaceOps,
    },
};

#[derive(Debug, Clone)]
pub enum EntryStat {
    File {
        /// The confinement point corresponds to the entry id of the parent folderish
        /// manifest that contains a child with a confined name in the path leading
        /// to our entry.
        confinement_point: Option<VlobID>,
        id: VlobID,
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
        created: DateTime,
        updated: DateTime,
        base_version: VersionInt,
        is_placeholder: bool,
        need_sync: bool,
        children: Vec<(EntryName, VlobID)>,
    },
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
) -> Result<EntryStat, WorkspaceStatEntryError> {
    if entry_id == ops.realm_id {
        let manifest = ops.store.get_workspace_manifest();

        // Ensure children are sorted alphabetically to simplify testing
        let children = {
            let mut children: Vec<_> = manifest
                .children
                .iter()
                .map(|(name, id)| (name.to_owned(), *id))
                .collect();
            children.sort_unstable_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));
            children
        };

        let info = EntryStat::Folder {
            confinement_point: None,
            id: manifest.base.id,
            created: manifest.base.created,
            updated: manifest.updated,
            base_version: manifest.base.version,
            is_placeholder: manifest.base.version == 0,
            need_sync: manifest.need_sync,
            children,
        };

        return Ok(info);
    }

    let manifest = ops
        .store
        .get_child_manifest(entry_id)
        .await
        .map_err(|err| match err {
            GetEntryError::Offline => WorkspaceStatEntryError::Offline,
            GetEntryError::Stopped => WorkspaceStatEntryError::Stopped,
            GetEntryError::EntryNotFound => WorkspaceStatEntryError::EntryNotFound,
            GetEntryError::NoRealmAccess => WorkspaceStatEntryError::NoRealmAccess,
            GetEntryError::InvalidKeysBundle(err) => {
                WorkspaceStatEntryError::InvalidKeysBundle(err)
            }
            GetEntryError::InvalidCertificate(err) => {
                WorkspaceStatEntryError::InvalidCertificate(err)
            }
            GetEntryError::InvalidManifest(err) => WorkspaceStatEntryError::InvalidManifest(err),
            GetEntryError::Internal(err) => err.context("cannot resolve path").into(),
        })?;

    let info = match manifest {
        ArcLocalChildManifest::Folder(manifest) => {
            // Ensure children are sorted alphabetically to simplify testing
            let children = {
                let mut children: Vec<_> = manifest
                    .children
                    .iter()
                    .map(|(name, id)| (name.to_owned(), *id))
                    .collect();
                children.sort_unstable_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));
                children
            };

            EntryStat::Folder {
                confinement_point: None,
                id: manifest.base.id,
                created: manifest.base.created,
                updated: manifest.updated,
                base_version: manifest.base.version,
                is_placeholder: manifest.base.version == 0,
                need_sync: manifest.need_sync,
                children,
            }
        }
        ArcLocalChildManifest::File(manifest) => EntryStat::File {
            confinement_point: None,
            id: manifest.base.id,
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
    let manifest = ops
        .store
        .resolve_path_and_get_manifest(path)
        .await
        .map_err(|err| match err {
            GetEntryError::Offline => WorkspaceStatEntryError::Offline,
            GetEntryError::Stopped => WorkspaceStatEntryError::Stopped,
            GetEntryError::EntryNotFound => WorkspaceStatEntryError::EntryNotFound,
            GetEntryError::NoRealmAccess => WorkspaceStatEntryError::NoRealmAccess,
            GetEntryError::InvalidKeysBundle(err) => {
                WorkspaceStatEntryError::InvalidKeysBundle(err)
            }
            GetEntryError::InvalidCertificate(err) => {
                WorkspaceStatEntryError::InvalidCertificate(err)
            }
            GetEntryError::InvalidManifest(err) => WorkspaceStatEntryError::InvalidManifest(err),
            GetEntryError::Internal(err) => err.context("cannot resolve path").into(),
        })?;

    let info = match manifest {
        FsPathResolutionAndManifest::Workspace { manifest } => {
            // Ensure children are sorted alphabetically to simplify testing
            let children = {
                let mut children: Vec<_> = manifest
                    .children
                    .iter()
                    .map(|(name, id)| (name.to_owned(), *id))
                    .collect();
                children.sort_unstable_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));
                children
            };

            EntryStat::Folder {
                // Root has no parent, hence confinement_point is never possible
                confinement_point: None,
                id: manifest.base.id,
                created: manifest.base.created,
                updated: manifest.updated,
                base_version: manifest.base.version,
                is_placeholder: manifest.base.version == 0,
                need_sync: manifest.need_sync,
                children,
            }
        }

        FsPathResolutionAndManifest::Folder {
            manifest,
            confinement_point,
        } => {
            // Ensure children are sorted alphabetically to simplify testing
            let children = {
                let mut children: Vec<_> = manifest
                    .children
                    .iter()
                    .map(|(name, id)| (name.to_owned(), *id))
                    .collect();
                children.sort_unstable_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));
                children
            };

            EntryStat::Folder {
                confinement_point,
                id: manifest.base.id,
                created: manifest.base.created,
                updated: manifest.updated,
                base_version: manifest.base.version,
                is_placeholder: manifest.base.version == 0,
                need_sync: manifest.need_sync,
                children,
            }
        }

        FsPathResolutionAndManifest::File {
            manifest,
            confinement_point,
        } => EntryStat::File {
            confinement_point,
            id: manifest.base.id,
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
