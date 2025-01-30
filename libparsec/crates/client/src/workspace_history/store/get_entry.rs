// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{
    populate_cache::populate_manifest_cache, CacheResolvedEntry, DataAccessFetchManifestError,
    WorkspaceHistoryStore,
};

pub(crate) type WorkspaceHistoryStoreGetEntryError = DataAccessFetchManifestError;

pub(super) async fn get_entry(
    ops: &WorkspaceHistoryStore,
    at: DateTime,
    entry_id: VlobID,
) -> Result<ArcChildManifest, WorkspaceHistoryStoreGetEntryError> {
    // Cache lookup

    {
        let cache = ops.cache.lock().expect("Mutex is poisoned");
        if let Some(cache_resolutions) = cache.resolutions.get(&at) {
            if let Some(cache_resolved_entry) = cache_resolutions.get(&entry_id) {
                match cache_resolved_entry {
                    CacheResolvedEntry::Exists(manifest) => return Ok(manifest.to_owned()),
                    CacheResolvedEntry::NotFound => {
                        return Err(WorkspaceHistoryStoreGetEntryError::EntryNotFound)
                    }
                }
            }
        }
    }

    // Cache miss, must fetch from server

    populate_manifest_cache(ops, at, entry_id).await
}
