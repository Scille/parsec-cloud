// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::{
        store::{FsPathResolutionAndManifest, GetEntryError},
        EntryStat, WorkspaceOps, WorkspaceStatEntryError,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceOpenFolderReaderError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Path points to a file")]
    EntryIsFile,
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

impl From<ConnectionError> for WorkspaceOpenFolderReaderError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FolderReaderStatEntryError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
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

impl From<ConnectionError> for FolderReaderStatEntryError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

#[derive(Debug)]
enum FolderReaderManifest {
    Workspace(Arc<LocalWorkspaceManifest>),
    Folder(Arc<LocalFolderManifest>),
}

#[derive(Debug)]
pub struct FolderReader {
    manifest: FolderReaderManifest,
}

impl FolderReader {
    // TODO: A possible future improvement here would be to read by batch in order
    //       to first ensure all children are available locally (and do a single
    //       batch request to the server if not)

    /// Return the stat of the folder itself.
    pub fn stat_folder(&self) -> EntryStat {
        match &self.manifest {
            FolderReaderManifest::Workspace(manifest) => EntryStat::Folder {
                confinement_point: None,
                id: manifest.base.id,
                parent: manifest.base.id,
                created: manifest.base.created,
                updated: manifest.updated,
                base_version: manifest.base.version,
                is_placeholder: manifest.base.version == 0,
                need_sync: manifest.need_sync,
            },
            FolderReaderManifest::Folder(manifest) => EntryStat::Folder {
                confinement_point: None,
                id: manifest.base.id,
                parent: manifest.base.parent,
                created: manifest.base.created,
                updated: manifest.updated,
                base_version: manifest.base.version,
                is_placeholder: manifest.base.version == 0,
                need_sync: manifest.need_sync,
            },
        }
    }

    /// Note children are listed in arbitrary order, and there is no '.' and '..'  special entries.
    pub async fn stat_next<'a>(
        &'a self,
        ops: &WorkspaceOps,
        mut offset: usize,
    ) -> Result<Option<(&'a EntryName, EntryStat)>, FolderReaderStatEntryError> {
        let (expected_parent_id, children) = match &self.manifest {
            FolderReaderManifest::Workspace(manifest) => (manifest.base.id, &manifest.children),
            FolderReaderManifest::Folder(manifest) => (manifest.base.id, &manifest.children),
        };

        loop {
            let (child_name, child_id) = match children.iter().nth(offset) {
                Some((child_name, child_id)) => (child_name, *child_id),
                None => return Ok(None),
            };
            let child_stat = match ops.stat_entry_by_id(child_id).await {
                Ok(stat) => stat,
                Err(err) => {
                    return Err(match err {
                        // Special case: if the entry is not found it means this child is
                        // invalid (e.g. the entry has been reparented during a move) and
                        // should just be ignored.
                        WorkspaceStatEntryError::EntryNotFound => {
                            offset += 1;
                            continue;
                        }
                        WorkspaceStatEntryError::Offline => FolderReaderStatEntryError::Offline,
                        WorkspaceStatEntryError::Stopped => FolderReaderStatEntryError::Stopped,
                        WorkspaceStatEntryError::NoRealmAccess => {
                            FolderReaderStatEntryError::NoRealmAccess
                        }
                        WorkspaceStatEntryError::InvalidKeysBundle(err) => {
                            FolderReaderStatEntryError::InvalidKeysBundle(err)
                        }
                        WorkspaceStatEntryError::InvalidCertificate(err) => {
                            FolderReaderStatEntryError::InvalidCertificate(err)
                        }
                        WorkspaceStatEntryError::InvalidManifest(err) => {
                            FolderReaderStatEntryError::InvalidManifest(err)
                        }
                        WorkspaceStatEntryError::Internal(err) => err.into(),
                    });
                }
            };

            // Last check is to ensure the parent and child manifests agree they are related,
            // if that's not the case we just ignore this child and move to the next one.
            let child_parent = match child_stat {
                EntryStat::File { parent, .. } => parent,
                EntryStat::Folder { parent, .. } => parent,
            };
            if child_parent != expected_parent_id {
                offset += 1;
                continue;
            }

            return Ok(Some((child_name, child_stat)));
        }
    }

    /// Needed by WinFSP
    pub fn get_offset_for_name(&self, name: &EntryName) -> Option<usize> {
        let children = match &self.manifest {
            FolderReaderManifest::Workspace(manifest) => &manifest.children,
            FolderReaderManifest::Folder(manifest) => &manifest.children,
        };

        children
            .iter()
            .enumerate()
            .find_map(|(offset, (child_name, _))| {
                if child_name == name {
                    Some(offset)
                } else {
                    None
                }
            })
    }
}

pub async fn open_folder_reader_by_id(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<FolderReader, WorkspaceOpenFolderReaderError> {
    if entry_id == ops.realm_id {
        let manifest = ops.store.get_workspace_manifest();

        return Ok(FolderReader {
            manifest: FolderReaderManifest::Workspace(manifest),
        });
    }

    let manifest = ops
        .store
        .get_child_manifest(entry_id)
        .await
        .map_err(|err| match err {
            GetEntryError::Offline => WorkspaceOpenFolderReaderError::Offline,
            GetEntryError::Stopped => WorkspaceOpenFolderReaderError::Stopped,
            GetEntryError::EntryNotFound => WorkspaceOpenFolderReaderError::EntryNotFound,
            GetEntryError::NoRealmAccess => WorkspaceOpenFolderReaderError::NoRealmAccess,
            GetEntryError::InvalidKeysBundle(err) => {
                WorkspaceOpenFolderReaderError::InvalidKeysBundle(err)
            }
            GetEntryError::InvalidCertificate(err) => {
                WorkspaceOpenFolderReaderError::InvalidCertificate(err)
            }
            GetEntryError::InvalidManifest(err) => {
                WorkspaceOpenFolderReaderError::InvalidManifest(err)
            }
            GetEntryError::Internal(err) => err.context("cannot resolve path").into(),
        })?;

    match manifest {
        ArcLocalChildManifest::Folder(manifest) => Ok(FolderReader {
            manifest: FolderReaderManifest::Folder(manifest),
        }),
        ArcLocalChildManifest::File(_) => Err(WorkspaceOpenFolderReaderError::EntryIsFile),
    }
}

pub async fn open_folder_reader(
    ops: &WorkspaceOps,
    path: &FsPath,
) -> Result<FolderReader, WorkspaceOpenFolderReaderError> {
    let manifest = ops
        .store
        .resolve_path_and_get_manifest(path)
        .await
        .map_err(|err| match err {
            GetEntryError::Offline => WorkspaceOpenFolderReaderError::Offline,
            GetEntryError::Stopped => WorkspaceOpenFolderReaderError::Stopped,
            GetEntryError::EntryNotFound => WorkspaceOpenFolderReaderError::EntryNotFound,
            GetEntryError::NoRealmAccess => WorkspaceOpenFolderReaderError::NoRealmAccess,
            GetEntryError::InvalidKeysBundle(err) => {
                WorkspaceOpenFolderReaderError::InvalidKeysBundle(err)
            }
            GetEntryError::InvalidCertificate(err) => {
                WorkspaceOpenFolderReaderError::InvalidCertificate(err)
            }
            GetEntryError::InvalidManifest(err) => {
                WorkspaceOpenFolderReaderError::InvalidManifest(err)
            }
            GetEntryError::Internal(err) => err.context("cannot resolve path").into(),
        })?;

    match manifest {
        FsPathResolutionAndManifest::Workspace { manifest } => Ok(FolderReader {
            manifest: FolderReaderManifest::Workspace(manifest),
        }),
        FsPathResolutionAndManifest::Folder { manifest, .. } => Ok(FolderReader {
            manifest: FolderReaderManifest::Folder(manifest),
        }),
        FsPathResolutionAndManifest::File { .. } => {
            Err(WorkspaceOpenFolderReaderError::EntryIsFile)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceStatFolderChildrenError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Path points to a file")]
    EntryIsFile,
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

impl From<ConnectionError> for WorkspaceStatFolderChildrenError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub async fn stat_folder_children(
    ops: &WorkspaceOps,
    path: &FsPath,
) -> Result<Vec<(EntryName, EntryStat)>, WorkspaceStatFolderChildrenError> {
    let reader = open_folder_reader(ops, path)
        .await
        .map_err(|err| match err {
            WorkspaceOpenFolderReaderError::Offline => WorkspaceStatFolderChildrenError::Offline,
            WorkspaceOpenFolderReaderError::Stopped => WorkspaceStatFolderChildrenError::Stopped,
            WorkspaceOpenFolderReaderError::EntryNotFound => {
                WorkspaceStatFolderChildrenError::EntryNotFound
            }
            WorkspaceOpenFolderReaderError::EntryIsFile => {
                WorkspaceStatFolderChildrenError::EntryIsFile
            }
            WorkspaceOpenFolderReaderError::NoRealmAccess => {
                WorkspaceStatFolderChildrenError::NoRealmAccess
            }
            WorkspaceOpenFolderReaderError::InvalidKeysBundle(err) => {
                WorkspaceStatFolderChildrenError::InvalidKeysBundle(err)
            }
            WorkspaceOpenFolderReaderError::InvalidCertificate(err) => {
                WorkspaceStatFolderChildrenError::InvalidCertificate(err)
            }
            WorkspaceOpenFolderReaderError::InvalidManifest(err) => {
                WorkspaceStatFolderChildrenError::InvalidManifest(err)
            }
            WorkspaceOpenFolderReaderError::Internal(err) => {
                err.context("cannot open folder reader").into()
            }
        })?;

    consume_reader(ops, reader).await
}

pub async fn stat_folder_children_by_id(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<Vec<(EntryName, EntryStat)>, WorkspaceStatFolderChildrenError> {
    let reader = open_folder_reader_by_id(ops, entry_id)
        .await
        .map_err(|err| match err {
            WorkspaceOpenFolderReaderError::Offline => WorkspaceStatFolderChildrenError::Offline,
            WorkspaceOpenFolderReaderError::Stopped => WorkspaceStatFolderChildrenError::Stopped,
            WorkspaceOpenFolderReaderError::EntryNotFound => {
                WorkspaceStatFolderChildrenError::EntryNotFound
            }
            WorkspaceOpenFolderReaderError::EntryIsFile => {
                WorkspaceStatFolderChildrenError::EntryIsFile
            }
            WorkspaceOpenFolderReaderError::NoRealmAccess => {
                WorkspaceStatFolderChildrenError::NoRealmAccess
            }
            WorkspaceOpenFolderReaderError::InvalidKeysBundle(err) => {
                WorkspaceStatFolderChildrenError::InvalidKeysBundle(err)
            }
            WorkspaceOpenFolderReaderError::InvalidCertificate(err) => {
                WorkspaceStatFolderChildrenError::InvalidCertificate(err)
            }
            WorkspaceOpenFolderReaderError::InvalidManifest(err) => {
                WorkspaceStatFolderChildrenError::InvalidManifest(err)
            }
            WorkspaceOpenFolderReaderError::Internal(err) => {
                err.context("cannot open folder reader").into()
            }
        })?;

    consume_reader(ops, reader).await
}

async fn consume_reader(
    ops: &WorkspaceOps,
    reader: FolderReader,
) -> Result<Vec<(EntryName, EntryStat)>, WorkspaceStatFolderChildrenError> {
    let mut children_stats = {
        // Manifest's children list may contains invalid entries (e.g. an entry that doesn't
        // exist, or that has a different parent that us), so it's only a hint.
        let children_hint_len = match &reader.manifest {
            FolderReaderManifest::Workspace(manifest) => manifest.children.len(),
            FolderReaderManifest::Folder(manifest) => manifest.children.len(),
        };
        Vec::with_capacity(children_hint_len)
    };

    let mut index = 0;
    loop {
        let maybe_entry = reader
            .stat_next(ops, index)
            .await
            .map_err(|err| match err {
                FolderReaderStatEntryError::Offline => WorkspaceStatFolderChildrenError::Offline,
                FolderReaderStatEntryError::Stopped => WorkspaceStatFolderChildrenError::Stopped,
                FolderReaderStatEntryError::NoRealmAccess => {
                    WorkspaceStatFolderChildrenError::NoRealmAccess
                }
                FolderReaderStatEntryError::InvalidKeysBundle(err) => {
                    WorkspaceStatFolderChildrenError::InvalidKeysBundle(err)
                }
                FolderReaderStatEntryError::InvalidCertificate(err) => {
                    WorkspaceStatFolderChildrenError::InvalidCertificate(err)
                }
                FolderReaderStatEntryError::InvalidManifest(err) => {
                    WorkspaceStatFolderChildrenError::InvalidManifest(err)
                }
                FolderReaderStatEntryError::Internal(err) => {
                    err.context("cannot stat next child").into()
                }
            })?;

        let (child_name, child_stat) = match maybe_entry {
            None => break,
            Some((child_name, child_stat)) => (child_name.to_owned(), child_stat),
        };
        children_stats.push((child_name, child_stat));
        index += 1;
    }

    Ok(children_stats)
}
