// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashSet;

use libparsec_types::prelude::*;

use super::{
    populate_cache::{populate_cache_from_server, PopulateCacheFromServerError},
    WorkspaceHistoryOps,
};
use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    server_fetch::{server_fetch_block, ServerFetchBlockError},
    workspace::history::CacheResolvedEntry,
};

#[derive(Debug, thiserror::Error)]
pub(super) enum WorkspaceHistoryResolvePathError {
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

pub(super) async fn resolve_path(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    path: &FsPath,
) -> Result<ArcLocalChildManifest, WorkspaceHistoryResolvePathError> {
    // A word about circular path detection:
    // - The path is resolved from the root, which itself is guaranteed to have its
    //   `parent` field pointing on itself.
    // - Each time we resolve a child, we ensure the parent and child agree on the
    //   parenting relationship.
    //
    // This in practice means our valid data form a direct acyclic graph (DAG) and hence
    // we cannot end up in a circular path.
    //
    // Note this is only valid because we start the resolution from the root, if we
    // would be to start from elsewhere (typically to determine the path of a manifest
    // by following its parent backward), we could end up in a circular path with
    // manifests involved forming a "island" disconnect from the root.

    let path_parts = path.parts();

    loop {
        // Most of the time we should have each entry in the path already in the cache,
        // so we want to lock the cache once and only release it in the unlikely case
        // we need to fetch from the local storage or server.
        let cache_only_outcome = {
            let mut cache = ops.cache.lock().expect("Mutex is poisoned");
            cache_only_path_resolution(ops, &mut cache, at, path_parts)
        };
        match cache_only_outcome {
            CacheOnlyPathResolutionOutcome::Done(manifest) => return Ok(manifest),
            CacheOnlyPathResolutionOutcome::EntryNotFound => {
                return Err(WorkspaceHistoryResolvePathError::EntryNotFound)
            }
            // We got a cache miss
            CacheOnlyPathResolutionOutcome::NeedPopulateCache(cache_miss_entry_id) => {
                populate_cache_from_server(ops, at, cache_miss_entry_id)
                    .await
                    .map_err(|err| match err {
                        PopulateCacheFromServerError::Offline => {
                            WorkspaceHistoryResolvePathError::Offline
                        }
                        PopulateCacheFromServerError::Stopped => {
                            WorkspaceHistoryResolvePathError::Stopped
                        }
                        PopulateCacheFromServerError::EntryNotFound => {
                            WorkspaceHistoryResolvePathError::EntryNotFound
                        }
                        PopulateCacheFromServerError::NoRealmAccess => {
                            WorkspaceHistoryResolvePathError::NoRealmAccess
                        }
                        PopulateCacheFromServerError::InvalidKeysBundle(err) => {
                            WorkspaceHistoryResolvePathError::InvalidKeysBundle(err)
                        }
                        PopulateCacheFromServerError::InvalidCertificate(err) => {
                            WorkspaceHistoryResolvePathError::InvalidCertificate(err)
                        }
                        PopulateCacheFromServerError::InvalidManifest(err) => {
                            WorkspaceHistoryResolvePathError::InvalidManifest(err)
                        }
                        PopulateCacheFromServerError::Internal(err) => {
                            err.context("cannot fetch manifest").into()
                        }
                    })?;
            }
        }
    }
}

enum CacheOnlyPathResolutionOutcome {
    Done(ArcLocalChildManifest),
    EntryNotFound,
    NeedPopulateCache(VlobID),
}

fn cache_only_path_resolution(
    ops: &WorkspaceHistoryOps,
    cache: &mut super::Cache,
    at: DateTime,
    path_parts: &[EntryName],
) -> CacheOnlyPathResolutionOutcome {
    enum StepKind<'a> {
        Root,
        Child(&'a ArcLocalChildManifest),
    }
    let cache_resolutions = match cache.resolutions.get(&at) {
        Some(cache_resolutions) => cache_resolutions,
        None => return CacheOnlyPathResolutionOutcome::NeedPopulateCache(ops.realm_id),
    };

    let mut last_step = StepKind::Root;
    let mut parts_index = 0;
    loop {
        let child_name = match path_parts.get(parts_index) {
            Some(part) => part,
            // The path is entirely resolved !
            None => match last_step {
                StepKind::Child(manifest) => {
                    return CacheOnlyPathResolutionOutcome::Done(manifest.to_owned());
                }
                StepKind::Root => {
                    return match cache_resolutions.get(&ops.realm_id) {
                        Some(CacheResolvedEntry::Exists(manifest)) => {
                            CacheOnlyPathResolutionOutcome::Done(manifest.to_owned())
                        }
                        Some(CacheResolvedEntry::NotFound) => {
                            CacheOnlyPathResolutionOutcome::EntryNotFound
                        }
                        None => CacheOnlyPathResolutionOutcome::NeedPopulateCache(ops.realm_id),
                    };
                }
            },
        };

        let parent_manifest = match &last_step {
            StepKind::Root => match cache_resolutions.get(&ops.realm_id) {
                Some(CacheResolvedEntry::Exists(ArcLocalChildManifest::Folder(manifest))) => {
                    manifest
                }
                // The root is always a folder
                Some(CacheResolvedEntry::Exists(ArcLocalChildManifest::File(_))) => unreachable!(),
                Some(CacheResolvedEntry::NotFound) => {
                    return CacheOnlyPathResolutionOutcome::EntryNotFound
                }
                None => return CacheOnlyPathResolutionOutcome::NeedPopulateCache(ops.realm_id),
            },
            StepKind::Child(manifest) => match &manifest {
                ArcLocalChildManifest::Folder(manifest) => manifest,
                ArcLocalChildManifest::File(_) => {
                    // Cannot continue to resolve the path !
                    return CacheOnlyPathResolutionOutcome::EntryNotFound;
                }
            },
        };
        let parent_id = parent_manifest.base.id;

        let child_id = match parent_manifest.children.get(child_name) {
            Some(id) => *id,
            None => return CacheOnlyPathResolutionOutcome::EntryNotFound,
        };

        let child_manifest = match cache_resolutions.get(&child_id) {
            Some(CacheResolvedEntry::Exists(manifest)) => manifest,
            Some(CacheResolvedEntry::NotFound) => {
                return CacheOnlyPathResolutionOutcome::EntryNotFound
            }
            // Cache miss !
            // `part_index` is not incremented here, so we are going to
            // leave the second loop, populate the cache, loop into first
            // loop and finally re-enter the second loop with the same
            // part in the path to resolve.
            None => return CacheOnlyPathResolutionOutcome::NeedPopulateCache(child_id),
        };

        // Ensure the child agrees with the parent on the parenting relationship
        if child_manifest.parent() != parent_id {
            return CacheOnlyPathResolutionOutcome::EntryNotFound;
        }

        last_step = StepKind::Child(child_manifest);
        parts_index += 1;
    }
}

enum CacheOnlyRetrievalPathOutcome {
    Done((ArcLocalChildManifest, FsPath)),
    EntryNotFound,
    NeedPopulateCache(VlobID),
}

fn cache_only_retrieve_path_from_id(
    ops: &WorkspaceHistoryOps,
    cache: &mut super::Cache,
    at: DateTime,
    entry_id: VlobID,
) -> CacheOnlyRetrievalPathOutcome {
    let cache_resolutions = match cache.resolutions.get(&at) {
        Some(cache_resolutions) => cache_resolutions,
        None => return CacheOnlyRetrievalPathOutcome::NeedPopulateCache(entry_id),
    };

    // Initialize the results
    let mut parts = Vec::new();

    // Get the root ID
    let root_entry_id = ops.realm_id;

    let entry_manifest = match cache_resolutions.get(&entry_id) {
        Some(CacheResolvedEntry::Exists(manifest)) => manifest,
        Some(CacheResolvedEntry::NotFound) => return CacheOnlyRetrievalPathOutcome::EntryNotFound,
        None => return CacheOnlyRetrievalPathOutcome::NeedPopulateCache(entry_id),
    };

    // Initialize the loop state
    let mut current_entry_id = entry_id;
    let mut current_parent_id = entry_manifest.parent();
    let mut seen: HashSet<VlobID> = HashSet::from_iter([current_entry_id]);

    // Loop until we reach the root
    while current_entry_id != root_entry_id {
        // Get the parent manifest
        let parent_manifest = match cache_resolutions.get(&current_parent_id) {
            Some(CacheResolvedEntry::Exists(ArcLocalChildManifest::Folder(manifest))) => manifest,
            Some(CacheResolvedEntry::Exists(ArcLocalChildManifest::File(_))) => {
                return CacheOnlyRetrievalPathOutcome::EntryNotFound
            }
            Some(CacheResolvedEntry::NotFound) => {
                return CacheOnlyRetrievalPathOutcome::EntryNotFound
            }
            None => return CacheOnlyRetrievalPathOutcome::NeedPopulateCache(current_parent_id),
        };

        // Update the path result
        let child_name = parent_manifest.children.iter().find_map(|(name, id)| {
            if *id == current_entry_id {
                Some(name)
            } else {
                None
            }
        });
        match child_name {
            None => return CacheOnlyRetrievalPathOutcome::EntryNotFound,
            Some(child_name) => parts.push(child_name.clone()),
        }

        // Update the loop state
        current_entry_id = current_parent_id;
        current_parent_id = parent_manifest.parent;

        // Protect against circular paths
        if !seen.insert(current_entry_id) {
            return CacheOnlyRetrievalPathOutcome::EntryNotFound;
        }
    }

    // Reverse the parts to get the path
    parts.reverse();
    CacheOnlyRetrievalPathOutcome::Done((entry_manifest.to_owned(), FsPath::from_parts(parts)))
}

/// Retrieve the path and the confinement point of a given entry ID.
pub(super) async fn retrieve_path_from_id(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    entry_id: VlobID,
) -> Result<(ArcLocalChildManifest, FsPath), WorkspaceHistoryResolvePathError> {
    loop {
        // Most of the time we should have each entry in the path already in the cache,
        // so we want to lock the cache once and only release it in the unlikely case
        // we need to fetch from the local storage or server.
        let cache_only_outcome = {
            let mut cache = ops.cache.lock().expect("Mutex is poisoned");
            cache_only_retrieve_path_from_id(ops, &mut cache, at, entry_id)
        };
        match cache_only_outcome {
            CacheOnlyRetrievalPathOutcome::Done((entry_manifest, path)) => {
                return Ok((entry_manifest, path))
            }
            CacheOnlyRetrievalPathOutcome::EntryNotFound => {
                return Err(WorkspaceHistoryResolvePathError::EntryNotFound)
            }
            // We got a cache miss
            CacheOnlyRetrievalPathOutcome::NeedPopulateCache(cache_miss_entry_id) => {
                populate_cache_from_server(ops, at, cache_miss_entry_id)
                    .await
                    .map_err(|err| match err {
                        PopulateCacheFromServerError::Offline => {
                            WorkspaceHistoryResolvePathError::Offline
                        }
                        PopulateCacheFromServerError::Stopped => {
                            WorkspaceHistoryResolvePathError::Stopped
                        }
                        PopulateCacheFromServerError::EntryNotFound => {
                            WorkspaceHistoryResolvePathError::EntryNotFound
                        }
                        PopulateCacheFromServerError::NoRealmAccess => {
                            WorkspaceHistoryResolvePathError::NoRealmAccess
                        }
                        PopulateCacheFromServerError::InvalidKeysBundle(err) => {
                            WorkspaceHistoryResolvePathError::InvalidKeysBundle(err)
                        }
                        PopulateCacheFromServerError::InvalidCertificate(err) => {
                            WorkspaceHistoryResolvePathError::InvalidCertificate(err)
                        }
                        PopulateCacheFromServerError::InvalidManifest(err) => {
                            WorkspaceHistoryResolvePathError::InvalidManifest(err)
                        }
                        PopulateCacheFromServerError::Internal(err) => {
                            err.context("cannot fetch manifest").into()
                        }
                    })?;
            }
        }
    }
}

pub(super) type WorkspaceHistoryGetEntryError = PopulateCacheFromServerError;

pub(super) async fn get_entry(
    ops: &WorkspaceHistoryOps,
    at: DateTime,
    entry_id: VlobID,
) -> Result<ArcLocalChildManifest, WorkspaceHistoryGetEntryError> {
    // Cache lookup

    {
        let cache = ops.cache.lock().expect("Mutex is poisoned");
        if let Some(cache_resolutions) = cache.resolutions.get(&at) {
            if let Some(cache_resolved_entry) = cache_resolutions.get(&entry_id) {
                match cache_resolved_entry {
                    CacheResolvedEntry::Exists(manifest) => return Ok(manifest.to_owned()),
                    CacheResolvedEntry::NotFound => {
                        return Err(WorkspaceHistoryGetEntryError::EntryNotFound)
                    }
                }
            }
        }
    }

    // Cache miss, must fetch from server

    populate_cache_from_server(ops, at, entry_id).await
}

pub(super) type WorkspaceHistoryGetBlockError = ServerFetchBlockError;

pub(super) async fn get_block(
    ops: &WorkspaceHistoryOps,
    access: &BlockAccess,
    remote_manifest: &FileManifest,
) -> Result<Bytes, WorkspaceHistoryGetBlockError> {
    // Cache lookup

    {
        let cache = ops.cache.lock().expect("Mutex is poisoned");
        if let Some(block) = cache.blocks.get(&access.id) {
            return Ok(block.to_owned());
        }
    }

    // Cache miss, must fetch from server

    let block = server_fetch_block(
        &ops.cmds,
        &ops.certificates_ops,
        ops.realm_id,
        remote_manifest,
        access,
    )
    .await?;

    let mut cache = ops.cache.lock().expect("Mutex is poisoned");
    let block = match cache.blocks.entry(access.id) {
        std::collections::hash_map::Entry::Vacant(entry) => {
            entry.insert(block.clone());
            block
        }
        std::collections::hash_map::Entry::Occupied(entry) => {
            // Plot twist: a concurrent operation has updated the cache !
            // So we discard the data we've fetched and pretend we got a cache hit in
            // the first place.
            entry.get().clone()
        }
    };

    Ok(block)
}
