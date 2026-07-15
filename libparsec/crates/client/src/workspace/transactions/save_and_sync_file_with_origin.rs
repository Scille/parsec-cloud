// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidManifestError},
    workspace::{
        FdWriteStrategy, OpenOptions, OutboundSyncOutcome, WorkspaceFdCloseError,
        WorkspaceFdWriteError, WorkspaceOpenFileError, WorkspaceOps, WorkspaceSyncError,
    },
    InvalidBlockAccessError, InvalidKeysBundleError,
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceSaveAndSyncFileWithOriginError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Only have read access on this workspace")]
    ReadOnlyRealm,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("The workspace's realm hasn't been created yet on server")]
    NoRealm,
    #[error("The workspace's realm has been archived")]
    RealmArchived,
    #[error("The workspace's realm has been deleted on the server")]
    RealmDeleted,
    #[error("The workspace's realm doesn't have any key yet")]
    NoKey,
    #[error("Entry doesn't exist")]
    EntryNotFound,
    #[error("Entry (ID: `{}`) is not a file", .entry_id)]
    EntryNotAFile { entry_id: VlobID },
    #[error("Block access is temporary unavailable on the server")]
    ServerBlockstoreUnavailable,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    InvalidBlockAccess(#[from] Box<InvalidBlockAccessError>),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

/// Create a new version of the file manifest identified by `entry_id`, containing
/// `content` and marked with a `FileManifestOrigin::Cryptpad` origin, then sync it
/// with the server right away.
///
/// This is meant to be used when a Cryptpad collaborative editing session saves its
/// document: the whole content is provided at once (Cryptpad doesn't work in terms
/// of incremental writes), and the save should be reflected on the server without
/// delay so that other participants of the session can see it.
pub async fn save_and_sync_file_with_origin(
    ops: &WorkspaceOps,
    entry_id: VlobID,
    origin: FileManifestOrigin,
    content: &[u8],
) -> Result<(), WorkspaceSaveAndSyncFileWithOriginError> {
    // 1) Open the file, truncating any existing content: the whole point of a
    // Cryptpad collaborative editing session is that its content is the single
    // source of truth, so there is no need to keep whatever was here before.

    let fd = super::open_file_by_id(
        ops,
        entry_id,
        OpenOptions {
            read: false,
            write: true,
            truncate: true,
            create: false,
            create_new: false,
        },
    )
    .await
    .map_err(|err| match err {
        WorkspaceOpenFileError::Offline(e) => WorkspaceSaveAndSyncFileWithOriginError::Offline(e),
        WorkspaceOpenFileError::Stopped => WorkspaceSaveAndSyncFileWithOriginError::Stopped,
        WorkspaceOpenFileError::ReadOnlyRealm => {
            WorkspaceSaveAndSyncFileWithOriginError::ReadOnlyRealm
        }
        WorkspaceOpenFileError::NoRealmAccess => {
            WorkspaceSaveAndSyncFileWithOriginError::NoRealmAccess
        }
        WorkspaceOpenFileError::RealmDeleted => {
            WorkspaceSaveAndSyncFileWithOriginError::RealmDeleted
        }
        WorkspaceOpenFileError::EntryNotFound => {
            WorkspaceSaveAndSyncFileWithOriginError::EntryNotFound
        }
        WorkspaceOpenFileError::EntryNotAFile { entry_id } => {
            WorkspaceSaveAndSyncFileWithOriginError::EntryNotAFile { entry_id }
        }
        WorkspaceOpenFileError::EntryExistsInCreateNewMode { .. } => {
            anyhow::anyhow!("Unexpected error: `create_new` is not used here").into()
        }
        WorkspaceOpenFileError::InvalidKeysBundle(err) => {
            WorkspaceSaveAndSyncFileWithOriginError::InvalidKeysBundle(err)
        }
        WorkspaceOpenFileError::InvalidCertificate(err) => {
            WorkspaceSaveAndSyncFileWithOriginError::InvalidCertificate(err)
        }
        WorkspaceOpenFileError::InvalidManifest(err) => {
            WorkspaceSaveAndSyncFileWithOriginError::InvalidManifest(err)
                super::inbound_sync(ops, entry_id)
                    .await
                    .map_err(map_sync_error)?;
        WorkspaceOpenFileError::Internal(err) => err.context("cannot open file").into(),
    })?;

    // From now on the file descriptor must always be closed no matter the outcome,
    // otherwise the file manifest would stay locked in the store forever.

    let write_outcome: Result<u64, WorkspaceSaveAndSyncFileWithOriginError> =
        super::fd_write(ops, fd, content, FdWriteStrategy::Normal { offset: 0 })
            .await
            .map_err(|err| match err {
                WorkspaceFdWriteError::BadFileDescriptor
                | WorkspaceFdWriteError::NotInWriteMode => {
                    anyhow::anyhow!("Unexpected error right after opening the file for write")
                        .into()
                }
                WorkspaceFdWriteError::Internal(err) => err.context("cannot write file").into(),
            });

    let origin_outcome = if write_outcome.is_ok() {
        set_pending_file_origin(ops, fd, origin).await
    } else {
        Ok(())
    };

    let close_outcome = super::fd_close(ops, fd).await.map_err(|err| match err {
        WorkspaceFdCloseError::Stopped => WorkspaceSaveAndSyncFileWithOriginError::Stopped,
        WorkspaceFdCloseError::BadFileDescriptor => {
            anyhow::anyhow!("Unexpected error while closing the just-opened file").into()
        }
        WorkspaceFdCloseError::Internal(err) => err.context("cannot close file").into(),
    });

    write_outcome?;
    origin_outcome?;
    close_outcome?;

    // 2) Sync the entry right away: the whole point of this method is to make the
    // Cryptpad session's save immediately visible to the other participants.
        let outcome = super::outbound_sync(ops, entry_id)
            .await
            .map_err(map_sync_error)?;
    loop {
        let outcome = super::outbound_sync(ops, entry_id).await?;
        match outcome {
            OutboundSyncOutcome::Done => break,
            // The entry is momentarily locked by a concurrent operation, retry until
            // it settles.
            OutboundSyncOutcome::EntryIsBusy => continue,
            OutboundSyncOutcome::InboundSyncNeeded => {
                super::inbound_sync(ops, entry_id).await?;
                continue;
            }
            OutboundSyncOutcome::EntryIsUnreachable => {
                return Err(WorkspaceSaveAndSyncFileWithOriginError::EntryNotFound)
            }
            // Confined entries never get synced with the server, nothing more to do.
            OutboundSyncOutcome::EntryIsConfined { .. } => break,
        }
    }

    Ok(())
}

/// Set the `origin` field on the manifest of a currently opened (for write) file,
/// so that it gets persisted the next time the file is flushed/closed.
async fn set_pending_file_origin(
    ops: &WorkspaceOps,
    fd: FileDescriptor,
    origin: FileManifestOrigin,
) -> Result<(), WorkspaceSaveAndSyncFileWithOriginError> {
    let opened_file = {
        let guard = ops.opened_files.lock().expect("Mutex is poisoned");
        let file_id = guard.file_descriptors.get(&fd).ok_or_else(|| {
            anyhow::anyhow!("Unexpected error: bad file descriptor right after opening the file")
        })?;
        guard
            .opened_files
            .get(file_id)
            .expect("File descriptor always refers to an opened file")
            .clone()
    };

    let mut opened_file = opened_file.lock().await;
    let manifest: &mut LocalFileManifest = Arc::make_mut(&mut opened_file.manifest);
    manifest.origin = origin;
    opened_file.flush_needed = true;
    opened_file.modified_since_opened = true;

    Ok(())
}

fn map_sync_error(err: WorkspaceSyncError) -> WorkspaceSaveAndSyncFileWithOriginError {
    match err {
        WorkspaceSyncError::Offline(e) => WorkspaceSaveAndSyncFileWithOriginError::Offline(e),
        WorkspaceSyncError::ServerBlockstoreUnavailable => {
            WorkspaceSaveAndSyncFileWithOriginError::ServerBlockstoreUnavailable
        }
        WorkspaceSyncError::Stopped => WorkspaceSaveAndSyncFileWithOriginError::Stopped,
        // Note `WorkspaceSyncError` doesn't discriminate between "no access" and "only read access"
        // but it's fine here since we call `open_file_by_id` (that does such discrimination)
        // before doing the sync operation.
        WorkspaceSyncError::NotAllowed => WorkspaceSaveAndSyncFileWithOriginError::NoRealmAccess,
        WorkspaceSyncError::NoKey => WorkspaceSaveAndSyncFileWithOriginError::NoKey,
        WorkspaceSyncError::NoRealm => WorkspaceSaveAndSyncFileWithOriginError::NoRealm,
        WorkspaceSyncError::RealmArchived => WorkspaceSaveAndSyncFileWithOriginError::RealmArchived,
        WorkspaceSyncError::RealmDeleted => WorkspaceSaveAndSyncFileWithOriginError::RealmDeleted,
        WorkspaceSyncError::InvalidManifest(err) => {
            WorkspaceSaveAndSyncFileWithOriginError::InvalidManifest(err)
        }
        WorkspaceSyncError::InvalidBlockAccess(err) => {
            WorkspaceSaveAndSyncFileWithOriginError::InvalidBlockAccess(err)
        }
        WorkspaceSyncError::InvalidKeysBundle(err) => {
            WorkspaceSaveAndSyncFileWithOriginError::InvalidKeysBundle(err)
        }
        WorkspaceSyncError::InvalidCertificate(err) => {
            WorkspaceSaveAndSyncFileWithOriginError::InvalidCertificate(err)
        }
        WorkspaceSyncError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
        } => WorkspaceSaveAndSyncFileWithOriginError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
        },
        WorkspaceSyncError::Internal(err) => err.context("cannot sync entry").into(),
    }
}
