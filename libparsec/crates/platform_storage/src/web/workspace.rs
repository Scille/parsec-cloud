// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Certificate storage relies on the upper layer to do the actual certification
// validation work and takes care of handling concurrency issues.
// Hence no unique violation should occur under normal circumstances here.

use std::path::Path;

use libparsec_types::prelude::*;

use crate::workspace::UpdateManifestData;

#[derive(Debug)]
pub(crate) struct PlatformWorkspaceStorage {}

impl PlatformWorkspaceStorage {
    pub async fn no_populate_start(
        _data_base_dir: &Path,
        _device: &LocalDevice,
        _realm_id: VlobID,
        _cache_max_blocks: u64,
    ) -> anyhow::Result<Self> {
        todo!()
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn get_realm_checkpoint(&mut self) -> anyhow::Result<IndexInt> {
        todo!()
    }

    pub async fn update_realm_checkpoint(
        &mut self,
        _new_checkpoint: IndexInt,
        _changed_vlobs: &[(VlobID, VersionInt)],
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn get_inbound_need_sync(&mut self, _limit: u32) -> anyhow::Result<Vec<VlobID>> {
        todo!()
    }

    pub async fn get_outbound_need_sync(&mut self, _limit: u32) -> anyhow::Result<Vec<VlobID>> {
        todo!()
    }

    pub async fn get_manifest(&mut self, _entry_id: VlobID) -> anyhow::Result<Option<Vec<u8>>> {
        todo!()
    }

    pub async fn get_chunk(&mut self, _chunk_id: ChunkID) -> anyhow::Result<Option<Vec<u8>>> {
        todo!()
    }

    pub async fn get_block(
        &mut self,
        _block_id: BlockID,
        _now: DateTime,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        todo!()
    }

    pub async fn update_manifest(&mut self, _manifest: &UpdateManifestData) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn update_manifests(
        &mut self,
        _manifests: impl Iterator<Item = UpdateManifestData>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn update_manifest_and_chunks(
        &mut self,
        _manifest: &UpdateManifestData,
        _new_chunks: impl Iterator<Item = (ChunkID, Vec<u8>)>,
        _removed_chunks: impl Iterator<Item = ChunkID>,
        _chunks_promoted_to_block: impl Iterator<Item = (ChunkID, BlockID, DateTime)>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn set_chunk(&mut self, _chunk_id: ChunkID, _encrypted: &[u8]) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn set_block(
        &mut self,
        _block_id: BlockID,
        _encrypted: &[u8],
        _now: DateTime,
    ) -> anyhow::Result<()> {
        todo!()
    }

    /// Only used for debugging tests
    #[allow(unused)]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        todo!()
    }
}

pub async fn workspace_storage_non_speculative_init(
    _data_base_dir: &Path,
    _device: &LocalDevice,
    _realm_id: VlobID,
) -> anyhow::Result<()> {
    todo!()
}
