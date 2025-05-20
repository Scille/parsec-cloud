// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace_history::{
        InvalidManifestHistoryError, WorkspaceHistoryEntryStat, WorkspaceHistoryOps,
        WorkspaceHistoryStatEntryError,
        store::{WorkspaceHistoryStoreGetEntryError, WorkspaceHistoryStoreResolvePathError},
    },
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistoryOpenFolderReaderError {
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
    InvalidHistory(#[from] Box<InvalidManifestHistoryError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistoryFolderReaderStatEntryError {
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
    InvalidHistory(#[from] Box<InvalidManifestHistoryError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct WorkspaceHistoryFolderReader {
    at: DateTime,
    manifest: Arc<FolderManifest>,
}

#[derive(Debug)]
pub enum WorkspaceHistoryFolderReaderStatNextOutcome<'a> {
    Entry {
        name: &'a EntryName,
        stat: WorkspaceHistoryEntryStat,
    },
    /// The entry listen in the parent manifest turned not to be an actual
    /// child (e.g. has been reparented)
    InvalidChild,
    NoMoreEntries,
}

impl WorkspaceHistoryFolderReader {
    // TODO: A possible future improvement here would be to read by batch in order
    //       to first ensure all children are available locally (and do a single
    //       batch request to the server if not)

    /// Return the stat of the folder itself.
    pub fn stat_folder(&self) -> WorkspaceHistoryEntryStat {
        WorkspaceHistoryEntryStat::Folder {
            id: self.manifest.id,
            parent: self.manifest.parent,
            created: self.manifest.created,
            updated: self.manifest.updated,
            version: self.manifest.version,
        }
    }

    /// Note children are listed in arbitrary order, and there is no '.' and '..'  special entries.
    pub async fn stat_child(
        &self,
        ops: &WorkspaceHistoryOps,
        index: usize,
    ) -> Result<
        WorkspaceHistoryFolderReaderStatNextOutcome,
        WorkspaceHistoryFolderReaderStatEntryError,
    > {
        let expected_parent_id = self.manifest.id;

        let (child_name, child_id) = match self.manifest.children.iter().nth(index) {
            Some((child_name, child_id)) => (child_name, *child_id),
            None => return Ok(WorkspaceHistoryFolderReaderStatNextOutcome::NoMoreEntries),
        };
        let child_stat = match super::stat_entry_by_id(ops, self.at, child_id).await {
            Ok(stat) => stat,
            Err(err) => {
                return Err(match err {
                    // Special case: if the entry is not found it means this child is
                    // invalid (e.g. the entry has been reparented during a move) and
                    // should just be ignored.
                    WorkspaceHistoryStatEntryError::EntryNotFound => {
                        return Ok(WorkspaceHistoryFolderReaderStatNextOutcome::InvalidChild);
                    }
                    WorkspaceHistoryStatEntryError::Offline(e) => {
                        WorkspaceHistoryFolderReaderStatEntryError::Offline(e)
                    }
                    WorkspaceHistoryStatEntryError::Stopped => {
                        WorkspaceHistoryFolderReaderStatEntryError::Stopped
                    }
                    WorkspaceHistoryStatEntryError::NoRealmAccess => {
                        WorkspaceHistoryFolderReaderStatEntryError::NoRealmAccess
                    }
                    WorkspaceHistoryStatEntryError::InvalidKeysBundle(err) => {
                        WorkspaceHistoryFolderReaderStatEntryError::InvalidKeysBundle(err)
                    }
                    WorkspaceHistoryStatEntryError::InvalidCertificate(err) => {
                        WorkspaceHistoryFolderReaderStatEntryError::InvalidCertificate(err)
                    }
                    WorkspaceHistoryStatEntryError::InvalidManifest(err) => {
                        WorkspaceHistoryFolderReaderStatEntryError::InvalidManifest(err)
                    }
                    WorkspaceHistoryStatEntryError::InvalidHistory(err) => {
                        WorkspaceHistoryFolderReaderStatEntryError::InvalidHistory(err)
                    }
                    WorkspaceHistoryStatEntryError::Internal(err) => err.into(),
                });
            }
        };

        // Last check is to ensure the parent and child manifests agree they are related,
        // if that's not the case we just ignore this child and move to the next one.
        let child_parent = match child_stat {
            WorkspaceHistoryEntryStat::File { parent, .. } => parent,
            WorkspaceHistoryEntryStat::Folder { parent, .. } => parent,
        };
        if child_parent != expected_parent_id {
            return Ok(WorkspaceHistoryFolderReaderStatNextOutcome::InvalidChild);
        }

        Ok(WorkspaceHistoryFolderReaderStatNextOutcome::Entry {
            name: child_name,
            stat: child_stat,
        })
    }

    /// Needed by WinFSP
    pub async fn get_index_for_name(
        &self,
        ops: &WorkspaceHistoryOps,
        name: &EntryName,
    ) -> Result<Option<usize>, WorkspaceHistoryFolderReaderStatEntryError> {
        for (offset, (child_name, child_id)) in self.manifest.children.iter().enumerate() {
            if child_name == name {
                let child_manifest = match ops.store.get_entry(self.at, *child_id).await {
                    Ok(child_manifest) => child_manifest,
                    Err(err) => {
                        return Err(match err {
                            WorkspaceHistoryStoreGetEntryError::EntryNotFound => continue,
                            WorkspaceHistoryStoreGetEntryError::Offline(e) => {
                                WorkspaceHistoryFolderReaderStatEntryError::Offline(e)
                            }
                            WorkspaceHistoryStoreGetEntryError::Stopped => {
                                WorkspaceHistoryFolderReaderStatEntryError::Stopped
                            }
                            WorkspaceHistoryStoreGetEntryError::NoRealmAccess => {
                                WorkspaceHistoryFolderReaderStatEntryError::NoRealmAccess
                            }
                            WorkspaceHistoryStoreGetEntryError::InvalidKeysBundle(
                                invalid_keys_bundle_error,
                            ) => WorkspaceHistoryFolderReaderStatEntryError::InvalidKeysBundle(
                                invalid_keys_bundle_error,
                            ),
                            WorkspaceHistoryStoreGetEntryError::InvalidCertificate(
                                invalid_certificate_error,
                            ) => WorkspaceHistoryFolderReaderStatEntryError::InvalidCertificate(
                                invalid_certificate_error,
                            ),
                            WorkspaceHistoryStoreGetEntryError::InvalidManifest(
                                invalid_manifest_error,
                            ) => WorkspaceHistoryFolderReaderStatEntryError::InvalidManifest(
                                invalid_manifest_error,
                            ),
                            WorkspaceHistoryStoreGetEntryError::InvalidHistory(
                                invalid_manifest_error,
                            ) => WorkspaceHistoryFolderReaderStatEntryError::InvalidHistory(
                                invalid_manifest_error,
                            ),
                            WorkspaceHistoryStoreGetEntryError::Internal(err) => err.into(),
                        });
                    }
                };

                let actual_child = child_manifest.parent() == self.manifest.id;
                if actual_child {
                    return Ok(Some(offset));
                }
            }
        }
        Ok(None)
    }
}

pub async fn open_folder_reader_by_id(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    entry_id: VlobID,
) -> Result<WorkspaceHistoryFolderReader, WorkspaceHistoryOpenFolderReaderError> {
    let manifest = ops
        .store
        .get_entry(at, entry_id)
        .await
        .map_err(|err| match err {
            WorkspaceHistoryStoreGetEntryError::Offline(e) => {
                WorkspaceHistoryOpenFolderReaderError::Offline(e)
            }
            WorkspaceHistoryStoreGetEntryError::Stopped => {
                WorkspaceHistoryOpenFolderReaderError::Stopped
            }
            WorkspaceHistoryStoreGetEntryError::EntryNotFound => {
                WorkspaceHistoryOpenFolderReaderError::EntryNotFound
            }
            WorkspaceHistoryStoreGetEntryError::NoRealmAccess => {
                WorkspaceHistoryOpenFolderReaderError::NoRealmAccess
            }
            WorkspaceHistoryStoreGetEntryError::InvalidKeysBundle(err) => {
                WorkspaceHistoryOpenFolderReaderError::InvalidKeysBundle(err)
            }
            WorkspaceHistoryStoreGetEntryError::InvalidCertificate(err) => {
                WorkspaceHistoryOpenFolderReaderError::InvalidCertificate(err)
            }
            WorkspaceHistoryStoreGetEntryError::InvalidManifest(err) => {
                WorkspaceHistoryOpenFolderReaderError::InvalidManifest(err)
            }
            WorkspaceHistoryStoreGetEntryError::InvalidHistory(err) => {
                WorkspaceHistoryOpenFolderReaderError::InvalidHistory(err)
            }
            WorkspaceHistoryStoreGetEntryError::Internal(err) => {
                err.context("cannot retrieve path").into()
            }
        })?;

    match manifest {
        ArcChildManifest::Folder(manifest) => Ok(WorkspaceHistoryFolderReader { at, manifest }),
        ArcChildManifest::File(_) => Err(WorkspaceHistoryOpenFolderReaderError::EntryIsFile),
    }
}

pub async fn open_folder_reader(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    path: &FsPath,
) -> Result<WorkspaceHistoryFolderReader, WorkspaceHistoryOpenFolderReaderError> {
    let manifest = ops
        .store
        .resolve_path(at, path)
        .await
        .map_err(|err| match err {
            WorkspaceHistoryStoreResolvePathError::Offline(e) => {
                WorkspaceHistoryOpenFolderReaderError::Offline(e)
            }
            WorkspaceHistoryStoreResolvePathError::Stopped => {
                WorkspaceHistoryOpenFolderReaderError::Stopped
            }
            WorkspaceHistoryStoreResolvePathError::EntryNotFound => {
                WorkspaceHistoryOpenFolderReaderError::EntryNotFound
            }
            WorkspaceHistoryStoreResolvePathError::NoRealmAccess => {
                WorkspaceHistoryOpenFolderReaderError::NoRealmAccess
            }
            WorkspaceHistoryStoreResolvePathError::InvalidKeysBundle(err) => {
                WorkspaceHistoryOpenFolderReaderError::InvalidKeysBundle(err)
            }
            WorkspaceHistoryStoreResolvePathError::InvalidCertificate(err) => {
                WorkspaceHistoryOpenFolderReaderError::InvalidCertificate(err)
            }
            WorkspaceHistoryStoreResolvePathError::InvalidManifest(err) => {
                WorkspaceHistoryOpenFolderReaderError::InvalidManifest(err)
            }
            WorkspaceHistoryStoreResolvePathError::InvalidHistory(err) => {
                WorkspaceHistoryOpenFolderReaderError::InvalidHistory(err)
            }
            WorkspaceHistoryStoreResolvePathError::Internal(err) => {
                err.context("cannot resolve path").into()
            }
        })?;

    match manifest {
        ArcChildManifest::Folder(manifest) => Ok(WorkspaceHistoryFolderReader { at, manifest }),
        ArcChildManifest::File(_) => Err(WorkspaceHistoryOpenFolderReaderError::EntryIsFile),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistoryStatFolderChildrenError {
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
    InvalidHistory(#[from] Box<InvalidManifestHistoryError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn stat_folder_children(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    path: &FsPath,
) -> Result<Vec<(EntryName, WorkspaceHistoryEntryStat)>, WorkspaceHistoryStatFolderChildrenError> {
    let reader = open_folder_reader(ops, at, path)
        .await
        .map_err(|err| match err {
            WorkspaceHistoryOpenFolderReaderError::Offline(e) => {
                WorkspaceHistoryStatFolderChildrenError::Offline(e)
            }
            WorkspaceHistoryOpenFolderReaderError::Stopped => {
                WorkspaceHistoryStatFolderChildrenError::Stopped
            }
            WorkspaceHistoryOpenFolderReaderError::EntryNotFound => {
                WorkspaceHistoryStatFolderChildrenError::EntryNotFound
            }
            WorkspaceHistoryOpenFolderReaderError::EntryIsFile => {
                WorkspaceHistoryStatFolderChildrenError::EntryIsFile
            }
            WorkspaceHistoryOpenFolderReaderError::NoRealmAccess => {
                WorkspaceHistoryStatFolderChildrenError::NoRealmAccess
            }
            WorkspaceHistoryOpenFolderReaderError::InvalidKeysBundle(err) => {
                WorkspaceHistoryStatFolderChildrenError::InvalidKeysBundle(err)
            }
            WorkspaceHistoryOpenFolderReaderError::InvalidCertificate(err) => {
                WorkspaceHistoryStatFolderChildrenError::InvalidCertificate(err)
            }
            WorkspaceHistoryOpenFolderReaderError::InvalidManifest(err) => {
                WorkspaceHistoryStatFolderChildrenError::InvalidManifest(err)
            }
            WorkspaceHistoryOpenFolderReaderError::InvalidHistory(err) => {
                WorkspaceHistoryStatFolderChildrenError::InvalidHistory(err)
            }
            WorkspaceHistoryOpenFolderReaderError::Internal(err) => {
                err.context("cannot open folder reader").into()
            }
        })?;

    consume_reader(ops, reader).await
}

pub async fn stat_folder_children_by_id(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    entry_id: VlobID,
) -> Result<Vec<(EntryName, WorkspaceHistoryEntryStat)>, WorkspaceHistoryStatFolderChildrenError> {
    let reader = open_folder_reader_by_id(ops, at, entry_id)
        .await
        .map_err(|err| match err {
            WorkspaceHistoryOpenFolderReaderError::Offline(e) => {
                WorkspaceHistoryStatFolderChildrenError::Offline(e)
            }
            WorkspaceHistoryOpenFolderReaderError::Stopped => {
                WorkspaceHistoryStatFolderChildrenError::Stopped
            }
            WorkspaceHistoryOpenFolderReaderError::EntryNotFound => {
                WorkspaceHistoryStatFolderChildrenError::EntryNotFound
            }
            WorkspaceHistoryOpenFolderReaderError::EntryIsFile => {
                WorkspaceHistoryStatFolderChildrenError::EntryIsFile
            }
            WorkspaceHistoryOpenFolderReaderError::NoRealmAccess => {
                WorkspaceHistoryStatFolderChildrenError::NoRealmAccess
            }
            WorkspaceHistoryOpenFolderReaderError::InvalidKeysBundle(err) => {
                WorkspaceHistoryStatFolderChildrenError::InvalidKeysBundle(err)
            }
            WorkspaceHistoryOpenFolderReaderError::InvalidCertificate(err) => {
                WorkspaceHistoryStatFolderChildrenError::InvalidCertificate(err)
            }
            WorkspaceHistoryOpenFolderReaderError::InvalidManifest(err) => {
                WorkspaceHistoryStatFolderChildrenError::InvalidManifest(err)
            }
            WorkspaceHistoryOpenFolderReaderError::InvalidHistory(err) => {
                WorkspaceHistoryStatFolderChildrenError::InvalidHistory(err)
            }
            WorkspaceHistoryOpenFolderReaderError::Internal(err) => {
                err.context("cannot open folder reader").into()
            }
        })?;

    consume_reader(ops, reader).await
}

async fn consume_reader(
    ops: &WorkspaceHistoryOps,
    reader: WorkspaceHistoryFolderReader,
) -> Result<Vec<(EntryName, WorkspaceHistoryEntryStat)>, WorkspaceHistoryStatFolderChildrenError> {
    // Manifest's children list may contains invalid entries (e.g. an entry that doesn't
    // exist, or that has a different parent that us), so it's only a hint.
    let max_children = reader.manifest.children.len();
    let mut children_stats = Vec::with_capacity(max_children);

    for index in 0..max_children {
        let stat_outcome = reader
            .stat_child(ops, index)
            .await
            .map_err(|err| match err {
                WorkspaceHistoryFolderReaderStatEntryError::Offline(e) => {
                    WorkspaceHistoryStatFolderChildrenError::Offline(e)
                }
                WorkspaceHistoryFolderReaderStatEntryError::Stopped => {
                    WorkspaceHistoryStatFolderChildrenError::Stopped
                }
                WorkspaceHistoryFolderReaderStatEntryError::NoRealmAccess => {
                    WorkspaceHistoryStatFolderChildrenError::NoRealmAccess
                }
                WorkspaceHistoryFolderReaderStatEntryError::InvalidKeysBundle(err) => {
                    WorkspaceHistoryStatFolderChildrenError::InvalidKeysBundle(err)
                }
                WorkspaceHistoryFolderReaderStatEntryError::InvalidCertificate(err) => {
                    WorkspaceHistoryStatFolderChildrenError::InvalidCertificate(err)
                }
                WorkspaceHistoryFolderReaderStatEntryError::InvalidManifest(err) => {
                    WorkspaceHistoryStatFolderChildrenError::InvalidManifest(err)
                }
                WorkspaceHistoryFolderReaderStatEntryError::InvalidHistory(err) => {
                    WorkspaceHistoryStatFolderChildrenError::InvalidHistory(err)
                }
                WorkspaceHistoryFolderReaderStatEntryError::Internal(err) => {
                    err.context("cannot stat next child").into()
                }
            })?;

        match stat_outcome {
            WorkspaceHistoryFolderReaderStatNextOutcome::Entry {
                name: child_name,
                stat: child_stat,
            } => {
                children_stats.push((child_name.to_owned(), child_stat));
            }
            WorkspaceHistoryFolderReaderStatNextOutcome::InvalidChild => (),
            // Our for loop already stops before `index` reaches `max_children`
            WorkspaceHistoryFolderReaderStatNextOutcome::NoMoreEntries => unreachable!(),
        }
    }

    Ok(children_stats)
}
