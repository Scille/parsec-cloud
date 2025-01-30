// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{
    populate_cache::populate_manifest_cache, CacheResolvedEntry, PopulateManifestCacheError,
    WorkspaceHistoryStore,
};

pub(crate) type WorkspaceHistoryStoreResolvePathError = PopulateManifestCacheError;

pub(super) async fn resolve_path(
    store: &WorkspaceHistoryStore,
    at: DateTime,
    path: &FsPath,
) -> Result<ArcChildManifest, WorkspaceHistoryStoreResolvePathError> {
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
            let mut cache = store.cache.lock().expect("Mutex is poisoned");
            cache_only_path_resolution(store, &mut cache, at, path_parts)
        };
        match cache_only_outcome {
            CacheOnlyPathResolutionOutcome::Done(manifest) => return Ok(manifest),
            CacheOnlyPathResolutionOutcome::EntryNotFound => {
                return Err(WorkspaceHistoryStoreResolvePathError::EntryNotFound)
            }
            // We got a cache miss
            CacheOnlyPathResolutionOutcome::NeedPopulateCache(cache_miss_entry_id) => {
                populate_manifest_cache(store, at, cache_miss_entry_id).await?;
            }
        }
    }
}

enum CacheOnlyPathResolutionOutcome {
    Done(ArcChildManifest),
    EntryNotFound,
    NeedPopulateCache(VlobID),
}

fn cache_only_path_resolution(
    store: &WorkspaceHistoryStore,
    cache: &mut super::WorkspaceHistoryStoreCache,
    at: DateTime,
    path_parts: &[EntryName],
) -> CacheOnlyPathResolutionOutcome {
    let root_manifest = match cache.resolve_manifest_at(at, store.realm_id) {
        CacheResolvedEntry::Exists(manifest) => manifest,
        CacheResolvedEntry::NotFound => return CacheOnlyPathResolutionOutcome::EntryNotFound,
        CacheResolvedEntry::CacheMiss => {
            return CacheOnlyPathResolutionOutcome::NeedPopulateCache(store.realm_id)
        }
    };

    let mut current_manifest = root_manifest;
    let mut parts_index = 0;
    loop {
        let child_name = match path_parts.get(parts_index) {
            Some(part) => part,
            // The path is entirely resolved !
            None => return CacheOnlyPathResolutionOutcome::Done(current_manifest),
        };

        let parent_manifest = match current_manifest {
            ArcChildManifest::Folder(manifest) => manifest,
            ArcChildManifest::File(_) => return CacheOnlyPathResolutionOutcome::EntryNotFound,
        };

        let child_id = match parent_manifest.children.get(child_name) {
            Some(id) => *id,
            None => return CacheOnlyPathResolutionOutcome::EntryNotFound,
        };

        let child_manifest = match cache.resolve_manifest_at(at, child_id) {
            CacheResolvedEntry::Exists(manifest) => manifest,
            CacheResolvedEntry::NotFound => return CacheOnlyPathResolutionOutcome::EntryNotFound,
            CacheResolvedEntry::CacheMiss => {
                return CacheOnlyPathResolutionOutcome::NeedPopulateCache(child_id)
            }
        };

        // Ensure the child agrees with the parent on the parenting relationship
        if child_manifest.parent() != parent_manifest.id {
            return CacheOnlyPathResolutionOutcome::EntryNotFound;
        }

        current_manifest = child_manifest;
        parts_index += 1;
    }
}
