// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Certificate storage relies on the upper layer to do the actual certification
// validation work and takes care of handling concurrency issues.
// Hence no unique violation should occur under normal circumstances here.

use std::{path::Path, sync::Arc};

use indexed_db_futures::{prelude::IdbTransaction, IdbDatabase};
use libparsec_types::prelude::*;

use crate::{
    web::{
        model::{RealmCheckpoint, Vlob},
        DB_VERSION,
    },
    workspace::UpdateManifestData,
};

#[derive(Debug)]
pub(crate) struct PlatformWorkspaceStorage {
    conn: Arc<IdbDatabase>,
    cache_max_blocks: u64,
}

// Safety: PlatformWorkspaceStorage is read only
unsafe impl Send for PlatformWorkspaceStorage {}

impl PlatformWorkspaceStorage {
    pub async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
        realm_id: VlobID,
        cache_max_blocks: u64,
    ) -> anyhow::Result<Self> {
        // 1) Open the database

        #[cfg(feature = "test-with-testbed")]
        let name = format!(
            "{}-{}-{}-workspace",
            data_base_dir.to_str().unwrap(),
            device.slug(),
            realm_id.hex()
        );

        #[cfg(not(feature = "test-with-testbed"))]
        let name = format!("{}-{}-workspace", device.slug(), realm_id.hex());

        let db_req =
            IdbDatabase::open_u32(&name, DB_VERSION).map_err(|e| anyhow::anyhow!("{e:?}"))?;

        // 2) Initialize the database (if needed)

        let conn = Arc::new(super::model::initialize_model_if_needed(db_req).await?);

        // 3) All done !

        let storage = Self {
            conn,
            cache_max_blocks,
        };
        Ok(storage)
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        self.conn.close();
        Ok(())
    }

    pub async fn get_realm_checkpoint(&mut self) -> anyhow::Result<IndexInt> {
        let transaction = RealmCheckpoint::read(&self.conn)?;

        Ok(RealmCheckpoint::get(&transaction)
            .await?
            .map(|x| x.checkpoint)
            .unwrap_or_default())
    }

    pub async fn update_realm_checkpoint(
        &mut self,
        new_checkpoint: IndexInt,
        changed_vlobs: &[(VlobID, VersionInt)],
    ) -> anyhow::Result<()> {
        let transaction = RealmCheckpoint::write(&self.conn)?;

        RealmCheckpoint {
            checkpoint: new_checkpoint,
        }
        .insert(&transaction)
        .await?;

        super::db::commit(transaction).await?;

        let transaction = Vlob::write(&self.conn)?;

        for (vlob_id, remote_version) in changed_vlobs {
            let vlob_id = vlob_id.as_bytes().to_vec().into();
            if let Some(mut vlob) = Vlob::get(&transaction, &vlob_id).await? {
                Vlob::remove(&transaction, &vlob_id).await?;
                vlob.remote_version = *remote_version;
                vlob.insert(&transaction).await?;
            }
        }

        super::db::commit(transaction).await?;

        Ok(())
    }

    pub async fn get_inbound_need_sync(&mut self, limit: u32) -> anyhow::Result<Vec<VlobID>> {
        Vlob::get_need_sync(&self.conn)
            .await?
            .into_iter()
            .take(limit as usize)
            .map(|x| VlobID::try_from(x.vlob_id.as_ref()).map_err(|e| anyhow::anyhow!(e)))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn get_outbound_need_sync(&mut self, limit: u32) -> anyhow::Result<Vec<VlobID>> {
        Vlob::get_all(&self.conn)
            .await?
            .into_iter()
            .filter(|x| x.base_version != x.remote_version)
            .take(limit as usize)
            .map(|x| VlobID::try_from(x.vlob_id.as_ref()).map_err(|e| anyhow::anyhow!(e)))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn get_manifest(&mut self, entry_id: VlobID) -> anyhow::Result<Option<Vec<u8>>> {
        let transaction = Vlob::read(&self.conn)?;

        Ok(
            Vlob::get(&transaction, &entry_id.as_bytes().to_vec().into())
                .await?
                .map(|x| x.blob.to_vec()),
        )
    }

    pub async fn get_chunk(&mut self, chunk_id: ChunkID) -> anyhow::Result<Option<Vec<u8>>> {
        let transaction = super::model::Chunk::read(&self.conn)?;
        db_get_chunk(&transaction, chunk_id).await
    }

    pub async fn get_block(
        &mut self,
        block_id: BlockID,
        timestamp: DateTime,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        let transaction = super::model::Chunk::write(&self.conn)?;

        match super::model::Chunk::get(&transaction, &block_id.as_bytes().to_vec().into()).await? {
            Some(mut chunk) if chunk.is_block == 1 => {
                super::model::Chunk::remove(&transaction, &chunk.chunk_id).await?;
                chunk.accessed_on = Some(timestamp.get_f64_with_us_precision());
                let data = chunk.data.to_vec();
                chunk.insert(&transaction).await?;

                Ok(Some(data))
            }
            _ => Ok(None),
        }
    }

    pub async fn update_manifest(&mut self, manifest: &UpdateManifestData) -> anyhow::Result<()> {
        let transaction = Vlob::write(&self.conn)?;
        db_update_manifest(&transaction, manifest).await
    }

    pub async fn update_manifests(
        &mut self,
        manifests: impl Iterator<Item = UpdateManifestData>,
    ) -> anyhow::Result<()> {
        let transaction = Vlob::write(&self.conn)?;

        for manifest in manifests {
            db_update_manifest(&transaction, &manifest).await?;
        }

        super::db::commit(transaction).await
    }

    pub async fn update_manifest_and_chunks(
        &mut self,
        manifest: &UpdateManifestData,
        new_chunks: impl Iterator<Item = (ChunkID, Vec<u8>)>,
        removed_chunks: impl Iterator<Item = ChunkID>,
    ) -> anyhow::Result<()> {
        let transaction = Vlob::write(&self.conn)?;

        db_update_manifest(&transaction, manifest).await?;

        super::db::commit(transaction).await?;

        let transaction = super::model::Chunk::write(&self.conn)?;

        for (new_chunk_id, new_chunk_data) in new_chunks {
            super::model::Chunk {
                chunk_id: new_chunk_id.as_bytes().to_vec().into(),
                size: new_chunk_data.len() as IndexInt,
                offline: false,
                accessed_on: None,
                data: new_chunk_data.into(),
                is_block: 0,
            }
            .insert(&transaction)
            .await?;
        }

        for removed_chunk_id in removed_chunks {
            db_remove_chunk(&transaction, removed_chunk_id).await?;
        }

        super::db::commit(transaction).await
    }

    pub async fn set_chunk(&mut self, chunk_id: ChunkID, encrypted: &[u8]) -> anyhow::Result<()> {
        let chunk_id = chunk_id.as_bytes().to_vec().into();

        let transaction = super::model::Chunk::write(&self.conn)?;

        if super::model::Chunk::get(&transaction, &chunk_id)
            .await?
            .is_some()
        {
            super::model::Chunk::remove(&transaction, &chunk_id).await?;
        }

        super::model::Chunk {
            chunk_id,
            size: encrypted.len() as IndexInt,
            offline: false,
            accessed_on: None,
            data: encrypted.to_vec().into(),
            is_block: 0,
        }
        .insert(&transaction)
        .await
    }

    pub async fn set_block(
        &mut self,
        block_id: BlockID,
        encrypted: &[u8],
        accessed_on: DateTime,
    ) -> anyhow::Result<()> {
        let transaction = super::model::Chunk::write(&self.conn)?;

        db_insert_block(&transaction, block_id, encrypted, accessed_on).await?;

        let nb_blocks = super::model::Chunk::count_blocks(&transaction).await?;

        let extra_blocks = nb_blocks.saturating_sub(self.cache_max_blocks);

        // Cleanup is needed
        if extra_blocks > 0 {
            // Remove the extra block plus 10% of the cache size, i.e 100 blocks
            let to_remove = extra_blocks + self.cache_max_blocks / 10;

            let mut blocks = super::model::Chunk::get_blocks(&transaction).await?;

            blocks.sort_by(|x, y| {
                x.accessed_on
                    .partial_cmp(&y.accessed_on)
                    .expect("Timestamp should not be undefined")
            });

            for block_id in blocks
                .into_iter()
                .take(to_remove as usize)
                .map(|x| x.chunk_id)
            {
                super::model::Chunk::remove(&transaction, &block_id).await?;
            }
        }

        super::db::commit(transaction).await
    }

    pub async fn promote_chunk_to_block(
        &mut self,
        chunk_id: ChunkID,
        now: DateTime,
    ) -> anyhow::Result<()> {
        let transaction = super::model::Chunk::read(&self.conn)?;

        let encrypted = match db_get_chunk(&transaction, chunk_id).await? {
            Some(encrypted) => encrypted,
            // Nothing to promote, this should not occur under normal circumstances
            None => return Ok(()),
        };

        let transaction = super::model::Chunk::write(&self.conn)?;

        db_remove_chunk(&transaction, chunk_id).await?;
        db_insert_block(&transaction, chunk_id.into(), &encrypted, now).await?;

        super::db::commit(transaction).await
    }

    /// Only used for debugging tests
    #[allow(unused)]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        let checkpoint = self.get_realm_checkpoint().await?;
        let mut output = format!("checkpoint: {checkpoint}\n");

        // Vlobs

        let vlobs = Vlob::get_all(&self.conn).await?;

        output += "vlobs: [\n";
        for vlob in vlobs {
            let vlob_id =
                VlobID::try_from(vlob.vlob_id.as_ref()).map_err(|e| anyhow::anyhow!(e))?;
            let need_sync = vlob.need_sync;
            let base_version = vlob.base_version;
            let remote_version = vlob.remote_version;
            output += &format!(
                "{{\n\
                \tvlob_id: {vlob_id}\n\
                \tneed_sync: {need_sync}\n\
                \tbase_version: {base_version}\n\
                \tremote_version: {remote_version}\n\
            }},\n",
            );
        }
        output += "]\n";

        // Chunks

        let transaction = super::model::Chunk::read(&self.conn)?;

        let chunks = super::model::Chunk::get_chunks(&transaction).await?;

        output += "chunks: [\n";
        for chunk in chunks {
            let chunk_id =
                ChunkID::try_from(chunk.chunk_id.as_ref()).map_err(|e| anyhow::anyhow!(e))?;
            let size = chunk.size;
            let offline = chunk.offline;
            output += &format!(
                "{{\n\
                \tchunk_id: {chunk_id}\n\
                \tsize: {size}\n\
                \toffline: {offline}\n\
            }},\n",
            );
        }
        output += "]\n";

        // Blocks

        let mut blocks = super::model::Chunk::get_blocks(&transaction).await?;
        blocks.sort_by(|x, y| x.chunk_id.cmp(&y.chunk_id));

        output += "blocks: [\n";
        for block in blocks {
            let block_id =
                BlockID::try_from(block.chunk_id.as_ref()).map_err(|e| anyhow::anyhow!(e))?;
            let size = block.size;
            let offline = block.offline;
            let accessed_on = DateTime::from_f64_with_us_precision(
                block
                    .accessed_on
                    .ok_or(anyhow::anyhow!("Missing accessed_on field"))?,
            )
            .to_rfc3339();
            output += &format!(
                "{{\n\
                \tblock_id: {block_id}\n\
                \tsize: {size}\n\
                \toffline: {offline}\n\
                \taccessed_on: {accessed_on}\n\
            }},\n",
            );
        }
        output += "]\n";

        Ok(output)
    }
}

pub async fn workspace_storage_non_speculative_init(
    data_base_dir: &Path,
    device: &LocalDevice,
    realm_id: VlobID,
) -> anyhow::Result<()> {
    // 1) Open & initialize the database

    let mut storage =
        PlatformWorkspaceStorage::no_populate_start(data_base_dir, device, realm_id, u64::MAX)
            .await
            .map_err(|err| err.context("cannot initialize database"))?;

    // 2) Populate the database with the workspace manifest

    let timestamp = device.now();
    let manifest =
        LocalWorkspaceManifest::new(device.device_id.clone(), timestamp, Some(realm_id), false);

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

async fn db_update_manifest<'a>(
    tx: &IdbTransaction<'a>,
    manifest: &UpdateManifestData,
) -> anyhow::Result<()> {
    match Vlob::get(tx, &manifest.entry_id.as_bytes().to_vec().into()).await? {
        Some(old_vlob) => {
            Vlob::remove(tx, &old_vlob.vlob_id).await?;
            Vlob {
                vlob_id: old_vlob.vlob_id.clone(),
                blob: manifest.encrypted.to_vec().into(),
                need_sync: manifest.need_sync,
                base_version: manifest.base_version,
                remote_version: std::cmp::max(manifest.base_version, old_vlob.remote_version),
            }
            .insert(tx)
            .await
        }
        None => {
            Vlob {
                vlob_id: manifest.entry_id.as_bytes().to_vec().into(),
                blob: manifest.encrypted.to_vec().into(),
                need_sync: manifest.need_sync,
                base_version: manifest.base_version,
                remote_version: manifest.base_version,
            }
            .insert(tx)
            .await
        }
    }
}

async fn db_get_chunk(
    tx: &IdbTransaction<'_>,
    chunk_id: ChunkID,
) -> anyhow::Result<Option<Vec<u8>>> {
    match super::model::Chunk::get(tx, &chunk_id.as_bytes().to_vec().into()).await? {
        Some(chunk) if chunk.is_block == 0 => Ok(Some(chunk.data.to_vec())),
        _ => Ok(None),
    }
}

async fn db_insert_block(
    tx: &IdbTransaction<'_>,
    block_id: BlockID,
    encrypted: &[u8],
    accessed_on: DateTime,
) -> anyhow::Result<()> {
    super::model::Chunk {
        chunk_id: block_id.as_bytes().to_vec().into(),
        size: encrypted.len() as IndexInt,
        offline: false,
        accessed_on: Some(accessed_on.get_f64_with_us_precision()),
        data: encrypted.to_vec().into(),
        is_block: 1,
    }
    .insert(tx)
    .await
}

async fn db_remove_chunk(tx: &IdbTransaction<'_>, chunk_id: ChunkID) -> anyhow::Result<()> {
    super::model::Chunk::remove(tx, &chunk_id.as_bytes().to_vec().into()).await
}
