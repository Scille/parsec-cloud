// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::{
        store::{
            EnsureManifestExistsWithParentError, PathConfinementPoint, ResolvePathError,
            RetrievePathFromIDEntry, RetrievePathFromIDError,
        },
        EntryStat, WorkspaceOps, WorkspaceStatEntryError,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceOpenFolderReaderError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
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

#[derive(Debug, thiserror::Error)]
pub enum FolderReaderStatEntryError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
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

#[derive(Debug)]
pub struct FolderReader {
    manifest: Arc<LocalFolderManifest>,
    confinement_point: PathConfinementPoint,
}

#[derive(Debug)]
pub enum FolderReaderStatNextOutcome<'a> {
    Entry {
        name: &'a EntryName,
        stat: EntryStat,
    },
    /// The entry listen in the parent manifest turned not to be an actual
    /// child (e.g. has been reparented)
    InvalidChild,
    NoMoreEntries,
}

impl FolderReader {
    // TODO: A possible future improvement here would be to read by batch in order
    //       to first ensure all children are available locally (and do a single
    //       batch request to the server if not)

    /// Return the stat of the folder itself.
    pub fn stat_folder(&self) -> EntryStat {
        EntryStat::Folder {
            confinement_point: self.confinement_point.into(),
            id: self.manifest.base.id,
            parent: self.manifest.base.parent,
            created: self.manifest.base.created,
            updated: self.manifest.updated,
            base_version: self.manifest.base.version,
            is_placeholder: self.manifest.base.version == 0,
            need_sync: self.manifest.need_sync,
            last_updater: self.manifest.base.author,
        }
    }

    /// Note children are listed in arbitrary order, and there is no '.' and '..'  special entries.
    pub async fn stat_child(
        &self,
        ops: &WorkspaceOps,
        index: usize,
    ) -> Result<FolderReaderStatNextOutcome<'_>, FolderReaderStatEntryError> {
        let expected_parent_id = self.manifest.base.id;

        let (child_name, child_id) = match self.manifest.children.iter().nth(index) {
            Some((child_name, child_id)) => (child_name, *child_id),
            None => return Ok(FolderReaderStatNextOutcome::NoMoreEntries),
        };
        // Check confinement point here instead of computing it in the `stat_entry_by_id` method
        // Note that the confinement point is the root-most parent that has its child with a confined
        let confinement_point = match self.confinement_point {
            PathConfinementPoint::Confined(_) => self.confinement_point,
            PathConfinementPoint::NotConfined => {
                if self.manifest.local_confinement_points.contains(&child_id) {
                    PathConfinementPoint::Confined(expected_parent_id)
                } else {
                    PathConfinementPoint::NotConfined
                }
            }
        };
        let child_stat = match ops
            .stat_entry_by_id_with_known_confinement_point(child_id, confinement_point)
            .await
        {
            Ok(stat) => stat,
            Err(err) => {
                return Err(match err {
                    // Special case: if the entry is not found it means this child is
                    // invalid (e.g. the entry has been reparented during a move) and
                    // should just be ignored.
                    WorkspaceStatEntryError::EntryNotFound => {
                        return Ok(FolderReaderStatNextOutcome::InvalidChild)
                    }
                    WorkspaceStatEntryError::Offline(e) => FolderReaderStatEntryError::Offline(e),
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
            return Ok(FolderReaderStatNextOutcome::InvalidChild);
        }

        Ok(FolderReaderStatNextOutcome::Entry {
            name: child_name,
            stat: child_stat,
        })
    }

    /// Needed by WinFSP
    pub async fn get_index_for_name(
        &self,
        ops: &WorkspaceOps,
        name: &EntryName,
    ) -> Result<Option<usize>, FolderReaderStatEntryError> {
        for (offset, (child_name, child_id)) in self.manifest.children.iter().enumerate() {
            if child_name == name {
                let maybe_child = ops
                    .store
                    .ensure_manifest_exists_with_parent(*child_id, self.manifest.base.id)
                    .await
                    .map_err(|err| match err {
                        EnsureManifestExistsWithParentError::Offline(e) => {
                            FolderReaderStatEntryError::Offline(e)
                        }
                        EnsureManifestExistsWithParentError::Stopped => {
                            FolderReaderStatEntryError::Stopped
                        }
                        EnsureManifestExistsWithParentError::NoRealmAccess => {
                            FolderReaderStatEntryError::NoRealmAccess
                        }
                        EnsureManifestExistsWithParentError::InvalidKeysBundle(err) => {
                            FolderReaderStatEntryError::InvalidKeysBundle(err)
                        }
                        EnsureManifestExistsWithParentError::InvalidCertificate(err) => {
                            FolderReaderStatEntryError::InvalidCertificate(err)
                        }
                        EnsureManifestExistsWithParentError::InvalidManifest(err) => {
                            FolderReaderStatEntryError::InvalidManifest(err)
                        }
                        EnsureManifestExistsWithParentError::Internal(err) => err.into(),
                    })?;
                if maybe_child.is_some() {
                    return Ok(Some(offset));
                }
            }
        }
        Ok(None)
    }
}

pub async fn open_folder_reader_by_id(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<FolderReader, WorkspaceOpenFolderReaderError> {
    let retrieval = ops
        .store
        .retrieve_path_from_id(entry_id)
        .await
        .map_err(|err| match err {
            RetrievePathFromIDError::Offline(e) => WorkspaceOpenFolderReaderError::Offline(e),
            RetrievePathFromIDError::Stopped => WorkspaceOpenFolderReaderError::Stopped,
            // RetrievePathFromIDError::EntryNotFound => WorkspaceOpenFolderReaderError::EntryNotFound,
            RetrievePathFromIDError::NoRealmAccess => WorkspaceOpenFolderReaderError::NoRealmAccess,
            RetrievePathFromIDError::InvalidKeysBundle(err) => {
                WorkspaceOpenFolderReaderError::InvalidKeysBundle(err)
            }
            RetrievePathFromIDError::InvalidCertificate(err) => {
                WorkspaceOpenFolderReaderError::InvalidCertificate(err)
            }
            RetrievePathFromIDError::InvalidManifest(err) => {
                WorkspaceOpenFolderReaderError::InvalidManifest(err)
            }
            RetrievePathFromIDError::Internal(err) => err.context("cannot retrieve path").into(),
        })?;
    let (manifest, confinement_point) = match retrieval {
        RetrievePathFromIDEntry::Missing => {
            return Err(WorkspaceOpenFolderReaderError::EntryNotFound)
        }
        RetrievePathFromIDEntry::Unreachable { manifest } => {
            (manifest, PathConfinementPoint::NotConfined)
        }
        RetrievePathFromIDEntry::Reachable {
            manifest,
            confinement_point: confinement,
            ..
        } => (manifest, confinement),
    };

    match manifest {
        ArcLocalChildManifest::Folder(manifest) => Ok(FolderReader {
            manifest,
            confinement_point,
        }),
        ArcLocalChildManifest::File(_) => Err(WorkspaceOpenFolderReaderError::EntryIsFile),
    }
}

pub async fn open_folder_reader(
    ops: &WorkspaceOps,
    path: &FsPath,
) -> Result<FolderReader, WorkspaceOpenFolderReaderError> {
    let (manifest, confinement_point) =
        ops.store
            .resolve_path(path)
            .await
            .map_err(|err| match err {
                ResolvePathError::Offline(e) => WorkspaceOpenFolderReaderError::Offline(e),
                ResolvePathError::Stopped => WorkspaceOpenFolderReaderError::Stopped,
                ResolvePathError::EntryNotFound => WorkspaceOpenFolderReaderError::EntryNotFound,
                ResolvePathError::NoRealmAccess => WorkspaceOpenFolderReaderError::NoRealmAccess,
                ResolvePathError::InvalidKeysBundle(err) => {
                    WorkspaceOpenFolderReaderError::InvalidKeysBundle(err)
                }
                ResolvePathError::InvalidCertificate(err) => {
                    WorkspaceOpenFolderReaderError::InvalidCertificate(err)
                }
                ResolvePathError::InvalidManifest(err) => {
                    WorkspaceOpenFolderReaderError::InvalidManifest(err)
                }
                ResolvePathError::Internal(err) => err.context("cannot resolve path").into(),
            })?;

    match manifest {
        ArcLocalChildManifest::Folder(manifest) => Ok(FolderReader {
            manifest,
            confinement_point,
        }),
        ArcLocalChildManifest::File(_) => Err(WorkspaceOpenFolderReaderError::EntryIsFile),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceStatFolderChildrenError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
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

pub async fn stat_folder_children(
    ops: &WorkspaceOps,
    path: &FsPath,
) -> Result<Vec<(EntryName, EntryStat)>, WorkspaceStatFolderChildrenError> {
    let reader = open_folder_reader(ops, path)
        .await
        .map_err(|err| match err {
            WorkspaceOpenFolderReaderError::Offline(e) => {
                WorkspaceStatFolderChildrenError::Offline(e)
            }
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
            WorkspaceOpenFolderReaderError::Offline(e) => {
                WorkspaceStatFolderChildrenError::Offline(e)
            }
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
    // Manifest's children list may contains invalid entries (e.g. an entry that doesn't
    // exist, or that has a different parent that us), so it's only a hint.
    let max_children = reader.manifest.children.len();
    let mut children_stats = Vec::with_capacity(max_children);

    for index in 0..max_children {
        let stat_outcome = reader
            .stat_child(ops, index)
            .await
            .map_err(|err| match err {
                FolderReaderStatEntryError::Offline(e) => {
                    WorkspaceStatFolderChildrenError::Offline(e)
                }
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

        match stat_outcome {
            FolderReaderStatNextOutcome::Entry {
                name: child_name,
                stat: child_stat,
            } => {
                children_stats.push((child_name.to_owned(), child_stat));
            }
            FolderReaderStatNextOutcome::InvalidChild => (),
            // Our for loop already stops before `index` reaches `max_children`
            FolderReaderStatNextOutcome::NoMoreEntries => unreachable!(),
        }
    }

    Ok(children_stats)
}
