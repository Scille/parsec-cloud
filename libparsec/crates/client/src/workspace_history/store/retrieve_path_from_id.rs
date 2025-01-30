// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashSet;

use libparsec_types::prelude::*;

use super::{
    populate_cache::populate_manifest_cache, CacheResolvedEntry, PopulateManifestCacheError,
    WorkspaceHistoryStore,
};

pub(crate) type WorkspaceHistoryStoreRetrievePathFromIDError = PopulateManifestCacheError;

/// Retrieve the path and the confinement point of a given entry ID.
pub(super) async fn retrieve_path_from_id(
    store: &WorkspaceHistoryStore,
    at: DateTime,
    entry_id: VlobID,
) -> Result<(ArcChildManifest, FsPath), WorkspaceHistoryStoreRetrievePathFromIDError> {
    loop {
        // Most of the time we should have each entry in the path already in the cache,
        // so we want to lock the cache once and only release it in the unlikely case
        // we need to fetch from the local storage or server.
        let cache_only_outcome = {
            let mut cache = store.cache.lock().expect("Mutex is poisoned");
            cache_only_retrieve_path_from_id(store, &mut cache, at, entry_id)
        };
        match cache_only_outcome {
            CacheOnlyRetrievalPathOutcome::Done((entry_manifest, path)) => {
                return Ok((entry_manifest, path))
            }
            CacheOnlyRetrievalPathOutcome::EntryNotFound => {
                return Err(WorkspaceHistoryStoreRetrievePathFromIDError::EntryNotFound)
            }
            // We got a cache miss
            CacheOnlyRetrievalPathOutcome::NeedPopulateCache(cache_miss_entry_id) => {
                populate_manifest_cache(store, at, cache_miss_entry_id).await?;
            }
        }
    }
}

enum CacheOnlyRetrievalPathOutcome {
    Done((ArcChildManifest, FsPath)),
    EntryNotFound,
    NeedPopulateCache(VlobID),
}

fn cache_only_retrieve_path_from_id(
    store: &WorkspaceHistoryStore,
    cache: &mut super::WorkspaceHistoryStoreCache,
    at: DateTime,
    entry_id: VlobID,
) -> CacheOnlyRetrievalPathOutcome {
    // Initialize the results
    let mut parts = Vec::new();

    // Get the root ID
    let root_entry_id = store.realm_id;

    let entry_manifest = match cache.resolve_manifest_at(at, entry_id) {
        CacheResolvedEntry::Exists(manifest) => manifest,
        CacheResolvedEntry::NotFound => return CacheOnlyRetrievalPathOutcome::EntryNotFound,
        CacheResolvedEntry::CacheMiss => {
            return CacheOnlyRetrievalPathOutcome::NeedPopulateCache(entry_id)
        }
    };

    // Initialize the loop state
    let mut current_entry_id = entry_id;
    let mut current_parent_id = entry_manifest.parent();
    let mut seen: HashSet<VlobID> = HashSet::from_iter([current_entry_id]);

    // Loop until we reach the root
    while current_entry_id != root_entry_id {
        // Get the parent manifest
        let parent_manifest = match cache.resolve_manifest_at(at, current_parent_id) {
            CacheResolvedEntry::Exists(ArcChildManifest::Folder(manifest)) => manifest,
            CacheResolvedEntry::Exists(ArcChildManifest::File(_)) => {
                return CacheOnlyRetrievalPathOutcome::EntryNotFound
            }
            CacheResolvedEntry::NotFound => return CacheOnlyRetrievalPathOutcome::EntryNotFound,
            CacheResolvedEntry::CacheMiss => {
                return CacheOnlyRetrievalPathOutcome::NeedPopulateCache(current_parent_id)
            }
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
