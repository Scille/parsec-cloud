// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use crate::certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError};

mod create_file;
mod create_folder;
mod fd_close;
mod fd_flush;
mod fd_read;
mod fd_resize;
mod fd_seek;
mod fd_write;
mod file_operations;
mod inbound_sync;
mod open_file;
mod outbound_sync;
mod remove_entry;
mod rename_entry;
mod stat_entry;

pub use create_file::*;
pub use create_folder::*;
pub use fd_close::*;
pub use fd_flush::*;
pub use fd_read::*;
pub use fd_resize::*;
pub use fd_seek::*;
pub use fd_write::*;
pub(crate) use file_operations::*;
pub use inbound_sync::*;
pub use open_file::*;
pub use outbound_sync::*;
pub use remove_entry::*;
pub use rename_entry::*;
pub use stat_entry::*;

pub use stat_entry::EntryStat;

#[derive(Debug, thiserror::Error)]
pub enum FsOperationError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot reach the server")]
    Offline,
    #[error("Path already exists")]
    EntryExists { entry_id: VlobID },
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Path points to a folder")]
    IsAFolder,
    #[error("Root path cannot be renamed")]
    CannotRenameRoot,
    #[error("Path doesn't point to a folder")]
    NotAFolder,
    #[error("Path points to a non-empty folder")]
    FolderNotEmpty,
    #[error("No longer allowed to access this workspace")]
    NoRealmAccess,
    #[error("Only have read access on this workspace")]
    ReadOnlyRealm,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
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
