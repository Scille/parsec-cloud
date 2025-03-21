// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! Workspace storage is only responsible for storing/fetching encrypted items:
//! - No cache&concurrency is handled at this level: better let the higher level
//!   components do that since they have a better idea of what should be cached.
//! - No encryption/serialization: simplify code (no need for type dispatching)
//!   and testing (no need to build valid certificates objects).

use std::path::Path;

use libparsec_types::prelude::*;

use crate::platform::workspace::PlatformWorkspaceStorage;

pub const TYPICAL_BLOCK_SIZE: u64 = 512 * 1024; // 512 KB

#[derive(Debug, thiserror::Error)]
pub enum MarkPreventSyncPatternFullyAppliedError {
    /// Error returned when `mark_prevent_sync_pattern_fully_applied` is called with a pattern that is not the same as the current one.
    #[error("Provided prevent sync pattern does not match the one in use")]
    PatternMismatch,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct WorkspaceStorage {
    platform: PlatformWorkspaceStorage,
}

pub struct UpdateManifestData {
    pub entry_id: VlobID,
    pub encrypted: RawEncryptedManifest,
    pub need_sync: bool,
    pub base_version: VersionInt,
}

pub type RawEncryptedManifest = Vec<u8>;
pub type RawEncryptedChunk = Vec<u8>;
pub type RawEncryptedBlock = Vec<u8>;
pub type RawEncryptedBytes = Vec<u8>;

pub enum PopulateManifestOutcome {
    Stored,
    AlreadyPresent,
}

#[cfg(any(test, feature = "expose-test-methods"))]
#[derive(Debug, PartialEq, Eq, Default)]
pub struct DebugDump {
    pub checkpoint: u64,
    pub vlobs: Vec<DebugVlob>,
    pub chunks: Vec<DebugChunk>,
    pub blocks: Vec<DebugBlock>,
}

#[cfg(any(test, feature = "expose-test-methods"))]
#[derive(Debug, PartialEq, Eq, Default)]
pub struct DebugVlob {
    pub id: VlobID,
    pub need_sync: bool,
    pub base_version: u32,
    pub remote_version: u32,
}

#[cfg(any(test, feature = "expose-test-methods"))]
#[derive(Debug, PartialEq, Eq, Default)]
pub struct DebugChunk {
    pub id: ChunkID,
    pub size: u32,
    pub offline: bool,
}

#[cfg(any(test, feature = "expose-test-methods"))]
#[derive(Debug, PartialEq, Eq, Default)]
pub struct DebugBlock {
    pub id: BlockID,
    pub size: u32,
    pub offline: bool,
    pub accessed_on: String,
}

impl WorkspaceStorage {
    pub async fn start(
        data_base_dir: &Path,
        device: &LocalDevice,
        realm_id: VlobID,
        cache_size: u64,
    ) -> anyhow::Result<Self> {
        // `maybe_populate_certificate_storage` needs to start a `WorkspaceStorage`,
        // leading to a recursive call which is not supported for async functions.
        // Hence `no_populate_start` which breaks the recursion.
        //
        // Also note we don't try to return the `WorkspaceStorage` that has been
        // used during the populate as it would change the internal state of the
        // storage (typically caches) depending of if populate has been needed or not.

        #[cfg(feature = "test-with-testbed")]
        crate::testbed::maybe_populate_workspace_storage(data_base_dir, device, realm_id).await;

        Self::no_populate_start(data_base_dir, device, realm_id, cache_size).await
    }

    pub(crate) async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
        realm_id: VlobID,
        cache_size: u64,
    ) -> anyhow::Result<Self> {
        let cache_max_blocks = cache_size / TYPICAL_BLOCK_SIZE;
        let platform = PlatformWorkspaceStorage::no_populate_start(
            data_base_dir,
            device,
            realm_id,
            cache_max_blocks,
        )
        .await?;
        Ok(Self { platform })
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        self.platform.stop().await
    }

    pub async fn get_realm_checkpoint(&mut self) -> anyhow::Result<IndexInt> {
        self.platform.get_realm_checkpoint().await
    }

    pub async fn update_realm_checkpoint(
        &mut self,
        new_checkpoint: IndexInt,
        changed_vlobs: &[(VlobID, VersionInt)],
    ) -> anyhow::Result<()> {
        self.platform
            .update_realm_checkpoint(new_checkpoint, changed_vlobs)
            .await
    }

    pub async fn get_inbound_need_sync(&mut self, limit: u32) -> anyhow::Result<Vec<VlobID>> {
        self.platform.get_inbound_need_sync(limit).await
    }

    pub async fn get_outbound_need_sync(&mut self, limit: u32) -> anyhow::Result<Vec<VlobID>> {
        self.platform.get_outbound_need_sync(limit).await
    }

    pub async fn get_manifest(
        &mut self,
        entry_id: VlobID,
    ) -> anyhow::Result<Option<RawEncryptedManifest>> {
        self.platform.get_manifest(entry_id).await
    }

    pub async fn list_manifests(
        &mut self,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Vec<RawEncryptedManifest>> {
        self.platform.list_manifests(offset, limit).await
    }

    pub async fn populate_manifest(
        &mut self,
        manifest: &UpdateManifestData,
    ) -> anyhow::Result<PopulateManifestOutcome> {
        self.platform.populate_manifest(manifest).await
    }

    pub async fn update_manifest(&mut self, manifest: &UpdateManifestData) -> anyhow::Result<()> {
        self.platform.update_manifest(manifest).await
    }

    pub async fn update_manifests(
        &mut self,
        manifests: impl Iterator<Item = UpdateManifestData>,
    ) -> anyhow::Result<()> {
        self.platform.update_manifests(manifests).await
    }

    pub async fn update_manifest_and_chunks(
        &mut self,
        manifest: &UpdateManifestData,
        new_chunks: impl Iterator<Item = (ChunkID, RawEncryptedChunk)>,
        removed_chunks: impl Iterator<Item = ChunkID>,
    ) -> anyhow::Result<()> {
        self.platform
            .update_manifest_and_chunks(manifest, new_chunks, removed_chunks)
            .await
    }

    pub async fn get_chunk(
        &mut self,
        chunk_id: ChunkID,
    ) -> anyhow::Result<Option<RawEncryptedChunk>> {
        self.platform.get_chunk(chunk_id).await
    }

    pub async fn get_chunk_or_block(
        &mut self,
        chunk_id: ChunkID,
        now: DateTime,
    ) -> anyhow::Result<Option<RawEncryptedBytes>> {
        match self.platform.get_chunk(chunk_id).await? {
            Some(data) => Ok(Some(data)),
            None => self.platform.get_block(chunk_id.into(), now).await,
        }
    }

    pub async fn set_chunk(&mut self, chunk_id: ChunkID, encrypted: &[u8]) -> anyhow::Result<()> {
        self.platform.set_chunk(chunk_id, encrypted).await
    }

    pub async fn get_block(
        &mut self,
        block_id: BlockID,
        now: DateTime,
    ) -> anyhow::Result<Option<RawEncryptedBlock>> {
        self.platform.get_block(block_id, now).await
    }

    pub async fn set_block(
        &mut self,
        block_id: BlockID,
        encrypted: &[u8],
        now: DateTime,
    ) -> anyhow::Result<()> {
        self.platform.set_block(block_id, encrypted, now).await
    }

    pub async fn promote_chunk_to_block(
        &mut self,
        chunk_id: ChunkID,
        now: DateTime,
    ) -> anyhow::Result<()> {
        self.platform.promote_chunk_to_block(chunk_id, now).await
    }

    /// Only used for debugging tests
    #[cfg(any(test, feature = "expose-test-methods"))]
    pub async fn debug_dump(&mut self) -> anyhow::Result<DebugDump> {
        self.platform.debug_dump().await
    }

    pub async fn set_prevent_sync_pattern(
        &mut self,
        pattern: &PreventSyncPattern,
    ) -> anyhow::Result<bool> {
        self.platform.set_prevent_sync_pattern(pattern).await
    }

    pub async fn get_prevent_sync_pattern(&mut self) -> anyhow::Result<(PreventSyncPattern, bool)> {
        self.platform.get_prevent_sync_pattern().await
    }

    pub async fn mark_prevent_sync_pattern_fully_applied(
        &mut self,
        pattern: &PreventSyncPattern,
    ) -> Result<(), MarkPreventSyncPatternFullyAppliedError> {
        self.platform
            .mark_prevent_sync_pattern_fully_applied(pattern)
            .await
    }
}

pub async fn workspace_storage_non_speculative_init(
    data_base_dir: &Path,
    device: &LocalDevice,
    realm_id: VlobID,
) -> anyhow::Result<()> {
    // 1) Open & initialize the database

    #[cfg(feature = "test-with-testbed")]
    crate::testbed::mark_as_populated_workspace_storage(data_base_dir, device, realm_id).await;

    let mut storage =
        PlatformWorkspaceStorage::no_populate_start(data_base_dir, device, realm_id, u64::MAX)
            .await
            .map_err(|err| err.context("cannot initialize database"))?;

    // 2) Populate the database with the workspace manifest

    let timestamp = device.now();
    let manifest = LocalFolderManifest::new_root(device.device_id, realm_id, timestamp, false);

    storage
        .update_manifest(&UpdateManifestData {
            entry_id: manifest.base.id,
            encrypted: manifest.dump_and_encrypt(&device.local_symkey),
            need_sync: manifest.need_sync,
            base_version: manifest.base.version,
        })
        .await
        .map_err(|err| err.context("cannot store workspace manifest"))?;

    // 4) All done ! Don't forget to close the database before exiting ;-)

    storage.stop().await?;

    Ok(())
}

#[cfg(test)]
#[path = "../tests/unit/workspace.rs"]
mod test;
