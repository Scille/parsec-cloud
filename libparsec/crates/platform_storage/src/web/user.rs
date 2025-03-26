// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{convert::Infallible, path::Path};

use indexed_db::{Database, Factory, ObjectStore, Transaction};
use libparsec_types::prelude::*;
use wasm_bindgen::JsCast;

use super::utils::{js_to_rs_bytes, js_to_rs_u64, rs_to_js_u64, with_transaction, CustomErrMarker};

pub(super) fn get_user_storage_db_name(data_base_dir: &Path, device_id: DeviceID) -> String {
    format!("{}-{}-user", data_base_dir.display(), device_id.hex())
}

// Note each database (certificates, workspace etc.) has its own version.
const DB_VERSION: u32 = 1;
// User store contains a single item with format: {
//   checkpoint: number,
//   remote_version: number,
//   base_version: number,
//   need_sync: boolean,
//   blob: Uint8Array,
// }
const STORE: &str = "data";
const SINGLETON_KEY: u32 = 1;
const SINGLETON_CHECKPOINT_FIELD: &str = "checkpoint";
const SINGLETON_REMOTE_VERSION_FIELD: &str = "remote_version";
const SINGLETON_BASE_VERSION_FIELD: &str = "base_version";
const SINGLETON_NEED_SYNC_FIELD: &str = "need_sync";
const SINGLETON_BLOB_FIELD: &str = "blob";

async fn initialize_database(
    evt: &indexed_db::VersionChangeEvent<Infallible>,
) -> indexed_db::Result<(), Infallible> {
    evt.build_object_store(STORE).create()?;

    Ok(())
}

#[derive(Debug)]
pub struct PlatformUserStorage {
    conn: Database,
    #[cfg(any(test, feature = "expose-test-methods"))]
    realm_id: VlobID,
}

// SAFETY: see `pretend_future_is_send_on_web`'s documentation for the full explanation.
// `PlatformUserStorage` contains `indexed_db::Database` which is `!Send` since it
// interfaces with the browser's IndexedDB API that is not thread-safe.
// Since are always mono-threaded in web, we can safely pretend to be `Send`, which is
// handy for our platform-agnostic code.
unsafe impl Send for PlatformUserStorage {}
// SAFETY: see `pretend_future_is_send_on_web`'s documentation for the full explanation.
unsafe impl Sync for PlatformUserStorage {}

impl Drop for PlatformUserStorage {
    fn drop(&mut self) {
        self.conn.close();
    }
}

impl PlatformUserStorage {
    pub(crate) async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
    ) -> anyhow::Result<Self> {
        // 1) Open the database

        let name = get_user_storage_db_name(data_base_dir, device.device_id);

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
            #[cfg(any(test, feature = "expose-test-methods"))]
            realm_id: device.user_realm_id,
        })
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        // Nothing to do here since the database is closed automatically on drop
        Ok(())
    }

    pub async fn get_realm_checkpoint(&self) -> anyhow::Result<IndexInt> {
        with_transaction!(
            &self.conn,
            &[STORE],
            false,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(STORE)?;

                let singleton = match get_singleton_object(&store).await? {
                    Some(singleton) => singleton,
                    None => return Ok(0),
                };

                let checkpoint_or_undefined =
                    js_sys::Reflect::get(&singleton, &SINGLETON_CHECKPOINT_FIELD.into()).map_err(
                        |e| anyhow::anyhow!("Invalid entry, got {singleton:?}: error {e:?}"),
                    )?;

                if checkpoint_or_undefined.is_undefined() {
                    return Ok(0);
                } else {
                    js_to_rs_u64(checkpoint_or_undefined)
                        .with_context(|| format!("Invalid entry, got {singleton:?}"))
                }
            }
        )
        .await?
    }

    pub async fn update_realm_checkpoint(
        &self,
        new_checkpoint: IndexInt,
        remote_user_manifest_version: Option<VersionInt>,
    ) -> anyhow::Result<()> {
        with_transaction!(
            &self.conn,
            &[STORE],
            true,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(STORE)?;

                let singleton = get_singleton_object(&store)
                    .await?
                    .unwrap_or_else(|| js_sys::Object::new());

                // Update checkpoint field

                let current_checkpoint = {
                    js_sys::Reflect::get(&singleton, &SINGLETON_CHECKPOINT_FIELD.into())
                        .map_err(|e| anyhow::anyhow!("{e:?}"))?
                        .as_f64()
                        .map(|f| f as IndexInt)
                        .unwrap_or(0)
                };
                js_sys::Reflect::set(
                    &singleton,
                    &SINGLETON_CHECKPOINT_FIELD.into(),
                    &rs_to_js_u64(std::cmp::max(current_checkpoint, new_checkpoint))?,
                )
                .expect("target is an object");

                // Update remove version field

                if let Some(new_remote_version) = remote_user_manifest_version {
                    let current_remote_version = {
                        js_sys::Reflect::get(&singleton, &SINGLETON_REMOTE_VERSION_FIELD.into())
                            .map_err(|e| anyhow::anyhow!("{e:?}"))?
                            .as_f64()
                            .map(|f| f as VersionInt)
                            .unwrap_or(0)
                    };
                    js_sys::Reflect::set(
                        &singleton,
                        &SINGLETON_REMOTE_VERSION_FIELD.into(),
                        &std::cmp::max(current_remote_version, new_remote_version).into(),
                    )
                    .expect("target is an object");
                }

                // Save in database

                store.put_kv(&SINGLETON_KEY.into(), &singleton).await?;

                Ok(())
            }
        )
        .await?
    }

    pub async fn get_user_manifest(&self) -> anyhow::Result<Option<Vec<u8>>> {
        with_transaction!(
            &self.conn,
            &[STORE],
            false,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(STORE)?;

                let singleton = get_singleton_object(&store)
                    .await?
                    .unwrap_or_else(|| js_sys::Object::new());

                let blob_or_undefined =
                    js_sys::Reflect::get(&singleton, &SINGLETON_BLOB_FIELD.into()).map_err(
                        |e| anyhow::anyhow!("Invalid entry, got {singleton:?}: error {e:?}"),
                    )?;

                if blob_or_undefined.is_undefined() {
                    Ok(None)
                } else {
                    let blob = js_to_rs_bytes(blob_or_undefined)
                        .with_context(|| format!("Invalid entry, got {singleton:?}"))?;
                    Ok(Some(blob))
                }
            }
        )
        .await?
    }

    pub async fn update_user_manifest(
        &mut self,
        encrypted: &[u8],
        need_sync: bool,
        base_version: VersionInt,
    ) -> anyhow::Result<()> {
        with_transaction!(
            &self.conn,
            &[STORE],
            true,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction
                    .object_store(STORE)
                    .map_err(|e| anyhow::anyhow!("{e:?}"))?;

                let singleton = get_singleton_object(&store)
                    .await?
                    .unwrap_or_else(|| js_sys::Object::new());

                // Update base version fields

                js_sys::Reflect::set(
                    &singleton,
                    &SINGLETON_BASE_VERSION_FIELD.into(),
                    &base_version.into(),
                )
                .expect("target is an object");

                // Update remote version fields

                let current_remote_version = {
                    js_sys::Reflect::get(&singleton, &SINGLETON_REMOTE_VERSION_FIELD.into())
                        .map_err(|e| anyhow::anyhow!("{e:?}"))?
                        .as_f64()
                        .map(|f| f as VersionInt)
                        .unwrap_or(0)
                };
                js_sys::Reflect::set(
                    &singleton,
                    &SINGLETON_REMOTE_VERSION_FIELD.into(),
                    &std::cmp::max(current_remote_version, base_version).into(),
                )
                .expect("target is an object");

                // Update need sync field

                js_sys::Reflect::set(
                    &singleton,
                    &SINGLETON_NEED_SYNC_FIELD.into(),
                    &need_sync.into(),
                )
                .expect("target is an object");

                // Update blob field

                js_sys::Reflect::set(
                    &singleton,
                    &SINGLETON_BLOB_FIELD.into(),
                    &js_sys::Uint8Array::from(encrypted.as_ref()).into(),
                )
                .expect("target is an object");

                // Save in database

                store.put_kv(&SINGLETON_KEY.into(), &singleton).await?;

                Ok(())
            }
        )
        .await?
    }

    #[cfg(any(test, feature = "expose-test-methods"))]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        with_transaction!(
            &self.conn,
            &[STORE],
            false,
            async |transaction: Transaction<CustomErrMarker>| {
                let store = transaction.object_store(STORE)?;

                let maybe_singleton = get_singleton_object(&store).await?;

                // 1) Get checkpoint

                let checkpoint = match &maybe_singleton {
                    Some(singleton) => {
                        let checkpoint_or_undefined =
                            js_sys::Reflect::get(&singleton, &SINGLETON_CHECKPOINT_FIELD.into())
                                .map_err(|e| {
                                    anyhow::anyhow!("Invalid entry, got {singleton:?}: error {e:?}")
                                })?;

                        if checkpoint_or_undefined.is_undefined() {
                            0
                        } else {
                            js_to_rs_u64(checkpoint_or_undefined)
                                .with_context(|| format!("Invalid entry, got {singleton:?}"))?
                        }
                    }
                    None => 0,
                };

                // 2) Get manifest

                let vlobs = match &maybe_singleton {
                    Some(singleton) => {
                        let need_sync_js =
                            js_sys::Reflect::get(&singleton, &SINGLETON_NEED_SYNC_FIELD.into())
                                .map_err(|e| {
                                    anyhow::anyhow!("Invalid entry, got {singleton:?}: error {e:?}")
                                })?;
                        let need_sync = need_sync_js.as_bool().ok_or_else(|| {
                            anyhow::anyhow!("Invalid need_sync, got {need_sync_js:?}")
                        })?;

                        let base_version_js =
                            js_sys::Reflect::get(&singleton, &SINGLETON_BASE_VERSION_FIELD.into())
                                .map_err(|e| {
                                    anyhow::anyhow!("Invalid entry, got {singleton:?}: error {e:?}")
                                })?;
                        let base_version = js_to_rs_u64(base_version_js)?;

                        let remote_version_js = js_sys::Reflect::get(
                            &singleton,
                            &SINGLETON_REMOTE_VERSION_FIELD.into(),
                        )
                        .map_err(|e| {
                            anyhow::anyhow!("Invalid entry, got {singleton:?}: error {e:?}")
                        })?;
                        let remote_version = js_to_rs_u64(remote_version_js)?;

                        vec![(self.realm_id, need_sync, base_version, remote_version)]
                    }
                    None => vec![],
                };

                // 3) Format output

                let mut output = format!("checkpoint: {checkpoint}\nvlobs: [\n");
                for (vlob_id, need_sync, base_version, remote_version) in vlobs {
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

                Ok(output)
            },
        )
        .await?
    }
}

async fn get_singleton_object(
    store: &ObjectStore<CustomErrMarker>,
) -> anyhow::Result<Option<js_sys::Object>> {
    let res = store.get(&SINGLETON_KEY.into()).await?;

    match res {
        Some(any) => {
            let obj = any
                .dyn_into()
                .map_err(|bad| anyhow::anyhow!("Expected Object, got {:?}", bad))?;
            Ok(Some(obj))
        }
        None => Ok(None),
    }
}
