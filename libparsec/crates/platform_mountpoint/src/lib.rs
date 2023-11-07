// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![doc = include_str!("../README.md")]
#![cfg(not(target_os = "macos"))]

mod error;
mod memfs;
#[cfg(target_os = "linux")]
mod unix;
#[cfg(target_os = "windows")]
mod windows;

use chrono::{DateTime, Utc};
use std::{collections::HashMap, path::Path};
#[cfg(target_os = "linux")]
pub(crate) use unix as platform;
#[cfg(target_os = "windows")]
pub(crate) use windows as platform;

use libparsec_types::{anyhow, EntryName, FileDescriptor, FsPath, VlobID};

pub use error::{MountpointError, MountpointResult};
pub use memfs::MemFS;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntryInfoType {
    Dir,
    File,
    // TODO: Implement Icon Handler
    /// Special Parsec type for icon overlay handler
    ParsecEntryInfo,
}

/// - Normal: write normally from offset to offset + data.len().
/// - Constrained: write from offset without exceeding the file size limit.
/// - StartEOF: write starting at end of file.
pub enum WriteMode {
    Normal,
    // Used only by Windows
    Constrained,
    // Used only by Windows
    StartEOF,
}

#[derive(Debug, Clone)]
pub struct EntryInfo {
    pub id: VlobID,
    pub ty: EntryInfoType,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub size: u64,
    pub need_sync: bool,
    pub children: HashMap<EntryName, VlobID>,
}

impl EntryInfo {
    fn new(id: VlobID, ty: EntryInfoType, now: DateTime<Utc>) -> Self {
        Self {
            id,
            ty,
            created: now,
            updated: now,
            size: 0,
            need_sync: false,
            children: HashMap::new(),
        }
    }
}

pub(crate) struct FileSystemWrapper<T: MountpointInterface> {
    interface: T,
    #[cfg(target_os = "linux")]
    pub(crate) buffer: std::sync::Mutex<Vec<u8>>,
    #[cfg(target_os = "linux")]
    pub(crate) inode_manager: crate::unix::InodeManager,
}

impl<T: MountpointInterface> FileSystemWrapper<T> {
    fn new(interface: T) -> Self {
        Self {
            interface,
            #[cfg(target_os = "linux")]
            buffer: Default::default(),
            #[cfg(target_os = "linux")]
            inode_manager: Default::default(),
        }
    }
}

pub trait MountpointInterface {
    // Rights check
    fn check_read_rights(&self, path: &FsPath) -> MountpointResult<()>;
    fn check_write_rights(&self, path: &FsPath) -> MountpointResult<()>;

    // Entry transactions

    fn entry_info(&self, path: &FsPath) -> MountpointResult<EntryInfo>;
    fn entry_rename(
        &self,
        source: &FsPath,
        destination: &FsPath,
        overwrite: bool,
    ) -> MountpointResult<()>;

    // Directory transactions

    fn dir_create(&self, path: &FsPath) -> MountpointResult<()>;
    fn dir_delete(&self, path: &FsPath) -> MountpointResult<()>;

    // File transactions

    fn file_create(&self, path: &FsPath, open: bool) -> MountpointResult<FileDescriptor>;
    fn file_open(
        &self,
        path: &FsPath,
        write_mode: bool,
    ) -> MountpointResult<Option<FileDescriptor>>;
    fn file_delete(&self, path: &FsPath) -> MountpointResult<()>;

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

pub struct FileSystemMounted<T: MountpointInterface> {
    #[cfg(target_os = "windows")]
    fs: winfsp_wrs::FileSystem<FileSystemWrapper<T>>,
    #[cfg(target_os = "linux")]
    unmounter: fuser::SessionUnmounter,
    #[cfg(target_os = "linux")]
    mountpoint: std::path::PathBuf,
    #[cfg(target_os = "linux")]
    phantom: std::marker::PhantomData<T>,
}

impl<T: MountpointInterface + Send + 'static> FileSystemMounted<T> {
    pub fn mount(mountpoint: &Path, interface: T) -> anyhow::Result<Self> {
        platform::mount(mountpoint, interface)
    }

    #[cfg_attr(target_os = "windows", allow(unused_mut))]
    pub fn stop(mut self) {
        #[cfg(target_os = "windows")]
        self.fs.stop();
        #[cfg(target_os = "linux")]
        {
            let _ = self.unmounter.unmount();
            // We do the same as WinFSP: remove the directory
            let _ = std::fs::remove_dir(self.mountpoint);
        }
    }
}
