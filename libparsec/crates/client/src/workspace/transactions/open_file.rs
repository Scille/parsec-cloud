// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_types::prelude::*;

use super::WorkspaceCreateFileError;
use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::{
        store::{ForUpdateFileError, GetEntryError},
        OpenedFile, OpenedFileCursor, ReadMode, WorkspaceOps, WriteMode,
    },
};

#[derive(Debug)]
pub struct OpenOptions {
    pub read: bool,
    pub write: bool,
    pub truncate: bool,
    pub create: bool,
    pub create_new: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceOpenFileError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Only have read access on this workspace")]
    ReadOnlyRealm,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Path points to entry that is not a file")]
    EntryNotAFile,
    #[error("Target entry already exists while in create new mode")]
    EntryExistsInCreateNewMode { entry_id: VlobID },
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for WorkspaceOpenFileError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub async fn open_file(
    ops: &WorkspaceOps,
    path: FsPath,
    options: OpenOptions,
) -> Result<(FileDescriptor, VlobID), WorkspaceOpenFileError> {
    // 0) Access control

    if options.write || options.truncate || options.create || options.create_new {
        let guard = ops
            .workspace_external_info
            .lock()
            .expect("mutex is poisoned");
        if !guard.entry.role.can_write() {
            return Err(WorkspaceOpenFileError::ReadOnlyRealm);
        }
    }

    // 1) Handle root early as it is a special case (cannot use `get_child_manifest` for it)

    if path.is_root() {
        return Err(WorkspaceOpenFileError::EntryNotAFile);
    }

    // 2) Resolve the file path (and create the file if needed)

    let entry_id = {
        let outcome = ops.store.resolve_path(&path).await;
        match outcome {
            Ok(resolution) => {
                if options.create_new {
                    return Err(WorkspaceOpenFileError::EntryExistsInCreateNewMode {
                        entry_id: resolution.entry_id,
                    });
                }
                resolution.entry_id
            }
            // Special case if the file doesn't exist but we are allowed to create it
            Err(GetEntryError::EntryNotFound) if options.create || options.create_new => {
                let outcome = super::create_file(ops, path).await;
                outcome.or_else(|err| match err {
                    // Concurrent operation has created the file in the meantime
                    WorkspaceCreateFileError::EntryExists { entry_id } => Ok(entry_id),
                    // Actual errors, republishing
                    WorkspaceCreateFileError::Offline => Err(WorkspaceOpenFileError::Offline),
                    WorkspaceCreateFileError::Stopped => Err(WorkspaceOpenFileError::Stopped),
                    WorkspaceCreateFileError::ReadOnlyRealm => {
                        Err(WorkspaceOpenFileError::ReadOnlyRealm)
                    }
                    WorkspaceCreateFileError::NoRealmAccess => {
                        Err(WorkspaceOpenFileError::NoRealmAccess)
                    }
                    WorkspaceCreateFileError::ParentNotFound => {
                        Err(WorkspaceOpenFileError::EntryNotFound)
                    }
                    WorkspaceCreateFileError::ParentIsFile => {
                        Err(WorkspaceOpenFileError::EntryNotFound)
                    }
                    WorkspaceCreateFileError::InvalidKeysBundle(err) => {
                        Err(WorkspaceOpenFileError::InvalidKeysBundle(err))
                    }
                    WorkspaceCreateFileError::InvalidCertificate(err) => {
                        Err(WorkspaceOpenFileError::InvalidCertificate(err))
                    }
                    WorkspaceCreateFileError::InvalidManifest(err) => {
                        Err(WorkspaceOpenFileError::InvalidManifest(err))
                    }
                    WorkspaceCreateFileError::Internal(err) => {
                        Err(err.context("cannot create file").into())
                    }
                })?
            }
            // Actual errors, republishing
            Err(err) => {
                return match err {
                    GetEntryError::Offline => Err(WorkspaceOpenFileError::Offline),
                    GetEntryError::Stopped => Err(WorkspaceOpenFileError::Stopped),
                    GetEntryError::EntryNotFound => Err(WorkspaceOpenFileError::EntryNotFound),
                    GetEntryError::NoRealmAccess => Err(WorkspaceOpenFileError::NoRealmAccess),
                    GetEntryError::InvalidKeysBundle(err) => {
                        Err(WorkspaceOpenFileError::InvalidKeysBundle(err))
                    }
                    GetEntryError::InvalidCertificate(err) => {
                        Err(WorkspaceOpenFileError::InvalidCertificate(err))
                    }
                    GetEntryError::InvalidManifest(err) => {
                        Err(WorkspaceOpenFileError::InvalidManifest(err))
                    }
                    GetEntryError::Internal(err) => Err(err.context("cannot resolve path").into()),
                }
            }
        }
    };

    // Handling of truncate-on-open, will be done in the next step

    let maybe_truncate_on_open =
        |file_manifest: &mut Arc<LocalFileManifest>| -> Option<Vec<ChunkID>> {
            // Handle truncate-on-open
            if options.truncate && file_manifest.size > 0 {
                let file_manifest = Arc::make_mut(file_manifest);
                let removed_chunks = super::prepare_resize(file_manifest, 0, ops.device.now());
                Some(removed_chunks.into_iter().collect())
            } else {
                None
            }
        };

    enum CursorInsertionOutcome {
        OpenedFile {
            file_descriptor: FileDescriptor,
        },
        FileAlreadyOpened {
            opened_file: Arc<AsyncMutex<OpenedFile>>,
            new_cursor: OpenedFileCursor,
        },
    }

    let cursor_insertion_outcome = loop {
        let outcome = ops.store.for_update_file(entry_id, false).await;
        let mut opened_files_guard = ops.opened_files.lock().expect("Mutex is poisoned");

        let file_descriptor = opened_files_guard.next_file_descriptor;
        let cursor = OpenedFileCursor {
            file_descriptor,
            read_mode: if options.read {
                ReadMode::Allowed
            } else {
                ReadMode::Denied
            },
            write_mode: if options.write {
                WriteMode::Allowed
            } else {
                WriteMode::Denied
            },
        };

        let cursor_insertion_outcome = match outcome {
            // The file exists and is not currently opened
            Ok((updater, mut manifest)) => {
                let (removed_chunks, flush_needed) = match maybe_truncate_on_open(&mut manifest) {
                    None => (vec![], false),
                    Some(removed_chunks) => (removed_chunks, true),
                };
                let opened_file = Arc::new(AsyncMutex::new(OpenedFile {
                    updater,
                    manifest,
                    bytes_written_since_last_flush: 0,
                    cursors: vec![cursor],
                    new_chunks: vec![],
                    removed_chunks,
                    flush_needed,
                }));
                opened_files_guard
                    .opened_files
                    .insert(entry_id, opened_file);

                CursorInsertionOutcome::OpenedFile { file_descriptor }
            }

            Err(err) => match err {
                // The file is already opened
                ForUpdateFileError::WouldBlock => {
                    match opened_files_guard.opened_files.get_mut(&entry_id) {
                        Some(opened_file) => {
                            // The opened file is behind an async mutex, which we cannot
                            // take now given we already hold the `opened_files` sync mutex.
                            // Hence we return this special outcome to delay the cursor insertion
                            // until the `opened_files` mutex is released.
                            CursorInsertionOutcome::FileAlreadyOpened {
                                opened_file: opened_file.clone(),
                                new_cursor: cursor,
                            }
                        }

                        // The file got closed in the meantime, retrying...
                        None => {
                            continue;
                        }
                    }
                }

                // Actual errors, republishing
                ForUpdateFileError::Offline => return Err(WorkspaceOpenFileError::Offline),
                ForUpdateFileError::Stopped => return Err(WorkspaceOpenFileError::Stopped),
                ForUpdateFileError::EntryNotFound => {
                    return Err(WorkspaceOpenFileError::EntryNotFound)
                }
                ForUpdateFileError::EntryNotAFile => {
                    return Err(WorkspaceOpenFileError::EntryNotAFile)
                }
                ForUpdateFileError::NoRealmAccess => {
                    return Err(WorkspaceOpenFileError::NoRealmAccess)
                }
                ForUpdateFileError::InvalidKeysBundle(err) => {
                    return Err(WorkspaceOpenFileError::InvalidKeysBundle(err))
                }
                ForUpdateFileError::InvalidCertificate(err) => {
                    return Err(WorkspaceOpenFileError::InvalidCertificate(err))
                }
                ForUpdateFileError::InvalidManifest(err) => {
                    return Err(WorkspaceOpenFileError::InvalidManifest(err))
                }
                ForUpdateFileError::Internal(err) => {
                    return Err(err.context("cannot resolve path").into())
                }
            },
        };

        // At this point our file is among the opened ones, so we should not fail anymore
        // (otherwise the file would never be able to be closed again !).

        opened_files_guard
            .file_descriptors
            .insert(file_descriptor, entry_id);
        opened_files_guard.next_file_descriptor.0 += 1;

        break cursor_insertion_outcome;
    };

    match cursor_insertion_outcome {
        CursorInsertionOutcome::OpenedFile { file_descriptor } => Ok((file_descriptor, entry_id)),
        CursorInsertionOutcome::FileAlreadyOpened {
            opened_file,
            new_cursor,
        } => {
            let file_descriptor = new_cursor.file_descriptor;
            let mut opened_file_guard = opened_file.lock().await;
            opened_file_guard.cursors.push(new_cursor);
            if let Some(removed_chunks) = maybe_truncate_on_open(&mut opened_file_guard.manifest) {
                opened_file_guard.flush_needed = true;
                opened_file_guard.removed_chunks.extend(removed_chunks);
            }
            Ok((file_descriptor, entry_id))
        }
    }
}
