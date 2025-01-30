// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod store;
mod transactions;

use store::*;
pub use transactions::{
    WorkspaceHistoryEntryStat, WorkspaceHistoryFdCloseError, WorkspaceHistoryFdReadError,
    WorkspaceHistoryFdStatError, WorkspaceHistoryFileStat, WorkspaceHistoryFolderReader,
    WorkspaceHistoryFolderReaderStatEntryError, WorkspaceHistoryFolderReaderStatNextOutcome,
    WorkspaceHistoryOpenFileError, WorkspaceHistoryOpenFolderReaderError,
    WorkspaceHistoryStatEntryError, WorkspaceHistoryStatFolderChildrenError,
};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_types::prelude::*;

use crate::{CertificateOps, ClientConfig};

struct WorkspaceHistoryOpsReadWriteAttributes {
    next_file_descriptor: FileDescriptor,
    // Given the files are only opened for read, each open only needs to have access
    // the manifest at the given point in time.
    // Note multiple opens of the same file leads to multiple independant entries
    // in the map which is totally fine since no write operation is allowed.
    opened_files: HashMap<FileDescriptor, Arc<FileManifest>>,
    timestamp_of_interest: DateTime,
}

pub struct WorkspaceHistoryOps {
    #[allow(unused)]
    config: Arc<ClientConfig>,
    rw: Mutex<WorkspaceHistoryOpsReadWriteAttributes>,
    store: store::WorkspaceHistoryStore,
}

pub enum WorkspaceHistoryRealmExportDecryptor {
    SequesterService {
        sequester_service_id: SequesterServiceID,
        private_key: Box<SequesterPrivateKeyDer>,
    },
    User {
        user_id: UserID,
        private_key: Box<PrivateKey>,
    },
}

pub type WorkspaceHistoryOpsStartError = WorkspaceHistoryStoreStartError;

impl WorkspaceHistoryOps {
    // Visibility is `pub(crate)` given `Client::start_workspace_history()` is the exposed
    // method to be used instead.
    pub(crate) async fn start_with_server_access(
        config: Arc<ClientConfig>,
        cmds: Arc<AuthenticatedCmds>,
        certificates_ops: Arc<CertificateOps>,
        realm_id: VlobID,
    ) -> Result<Self, WorkspaceHistoryOpsStartError> {
        let store =
            WorkspaceHistoryStore::start_with_server_access(cmds, certificates_ops, realm_id)
                .await?;
        Ok(Self::new(config, store))
    }

    pub async fn start_with_realm_export(
        config: Arc<ClientConfig>,
        export_db_path: std::path::PathBuf,
        decryptors: Vec<WorkspaceHistoryRealmExportDecryptor>,
    ) -> Result<Self, WorkspaceHistoryOpsStartError> {
        let store =
            WorkspaceHistoryStore::start_with_realm_export(export_db_path, decryptors).await?;
        Ok(Self::new(config, store))
    }

    fn new(config: Arc<ClientConfig>, store: WorkspaceHistoryStore) -> Self {
        Self {
            config,
            store,
            rw: Mutex::new(WorkspaceHistoryOpsReadWriteAttributes {
                // Avoid using 0 as file descriptor, as it is error-prone
                next_file_descriptor: FileDescriptor(1),
                opened_files: HashMap::new(),
                timestamp_of_interest: DateTime::now(),
            }),
        }
    }

    /*
     * Public API
     */

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    pub fn realm_id(&self) -> VlobID {
        self.store.realm_id()
    }

    /// Earliest date we can go back. It corresponds to the timestamp of the very first
    /// version of the workspace manifest.
    ///
    /// It's possible to have non root manifest older than this date, however
    /// they were in practice not accessible until the root manifest got updated.
    pub fn timestamp_lower_bound(&self) -> DateTime {
        self.store.timestamp_lower_bound()
    }

    /// Latest date we can go back.
    ///
    /// - With server-based history, it corresponds to when the `WorkspaceHistoryOps` was created.
    /// - With realm export history, it corresponds to the latest timestamp of the exported realm.
    pub fn timestamp_higher_bound(&self) -> DateTime {
        self.store.timestamp_higher_bound()
    }

    pub fn timestamp_of_interest(&self) -> DateTime {
        self.rw
            .lock()
            .expect("Mutex is poisoned")
            .timestamp_of_interest
    }

    /// Set the timestamp of interest.
    ///
    /// If the provided timestamp is outside of the bounds, the closest possible value
    /// is selected.
    ///
    /// Return the new timestamp of interest.
    pub fn set_timestamp_of_interest(&self, mut toi: DateTime) -> DateTime {
        toi = std::cmp::max(self.timestamp_lower_bound(), toi);
        toi = std::cmp::min(self.timestamp_higher_bound(), toi);

        self.rw
            .lock()
            .expect("Mutex is poisoned")
            .timestamp_of_interest = toi;

        toi
    }

    pub async fn stat_entry(
        &self,
        path: &FsPath,
    ) -> Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError> {
        transactions::stat_entry(self, self.timestamp_of_interest(), path).await
    }

    pub async fn stat_entry_by_id(
        &self,
        entry_id: VlobID,
    ) -> Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError> {
        transactions::stat_entry_by_id(self, self.timestamp_of_interest(), entry_id).await
    }

    pub async fn open_folder_reader(
        &self,
        path: &FsPath,
    ) -> Result<WorkspaceHistoryFolderReader, WorkspaceHistoryOpenFolderReaderError> {
        transactions::open_folder_reader(self, self.timestamp_of_interest(), path).await
    }

    pub async fn open_folder_reader_by_id(
        &self,
        entry_id: VlobID,
    ) -> Result<WorkspaceHistoryFolderReader, WorkspaceHistoryOpenFolderReaderError> {
        transactions::open_folder_reader_by_id(self, self.timestamp_of_interest(), entry_id).await
    }

    /// Note children are listed in arbitrary order, and there is no '.' and '..'  special entries.
    pub async fn stat_folder_children(
        &self,
        path: &FsPath,
    ) -> Result<Vec<(EntryName, WorkspaceHistoryEntryStat)>, WorkspaceHistoryStatFolderChildrenError>
    {
        transactions::stat_folder_children(self, self.timestamp_of_interest(), path).await
    }

    pub async fn stat_folder_children_by_id(
        &self,
        entry_id: VlobID,
    ) -> Result<Vec<(EntryName, WorkspaceHistoryEntryStat)>, WorkspaceHistoryStatFolderChildrenError>
    {
        transactions::stat_folder_children_by_id(self, self.timestamp_of_interest(), entry_id).await
    }

    pub async fn open_file(
        &self,
        path: FsPath,
    ) -> Result<FileDescriptor, WorkspaceHistoryOpenFileError> {
        transactions::open_file(self, self.timestamp_of_interest(), path)
            .await
            .map(|(fd, _)| fd)
    }

    pub async fn open_file_by_id(
        &self,
        entry_id: VlobID,
    ) -> Result<FileDescriptor, WorkspaceHistoryOpenFileError> {
        transactions::open_file_by_id(self, self.timestamp_of_interest(), entry_id).await
    }

    pub async fn open_file_and_get_id(
        &self,
        path: FsPath,
    ) -> Result<(FileDescriptor, VlobID), WorkspaceHistoryOpenFileError> {
        transactions::open_file(self, self.timestamp_of_interest(), path).await
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
    ) -> Result<WorkspaceHistoryFileStat, WorkspaceHistoryFdStatError> {
        transactions::fd_stat(self, fd).await
    }
}
