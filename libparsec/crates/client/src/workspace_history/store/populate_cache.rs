// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{CacheResolvedEntry, DataAccessFetchManifestError, WorkspaceHistoryStore};

pub(crate) type PopulateManifestCacheError = DataAccessFetchManifestError;

pub(super) async fn populate_manifest_cache(
    ops: &WorkspaceHistoryStore,
    at: DateTime,
    entry_id: VlobID,
) -> Result<ArcChildManifest, PopulateManifestCacheError> {
    // 1) Server lookup

    let maybe_manifest = ops.access.fetch_manifest(at, entry_id).await?;

    // 2) We got our manifest, update cache with it

    let mut cache = ops.cache.lock().expect("Mutex is poisoned");

    let resolutions_at = match cache.resolutions.entry(at) {
        std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
        std::collections::hash_map::Entry::Vacant(entry) => entry.insert(Default::default()),
    };

    let manifest = match resolutions_at.entry(entry_id) {
        std::collections::hash_map::Entry::Vacant(entry) => match maybe_manifest {
            Some(manifest) => {
                entry.insert(CacheResolvedEntry::Exists(manifest.clone()));
                manifest
            }
            None => {
                entry.insert(CacheResolvedEntry::NotFound);
                return Err(PopulateManifestCacheError::EntryNotFound);
            }
        },
        std::collections::hash_map::Entry::Occupied(entry) => {
            // Plot twist: a concurrent operation has updated the cache !
            // So we discard the data we've fetched and pretend we got a cache hit in
            // the first place.
            match entry.get() {
                CacheResolvedEntry::Exists(manifest) => manifest.to_owned(),
                CacheResolvedEntry::NotFound => {
                    return Err(PopulateManifestCacheError::EntryNotFound)
                }
            }
        }
    };

    Ok(manifest)
}
