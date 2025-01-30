// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{
    populate_manifest_cache, CacheResolvedEntry, PopulateManifestCacheError, WorkspaceHistoryStore,
};

pub(crate) type WorkspaceHistoryStoreGetEntryError = PopulateManifestCacheError;

pub(super) async fn get_entry(
    ops: &WorkspaceHistoryStore,
    at: DateTime,
    entry_id: VlobID,
) -> Result<ArcChildManifest, WorkspaceHistoryStoreGetEntryError> {
    // Cache lookup

    {
        let cache = ops.cache.lock().expect("Mutex is poisoned");
        match cache.resolve_manifest_at(at, entry_id) {
            CacheResolvedEntry::Exists(manifest) => return Ok(manifest),
            CacheResolvedEntry::NotFound => {
                return Err(WorkspaceHistoryStoreGetEntryError::EntryNotFound)
            }
            CacheResolvedEntry::CacheMiss => {}
        }
    }

    // Cache miss, must fetch from server

    populate_manifest_cache(ops, at, entry_id).await
}
