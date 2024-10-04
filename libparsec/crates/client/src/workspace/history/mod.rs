// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod fd_close;
mod fd_read;
mod fd_stat;
mod open_file;
mod populate_cache;
mod read_folder;
mod resolve_path;
mod stat_entry;

mod transactions {
    pub use super::fd_close::*;
    pub use super::fd_read::*;
    pub use super::fd_stat::*;
    pub use super::open_file::*;
    pub use super::read_folder::*;
    pub(super) use super::resolve_path::*;
    pub use super::stat_entry::*;
}

pub use transactions::{
    WorkspaceHistoryFdCloseError, WorkspaceHistoryFdReadError, WorkspaceHistoryFdStatError,
    WorkspaceHistoryFolderReader, WorkspaceHistoryFolderReaderStatEntryError,
    WorkspaceHistoryFolderReaderStatNextOutcome, WorkspaceHistoryOpenFileError,
    WorkspaceHistoryOpenFolderReaderError, WorkspaceHistoryStatEntryError,
    WorkspaceHistoryStatFolderChildrenError,
};
use transactions::{
    WorkspaceHistoryGetBlockError, WorkspaceHistoryGetEntryError, WorkspaceHistoryResolvePathError,
};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_types::prelude::*;

use super::{EntryStat, FileStat};
use crate::{CertificateOps, ClientConfig};

#[derive(Debug)]
enum CacheResolvedEntry {
    // TODO: we could be using remote manifests here (which should roughly divide
    // the memory consumption by 2) since we only do read operations.
    Exists(ArcLocalChildManifest),
    NotFound,
}

struct Cache {
    resolutions: HashMap<DateTime, HashMap<VlobID, CacheResolvedEntry>>,
    next_file_descriptor: FileDescriptor,
    // Given the files are only opened for read, each open only needs to have access
    // the manifest at the given point in time.
    // Note multiple opens of the same file leave to multiple entries in the map
    // which is totally fine since no write operation is allowed.
    opened_files: HashMap<FileDescriptor, Arc<LocalFileManifest>>,
    blocks: HashMap<BlockID, Bytes>,
}

pub struct WorkspaceHistoryOps {
    #[allow(unused)]
    config: Arc<ClientConfig>,
    cmds: Arc<AuthenticatedCmds>,
    certificates_ops: Arc<CertificateOps>,
    cache: Mutex<Cache>,
    realm_id: VlobID,
}

impl WorkspaceHistoryOps {
    pub(crate) fn new(
        config: Arc<ClientConfig>,
        cmds: Arc<AuthenticatedCmds>,
        certificates_ops: Arc<CertificateOps>,
        realm_id: VlobID,
    ) -> Self {
        Self {
            config,
            cmds,
            certificates_ops,
            realm_id,
            cache: Mutex::new(Cache {
                resolutions: HashMap::new(),
                // Avoid using 0 as file descriptor, as it is error-prone
                next_file_descriptor: FileDescriptor(1),
                opened_files: HashMap::new(),
                blocks: HashMap::new(),
            }),
        }
    }

    /*
     * Internal helpers
     */

    async fn resolve_path(
        &self,
        at: DateTime,
        path: &FsPath,
    ) -> Result<ArcLocalChildManifest, WorkspaceHistoryResolvePathError> {
        transactions::resolve_path(self, at, path).await
    }

    #[allow(unused)]
    async fn retrieve_path_from_id(
        ops: &WorkspaceHistoryOps,
        at: DateTime,
        entry_id: VlobID,
    ) -> Result<(ArcLocalChildManifest, FsPath), WorkspaceHistoryResolvePathError> {
        transactions::retrieve_path_from_id(ops, at, entry_id).await
    }

    async fn get_entry(
        &self,
        at: DateTime,
        entry_id: VlobID,
    ) -> Result<ArcLocalChildManifest, WorkspaceHistoryGetEntryError> {
        transactions::get_entry(self, at, entry_id).await
    }

    async fn get_block(
        &self,
        access: &BlockAccess,
        remote_manifest: &FileManifest,
    ) -> Result<Bytes, WorkspaceHistoryGetBlockError> {
        transactions::get_block(self, access, remote_manifest).await
    }

    /*
     * Public API
     */

    pub async fn stat_entry(
        &self,
        at: DateTime,
        path: &FsPath,
    ) -> Result<EntryStat, WorkspaceHistoryStatEntryError> {
        transactions::stat_entry(self, at, path).await
    }

    pub async fn stat_entry_by_id(
        &self,
        at: DateTime,
        entry_id: VlobID,
    ) -> Result<EntryStat, WorkspaceHistoryStatEntryError> {
        transactions::stat_entry_by_id(self, at, entry_id).await
    }

    pub async fn open_folder_reader(
        &self,
        at: DateTime,
        path: &FsPath,
    ) -> Result<WorkspaceHistoryFolderReader, WorkspaceHistoryOpenFolderReaderError> {
        transactions::open_folder_reader(self, at, path).await
    }

    pub async fn open_folder_reader_by_id(
        &self,
        at: DateTime,
        entry_id: VlobID,
    ) -> Result<WorkspaceHistoryFolderReader, WorkspaceHistoryOpenFolderReaderError> {
        transactions::open_folder_reader_by_id(self, at, entry_id).await
    }

    /// Note children are listed in arbitrary order, and there is no '.' and '..'  special entries.
    pub async fn stat_folder_children(
        &self,
        at: DateTime,
        path: &FsPath,
    ) -> Result<Vec<(EntryName, EntryStat)>, WorkspaceHistoryStatFolderChildrenError> {
        transactions::stat_folder_children(self, at, path).await
    }

    pub async fn stat_folder_children_by_id(
        &self,
        at: DateTime,
        entry_id: VlobID,
    ) -> Result<Vec<(EntryName, EntryStat)>, WorkspaceHistoryStatFolderChildrenError> {
        transactions::stat_folder_children_by_id(self, at, entry_id).await
    }

    pub async fn open_file(
        &self,
        at: DateTime,
        path: FsPath,
    ) -> Result<FileDescriptor, WorkspaceHistoryOpenFileError> {
        transactions::open_file(self, at, path)
            .await
            .map(|(fd, _)| fd)
    }

    pub async fn open_file_by_id(
        &self,
        at: DateTime,
        entry_id: VlobID,
    ) -> Result<FileDescriptor, WorkspaceHistoryOpenFileError> {
        transactions::open_file_by_id(self, at, entry_id).await
    }

    pub async fn open_file_and_get_id(
        &self,
        at: DateTime,
        path: FsPath,
    ) -> Result<(FileDescriptor, VlobID), WorkspaceHistoryOpenFileError> {
        transactions::open_file(self, at, path).await
    }

    pub fn fd_close(&self, fd: FileDescriptor) -> Result<(), WorkspaceHistoryFdCloseError> {
        transactions::fd_close(self, fd)
    }

    pub async fn fd_read(
        &self,
        fd: FileDescriptor,
        offset: u64,
        size: u64,
        buf: &mut impl std::io::Write,
    ) -> Result<u64, WorkspaceHistoryFdReadError> {
        transactions::fd_read(self, fd, offset, size, buf).await
    }

    pub async fn fd_stat(
        &self,
        fd: FileDescriptor,
    ) -> Result<FileStat, WorkspaceHistoryFdStatError> {
        transactions::fd_stat(self, fd).await
    }
}
