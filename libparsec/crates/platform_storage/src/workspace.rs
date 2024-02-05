// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! Workspace storage is only responsible for storing/fetching encrypted items:
//! - No cache&concurrency is handled at this level: better let the higher level
//!   components do that since they have a better idea of what should be cached.
//! - No encryption/serialization: simplify code (no need for type dispatching)
//!   and testing (no need to build valid certificates objects.

use std::path::Path;

use libparsec_types::prelude::*;

use crate::platform::workspace::PlatformWorkspaceStorage;

// Re-expose `workspace_storage_non_speculative_init`
pub use crate::platform::workspace::workspace_storage_non_speculative_init;

#[derive(Debug)]
pub struct WorkspaceStorage {
    platform: PlatformWorkspaceStorage,
}

pub struct UpdateManifestData {
    pub entry_id: VlobID,
    pub encrypted: Vec<u8>,
    pub need_sync: bool,
    pub base_version: VersionInt,
}

impl WorkspaceStorage {
    pub async fn start(
        data_base_dir: &Path,
        device: &LocalDevice,
        realm_id: VlobID,
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

        Self::no_populate_start(data_base_dir, device, realm_id).await
    }

    pub(crate) async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
        realm_id: VlobID,
    ) -> anyhow::Result<Self> {
        let platform =
            PlatformWorkspaceStorage::no_populate_start(data_base_dir, device, realm_id).await?;
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

    pub async fn get_manifest(&mut self, entry_id: VlobID) -> anyhow::Result<Option<Vec<u8>>> {
        self.platform.get_manifest(entry_id).await
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
        new_chunks: impl Iterator<Item = (ChunkID, Vec<u8>)>,
        removed_chunks: impl Iterator<Item = ChunkID>,
    ) -> anyhow::Result<()> {
        self.platform
            .update_manifest_and_chunks(manifest, new_chunks, removed_chunks)
            .await
    }

    pub async fn get_chunk(&mut self, id: ChunkID) -> anyhow::Result<Option<Vec<u8>>> {
        self.platform.get_chunk(id).await
    }

    pub async fn set_chunk(&mut self, id: ChunkID, encrypted: &[u8]) -> anyhow::Result<()> {
        self.platform.set_chunk(id, encrypted).await
    }

    pub async fn get_block(&mut self, block_id: BlockID) -> anyhow::Result<Option<Vec<u8>>> {
        self.platform.get_block(block_id).await
    }

    pub async fn set_block(&mut self, block_id: BlockID, encrypted: &[u8]) -> anyhow::Result<()> {
        self.platform.set_block(block_id, encrypted).await
    }

    /// Only used for debugging tests
    #[allow(unused)]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        self.platform.debug_dump().await
    }
}

#[cfg(test)]
#[path = "../tests/unit/workspace.rs"]
mod test;
