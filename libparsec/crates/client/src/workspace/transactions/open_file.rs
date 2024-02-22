// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_types::prelude::*;

use super::CreateFileError;
use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::{
        store::{ForUpdateFileError, GetEntryError},
        OpenedFile, OpenedFileCursor, ReadMode, WorkspaceOps, WriteMode,
    },
};

pub struct OpenOptions {
    pub read: bool,
    pub write: bool,
    pub append: bool,
    pub truncate: bool,
    pub create: bool,
    pub create_new: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum OpenFileError {
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

impl From<ConnectionError> for OpenFileError {
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
) -> Result<FileDescriptor, OpenFileError> {
    // 0) Handle root early as it is a special case (cannot use `get_child_manifest` for it)

    if path.is_root() {
        return Err(OpenFileError::EntryNotAFile);
    }

    // 1) Resolve the file path (and create the file if needed)

    let entry_id = {
        let outcome = ops.store.resolve_path(&path).await;
        match outcome {
            Ok(resolution) => {
                if options.create_new {
                    return Err(OpenFileError::EntryExistsInCreateNewMode {
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
                    CreateFileError::EntryExists { entry_id } => Ok(entry_id),
                    // Actual errors, republishing
                    CreateFileError::Offline => Err(OpenFileError::Offline),
                    CreateFileError::Stopped => Err(OpenFileError::Stopped),
                    CreateFileError::ReadOnlyRealm => Err(OpenFileError::ReadOnlyRealm),
                    CreateFileError::NoRealmAccess => Err(OpenFileError::NoRealmAccess),
                    CreateFileError::ParentNotFound => Err(OpenFileError::EntryNotFound),
                    CreateFileError::ParentIsFile => Err(OpenFileError::EntryNotFound),
                    CreateFileError::InvalidKeysBundle(err) => {
                        Err(OpenFileError::InvalidKeysBundle(err))
                    }
                    CreateFileError::InvalidCertificate(err) => {
                        Err(OpenFileError::InvalidCertificate(err))
                    }
                    CreateFileError::InvalidManifest(err) => {
                        Err(OpenFileError::InvalidManifest(err))
                    }
                    CreateFileError::Internal(err) => Err(err.context("cannot create file").into()),
                })?
            }
            // Actual errors, republishing
            Err(err) => {
                return match err {
                    GetEntryError::Offline => Err(OpenFileError::Offline),
                    GetEntryError::Stopped => Err(OpenFileError::Stopped),
                    GetEntryError::EntryNotFound => Err(OpenFileError::EntryNotFound),
                    GetEntryError::NoRealmAccess => Err(OpenFileError::NoRealmAccess),
                    GetEntryError::InvalidKeysBundle(err) => {
                        Err(OpenFileError::InvalidKeysBundle(err))
                    }
                    GetEntryError::InvalidCertificate(err) => {
                        Err(OpenFileError::InvalidCertificate(err))
                    }
                    GetEntryError::InvalidManifest(err) => Err(OpenFileError::InvalidManifest(err)),
                    GetEntryError::Internal(err) => Err(err.context("cannot resolve path").into()),
                }
            }
        }
    };

    // Handling of truncate-on-open, will be done in the next step

    let maybe_truncate_on_open = |file_manifest: &mut Arc<LocalFileManifest>| {
        // Handle truncate-on-open
        if options.truncate {
            let file_manifest = Arc::make_mut(file_manifest);
            let (_, removed_chunks) = super::prepare_resize(file_manifest, 0, ops.device.now());
            // TODO: prepare_resize should return a Vec instead of a HashSet
            removed_chunks.into_iter().collect()
        } else {
            vec![]
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
            write_mode: if options.append {
                WriteMode::AllowedAppend
            } else if options.write {
                WriteMode::AllowedAtCursor
            } else {
                WriteMode::Denied
            },
            position: 0,
        };

        let cursor_insertion_outcome = match outcome {
            // The file exists and is not currently opened
            Ok((updater, mut manifest)) => {
                let removed_chunks = maybe_truncate_on_open(&mut manifest);
                let opened_file = Arc::new(AsyncMutex::new(OpenedFile {
                    updater,
                    manifest,
                    bytes_written_since_last_flush: 0,
                    cursors: vec![cursor],
                    new_chunks: vec![],
                    removed_chunks,
                    flush_needed: false,
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
                ForUpdateFileError::Offline => return Err(OpenFileError::Offline),
                ForUpdateFileError::Stopped => return Err(OpenFileError::Stopped),
                ForUpdateFileError::EntryNotFound => return Err(OpenFileError::EntryNotFound),
                ForUpdateFileError::EntryNotAFile => return Err(OpenFileError::EntryNotAFile),
                ForUpdateFileError::NoRealmAccess => return Err(OpenFileError::NoRealmAccess),
                ForUpdateFileError::InvalidKeysBundle(err) => {
                    return Err(OpenFileError::InvalidKeysBundle(err))
                }
                ForUpdateFileError::InvalidCertificate(err) => {
                    return Err(OpenFileError::InvalidCertificate(err))
                }
                ForUpdateFileError::InvalidManifest(err) => {
                    return Err(OpenFileError::InvalidManifest(err))
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
        CursorInsertionOutcome::OpenedFile { file_descriptor } => Ok(file_descriptor),
        CursorInsertionOutcome::FileAlreadyOpened {
            opened_file,
            new_cursor,
        } => {
            let file_descriptor = new_cursor.file_descriptor;
            let mut opened_file_guard = opened_file.lock().await;
            opened_file_guard.cursors.push(new_cursor);
            let removed_chunks = maybe_truncate_on_open(&mut opened_file_guard.manifest);
            opened_file_guard.removed_chunks.extend(removed_chunks);
            Ok(file_descriptor)
        }
    }
}
