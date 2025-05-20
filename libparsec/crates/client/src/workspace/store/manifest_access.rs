// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError};

use super::cache::{
    PopulateCacheFromLocalStorageOrServerError, populate_cache_from_local_storage_or_server,
};

#[derive(Debug, thiserror::Error)]
pub(crate) enum GetManifestError {
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
    Internal(#[from] anyhow::Error),
}

pub(super) async fn get_manifest(
    store: &super::WorkspaceStore,
    entry_id: VlobID,
) -> Result<ArcLocalChildManifest, GetManifestError> {
    // Fast path: cache lookup
    let maybe_found = store
        .data
        .with_current_view_cache(|cache| cache.manifests.get(&entry_id).cloned());
    if let Some(manifest) = maybe_found {
        return Ok(manifest);
    }

    // Entry not in the cache, try to fetch it from the local storage...
    let manifest = populate_cache_from_local_storage_or_server(store, entry_id)
        .await
        .map_err(|err| match err {
            PopulateCacheFromLocalStorageOrServerError::Offline(e) => GetManifestError::Offline(e),
            PopulateCacheFromLocalStorageOrServerError::Stopped => GetManifestError::Stopped,
            PopulateCacheFromLocalStorageOrServerError::EntryNotFound => {
                GetManifestError::EntryNotFound
            }
            PopulateCacheFromLocalStorageOrServerError::NoRealmAccess => {
                GetManifestError::NoRealmAccess
            }
            PopulateCacheFromLocalStorageOrServerError::InvalidKeysBundle(err) => {
                GetManifestError::InvalidKeysBundle(err)
            }
            PopulateCacheFromLocalStorageOrServerError::InvalidCertificate(err) => {
                GetManifestError::InvalidCertificate(err)
            }
            PopulateCacheFromLocalStorageOrServerError::InvalidManifest(err) => {
                GetManifestError::InvalidManifest(err)
            }
            PopulateCacheFromLocalStorageOrServerError::Internal(err) => err.into(),
        })?;

    Ok(manifest)
}
