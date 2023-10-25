// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use crate::certificates_ops::{InvalidCertificateError, InvalidManifestError};

mod create_file;
mod create_folder;
mod inbound_sync;
mod open_file;
mod outbound_sync;
mod remove_entry;
mod rename_entry;
mod resize_file;
mod stat_entry;
mod utils;

pub(crate) use create_file::*;
pub(crate) use create_folder::*;
pub(crate) use inbound_sync::*;
pub(crate) use open_file::*;
pub(crate) use outbound_sync::*;
pub(crate) use remove_entry::*;
pub(crate) use rename_entry::*;
pub(crate) use resize_file::*;
pub(crate) use stat_entry::*;
use utils::*;

pub use stat_entry::EntryStat;

#[derive(Debug, thiserror::Error)]
pub enum FsOperationError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Path already exists")]
    EntryExists,
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
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    InvalidManifest(#[from] InvalidManifestError),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    BadTimestamp {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}
