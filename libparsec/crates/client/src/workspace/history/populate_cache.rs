// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, sync::Arc};

use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::fetch::FetchRemoteManifestError,
};

use super::CacheResolvedEntry;

#[derive(Debug, thiserror::Error)]
pub(super) enum PopulateCacheFromServerError {
    #[error("Cannot reach the server")]
    Offline,
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

pub(super) async fn populate_cache_from_server(
    ops: &super::WorkspaceHistoryOps,
    at: DateTime,
    entry_id: VlobID,
) -> Result<ArcLocalChildManifest, PopulateCacheFromServerError> {
    // 1) Server lookup

    let outcome = if ops.realm_id == entry_id {
        super::super::fetch::fetch_remote_workspace_manifest(
            &ops.cmds,
            &ops.certificates_ops,
            ops.realm_id,
            Some(at),
        )
        .await
        .map(ChildManifest::Folder)
    } else {
        super::super::fetch::fetch_remote_child_manifest(
            &ops.cmds,
            &ops.certificates_ops,
            ops.realm_id,
            entry_id,
            Some(at),
        )
        .await
    };

    let maybe_manifest = match outcome {
        Ok(ChildManifest::File(manifest)) => Some(ArcLocalChildManifest::File(Arc::new(
            LocalFileManifest::from_remote(manifest),
        ))),
        Ok(ChildManifest::Folder(manifest)) => Some(ArcLocalChildManifest::Folder(Arc::new(
            LocalFolderManifest::from_remote(manifest, &PreventSyncPattern::empty()),
        ))),
        // This is unexpected: we got an entry ID from a parent folder/workspace
        // manifest, but this ID points to nothing according to the server :/
        //
        // That could means two things:
        // - the server is lying to us
        // - the client that have uploaded the parent folder/workspace manifest
        //   was buggy and include the ID of a not-yet-synchronized entry
        //
        // We just pretend the entry doesn't exist
        Err(FetchRemoteManifestError::VlobNotFound) => None,
        Err(FetchRemoteManifestError::Stopped) => {
            return Err(PopulateCacheFromServerError::Stopped);
        }
        Err(FetchRemoteManifestError::Offline) => {
            return Err(PopulateCacheFromServerError::Offline);
        }
        // The realm doesn't exist on server side, hence we are it creator and
        // it data only live on our local storage, which we have already checked.
        Err(FetchRemoteManifestError::RealmNotFound) => {
            return Err(PopulateCacheFromServerError::EntryNotFound);
        }
        Err(FetchRemoteManifestError::NoRealmAccess) => {
            return Err(PopulateCacheFromServerError::NoRealmAccess);
        }
        Err(FetchRemoteManifestError::InvalidKeysBundle(err)) => {
            return Err(PopulateCacheFromServerError::InvalidKeysBundle(err));
        }
        Err(FetchRemoteManifestError::InvalidCertificate(err)) => {
            return Err(PopulateCacheFromServerError::InvalidCertificate(err));
        }
        Err(FetchRemoteManifestError::InvalidManifest(err)) => {
            return Err(PopulateCacheFromServerError::InvalidManifest(err));
        }
        Err(FetchRemoteManifestError::Internal(err)) => {
            return Err(err.context("cannot fetch from server").into());
        }
    };

    // 2) We got our manifest, update cache with it

    let mut cache = ops.cache.lock().expect("Mutex is poisoned");

    let resolutions = match cache.resolutions.entry(at) {
        std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
        std::collections::hash_map::Entry::Vacant(entry) => entry.insert(HashMap::default()),
    };

    let manifest = match resolutions.entry(entry_id) {
        std::collections::hash_map::Entry::Vacant(entry) => match maybe_manifest {
            Some(manifest) => {
                entry.insert(CacheResolvedEntry::Exists(manifest.clone()));
                manifest
            }
            None => {
                entry.insert(CacheResolvedEntry::NotFound);
                return Err(PopulateCacheFromServerError::EntryNotFound);
            }
        },
        std::collections::hash_map::Entry::Occupied(entry) => {
            // Plot twist: a concurrent operation has updated the cache !
            // So we discard the data we've fetched and pretend we got a cache hit in
            // the first place.
            match entry.get() {
                CacheResolvedEntry::Exists(arc_local_child_manifest) => {
                    arc_local_child_manifest.to_owned()
                }
                CacheResolvedEntry::NotFound => {
                    return Err(PopulateCacheFromServerError::EntryNotFound)
                }
            }
        }
    };

    Ok(manifest)
}
