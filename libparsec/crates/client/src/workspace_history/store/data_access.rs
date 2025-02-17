// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use crate::certif::{
    InvalidBlockAccessError, InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError,
};
use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum DataAccessFetchManifestError {
    // All those errors are for server access mode
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),

    /// `Internal` error is use by both server access and realm export access mode
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum DataAccessFetchBlockError {
    // All those errors are for server access mode
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("The block doesn't exist on the server")]
    BlockNotFound,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("Block access is temporary unavailable on the server")]
    ServerBlockstoreUnavailable,
    #[error(transparent)]
    InvalidBlockAccess(#[from] Box<InvalidBlockAccessError>),
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),

    /// `Internal` error is use by both server access and realm export access mode
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) enum DataAccess {
    // Realm export database support is not available on web.
    #[cfg(not(target_arch = "wasm32"))]
    RealmExport(super::RealmExportDataAccess),
    Server(super::ServerDataAccess),
}

impl DataAccess {
    pub async fn fetch_manifest(
        &self,
        at: DateTime,
        entry_id: VlobID,
    ) -> Result<ArcChildManifest, DataAccessFetchManifestError> {
        match self {
            // Realm export database support is not available on web.
            #[cfg(not(target_arch = "wasm32"))]
            DataAccess::RealmExport(data_access) => data_access.fetch_manifest(at, entry_id).await,
            DataAccess::Server(data_access) => data_access.fetch_manifest(at, entry_id).await,
        }
    }

    pub async fn fetch_block(
        &self,
        manifest: &FileManifest,
        access: &BlockAccess,
    ) -> Result<Bytes, DataAccessFetchBlockError> {
        match self {
            // Realm export database support is not available on web.
            #[cfg(not(target_arch = "wasm32"))]
            DataAccess::RealmExport(data_access) => data_access.fetch_block(manifest, access).await,
            DataAccess::Server(data_access) => data_access.fetch_block(manifest, access).await,
        }
    }

    pub async fn get_workspace_manifest_v1(
        &self,
    ) -> Result<Arc<FolderManifest>, DataAccessFetchManifestError> {
        match self {
            // Realm export database support is not available on web.
            #[cfg(not(target_arch = "wasm32"))]
            DataAccess::RealmExport(data_access) => data_access.get_workspace_manifest_v1().await,
            DataAccess::Server(data_access) => data_access.get_workspace_manifest_v1().await,
        }
    }
}
