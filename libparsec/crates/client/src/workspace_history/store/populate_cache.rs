// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError};

use super::{
    CachePopulateManifestAtError, CachePopulateManifestEntry, DataAccessFetchManifestError,
    InvalidManifestHistoryError, WorkspaceHistoryStore,
};

#[derive(Debug, thiserror::Error)]
pub enum PopulateManifestCacheError {
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
    #[error(transparent)]
    InvalidHistory(#[from] Box<InvalidManifestHistoryError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<DataAccessFetchManifestError> for PopulateManifestCacheError {
    fn from(err: DataAccessFetchManifestError) -> Self {
        match err {
            DataAccessFetchManifestError::Offline(e) => PopulateManifestCacheError::Offline(e),
            DataAccessFetchManifestError::Stopped => PopulateManifestCacheError::Stopped,
            DataAccessFetchManifestError::EntryNotFound => {
                PopulateManifestCacheError::EntryNotFound
            }
            DataAccessFetchManifestError::NoRealmAccess => {
                PopulateManifestCacheError::NoRealmAccess
            }
            DataAccessFetchManifestError::InvalidKeysBundle(err) => {
                PopulateManifestCacheError::InvalidKeysBundle(err)
            }
            DataAccessFetchManifestError::InvalidCertificate(err) => {
                PopulateManifestCacheError::InvalidCertificate(err)
            }
            DataAccessFetchManifestError::InvalidManifest(err) => {
                PopulateManifestCacheError::InvalidManifest(err)
            }
            DataAccessFetchManifestError::Internal(err) => {
                PopulateManifestCacheError::Internal(err)
            }
        }
    }
}

impl From<CachePopulateManifestAtError> for PopulateManifestCacheError {
    fn from(err: CachePopulateManifestAtError) -> Self {
        match err {
            CachePopulateManifestAtError::InvalidHistory(err) => {
                PopulateManifestCacheError::InvalidHistory(err)
            }
        }
    }
}

pub(super) async fn populate_manifest_cache(
    ops: &WorkspaceHistoryStore,
    at: DateTime,
    entry_id: VlobID,
) -> Result<ArcChildManifest, PopulateManifestCacheError> {
    // 1) Server lookup

    let maybe_manifest = ops.access.fetch_manifest(at, entry_id).await?;

    // 2) We got our manifest, update cache with it

    let mut cache = ops.cache.lock().expect("Mutex is poisoned");

    match maybe_manifest {
        Some(manifest) => {
            cache.populate_manifest_at(at, CachePopulateManifestEntry::Exists(manifest.clone()))?;
            Ok(manifest)
        }
        None => {
            cache.populate_manifest_at(at, CachePopulateManifestEntry::NotFound(entry_id))?;
            Err(PopulateManifestCacheError::EntryNotFound)
        }
    }
}
