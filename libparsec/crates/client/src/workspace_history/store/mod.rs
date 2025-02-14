// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::{Arc, Mutex};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

// Realm export database support is not available on web.
#[cfg(not(target_arch = "wasm32"))]
use super::WorkspaceHistoryRealmExportDecryptor;
use crate::certif::{
    CertificateOps, InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError,
};

mod cache;
mod data_access;
// Realm export database support is not available on web.
#[cfg(not(target_arch = "wasm32"))]
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
// Realm export database support is not available on web.
#[cfg(not(target_arch = "wasm32"))]
use data_access_realm_export::*;
use data_access_server::*;
pub(super) use get_block::*;
pub(super) use get_entry::*;
use populate_cache::*;
pub(super) use resolve_path::*;
pub(super) use retrieve_path_from_id::*;

pub(super) struct WorkspaceHistoryStore {
    organization_id: OrganizationID,
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
    // All those errors are for server access mode
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Workspace has no history yet (root manifest has never been synchronized)")]
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

    // Those errors are for realm export access mode
    #[error("Cannot open the realm export database: {0}")]
    CannotOpenRealmExportDatabase(anyhow::Error),
    #[error("The database is not a valid realm export: {0}")]
    InvalidRealmExportDatabase(anyhow::Error),
    #[error("Unsupported realm export format version `{found}` (supported: `{supported}`)")]
    UnsupportedRealmExportDatabaseVersion { supported: u32, found: u32 },
    #[error("The database contains an incomplete realm export")]
    IncompleteRealmExportDatabase,
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
        organization_id: OrganizationID,
        realm_id: VlobID,
    ) -> Result<(Self, DateTime), WorkspaceHistoryStoreStartError> {
        let access = DataAccess::Server(data_access_server::ServerDataAccess::new(
            cmds,
            certificates_ops,
            realm_id,
        ));

        let timestamp_higher_bound = DateTime::now();
        Self::start(organization_id, realm_id, access, timestamp_higher_bound).await
    }

    // Realm export database support is not available on web.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn start_with_realm_export(
        export_db_path: &std::path::Path,
        decryptors: Vec<WorkspaceHistoryRealmExportDecryptor>,
    ) -> Result<(Self, DateTime), WorkspaceHistoryStoreStartError> {
        let (access, organization_id, realm_id, timestamp_higher_bound) =
            data_access_realm_export::RealmExportDataAccess::start(export_db_path, decryptors)
                .await
                .map_err(|e| match e {
                    RealmExportDataAccessStartError::CannotOpenDatabase(error) => {
                        WorkspaceHistoryStoreStartError::CannotOpenRealmExportDatabase(error)
                    }
                    RealmExportDataAccessStartError::InvalidDatabase(error) => {
                        WorkspaceHistoryStoreStartError::InvalidRealmExportDatabase(error)
                    }
                    RealmExportDataAccessStartError::UnsupportedDatabaseVersion {
                        supported,
                        found,
                    } => WorkspaceHistoryStoreStartError::UnsupportedRealmExportDatabaseVersion {
                        supported,
                        found,
                    },
                    RealmExportDataAccessStartError::IncompleteRealmExport => {
                        WorkspaceHistoryStoreStartError::IncompleteRealmExportDatabase
                    }
                })?;

        Self::start(
            organization_id,
            realm_id,
            DataAccess::RealmExport(access),
            timestamp_higher_bound,
        )
        .await
    }

    async fn start(
        organization_id: OrganizationID,
        realm_id: VlobID,
        access: DataAccess,
        timestamp_higher_bound: DateTime,
    ) -> Result<(Self, DateTime), WorkspaceHistoryStoreStartError> {
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
            organization_id,
            realm_id,
            access,
            timestamp_lower_bound,
            timestamp_higher_bound,
            cache: Mutex::new(cache),
        };

        let initial_timestamp_of_interest = timestamp_lower_bound;

        Ok((ops, initial_timestamp_of_interest))
    }

    pub fn organization_id(&self) -> &OrganizationID {
        &self.organization_id
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
