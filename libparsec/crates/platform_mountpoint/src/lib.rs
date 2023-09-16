// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![doc = include_str!("../README.md")]

mod error;
mod memfs;
#[cfg(target_os = "windows")]
mod windows;

use chrono::{DateTime, Utc};
use std::{collections::HashMap, path::Path};
#[cfg(target_os = "windows")]
pub use windows::mount;

use libparsec_types::{EntryName, FileDescriptor, VlobID};

pub(crate) use error::{MountpointError, MountpointResult};
pub use memfs::MountpointManager;

// TODO: remove me once ParsecEntryInfo is used
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum EntryInfoType {
    Dir,
    File,
    /// Special Parsec type for icon overlay handler
    ParsecEntryInfo,
}

/// - Normal: write normally from offset to offset + data.len().
/// - Constrained: write from offset without exceeding the file size limit.
/// - StartEOF: write starting at end of file.
// Remove me once fuse use WriteMode
#[allow(dead_code)]
pub(crate) enum WriteMode {
    Normal,
    Constrained,
    StartEOF,
}

// TODO: remove me once fuse is implemented
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct EntryInfo {
    id: VlobID,
    ty: EntryInfoType,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    size: u64,
    need_sync: bool,
    children: HashMap<EntryName, VlobID>,
}

pub(crate) trait MountpointInterface {
    // Rights check
    fn check_read_rights(&self, path: &Path) -> MountpointResult<()>;
    fn check_write_rights(&self, path: &Path) -> MountpointResult<()>;

    // Entry transactions

    fn entry_info(&self, path: &Path) -> MountpointResult<EntryInfo>;
    fn entry_rename(
        &self,
        source: &Path,
        destination: &Path,
        overwrite: bool,
    ) -> MountpointResult<()>;

    // Directory transactions

    fn dir_create(&self, path: &Path) -> MountpointResult<()>;
    fn dir_delete(&self, path: &Path) -> MountpointResult<()>;

    // File transactions

    fn file_create(&self, path: &Path, open: bool) -> MountpointResult<FileDescriptor>;
    fn file_open(&self, path: &Path, write_mode: bool) -> MountpointResult<Option<FileDescriptor>>;
    fn file_delete(&self, path: &Path) -> MountpointResult<()>;
    fn file_resize(&self, path: &Path, len: u64) -> VlobID;

    // File descriptor transactions

    fn fd_close(&self, fd: FileDescriptor);
    fn fd_read(
        &self,
        fd: FileDescriptor,
        buffer: &mut [u8],
        offset: u64,
    ) -> MountpointResult<usize>;
    fn fd_write(
        &self,
        fd: FileDescriptor,
        data: &[u8],
        offset: u64,
        mode: WriteMode,
    ) -> MountpointResult<usize>;
    fn fd_resize(&self, fd: FileDescriptor, len: u64, truncate_only: bool) -> MountpointResult<()>;
    fn fd_flush(&self, fd: FileDescriptor);
}
