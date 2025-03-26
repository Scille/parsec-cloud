// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::convert::Infallible;
use std::path::Path;

use indexed_db::{Database, Factory, ObjectStore, Transaction};
use libparsec_types::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

use super::utils::{
    js_to_rs_bytes, js_to_rs_string, js_to_rs_u32, js_to_rs_u64, js_to_rs_vlob_id,
    rs_to_js_timestamp, rs_to_js_u64, with_transaction, CustomErrMarker,
};
#[cfg(any(test, feature = "expose-test-methods"))]
use crate::workspace::{DebugBlock, DebugChunk, DebugDump, DebugVlob};
use crate::workspace::{
    MarkPreventSyncPatternFullyAppliedError, PopulateManifestOutcome, RawEncryptedBlock,
    RawEncryptedChunk, RawEncryptedManifest, UpdateManifestData,
};
use crate::PREVENT_SYNC_PATTERN_EMPTY_PATTERN;

pub(super) fn get_workspace_storage_db_name(
    data_base_dir: &Path,
    device_id: DeviceID,
    realm_id: VlobID,
) -> String {
    format!(
        "{}-{}-{}-workspace",
        data_base_dir.display(),
        device_id.hex(),
        realm_id.hex()
    )
}

// Note each database (certificates, workspace etc.) has its own version.
const DB_VERSION: u32 = 1;
// Prevent sync pattern store contains: {pattern: string, fully_applied: boolean}
const PREVENT_SYNC_PATTERN_STORE: &str = "prevent_sync_pattern";
// Prevent sync pattern is a singleton, so we use a single key.
const PREVENT_SYNC_PATTERN_SINGLETON_KEY: u32 = 1;
const PREVENT_SYNC_PATTERN_PATTERN_FIELD: &str = "pattern";
const PREVENT_SYNC_PATTERN_FULLY_APPLIED_FIELD: &str = "fully_applied";

// Checkpoint store contains a singleton with just a number
const CHECKPOINT_STORE: &str = "checkpoint";
const CHECKPOINT_SINGLETON_KEY: u32 = 1;

// Vlobs store contains:
// - key: Vlob ID (as Uint8Array)
// - value: {
//     base_version: number,
//     remote_version: number,
//     inbound_need_sync: number, // 0 is false, 1 is true
//     outbound_need_sync: number, // 0 is false, 1 is true
//     blob: Uint8Array
//   }
// Note the need to use a number to store boolean, given that IndexedDB
// doesn't support booleans in indexes ><''
const VLOBS_STORE: &str = "vlobs";
const VLOBS_INDEX_INBOUND_NEED_SYNC: &str = "_idx_inbound_need_sync";
const VLOBS_INDEX_OUTBOUND_NEED_SYNC: &str = "_idx_outbound_need_sync";
const VLOBS_INBOUND_NEED_SYNC_FIELD: &str = "inbound_need_sync";
const VLOBS_OUTBOUND_NEED_SYNC_FIELD: &str = "outbound_need_sync";
const VLOBS_BASE_VERSION_FIELD: &str = "base_version";
const VLOBS_REMOTE_VERSION_FIELD: &str = "remote_version";
const VLOBS_BLOB_FIELD: &str = "blob";

// Chunks store contains:
// - key: Chunk ID (as Uint8Array)
// - value: {size: number, data: Uint8Array}
const CHUNKS_STORE: &str = "chunks";
const CHUNKS_SIZE_FIELD: &str = "size";
const CHUNKS_DATA_FIELD: &str = "data";

// Blocks store contains:
// - key: Block ID (as Uint8Array)
// - value: {size: number, offline: bool, accessed_on: integer, data: Uint8Array}
const BLOCKS_STORE: &str = "blocks";
const BLOCKS_INDEX_ACCESSED_ON: &str = "_idx_accessed_on";
const BLOCKS_SIZE_FIELD: &str = "size";
const BLOCKS_OFFLINE_FIELD: &str = "offline";
const BLOCKS_ACCESSED_ON_FIELD: &str = "accessed_on";
const BLOCKS_DATA_FIELD: &str = "data";

async fn initialize_database(
    evt: &indexed_db::VersionChangeEvent<Infallible>,
) -> indexed_db::Result<(), Infallible> {
    // 1) Create the stores

    evt.build_object_store(PREVENT_SYNC_PATTERN_STORE)
        .create()?;
    evt.build_object_store(CHECKPOINT_STORE).create()?;

    let vlob_store = evt.build_object_store(VLOBS_STORE).create()?;
    vlob_store
        .build_index(VLOBS_INDEX_INBOUND_NEED_SYNC, VLOBS_INBOUND_NEED_SYNC_FIELD)
        .create()?;
    vlob_store
        .build_index(
            VLOBS_INDEX_OUTBOUND_NEED_SYNC,
            VLOBS_OUTBOUND_NEED_SYNC_FIELD,
        )
        .create()?;

    evt.build_object_store(CHUNKS_STORE).create()?;

    let block_store = evt.build_object_store(BLOCKS_STORE).create()?;
    block_store
        .build_index(BLOCKS_INDEX_ACCESSED_ON, BLOCKS_ACCESSED_ON_FIELD)
        .create()?;

    // 2) Insert the default prevent sync pattern

    let transaction = evt.transaction();

    let store = transaction.object_store(PREVENT_SYNC_PATTERN_STORE)?;

    let singleton = js_sys::Object::new();

    js_sys::Reflect::set(
        &singleton,
        &PREVENT_SYNC_PATTERN_PATTERN_FIELD.into(),
        &PREVENT_SYNC_PATTERN_EMPTY_PATTERN.into(),
    )
    .expect("target is an object");

    js_sys::Reflect::set(
        &singleton,
        &PREVENT_SYNC_PATTERN_FULLY_APPLIED_FIELD.into(),
        &false.into(),
    )
    .expect("target is an object");

    store
        .put_kv(&PREVENT_SYNC_PATTERN_SINGLETON_KEY.into(), &singleton)
        .await?;

    Ok(())
}

#[derive(Debug)]
pub struct PlatformWorkspaceStorage {
    conn: Database,
    max_blocks: u64,
}

// SAFETY: see `pretend_future_is_send_on_web`'s documentation for the full explanation.
// `PlatformWorkspaceStorage` contains `indexed_db::Database` which is `!Send` since it
// interfaces with the browser's IndexedDB API that is not thread-safe.
// Since are always mono-threaded in web, we can safely pretend to be `Send`, which is
// handy for our platform-agnostic code.
unsafe impl Send for PlatformWorkspaceStorage {}
// SAFETY: see `pretend_future_is_send_on_web`'s documentation for the full explanation.
unsafe impl Sync for PlatformWorkspaceStorage {}

impl Drop for PlatformWorkspaceStorage {
    fn drop(&mut self) {
        self.conn.close();
    }
}

impl PlatformWorkspaceStorage {
    pub async fn no_populate_start(
        #[cfg_attr(not(feature = "test-with-testbed"), allow(unused_variables))]
        data_base_dir: &Path,
        device: &LocalDevice,
        realm_id: VlobID,
        cache_max_blocks: u64,
    ) -> anyhow::Result<Self> {
        // 1) Open the database

        let name = get_workspace_storage_db_name(data_base_dir, device.device_id, realm_id);

        let factory = Factory::get().map_err(|e| anyhow::anyhow!("{e:?}"))?;
        let conn = factory
            .open(&name, DB_VERSION, async |evt| {
                // 2) Initialize the database (if needed)

                initialize_database(&evt).await
            })
            .await?;

        // 3) All done !

        Ok(Self {
            conn,
            max_blocks: cache_max_blocks,
        })
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        // Nothing to do here since the database is closed automatically on drop
        Ok(())
    }

    pub async fn get_realm_checkpoint(&mut self) -> anyhow::Result<IndexInt> {
        with_transaction!(
            &self.conn,
            &[CHECKPOINT_STORE],
            false,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(CHECKPOINT_STORE)?;

                let maybe_singleton = store.get(&CHECKPOINT_SINGLETON_KEY.into()).await?;

                let checkpoint_js = match maybe_singleton {
                    Some(checkpoint_js) => checkpoint_js,
                    None => return Ok(0),
                };

                let checkpoint = js_to_rs_u64(checkpoint_js)?;

                Ok(checkpoint)
            },
        )
        .await?
    }

    pub async fn update_realm_checkpoint(
        &mut self,
        new_checkpoint: IndexInt,
        changed_vlobs: &[(VlobID, VersionInt)],
    ) -> anyhow::Result<()> {
        with_transaction!(
            &self.conn,
            &[CHECKPOINT_STORE, VLOBS_STORE],
            true,
            async |transaction: Transaction<CustomErrMarker>| {
                // 1) Update checkpoint

                let store = transaction.object_store(CHECKPOINT_STORE)?;

                let new_checkpoint_js = rs_to_js_u64(new_checkpoint)?;
                store
                    .put_kv(&CHECKPOINT_SINGLETON_KEY.into(), &new_checkpoint_js)
                    .await?;

                // 2) Update vlobs

                let store = transaction
                    .object_store(VLOBS_STORE)
                    .map_err(anyhow::Error::from)?;

                for (vlob_id, new_remote_version) in changed_vlobs {
                    let vlob_id_js: JsValue = js_sys::Uint8Array::from(vlob_id.as_bytes()).into();
                    let cursor = store
                        .cursor()
                        .range(&vlob_id_js..=&vlob_id_js)?
                        .open()
                        .await?;

                    let obj = match cursor.value() {
                        Some(obj) => obj.dyn_into::<js_sys::Object>().map_err(|bad| {
                            anyhow::anyhow!("Invalid entry, expected Object, got {bad:?}")
                        })?,
                        None => continue,
                    };

                    let new_remote_version_js = JsValue::from(*new_remote_version);
                    js_sys::Reflect::set(
                        &obj,
                        &VLOBS_REMOTE_VERSION_FIELD.into(),
                        &new_remote_version_js,
                    )
                    .expect("target is an object");

                    let base_version_js =
                        js_sys::Reflect::get(&obj, &VLOBS_BASE_VERSION_FIELD.into()).map_err(
                            |e| anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}"),
                        )?;
                    let base_version = js_to_rs_u32(base_version_js)?;
                    let inbound_need_sync = base_version != *new_remote_version;

                    js_sys::Reflect::set(
                        &obj,
                        &VLOBS_INBOUND_NEED_SYNC_FIELD.into(),
                        // Don't store boolean, IndexedDb doesn't support them in indexes !
                        &(inbound_need_sync as u32).into(),
                    )
                    .expect("target is an object");

                    cursor.update(&obj).await.map_err(anyhow::Error::from)?;
                }

                Ok(())
            },
        )
        .await?
    }

    pub async fn get_outbound_need_sync(&mut self, limit: u32) -> anyhow::Result<Vec<VlobID>> {
        with_transaction!(
            &self.conn,
            &[VLOBS_STORE],
            false,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(VLOBS_STORE)?;

                let mut cursor = store
                    .index(VLOBS_INDEX_OUTBOUND_NEED_SYNC)?
                    .cursor()
                    .range(JsValue::from(1)..=JsValue::from(1))?
                    .open_key()
                    .await?;

                let mut vlobs = Vec::new();
                while let Some(vlob_id_js) = cursor.primary_key() {
                    if vlobs.len() >= limit as usize {
                        break;
                    }
                    let vlob_id = js_to_rs_vlob_id(vlob_id_js)?;
                    vlobs.push(vlob_id);
                    cursor.advance(1).await?;
                }

                Ok(vlobs)
            },
        )
        .await?
    }

    pub async fn get_inbound_need_sync(&mut self, limit: u32) -> anyhow::Result<Vec<VlobID>> {
        with_transaction!(
            &self.conn,
            &[VLOBS_STORE],
            false,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(VLOBS_STORE)?;

                let mut cursor = store
                    .index(VLOBS_INDEX_INBOUND_NEED_SYNC)?
                    .cursor()
                    .range(JsValue::from(1)..=JsValue::from(1))?
                    .open_key()
                    .await?;

                let mut vlobs = Vec::new();
                while let Some(vlob_id_js) = cursor.primary_key() {
                    if vlobs.len() >= limit as usize {
                        break;
                    }
                    let vlob_id = js_to_rs_vlob_id(vlob_id_js)?;
                    vlobs.push(vlob_id);
                    cursor.advance(1).await?;
                }

                Ok(vlobs)
            },
        )
        .await?
    }

    pub async fn get_manifest(
        &mut self,
        entry_id: VlobID,
    ) -> anyhow::Result<Option<RawEncryptedManifest>> {
        with_transaction!(
            &self.conn,
            &[VLOBS_STORE],
            false,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(VLOBS_STORE)?;

                let entry_id_js = js_sys::Uint8Array::from(entry_id.as_bytes());
                let maybe_obj = store.get(&entry_id_js.into()).await?;

                let obj_js = match maybe_obj {
                    Some(obj_js) => obj_js,
                    None => return Ok(None),
                };

                let blob_js = js_sys::Reflect::get(&obj_js, &VLOBS_BLOB_FIELD.into())
                    .map_err(|e| anyhow::anyhow!("Invalid entry, got {obj_js:?}: error {e:?}"))?;
                let blob = js_to_rs_bytes(blob_js)?;

                Ok(Some(blob))
            },
        )
        .await?
    }

    pub async fn list_manifests(
        &mut self,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Vec<RawEncryptedManifest>> {
        with_transaction!(
            &self.conn,
            &[VLOBS_STORE],
            false,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(VLOBS_STORE)?;

                let mut cursor = store.cursor().open().await?;

                if offset > 0 {
                    cursor.advance(offset).await?;
                }

                let mut items = Vec::with_capacity(limit as usize);
                while let Some(obj) = cursor.value() {
                    let blob_js = js_sys::Reflect::get(&obj, &VLOBS_BLOB_FIELD.into())
                        .map_err(|e| anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}"))?;
                    let blob = js_to_rs_bytes(blob_js)?;
                    items.push(blob);
                    if items.len() >= limit as usize {
                        break;
                    }

                    cursor.advance(1).await?;
                }

                Ok(items)
            },
        )
        .await?
    }

    async fn remove_chunk_internal(
        store: &ObjectStore<CustomErrMarker>,
        removed_chunk_id: ChunkID,
    ) -> anyhow::Result<()> {
        let removed_chunk_id_js = js_sys::Uint8Array::from(removed_chunk_id.as_bytes());

        store.delete(&removed_chunk_id_js).await?;

        Ok(())
    }

    async fn set_chunk_internal(
        store: &ObjectStore<CustomErrMarker>,
        chunk_id: ChunkID,
        encrypted: &[u8],
    ) -> anyhow::Result<()> {
        let obj = js_sys::Object::new();
        let data_js = js_sys::Uint8Array::from(encrypted);
        js_sys::Reflect::set(&obj, &CHUNKS_DATA_FIELD.into(), &data_js)
            .expect("target is an object");
        js_sys::Reflect::set(&obj, &CHUNKS_SIZE_FIELD.into(), &encrypted.len().into())
            .expect("target is an object");

        let chunk_id_js = js_sys::Uint8Array::from(chunk_id.as_bytes());

        store.put_kv(&chunk_id_js, &obj).await?;

        Ok(())
    }

    pub async fn set_chunk(&mut self, chunk_id: ChunkID, encrypted: &[u8]) -> anyhow::Result<()> {
        with_transaction!(
            &self.conn,
            &[CHUNKS_STORE],
            true,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(CHUNKS_STORE)?;

                Self::set_chunk_internal(&store, chunk_id, encrypted).await
            },
        )
        .await?
    }

    pub async fn get_chunk(
        &mut self,
        chunk_id: ChunkID,
    ) -> anyhow::Result<Option<RawEncryptedChunk>> {
        with_transaction!(
            &self.conn,
            &[CHUNKS_STORE],
            false,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(CHUNKS_STORE)?;

                let chunk_id_js = js_sys::Uint8Array::from(chunk_id.as_bytes());
                let maybe_obj = store.get(&chunk_id_js).await?;

                let obj = match maybe_obj {
                    Some(obj) => obj,
                    None => return Ok(None),
                };
                let data_js = js_sys::Reflect::get(&obj, &CHUNKS_DATA_FIELD.into())
                    .map_err(|e| anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}"))?;

                let data = data_js
                    .dyn_ref::<js_sys::Uint8Array>()
                    .ok_or_else(|| anyhow::anyhow!("Invalid data, got {data_js:?}"))?
                    .to_vec();

                Ok(Some(data))
            },
        )
        .await?
    }

    pub async fn set_block(
        &mut self,
        block_id: BlockID,
        encrypted: &[u8],
        accessed_on: DateTime,
    ) -> anyhow::Result<()> {
        with_transaction!(
            &self.conn,
            &[BLOCKS_STORE],
            true,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(BLOCKS_STORE)?;

                let obj = js_sys::Object::new();
                let data_js = js_sys::Uint8Array::from(encrypted);
                js_sys::Reflect::set(&obj, &BLOCKS_DATA_FIELD.into(), &data_js)
                    .expect("target is an object");
                js_sys::Reflect::set(&obj, &BLOCKS_SIZE_FIELD.into(), &encrypted.len().into())
                    .expect("target is an object");
                js_sys::Reflect::set(&obj, &BLOCKS_OFFLINE_FIELD.into(), &false.into())
                    .expect("target is an object");
                let accessed_on_js = rs_to_js_timestamp(accessed_on)?;
                js_sys::Reflect::set(&obj, &BLOCKS_ACCESSED_ON_FIELD.into(), &accessed_on_js)
                    .expect("target is an object");

                let block_id_js = js_sys::Uint8Array::from(block_id.as_bytes());

                store.put_kv(&block_id_js, &obj).await?;

                self.may_cleanup_blocks(&store).await?;

                Ok(())
            },
        )
        .await?
    }

    pub async fn get_block(
        &mut self,
        block_id: BlockID,
        timestamp: DateTime,
    ) -> anyhow::Result<Option<RawEncryptedBlock>> {
        with_transaction!(
            &self.conn,
            &[BLOCKS_STORE],
            true,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(BLOCKS_STORE)?;

                // 1) Get bock the block

                let block_id_js: JsValue = js_sys::Uint8Array::from(block_id.as_bytes()).into();

                let cursor = store
                    .cursor()
                    .range(&block_id_js..=&block_id_js)?
                    .open()
                    .await?;

                let obj = match cursor.value() {
                    Some(obj) => obj.dyn_into::<js_sys::Object>().map_err(|bad| {
                        anyhow::anyhow!("Invalid entry, expected Object, got {bad:?}")
                    })?,
                    None => return Ok(None),
                };

                let data_js = js_sys::Reflect::get(&obj, &BLOCKS_DATA_FIELD.into())
                    .map_err(|e| anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}"))?;

                let data = data_js
                    .dyn_ref::<js_sys::Uint8Array>()
                    .ok_or_else(|| anyhow::anyhow!("Invalid data, got {data_js:?}"))?
                    .to_vec();

                // 2) Update block's accessed_on field

                js_sys::Reflect::set(
                    &obj,
                    &BLOCKS_ACCESSED_ON_FIELD.into(),
                    &rs_to_js_timestamp(timestamp)?,
                )
                .expect("target is an object");
                cursor.update(&obj).await?;

                Ok(Some(data))
            },
        )
        .await?
    }

    pub async fn populate_manifest(
        &mut self,
        manifest: &UpdateManifestData,
    ) -> anyhow::Result<PopulateManifestOutcome> {
        with_transaction!(
            &self.conn,
            &[VLOBS_STORE],
            true,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(VLOBS_STORE)?;

                let entry_id_js = js_sys::Uint8Array::from(manifest.entry_id.as_bytes());

                let obj = js_sys::Object::new();

                let blob_js = js_sys::Uint8Array::from(manifest.encrypted.as_ref());
                js_sys::Reflect::set(&obj, &VLOBS_BLOB_FIELD.into(), &blob_js.into())
                    .expect("target is an object");

                js_sys::Reflect::set(
                    &obj,
                    &VLOBS_BASE_VERSION_FIELD.into(),
                    &manifest.base_version.into(),
                )
                .expect("target is an object");

                // Use base version as default for remote version
                js_sys::Reflect::set(
                    &obj,
                    &VLOBS_REMOTE_VERSION_FIELD.into(),
                    &manifest.base_version.into(),
                )
                .expect("target is an object");

                js_sys::Reflect::set(
                    &obj,
                    &VLOBS_OUTBOUND_NEED_SYNC_FIELD.into(),
                    // Don't store boolean, IndexedDb doesn't support them in indexes !
                    &(manifest.need_sync as u32).into(),
                )
                .expect("target is an object");
                // Don't store boolean, IndexedDb doesn't support them in indexes !
                let inbound_need_sync_js: JsValue = (false as u32).into();
                js_sys::Reflect::set(
                    &obj,
                    &VLOBS_INBOUND_NEED_SYNC_FIELD.into(),
                    &inbound_need_sync_js,
                )
                .expect("target is an object");

                match store.add_kv(&entry_id_js, &obj).await {
                    Ok(()) => Ok(PopulateManifestOutcome::Stored),
                    Err(indexed_db::Error::AlreadyExists) => {
                        Ok(PopulateManifestOutcome::AlreadyPresent)
                    }
                    Err(err) => Err(err.into()),
                }
            },
        )
        .await?
    }

    async fn update_manifest_internal(
        store: &ObjectStore<CustomErrMarker>,
        manifest: &UpdateManifestData,
    ) -> anyhow::Result<()> {
        let entry_id_js: JsValue = js_sys::Uint8Array::from(manifest.entry_id.as_bytes()).into();

        let cursor = store
            .cursor()
            .range(&entry_id_js..=&entry_id_js)?
            .open()
            .await?;

        let remote_version = match cursor.value() {
            Some(obj) => {
                let remote_version_js =
                    js_sys::Reflect::get(&obj, &VLOBS_REMOTE_VERSION_FIELD.into())
                        .map_err(|e| anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}"))?;
                let remote_version = js_to_rs_u32(remote_version_js)?;
                std::cmp::max(remote_version, manifest.base_version)
            }
            None => manifest.base_version,
        };
        let inbound_need_sync = remote_version != manifest.base_version;

        let obj = js_sys::Object::new();

        let blob_js = js_sys::Uint8Array::from(manifest.encrypted.as_ref());
        js_sys::Reflect::set(&obj, &VLOBS_BLOB_FIELD.into(), &blob_js.into())
            .expect("target is an object");

        js_sys::Reflect::set(
            &obj,
            &VLOBS_BASE_VERSION_FIELD.into(),
            &manifest.base_version.into(),
        )
        .expect("target is an object");

        // Use base version as default for remote version
        js_sys::Reflect::set(
            &obj,
            &VLOBS_REMOTE_VERSION_FIELD.into(),
            &remote_version.into(),
        )
        .expect("target is an object");

        js_sys::Reflect::set(
            &obj,
            &VLOBS_OUTBOUND_NEED_SYNC_FIELD.into(),
            // Don't store boolean, IndexedDb doesn't support them in indexes !
            &(manifest.need_sync as u32).into(),
        )
        .expect("target is an object");
        js_sys::Reflect::set(
            &obj,
            &VLOBS_INBOUND_NEED_SYNC_FIELD.into(),
            // Don't store boolean, IndexedDb doesn't support them in indexes !
            &(inbound_need_sync as u32).into(),
        )
        .expect("target is an object");

        store.put_kv(&entry_id_js, &obj).await?;

        Ok(())
    }

    pub async fn update_manifest(&mut self, manifest: &UpdateManifestData) -> anyhow::Result<()> {
        with_transaction!(&self.conn, &[VLOBS_STORE], true, async |transaction| {
            let store = transaction.object_store(VLOBS_STORE)?;
            Self::update_manifest_internal(&store, manifest).await
        })
        .await?
    }

    pub async fn update_manifests(
        &mut self,
        manifests: impl Iterator<Item = UpdateManifestData>,
    ) -> anyhow::Result<()> {
        with_transaction!(
            &self.conn,
            &[VLOBS_STORE],
            true,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(VLOBS_STORE)?;

                for manifest in manifests {
                    Self::update_manifest_internal(&store, &manifest).await?;
                }

                Ok(())
            },
        )
        .await?
    }

    pub async fn update_manifest_and_chunks(
        &mut self,
        manifest: &UpdateManifestData,
        new_chunks: impl Iterator<Item = (ChunkID, RawEncryptedChunk)>,
        removed_chunks: impl Iterator<Item = ChunkID>,
    ) -> anyhow::Result<()> {
        with_transaction!(
            &self.conn,
            &[VLOBS_STORE, CHUNKS_STORE],
            true,
            async |transaction: Transaction<CustomErrMarker>| {
                // 1) Update manifest

                let store = transaction.object_store(VLOBS_STORE)?;
                Self::update_manifest_internal(&store, &manifest).await?;

                // 2) Update chunks

                let store = transaction.object_store(CHUNKS_STORE)?;

                for (new_chunk_id, new_chunk_data) in new_chunks {
                    Self::set_chunk_internal(&store, new_chunk_id, &new_chunk_data).await?;
                }
                for removed_chunk_id in removed_chunks {
                    Self::remove_chunk_internal(&store, removed_chunk_id).await?;
                }

                Ok(())
            },
        )
        .await?
    }

    async fn may_cleanup_blocks(&self, store: &ObjectStore<CustomErrMarker>) -> anyhow::Result<()> {
        let nb_blocks = store.count().await? as u64;

        let extra_blocks = nb_blocks.saturating_sub(self.max_blocks);

        // Cleanup is needed
        if extra_blocks > 0 {
            // Remove the extra block plus 10% of the cache size, i.e 100 blocks
            let to_remove = extra_blocks + self.max_blocks / 10;

            for _ in 0..to_remove {
                let cursor = store
                    .index(BLOCKS_INDEX_ACCESSED_ON)?
                    .cursor()
                    .direction(indexed_db::CursorDirection::Next)
                    .open()
                    .await?;

                cursor.delete().await?;
            }
        }

        Ok(())
    }

    pub async fn promote_chunk_to_block(
        &mut self,
        chunk_id: ChunkID,
        now: DateTime,
    ) -> anyhow::Result<()> {
        with_transaction!(
            &self.conn,
            &[CHUNKS_STORE, BLOCKS_STORE],
            true,
            async |transaction: Transaction<CustomErrMarker>| {
                // 1) Retrieve the chunk

                let store = transaction.object_store(CHUNKS_STORE)?;

                let chunk_id_js: JsValue = js_sys::Uint8Array::from(chunk_id.as_bytes()).into();
                let cursor = store
                    .cursor()
                    .range(&chunk_id_js..=&chunk_id_js)?
                    .open()
                    .await?;

                let obj = match cursor.value() {
                    Some(obj) => obj,
                    // Nothing to promote, this should not occur under normal circumstances
                    None => return Ok(()),
                };

                let data_js = js_sys::Reflect::get(&obj, &CHUNKS_DATA_FIELD.into())
                    .map_err(|e| anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}"))?;
                let size = data_js
                    .dyn_ref::<js_sys::Uint8Array>()
                    .ok_or_else(|| anyhow::anyhow!("Invalid data, got {data_js:?}"))?
                    .length();

                // 2) Remove the chunk

                cursor.delete().await?;

                // 3) Insert the block

                let store = transaction.object_store(BLOCKS_STORE)?;

                let obj = js_sys::Object::new();
                js_sys::Reflect::set(&obj, &BLOCKS_DATA_FIELD.into(), &data_js)
                    .expect("target is an object");
                js_sys::Reflect::set(&obj, &BLOCKS_SIZE_FIELD.into(), &size.into())
                    .expect("target is an object");
                js_sys::Reflect::set(&obj, &BLOCKS_OFFLINE_FIELD.into(), &false.into())
                    .expect("target is an object");
                let now_js = rs_to_js_timestamp(now)?;
                js_sys::Reflect::set(&obj, &BLOCKS_ACCESSED_ON_FIELD.into(), &now_js)
                    .expect("target is an object");

                store.put_kv(&chunk_id_js, &obj).await?;

                self.may_cleanup_blocks(&store).await?;

                Ok(())
            },
        )
        .await?
    }

    pub async fn set_prevent_sync_pattern(
        &mut self,
        pattern: &PreventSyncPattern,
    ) -> anyhow::Result<bool> {
        let new_pattern_js: JsValue = js_sys::JsString::from(pattern.to_string()).into();

        with_transaction!(
            &self.conn,
            &[PREVENT_SYNC_PATTERN_STORE],
            true,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(PREVENT_SYNC_PATTERN_STORE)?;

                // 1) Get back the current prevent sync pattern

                let current_singleton = store
                    .get(&PREVENT_SYNC_PATTERN_SINGLETON_KEY.into())
                    .await?
                    .ok_or_else(|| anyhow::anyhow!("Prevent sync pattern is missing"))?;

                // Get pattern field

                let current_pattern_js = js_sys::Reflect::get(
                    &current_singleton,
                    &PREVENT_SYNC_PATTERN_PATTERN_FIELD.into(),
                )
                .map_err(|e| {
                    anyhow::anyhow!("Invalid entry, got {current_singleton:?}: error {e:?}")
                })?;

                if current_pattern_js == new_pattern_js {
                    // The pattern hasn't changed
                    let current_fully_applied_js = js_sys::Reflect::get(
                        &current_singleton,
                        &PREVENT_SYNC_PATTERN_FULLY_APPLIED_FIELD.into(),
                    )
                    .map_err(|e| {
                        anyhow::anyhow!("Invalid entry, got {current_singleton:?}: error {e:?}")
                    })?;
                    let current_fully_applied =
                        current_fully_applied_js.as_bool().ok_or_else(|| {
                            anyhow::anyhow!("Invalid boolean, got {current_fully_applied_js:?}")
                        })?;
                    return Ok(current_fully_applied);
                }

                // 2) The pattern differs, must update the database

                let new_singleton = js_sys::Object::new();

                js_sys::Reflect::set(
                    &new_singleton,
                    &PREVENT_SYNC_PATTERN_PATTERN_FIELD.into(),
                    &new_pattern_js,
                )
                .expect("target is an object");

                js_sys::Reflect::set(
                    &new_singleton,
                    &PREVENT_SYNC_PATTERN_FULLY_APPLIED_FIELD.into(),
                    &false.into(),
                )
                .expect("target is an object");

                store
                    .put_kv(&PREVENT_SYNC_PATTERN_SINGLETON_KEY.into(), &new_singleton)
                    .await?;

                Ok(false)
            },
        )
        .await?
    }

    pub async fn get_prevent_sync_pattern(&mut self) -> anyhow::Result<(PreventSyncPattern, bool)> {
        with_transaction!(
            &self.conn,
            &[PREVENT_SYNC_PATTERN_STORE],
            false,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(PREVENT_SYNC_PATTERN_STORE)?;

                let maybe_singleton = store
                    .get(&PREVENT_SYNC_PATTERN_SINGLETON_KEY.into())
                    .await?;

                let singleton = match maybe_singleton {
                    Some(singleton) => singleton,
                    None => return Err(anyhow::anyhow!("Prevent sync pattern is missing")),
                };

                // Get pattern field

                let pattern_js =
                    js_sys::Reflect::get(&singleton, &PREVENT_SYNC_PATTERN_PATTERN_FIELD.into())
                        .map_err(|e| {
                            anyhow::anyhow!("Invalid entry, got {singleton:?}: error {e:?}")
                        })?;

                let pattern = {
                    let raw = js_to_rs_string(pattern_js)?;
                    PreventSyncPattern::from_regex(&raw)?
                };

                // Get fully_applied field

                let fully_applied_js = js_sys::Reflect::get(
                    &singleton,
                    &PREVENT_SYNC_PATTERN_FULLY_APPLIED_FIELD.into(),
                )
                .map_err(|e| anyhow::anyhow!("Invalid entry, got {singleton:?}: error {e:?}"))?;
                let fully_applied = fully_applied_js
                    .as_bool()
                    .ok_or_else(|| anyhow::anyhow!("Invalid boolean, got {fully_applied_js:?}"))?;

                Ok((pattern, fully_applied))
            },
        )
        .await?
    }

    pub async fn mark_prevent_sync_pattern_fully_applied(
        &mut self,
        pattern: &PreventSyncPattern,
    ) -> Result<(), MarkPreventSyncPatternFullyAppliedError> {
        let expected_pattern_js: JsValue = js_sys::JsString::from(pattern.to_string()).into();

        with_transaction!(
            &self.conn,
            &[PREVENT_SYNC_PATTERN_STORE],
            true,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction
                    .object_store(PREVENT_SYNC_PATTERN_STORE)
                    .map_err(anyhow::Error::from)?;

                // 1) Get back the current prevent sync pattern

                let current_singleton = store
                    .get(&PREVENT_SYNC_PATTERN_SINGLETON_KEY.into())
                    .await
                    .map_err(anyhow::Error::from)?
                    .ok_or_else(|| anyhow::anyhow!("Prevent sync pattern is missing"))?;

                // Get pattern field

                let current_pattern_js = js_sys::Reflect::get(
                    &current_singleton,
                    &PREVENT_SYNC_PATTERN_PATTERN_FIELD.into(),
                )
                .map_err(|e| {
                    anyhow::anyhow!("Invalid entry, got {current_singleton:?}: error {e:?}")
                })?;

                if current_pattern_js != expected_pattern_js {
                    return Err(MarkPreventSyncPatternFullyAppliedError::PatternMismatch);
                }

                // 2) The pattern match, we can update the database

                let current_fully_applied_js = js_sys::Reflect::get(
                    &current_singleton,
                    &PREVENT_SYNC_PATTERN_FULLY_APPLIED_FIELD.into(),
                )
                .map_err(|e| {
                    anyhow::anyhow!("Invalid entry, got {current_singleton:?}: error {e:?}")
                })?;

                if current_fully_applied_js.as_bool().unwrap_or(false) {
                    return Ok(()); // Already marked as fully applied
                }

                js_sys::Reflect::set(
                    &current_singleton,
                    &PREVENT_SYNC_PATTERN_FULLY_APPLIED_FIELD.into(),
                    &true.into(),
                )
                .expect("target is an object");

                store
                    .put_kv(
                        &PREVENT_SYNC_PATTERN_SINGLETON_KEY.into(),
                        &current_singleton,
                    )
                    .await
                    .map_err(anyhow::Error::from)?;

                Ok(())
            },
        )
        .await?
    }

    /// Only used for debugging tests
    #[cfg(any(test, feature = "expose-test-methods"))]
    pub async fn debug_dump(&mut self) -> anyhow::Result<DebugDump> {
        use super::utils::{js_to_rs_block_id, js_to_rs_chunk_id, js_to_rs_timestamp};

        with_transaction!(
            &self.conn,
            &[CHECKPOINT_STORE, VLOBS_STORE, CHUNKS_STORE, BLOCKS_STORE],
            false,
            async |transaction: Transaction<CustomErrMarker>| {
                // 1) Get checkpoint

                let store = transaction.object_store(CHECKPOINT_STORE)?;
                let maybe_singleton = store.get(&CHECKPOINT_SINGLETON_KEY.into()).await?;
                let checkpoint = match maybe_singleton {
                    Some(checkpoint_js) => js_to_rs_u64(checkpoint_js)?,
                    None => 0,
                };

                // 2) Get vlobs

                let mut vlobs = vec![];
                let store = transaction.object_store(VLOBS_STORE)?;
                let mut cursor = store.cursor().open().await?;
                while let Some(obj) = cursor.value() {
                    vlobs.push(DebugVlob {
                        id: {
                            let id_js = cursor.primary_key().expect("value is present");
                            js_to_rs_vlob_id(id_js)?
                        },
                        need_sync: {
                            let need_sync_js =
                                js_sys::Reflect::get(&obj, &VLOBS_OUTBOUND_NEED_SYNC_FIELD.into())
                                    .map_err(|e| {
                                        anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}")
                                    })?;
                            need_sync_js
                                .as_f64()
                                .and_then(|raw| match raw as u32 {
                                    0 => Some(false),
                                    1 => Some(true),
                                    _ => None,
                                })
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "Invalid number-as-boolean, got {need_sync_js:?}"
                                    )
                                })?
                        },
                        base_version: {
                            let base_version_js =
                                js_sys::Reflect::get(&obj, &VLOBS_BASE_VERSION_FIELD.into())
                                    .map_err(|e| {
                                        anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}")
                                    })?;
                            base_version_js.as_f64().ok_or_else(|| {
                                anyhow::anyhow!("Invalid number, got {base_version_js:?}")
                            })? as VersionInt
                        },
                        remote_version: {
                            let remote_version_js =
                                js_sys::Reflect::get(&obj, &VLOBS_REMOTE_VERSION_FIELD.into())
                                    .map_err(|e| {
                                        anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}")
                                    })?;
                            remote_version_js.as_f64().ok_or_else(|| {
                                anyhow::anyhow!("Invalid number, got {remote_version_js:?}")
                            })? as VersionInt
                        },
                    });

                    cursor.advance(1).await?;
                }

                // 3) Get chunks

                let mut chunks = vec![];
                let store = transaction.object_store(CHUNKS_STORE)?;
                let mut cursor = store.cursor().open().await?;
                while let Some(obj) = cursor.value() {
                    chunks.push(DebugChunk {
                        id: {
                            let id_js = cursor.primary_key().expect("value is present");
                            js_to_rs_chunk_id(id_js)?
                        },
                        size: {
                            let size_js = js_sys::Reflect::get(&obj, &CHUNKS_SIZE_FIELD.into())
                                .map_err(|e| {
                                    anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}")
                                })?;
                            size_js
                                .as_f64()
                                .ok_or_else(|| anyhow::anyhow!("Invalid number, got {size_js:?}"))?
                                as u32
                        },
                        offline: false,
                    });

                    cursor.advance(1).await?;
                }

                // 4) Get blocks

                let mut blocks = vec![];
                let store = transaction.object_store(BLOCKS_STORE)?;
                let mut cursor = store.cursor().open().await?;
                while let Some(obj) = cursor.value() {
                    blocks.push(DebugBlock {
                        id: {
                            let id_js = cursor.primary_key().expect("value is present");
                            js_to_rs_block_id(id_js)?
                        },
                        size: {
                            let size_js = js_sys::Reflect::get(&obj, &BLOCKS_SIZE_FIELD.into())
                                .map_err(|e| {
                                    anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}")
                                })?;
                            size_js
                                .as_f64()
                                .ok_or_else(|| anyhow::anyhow!("Invalid number, got {size_js:?}"))?
                                as u32
                        },
                        offline: {
                            let offline_js =
                                js_sys::Reflect::get(&obj, &BLOCKS_OFFLINE_FIELD.into()).map_err(
                                    |e| anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}"),
                                )?;
                            offline_js.as_bool().ok_or_else(|| {
                                anyhow::anyhow!("Invalid boolean, got {offline_js:?}")
                            })?
                        },
                        accessed_on: {
                            let accessed_on_js =
                                js_sys::Reflect::get(&obj, &BLOCKS_ACCESSED_ON_FIELD.into())
                                    .map_err(|e| {
                                        anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}")
                                    })?;
                            let accessed_on = js_to_rs_timestamp(accessed_on_js)?;
                            accessed_on.to_rfc3339()
                        },
                    });

                    cursor.advance(1).await?;
                }

                Ok(DebugDump {
                    checkpoint,
                    vlobs,
                    chunks,
                    blocks,
                })
            },
        )
        .await?
    }
}
