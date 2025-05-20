// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_types::prelude::*;

use super::WorkspaceCreateFileError;
use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::{
        FileUpdater, OpenedFile, OpenedFileCursor, ReadMode, WorkspaceOps, WriteMode,
        store::{ForUpdateFileError, ResolvePathError},
    },
};

#[derive(Debug)]
pub struct OpenOptions {
    /// Open the file for reading
    pub read: bool,
    /// Open the file for writing
    pub write: bool,
    /// Truncate the file to 0 bytes after opening
    pub truncate: bool,
    /// Allow to create a new file if it doesn't exist
    pub create: bool,
    /// Require to create a new file, failing if the file already exists
    pub create_new: bool,
}

impl OpenOptions {
    pub fn read_only() -> Self {
        Self {
            read: true,
            write: false,
            truncate: false,
            create: false,
            create_new: false,
        }
    }

    pub fn read_write() -> Self {
        Self {
            read: true,
            write: true,
            truncate: false,
            create: false,
            create_new: false,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceOpenFileError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Only have read access on this workspace")]
    ReadOnlyRealm,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Path points to an entry (ID: `{}`) that is not a file", .entry_id)]
    EntryNotAFile { entry_id: VlobID },
    #[error("Target entry already exists (ID: `{}`) while in create new mode", .entry_id)]
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

pub async fn open_file(
    ops: &WorkspaceOps,
    path: FsPath,
    options: OpenOptions,
) -> Result<(FileDescriptor, VlobID), WorkspaceOpenFileError> {
    // 1) Access control

    if options.write || options.truncate || options.create || options.create_new {
        let guard = ops
            .workspace_external_info
            .lock()
            .expect("mutex is poisoned");
        if !guard.entry.role.can_write() {
            return Err(WorkspaceOpenFileError::ReadOnlyRealm);
        }
    }

    // 2) Resolve the file path (and create the file if needed)

    let entry_id = {
        let outcome = ops.store.resolve_path(&path).await;
        match outcome {
            Ok((manifest, _)) => {
                if options.create_new {
                    return Err(WorkspaceOpenFileError::EntryExistsInCreateNewMode {
                        entry_id: manifest.id(),
                    });
                }
                manifest.id()
            }
            // Special case if the file doesn't exist but we are allowed to create it
            Err(ResolvePathError::EntryNotFound) if options.create || options.create_new => {
                let outcome = super::create_file(ops, path).await;
                outcome.or_else(|err| match err {
                    // Concurrent operation has created the file in the meantime
                    WorkspaceCreateFileError::EntryExists { entry_id } => Ok(entry_id),
                    // Actual errors, republishing
                    WorkspaceCreateFileError::Offline(e) => Err(WorkspaceOpenFileError::Offline(e)),
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
                    WorkspaceCreateFileError::ParentNotAFolder => {
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
                    ResolvePathError::Offline(e) => Err(WorkspaceOpenFileError::Offline(e)),
                    ResolvePathError::Stopped => Err(WorkspaceOpenFileError::Stopped),
                    ResolvePathError::EntryNotFound => Err(WorkspaceOpenFileError::EntryNotFound),
                    ResolvePathError::NoRealmAccess => Err(WorkspaceOpenFileError::NoRealmAccess),
                    ResolvePathError::InvalidKeysBundle(err) => {
                        Err(WorkspaceOpenFileError::InvalidKeysBundle(err))
                    }
                    ResolvePathError::InvalidCertificate(err) => {
                        Err(WorkspaceOpenFileError::InvalidCertificate(err))
                    }
                    ResolvePathError::InvalidManifest(err) => {
                        Err(WorkspaceOpenFileError::InvalidManifest(err))
                    }
                    ResolvePathError::Internal(err) => {
                        Err(err.context("cannot resolve path").into())
                    }
                };
            }
        }
    };

    open_file_by_id(ops, entry_id, options).await
}

pub async fn open_file_by_id(
    ops: &WorkspaceOps,
    entry_id: VlobID,
    options: OpenOptions,
) -> Result<(FileDescriptor, VlobID), WorkspaceOpenFileError> {
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
            updater: Option<FileUpdater>,
        },
    }

    let cursor_insertion_outcome = loop {
        let for_update_outcome = ops.store.for_update_file(entry_id, false).await;
        // /!\ From now on, `outcome` may contain a file updater requiring manual
        //     close if we encounter an error !

        let mut opened_files_guard = ops.opened_files.lock().expect("Mutex is poisoned");
        if !opened_files_guard.new_open_allowed {
            // File updater must be manually closed before returning error
            if let Ok((updater, _)) = for_update_outcome {
                updater.close(&ops.store);
            }
            return Err(WorkspaceOpenFileError::Stopped);
        }

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

        let cursor_insertion_outcome = match for_update_outcome {
            // Two possibilities:
            // - The file exists and is not currently opened.
            // - The file exists and is currently opened, but without an updater
            //   since it contains no write cursors.
            Ok((updater, mut manifest)) => {
                let (removed_chunks, flush_needed) = match maybe_truncate_on_open(&mut manifest) {
                    None => (vec![], false),
                    Some(removed_chunks) => (removed_chunks, true),
                };

                let updater = match cursor.write_mode {
                    WriteMode::Allowed => Some(updater),
                    WriteMode::Denied => {
                        // Since we are not going to update the file, we can release the updater.
                        updater.close(&ops.store);
                        None
                    }
                };

                match opened_files_guard.opened_files.entry(entry_id) {
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        let opened_file = Arc::new(AsyncMutex::new(OpenedFile {
                            updater,
                            manifest,
                            bytes_written_since_last_flush: 0,
                            cursors: vec![cursor],
                            new_chunks: vec![],
                            removed_chunks,
                            flush_needed,
                            modified_since_opened: flush_needed,
                        }));
                        entry.insert(opened_file);
                        CursorInsertionOutcome::OpenedFile { file_descriptor }
                    }
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        let opened_file = entry.get_mut();
                        // The opened file is behind an async mutex, which we cannot
                        // take now given we already hold the `opened_files` sync mutex.
                        // Hence we return this special outcome to delay the cursor insertion
                        // until the `opened_files` mutex is released.
                        CursorInsertionOutcome::FileAlreadyOpened {
                            opened_file: opened_file.clone(),
                            new_cursor: cursor,
                            updater,
                        }
                    }
                }
            }

            Err(err) => match err {
                // The file is already opened, or is being reparented (as this operation
                // occurs without opening the file).
                ForUpdateFileError::WouldBlock => {
                    match opened_files_guard.opened_files.get_mut(&entry_id) {
                        // The file is already opened
                        Some(opened_file) => {
                            // The opened file is behind an async mutex, which we cannot
                            // take now given we already hold the `opened_files` sync mutex.
                            // Hence we return this special outcome to delay the cursor insertion
                            // until the `opened_files` mutex is released.
                            CursorInsertionOutcome::FileAlreadyOpened {
                                opened_file: opened_file.clone(),
                                new_cursor: cursor,
                                updater: None,
                            }
                        }

                        // Two possibilities:
                        // - The file is being reparented.
                        // - The file was opened, but got closed in the meantime.
                        //
                        // In both cases, we should retry the operation.
                        None => {
                            continue;
                        }
                    }
                }

                // Actual errors, republishing
                ForUpdateFileError::Offline(e) => return Err(WorkspaceOpenFileError::Offline(e)),
                ForUpdateFileError::Stopped => return Err(WorkspaceOpenFileError::Stopped),
                ForUpdateFileError::EntryNotFound => {
                    return Err(WorkspaceOpenFileError::EntryNotFound);
                }
                ForUpdateFileError::EntryNotAFile { entry_id } => {
                    return Err(WorkspaceOpenFileError::EntryNotAFile { entry_id });
                }
                ForUpdateFileError::NoRealmAccess => {
                    return Err(WorkspaceOpenFileError::NoRealmAccess);
                }
                ForUpdateFileError::InvalidKeysBundle(err) => {
                    return Err(WorkspaceOpenFileError::InvalidKeysBundle(err));
                }
                ForUpdateFileError::InvalidCertificate(err) => {
                    return Err(WorkspaceOpenFileError::InvalidCertificate(err));
                }
                ForUpdateFileError::InvalidManifest(err) => {
                    return Err(WorkspaceOpenFileError::InvalidManifest(err));
                }
                ForUpdateFileError::Internal(err) => {
                    return Err(err.context("cannot resolve path").into());
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
            updater,
        } => {
            let file_descriptor = new_cursor.file_descriptor;
            let mut opened_file_guard = opened_file.lock().await;
            match &opened_file_guard.updater {
                None => {
                    opened_file_guard.updater = updater;
                }
                Some(_) => {
                    assert!(updater.is_none());
                }
            }
            opened_file_guard.cursors.push(new_cursor);
            if let Some(removed_chunks) = maybe_truncate_on_open(&mut opened_file_guard.manifest) {
                opened_file_guard.flush_needed = true;
                opened_file_guard.removed_chunks.extend(removed_chunks);
            }
            Ok((file_descriptor, entry_id))
        }
    }
}
