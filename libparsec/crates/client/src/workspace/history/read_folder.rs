// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::{
    WorkspaceHistoryEntryStat, WorkspaceHistoryGetEntryError, WorkspaceHistoryOps,
    WorkspaceHistoryResolvePathError, WorkspaceHistoryStatEntryError,
};
use crate::certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError};

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
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct WorkspaceHistoryFolderReader {
    at: DateTime,
    manifest: Arc<LocalFolderManifest>,
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
            id: self.manifest.base.id,
            parent: self.manifest.base.parent,
            created: self.manifest.base.created,
            updated: self.manifest.updated,
            version: self.manifest.base.version,
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
        let expected_parent_id = self.manifest.base.id;

        let (child_name, child_id) = match self.manifest.children.iter().nth(index) {
            Some((child_name, child_id)) => (child_name, *child_id),
            None => return Ok(WorkspaceHistoryFolderReaderStatNextOutcome::NoMoreEntries),
        };
        let child_stat = match ops.stat_entry_by_id(self.at, child_id).await {
            Ok(stat) => stat,
            Err(err) => {
                return Err(match err {
                    // Special case: if the entry is not found it means this child is
                    // invalid (e.g. the entry has been reparented during a move) and
                    // should just be ignored.
                    WorkspaceHistoryStatEntryError::EntryNotFound => {
                        return Ok(WorkspaceHistoryFolderReaderStatNextOutcome::InvalidChild)
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
                let child_manifest = match ops.get_entry(self.at, *child_id).await {
                    Ok(child_manifest) => child_manifest,
                    Err(err) => {
                        return Err(match err {
                            WorkspaceHistoryGetEntryError::EntryNotFound => continue,
                            WorkspaceHistoryGetEntryError::Offline(e) => {
                                WorkspaceHistoryFolderReaderStatEntryError::Offline(e)
                            }
                            WorkspaceHistoryGetEntryError::Stopped => {
                                WorkspaceHistoryFolderReaderStatEntryError::Stopped
                            }
                            WorkspaceHistoryGetEntryError::NoRealmAccess => {
                                WorkspaceHistoryFolderReaderStatEntryError::NoRealmAccess
                            }
                            WorkspaceHistoryGetEntryError::InvalidKeysBundle(
                                invalid_keys_bundle_error,
                            ) => WorkspaceHistoryFolderReaderStatEntryError::InvalidKeysBundle(
                                invalid_keys_bundle_error,
                            ),
                            WorkspaceHistoryGetEntryError::InvalidCertificate(
                                invalid_certificate_error,
                            ) => WorkspaceHistoryFolderReaderStatEntryError::InvalidCertificate(
                                invalid_certificate_error,
                            ),
                            WorkspaceHistoryGetEntryError::InvalidManifest(
                                invalid_manifest_error,
                            ) => WorkspaceHistoryFolderReaderStatEntryError::InvalidManifest(
                                invalid_manifest_error,
                            ),
                            WorkspaceHistoryGetEntryError::Internal(err) => err.into(),
                        })
                    }
                };

                let actual_child = child_manifest.parent() == self.manifest.base.id;
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
    let manifest = ops.get_entry(at, entry_id).await.map_err(|err| match err {
        WorkspaceHistoryGetEntryError::Offline(e) => {
            WorkspaceHistoryOpenFolderReaderError::Offline(e)
        }
        WorkspaceHistoryGetEntryError::Stopped => WorkspaceHistoryOpenFolderReaderError::Stopped,
        WorkspaceHistoryGetEntryError::EntryNotFound => {
            WorkspaceHistoryOpenFolderReaderError::EntryNotFound
        }
        WorkspaceHistoryGetEntryError::NoRealmAccess => {
            WorkspaceHistoryOpenFolderReaderError::NoRealmAccess
        }
        WorkspaceHistoryGetEntryError::InvalidKeysBundle(err) => {
            WorkspaceHistoryOpenFolderReaderError::InvalidKeysBundle(err)
        }
        WorkspaceHistoryGetEntryError::InvalidCertificate(err) => {
            WorkspaceHistoryOpenFolderReaderError::InvalidCertificate(err)
        }
        WorkspaceHistoryGetEntryError::InvalidManifest(err) => {
            WorkspaceHistoryOpenFolderReaderError::InvalidManifest(err)
        }
        WorkspaceHistoryGetEntryError::Internal(err) => err.context("cannot retrieve path").into(),
    })?;

    match manifest {
        ArcLocalChildManifest::Folder(manifest) => {
            Ok(WorkspaceHistoryFolderReader { at, manifest })
        }
        ArcLocalChildManifest::File(_) => Err(WorkspaceHistoryOpenFolderReaderError::EntryIsFile),
    }
}

pub async fn open_folder_reader(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    path: &FsPath,
) -> Result<WorkspaceHistoryFolderReader, WorkspaceHistoryOpenFolderReaderError> {
    let manifest = ops.resolve_path(at, path).await.map_err(|err| match err {
        WorkspaceHistoryResolvePathError::Offline(e) => {
            WorkspaceHistoryOpenFolderReaderError::Offline(e)
        }
        WorkspaceHistoryResolvePathError::Stopped => WorkspaceHistoryOpenFolderReaderError::Stopped,
        WorkspaceHistoryResolvePathError::EntryNotFound => {
            WorkspaceHistoryOpenFolderReaderError::EntryNotFound
        }
        WorkspaceHistoryResolvePathError::NoRealmAccess => {
            WorkspaceHistoryOpenFolderReaderError::NoRealmAccess
        }
        WorkspaceHistoryResolvePathError::InvalidKeysBundle(err) => {
            WorkspaceHistoryOpenFolderReaderError::InvalidKeysBundle(err)
        }
        WorkspaceHistoryResolvePathError::InvalidCertificate(err) => {
            WorkspaceHistoryOpenFolderReaderError::InvalidCertificate(err)
        }
        WorkspaceHistoryResolvePathError::InvalidManifest(err) => {
            WorkspaceHistoryOpenFolderReaderError::InvalidManifest(err)
        }
        WorkspaceHistoryResolvePathError::Internal(err) => {
            err.context("cannot resolve path").into()
        }
    })?;

    match manifest {
        ArcLocalChildManifest::Folder(manifest) => {
            Ok(WorkspaceHistoryFolderReader { at, manifest })
        }
        ArcLocalChildManifest::File(_) => Err(WorkspaceHistoryOpenFolderReaderError::EntryIsFile),
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
