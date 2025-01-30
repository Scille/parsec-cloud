// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_types::prelude::*;

use super::WorkspaceHistoryRealmExportDecryptor;
use crate::certif::{
    CertificateOps, InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError,
};

mod data_access;
mod data_access_realm_export;
mod data_access_server;
mod get_block;
mod get_entry;
mod populate_cache;
mod resolve_path;
mod retrieve_path_from_id;

use data_access::*;
use data_access_realm_export::*;
use data_access_server::*;
pub(super) use get_block::*;
pub(super) use get_entry::*;
pub(super) use resolve_path::*;
pub(super) use retrieve_path_from_id::*;

#[derive(Debug)]
enum CacheResolvedEntry {
    Exists(ArcChildManifest),
    NotFound,
}

#[derive(Default)]
pub(super) struct WorkspaceHistoryStoreCache {
    resolutions: HashMap<DateTime, HashMap<VlobID, CacheResolvedEntry>>,
    blocks: HashMap<BlockID, Bytes>,
}

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
    #[error("Cannot reach the server")]
    Offline,
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
    pub async fn start_with_server_access(
        cmds: Arc<AuthenticatedCmds>,
        certificates_ops: Arc<CertificateOps>,
        realm_id: VlobID,
    ) -> Result<Self, WorkspaceHistoryStoreStartError> {
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
    ) -> Result<Self, WorkspaceHistoryStoreStartError> {
        let (access, realm_id) =
            data_access_realm_export::RealmExportDataAccess::start(export_db_path, decryptors)
                .await;

        Self::start(realm_id, DataAccess::RealmExport(access)).await
    }

    async fn start(
        realm_id: VlobID,
        access: DataAccess,
    ) -> Result<Self, WorkspaceHistoryStoreStartError> {
        let timestamp_higher_bound = DateTime::now();
        let timestamp_lower_bound = access
            .get_workspace_manifest_v1_timestamp()
            .await
            .map_err(|err| match err {
                DataAccessGetWorkspaceManifestV1TimestampError::Offline => {
                    WorkspaceHistoryStoreStartError::Offline
                }
                DataAccessGetWorkspaceManifestV1TimestampError::Stopped => {
                    WorkspaceHistoryStoreStartError::Stopped
                }
                DataAccessGetWorkspaceManifestV1TimestampError::NoRealmAccess => {
                    WorkspaceHistoryStoreStartError::NoRealmAccess
                }
                DataAccessGetWorkspaceManifestV1TimestampError::InvalidKeysBundle(err) => {
                    WorkspaceHistoryStoreStartError::InvalidKeysBundle(err)
                }
                DataAccessGetWorkspaceManifestV1TimestampError::InvalidCertificate(err) => {
                    WorkspaceHistoryStoreStartError::InvalidCertificate(err)
                }
                DataAccessGetWorkspaceManifestV1TimestampError::InvalidManifest(err) => {
                    WorkspaceHistoryStoreStartError::InvalidManifest(err)
                }
                DataAccessGetWorkspaceManifestV1TimestampError::Internal(err) => err.into(),
            })
            .and_then(|maybe_timestamp| match maybe_timestamp {
                Some(timestamp) => Ok(timestamp),
                None => Err(WorkspaceHistoryStoreStartError::NoHistory),
            })?;

        Ok(Self {
            realm_id,
            access,
            timestamp_lower_bound,
            timestamp_higher_bound,
            cache: Default::default(),
        })
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
