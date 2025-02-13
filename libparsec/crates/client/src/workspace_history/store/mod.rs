// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::{Arc, Mutex};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::WorkspaceHistoryRealmExportDecryptor;
use crate::certif::{
    CertificateOps, InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError,
};

mod cache;
mod data_access;
mod data_access_realm_export;
mod data_access_server;
mod get_block;
mod get_entry;
mod populate_cache;
mod resolve_path;
mod retrieve_path_from_id;

pub(super) use cache::InvalidManifestHistoryError;
use cache::*;
use data_access::*;
use data_access_realm_export::*;
use data_access_server::*;
pub(super) use get_block::*;
pub(super) use get_entry::*;
use populate_cache::*;
pub(super) use resolve_path::*;
pub(super) use retrieve_path_from_id::*;

pub(super) struct WorkspaceHistoryStore {
    realm_id: VlobID,
    cache: Mutex<WorkspaceHistoryStoreCache>,
    access: DataAccess,
    /// Earliest date we can go back. It corresponds to the timestamp of the very first
    /// version of the workspace manifest.
    ///
    /// It's possible to have non root manifest older than this date, however
    /// they were in practice not accessible until the root manifest got updated.
    timestamp_lower_bound: DateTime,
    /// Latest date we can go back.
    ///
    /// - With server-based history, it corresponds to when the `WorkspaceHistoryOps` was created.
    /// - With realm export history, it corresponds to the latest timestamp of the exported realm.
    timestamp_higher_bound: DateTime,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceHistoryStoreStartError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Workspace has not history yet (root manifest has never been synchronized)")]
    NoHistory,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl WorkspaceHistoryStore {
    /// Note the `DateTime` returned along with the `WorkspaceHistoryStore` instance, it should
    /// be used as the initial timestamp of interest.
    ///
    /// The idea here is to provide a timestamp of interest where the root manifest is guaranteed
    /// to be available in cache.
    ///
    /// This in turn allows the `WorkspaceHistoryOps` to always guaranteed that the root manifest
    /// can be queried without involving network access (as the cache is also updated whenever the
    /// timestamp of interest is modified).
    pub async fn start_with_server_access(
        cmds: Arc<AuthenticatedCmds>,
        certificates_ops: Arc<CertificateOps>,
        realm_id: VlobID,
    ) -> Result<(Self, DateTime), WorkspaceHistoryStoreStartError> {
        let access = DataAccess::Server(data_access_server::ServerDataAccess::new(
            cmds,
            certificates_ops,
            realm_id,
        ));

        Self::start(realm_id, access).await
    }

    pub async fn start_with_realm_export(
        export_db_path: std::path::PathBuf,
        decryptors: Vec<WorkspaceHistoryRealmExportDecryptor>,
    ) -> Result<(Self, DateTime), WorkspaceHistoryStoreStartError> {
        let (access, realm_id) =
            data_access_realm_export::RealmExportDataAccess::start(export_db_path, decryptors)
                .await;

        Self::start(realm_id, DataAccess::RealmExport(access)).await
    }

    async fn start(
        realm_id: VlobID,
        access: DataAccess,
    ) -> Result<(Self, DateTime), WorkspaceHistoryStoreStartError> {
        let timestamp_higher_bound = DateTime::now();
        let workspace_manifest_v1 =
            access
                .get_workspace_manifest_v1()
                .await
                .map_err(|err| match err {
                    DataAccessFetchManifestError::Offline(e) => {
                        WorkspaceHistoryStoreStartError::Offline(e)
                    }
                    DataAccessFetchManifestError::Stopped => {
                        WorkspaceHistoryStoreStartError::Stopped
                    }
                    DataAccessFetchManifestError::EntryNotFound => {
                        WorkspaceHistoryStoreStartError::NoHistory
                    }
                    DataAccessFetchManifestError::NoRealmAccess => {
                        WorkspaceHistoryStoreStartError::NoRealmAccess
                    }
                    DataAccessFetchManifestError::InvalidKeysBundle(err) => {
                        WorkspaceHistoryStoreStartError::InvalidKeysBundle(err)
                    }
                    DataAccessFetchManifestError::InvalidCertificate(err) => {
                        WorkspaceHistoryStoreStartError::InvalidCertificate(err)
                    }
                    DataAccessFetchManifestError::InvalidManifest(err) => {
                        WorkspaceHistoryStoreStartError::InvalidManifest(err)
                    }
                    DataAccessFetchManifestError::Internal(err) => err.into(),
                })?;
        let timestamp_lower_bound = workspace_manifest_v1.timestamp;

        let mut cache = WorkspaceHistoryStoreCache::default();
        cache
            .populate_manifest_at(
                timestamp_lower_bound,
                CachePopulateManifestEntry::Exists(ArcChildManifest::Folder(workspace_manifest_v1)),
            )
            .expect("empty cache cannot fail");

        let ops = Self {
            realm_id,
            access,
            timestamp_lower_bound,
            timestamp_higher_bound,
            cache: Mutex::new(cache),
        };

        let initial_timestamp_of_interest = timestamp_lower_bound;

        Ok((ops, initial_timestamp_of_interest))
    }

    pub fn realm_id(&self) -> VlobID {
        self.realm_id
    }

    pub fn timestamp_lower_bound(&self) -> DateTime {
        self.timestamp_lower_bound
    }

    pub fn timestamp_higher_bound(&self) -> DateTime {
        self.timestamp_higher_bound
    }

    pub async fn resolve_path(
        &self,
        at: DateTime,
        path: &FsPath,
    ) -> Result<ArcChildManifest, WorkspaceHistoryStoreResolvePathError> {
        resolve_path::resolve_path(self, at, path).await
    }

    #[allow(dead_code)]
    pub async fn retrieve_path_from_id(
        &self,
        at: DateTime,
        entry_id: VlobID,
    ) -> Result<(ArcChildManifest, FsPath), WorkspaceHistoryStoreRetrievePathFromIDError> {
        retrieve_path_from_id::retrieve_path_from_id(self, at, entry_id).await
    }

    pub async fn get_entry(
        &self,
        at: DateTime,
        entry_id: VlobID,
    ) -> Result<ArcChildManifest, WorkspaceHistoryStoreGetEntryError> {
        get_entry::get_entry(self, at, entry_id).await
    }

    pub async fn get_block(
        &self,
        manifest: &FileManifest,
        access: &BlockAccess,
    ) -> Result<Bytes, WorkspaceHistoryStoreGetBlockError> {
        get_block::get_block(self, manifest, access).await
    }
}
