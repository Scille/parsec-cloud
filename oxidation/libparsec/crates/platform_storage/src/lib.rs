// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_safety_doc)]
#![deny(clippy::future_not_send)]
#![deny(clippy::undocumented_unsafe_blocks)]

mod error;
#[cfg(not(target_arch = "wasm32"))]
pub mod sqlite;

use std::collections::HashSet;

use async_trait::async_trait;

use libparsec_types::{BlockID, ChunkID, EntryID, LocalManifest, Regex};

pub use error::{Result, StorageError};

/// Component that is able to read & write chunk from/to an internal storage.
///
/// Chunk is a block of data this is used alongside manifest
#[async_trait]
pub trait ChunkStorage {
    /// Get the number of chunks in the storage.
    async fn get_nb_chunks(&self) -> Result<usize>;
    /// Get accumulated size of all chunks.
    async fn get_total_size(&self) -> Result<usize>;
    /// `true` if the provided `chunk_id` is stored in the storage.
    async fn is_chunk(&self, chunk_id: ChunkID) -> Result<bool>;
    /// Return a sub-list of `chunk_ids` whose ids is present in the storage.
    async fn get_local_chunk_ids(&self, chunk_ids: &[ChunkID]) -> Result<Vec<ChunkID>>;
    /// Get the chunk data identified by `chunk_id`.
    async fn get_chunk(&self, chunk_id: ChunkID) -> Result<Vec<u8>>;
    /// Set the chunk data identified by `chunk_id`.
    async fn set_chunk(&self, chunk_id: ChunkID, raw: &[u8]) -> Result<()>;
    /// Remove the chunk identified by `chunk_id` from the storage.
    async fn clear_chunk(&self, chunk_id: ChunkID) -> Result<()>;
    /// Remove the chunks whose ids are listed in `chunk_ids` from the storage.
    async fn clear_chunks(&self, chunk_ids: &[ChunkID]) -> Result<()>;
    /// Try to remove unused data from the storage.
    async fn vacuum(&self) -> Result<()>;
}

/// Entries that need to be sync.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct NeedSyncEntries {
    pub local_changes: HashSet<EntryID>,
    pub remote_changes: HashSet<EntryID>,
}

#[async_trait]
pub trait ManifestStorage {
    /// Commit the data that aren't already persisted in the storage.
    async fn commit_deferred_manifest(&self) -> Result<()>;
    /// Drop the data that aren't already persisted in the storage.
    #[cfg(any(test, feature = "test-utils"))]
    async fn drop_deferred_commit_manifest(&self) -> Result<()>;
    /// Prevent sync pattern is a global configuration, hence at each startup `WorkspaceStorage`
    /// has to make sure this configuration is consistent with what is stored in the DB (see
    /// `set_prevent_sync_pattern`). Long story short, we never need to retrieve the pattern
    /// from the DB when in production. However this is still convenient for testing ;-)
    #[cfg(test)]
    async fn get_prevent_sync_pattern(&self) -> Result<(Regex, bool)>;
    /// Set the "prevent sync" pattern for the corresponding workspace
    /// This operation is idempotent,
    /// i.e it does not reset the `fully_applied` flag if the pattern hasn't changed.
    /// Return whether the pattern is fully applied.
    async fn set_prevent_sync_pattern(&self, pattern: &Regex) -> Result<bool>;
    #[cfg(any(test, feature = "test-utils"))]
    /// Like [ManifestStorage::set_prevent_sync_pattern] but on a more basic level for testing purpose
    async fn set_raw_prevent_sync_pattern(&self, pattern: &str) -> Result<bool>;
    /// Mark the provided pattern as fully applied.
    /// This is meant to be called after one made sure that all the manifests in the
    /// workspace are compliant with the new pattern. The applied pattern is provided
    /// as an argument in order to avoid concurrency issues.
    /// Return whether the pattern is fully applied.
    async fn mark_prevent_sync_pattern_fully_applied(&self, pattern: &Regex) -> Result<bool>;
    /// Get the current realm checkpoint.
    async fn get_realm_checkpoint(&self) -> i64;
    /// Update the realm checkpoint + related vlobs.
    async fn update_realm_checkpoint(
        &self,
        new_checkpoint: i64,
        changed_vlobs: Vec<(EntryID, i64)>,
    ) -> Result<()>;
    /// Return a set of modified entries in the local storage & the remote.
    async fn get_need_sync_entries(&self) -> Result<NeedSyncEntries>;
    /// Get a manifest identified by `entry_id` from the cache.
    /// Will return `None` if not in cache (but could be present in the storage see [ManifestStorage::get_manifest])
    fn get_manifest_in_cache(&self, entry_id: EntryID) -> Option<LocalManifest>;
    /// Get a manifest identified by `entry_id`.
    ///
    /// Will look first in it's internal cache before looking in the storage.
    async fn get_manifest(&self, entry_id: EntryID) -> Result<LocalManifest>;
    /// Set a manifest in the cache (will not save it in the storage).
    fn set_manifest_deferred_commit(
        &self,
        entry_id: EntryID,
        manifest: LocalManifest,
        removed_ids: Option<HashSet<ChunkID>>,
    );
    /// Like [ManifestStorage::set_manifest_cache_only] but will save the manifest in the storage.
    async fn set_manifest(
        &self,
        entry_id: EntryID,
        manifest: LocalManifest,
        removed_ids: Option<HashSet<ChunkID>>,
    ) -> Result<()>;
    /// Ensure that the manifest identified by `entry_id` is save on the storage and not only on the cache.
    async fn ensure_manifest_persistent(&self, entry_id: EntryID) -> Result<()>;
    /// Remove a manifest from the cache & the storage.
    #[cfg(any(test, feature = "test-utils"))]
    async fn clear_manifest(&self, entry_id: EntryID) -> Result<()>;
    /// `true` if the manifest don't have chunk that need to be save to the storage.
    #[cfg(any(test, feature = "test-utils"))]
    fn is_manifest_cache_ahead_of_persistance(&self, entry_id: EntryID) -> bool;
}

#[async_trait]
pub trait Closable {
    async fn close(&self);
}

/// Componenent that is able to read & write block to an internal storage.
///
/// A block store a real data that once grouped together build the encrypted data of a file.
#[async_trait]
pub trait BlockStorage {
    fn cache_size(&self) -> u64;

    fn block_limit(&self) -> u64;

    /// Remove all blocks from the storage.
    async fn clear_all_blocks(&self) -> Result<()>;
    /// Set a block and perform a [BlockStorage::cleanup].
    async fn set_clean_block(&self, block_id: BlockID, raw: &[u8]) -> Result<HashSet<BlockID>>;
    /// Will remove blocks if we have more than [BlockStorage::block_limit]
    ///
    /// The removed blocks will be least used one.
    async fn cleanup(&self) -> Result<HashSet<BlockID>>;
    /// Is block remanence enabled ?
    fn is_block_remanent(&self) -> bool;
    /// Enable block remanence.
    /// Return `true` if the value has changed from the previous state.
    async fn enable_block_remanence(&self) -> Result<bool>;
    /// Disable block remanence.
    /// If the block remanence was disabled by this call,
    /// It will return the list of block that where clean-up.
    async fn disable_block_remanence(&self) -> Result<Option<HashSet<BlockID>>>;
    /// Remove from the database, the chunk whose id is in `chunk_ids` and it's access time older than `not_accessed_after`.
    async fn clear_unreferenced_chunks(
        &self,
        chunk_ids: &[ChunkID],
        not_accessed_after: libparsec_types::DateTime,
    ) -> Result<()>;
}
