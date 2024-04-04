// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use indexed_db_futures::{prelude::*, web_sys::DomException};
use js_sys::{wasm_bindgen::JsValue, Array};
use serde::{Deserialize, Serialize};

use libparsec_types::prelude::*;

use crate::PREVENT_SYNC_PATTERN_EMPTY_PATTERN;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub(super) struct CertificateFilter {
    pub certificate_type: String,
    pub filter1: Option<String>,
    pub filter2: Option<String>,
}

impl CertificateFilter {
    fn index(&self) -> &str {
        match (&self.filter1, &self.filter2) {
            (None, None) => Certificate::INDEX_CERTIFICATE_TYPE,
            (Some(_), None) => Certificate::INDEX_FILTER1,
            (None, Some(_)) => Certificate::INDEX_FILTER2,
            (Some(_), Some(_)) => Certificate::INDEX_FILTERS,
        }
    }

    fn to_js_array(&self) -> JsValue {
        let mut len = 1;
        if self.filter1.is_some() {
            len += 1;
        }
        if self.filter2.is_some() {
            len += 1;
        }

        let array = Array::new_with_length(len);

        array.set(0, JsValue::from(&self.certificate_type));
        let mut index = 1;
        if let Some(filter1) = &self.filter1 {
            array.set(index, JsValue::from(filter1));
            index += 1
        }
        if let Some(filter2) = &self.filter2 {
            array.set(index, JsValue::from(filter2));
        }

        array.into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Certificate {
    pub certificate_timestamp: Float,
    pub certificate: Bytes,
    pub certificate_type: String,
    pub filter1: Option<String>,
    pub filter2: Option<String>,
}

impl Certificate {
    const STORE: &'static str = "certificates";
    const INDEX_CERTIFICATE_TYPE: &'static str = "certificate_type";
    const INDEX_FILTER1: &'static str = "filter1";
    const INDEX_FILTER2: &'static str = "filter2";
    const INDEX_FILTERS: &'static str = "filters";

    fn create(evt: &IdbVersionChangeEvent) -> Result<(), DomException> {
        if !evt.db().object_store_names().any(|n| n == Self::STORE) {
            let store = evt.db().create_object_store_with_params(
                Self::STORE,
                IdbObjectStoreParameters::default().auto_increment(true),
            )?;
            store.create_index(
                Self::INDEX_CERTIFICATE_TYPE,
                &IdbKeyPath::str_sequence(&[Self::INDEX_CERTIFICATE_TYPE]),
            )?;
            store.create_index(
                Self::INDEX_FILTER1,
                &IdbKeyPath::str_sequence(&[Self::INDEX_CERTIFICATE_TYPE, Self::INDEX_FILTER1]),
            )?;
            store.create_index(
                Self::INDEX_FILTER2,
                &IdbKeyPath::str_sequence(&[Self::INDEX_CERTIFICATE_TYPE, Self::INDEX_FILTER2]),
            )?;
            store.create_index(
                Self::INDEX_FILTERS,
                &IdbKeyPath::str_sequence(&[
                    Self::INDEX_CERTIFICATE_TYPE,
                    Self::INDEX_FILTER1,
                    Self::INDEX_FILTER2,
                ]),
            )?;
        }

        Ok(())
    }

    pub(super) fn write(conn: &IdbDatabase) -> anyhow::Result<IdbTransaction> {
        super::db::write(conn, Self::STORE)
    }

    pub(super) async fn clear(tx: &IdbTransaction<'_>) -> anyhow::Result<()> {
        super::db::clear(tx, Self::STORE).await
    }

    pub(super) async fn get_values(
        conn: &IdbTransaction<'_>,
        filter: CertificateFilter,
    ) -> anyhow::Result<Vec<Self>> {
        super::db::get_values(conn, Self::STORE, filter.index(), filter.to_js_array()).await
    }

    pub(super) async fn insert(&self, tx: &IdbTransaction<'_>) -> anyhow::Result<()> {
        let value = serde_wasm_bindgen::to_value(self).map_err(|e| anyhow::anyhow!("{e:?}"))?;
        super::db::insert(tx, Self::STORE, value).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Chunk {
    pub chunk_id: Bytes,
    pub size: IndexInt,
    pub offline: bool,
    pub accessed_on: Option<Float>,
    pub data: Bytes,
    // IndexedDB cannot index on boolean, so we have to deal with
    pub is_block: u8,
}

impl Chunk {
    const STORE: &'static str = "chunks";
    const INDEX_CHUNK_ID: &'static str = "chunk_id";
    const INDEX_IS_BLOCK: &'static str = "is_block";

    fn create(evt: &IdbVersionChangeEvent) -> Result<(), DomException> {
        if !evt.db().object_store_names().any(|n| n == Self::STORE) {
            let store = evt.db().create_object_store_with_params(
                Self::STORE,
                IdbObjectStoreParameters::default().auto_increment(true),
            )?;
            store.create_index_with_params(
                Self::INDEX_CHUNK_ID,
                &IdbKeyPath::str(Self::INDEX_CHUNK_ID),
                IdbIndexParameters::new().unique(true),
            )?;
            store.create_index(Self::INDEX_IS_BLOCK, &IdbKeyPath::str(Self::INDEX_IS_BLOCK))?;
        }

        Ok(())
    }

    pub(super) fn read(conn: &IdbDatabase) -> anyhow::Result<IdbTransaction> {
        super::db::read(conn, Self::STORE)
    }

    pub(super) fn write(conn: &IdbDatabase) -> anyhow::Result<IdbTransaction> {
        super::db::write(conn, Self::STORE)
    }

    pub(super) async fn count_blocks(tx: &IdbTransaction<'_>) -> anyhow::Result<IndexInt> {
        super::db::count(tx, Self::STORE, Self::INDEX_IS_BLOCK, 1.into())
            .await
            .map(|x| x as IndexInt)
    }

    pub(super) async fn get(
        tx: &IdbTransaction<'_>,
        chunk_id: &Bytes,
    ) -> anyhow::Result<Option<Self>> {
        let chunk_id =
            serde_wasm_bindgen::to_value(&chunk_id).map_err(|e| anyhow::anyhow!("{e:?}"))?;
        super::db::get_values(tx, Self::STORE, Self::INDEX_CHUNK_ID, chunk_id)
            .await
            .map(|x| x.into_iter().nth(0))
    }

    pub(super) async fn get_chunks(tx: &IdbTransaction<'_>) -> anyhow::Result<Vec<Self>> {
        super::db::get_values(tx, Self::STORE, Self::INDEX_IS_BLOCK, 0.into()).await
    }

    pub(super) async fn get_blocks(tx: &IdbTransaction<'_>) -> anyhow::Result<Vec<Self>> {
        super::db::get_values(tx, Self::STORE, Self::INDEX_IS_BLOCK, 1.into()).await
    }

    pub(super) async fn insert(&self, tx: &IdbTransaction<'_>) -> anyhow::Result<()> {
        let value = serde_wasm_bindgen::to_value(self).map_err(|e| anyhow::anyhow!("{e:?}"))?;
        super::db::insert(tx, Self::STORE, value).await
    }

    pub(super) async fn remove(tx: &IdbTransaction<'_>, chunk_id: &Bytes) -> anyhow::Result<()> {
        let chunk_id =
            serde_wasm_bindgen::to_value(chunk_id).map_err(|e| anyhow::anyhow!("{e:?}"))?;
        super::db::remove(tx, Self::STORE, Self::INDEX_CHUNK_ID, chunk_id).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct PreventSyncPattern {
    pub pattern: String,
    pub fully_applied: bool,
}

impl PreventSyncPattern {
    const STORE: &'static str = "prevent_sync_pattern";

    fn create(evt: &IdbVersionChangeEvent) -> Result<(), DomException> {
        if !evt.db().object_store_names().any(|n| n == Self::STORE) {
            evt.db().create_object_store_with_params(
                Self::STORE,
                &IdbObjectStoreParameters::default(),
            )?;
        }

        Ok(())
    }

    pub(super) fn read(conn: &IdbDatabase) -> anyhow::Result<IdbTransaction> {
        super::db::read(conn, Self::STORE)
    }

    pub(super) fn write(conn: &IdbDatabase) -> anyhow::Result<IdbTransaction> {
        super::db::write(conn, Self::STORE)
    }

    pub(super) async fn get(conn: &IdbDatabase) -> anyhow::Result<Option<Self>> {
        let tx = Self::read(conn)?;
        super::db::get_value(&tx, Self::STORE, 0.into()).await
    }

    pub(super) async fn insert(&self, conn: &IdbDatabase) -> anyhow::Result<()> {
        let value = serde_wasm_bindgen::to_value(self).map_err(|e| anyhow::anyhow!("{e:?}"))?;

        let tx = Self::write(conn)?;
        super::db::insert_with_key(&tx, Self::STORE, 0.into(), value).await?;
        super::db::commit(tx).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct RealmCheckpoint {
    pub checkpoint: IndexInt,
}

impl RealmCheckpoint {
    const STORE: &'static str = "realm_checkpoint";

    fn create(evt: &IdbVersionChangeEvent) -> Result<(), DomException> {
        if !evt.db().object_store_names().any(|n| n == Self::STORE) {
            evt.db().create_object_store_with_params(
                Self::STORE,
                IdbObjectStoreParameters::default().auto_increment(true),
            )?;
        }

        Ok(())
    }

    pub(super) fn read(conn: &IdbDatabase) -> anyhow::Result<IdbTransaction> {
        super::db::read(conn, Self::STORE)
    }

    pub(super) fn write(conn: &IdbDatabase) -> anyhow::Result<IdbTransaction> {
        super::db::write(conn, Self::STORE)
    }

    pub(super) async fn get(tx: &IdbTransaction<'_>) -> anyhow::Result<Option<Self>> {
        super::db::get_value(&tx, Self::STORE, 0.into()).await
    }

    pub(super) async fn insert(&self, tx: &IdbTransaction<'_>) -> anyhow::Result<()> {
        let value = serde_wasm_bindgen::to_value(self).map_err(|e| anyhow::anyhow!("{e:?}"))?;
        super::db::insert_with_key(&tx, Self::STORE, 0.into(), value).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Remanence {
    pub block_remanent: bool,
}

impl Remanence {
    const STORE: &'static str = "remanence";

    fn create(evt: &IdbVersionChangeEvent) -> Result<(), DomException> {
        if !evt.db().object_store_names().any(|n| n == Self::STORE) {
            evt.db().create_object_store_with_params(
                Self::STORE,
                IdbObjectStoreParameters::default().auto_increment(true),
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Vlob {
    pub vlob_id: Bytes,
    pub base_version: VersionInt,
    pub remote_version: VersionInt,
    pub need_sync: bool,
    pub blob: Bytes,
}

impl Vlob {
    const STORE: &'static str = "vlobs";
    const INDEX_VLOB_ID: &'static str = "vlob_id";
    const INDEX_NEED_SYNC: &'static str = "need_sync";

    fn create(evt: &IdbVersionChangeEvent) -> Result<(), DomException> {
        if !evt.db().object_store_names().any(|n| n == Self::STORE) {
            let store = evt.db().create_object_store_with_params(
                Self::STORE,
                IdbObjectStoreParameters::default().auto_increment(true),
            )?;
            store.create_index_with_params(
                Self::INDEX_VLOB_ID,
                &IdbKeyPath::str(Self::INDEX_VLOB_ID),
                IdbIndexParameters::new().unique(true),
            )?;
            store.create_index(
                Self::INDEX_NEED_SYNC,
                &IdbKeyPath::str(Self::INDEX_NEED_SYNC),
            )?;
        }

        Ok(())
    }

    pub(super) fn read(conn: &IdbDatabase) -> anyhow::Result<IdbTransaction> {
        super::db::read(conn, Self::STORE)
    }

    pub(super) fn write(conn: &IdbDatabase) -> anyhow::Result<IdbTransaction> {
        super::db::write(conn, Self::STORE)
    }

    pub(super) async fn get(
        tx: &IdbTransaction<'_>,
        vlob_id: &Bytes,
    ) -> anyhow::Result<Option<Self>> {
        let vlob_id =
            serde_wasm_bindgen::to_value(&vlob_id).map_err(|e| anyhow::anyhow!("{e:?}"))?;
        super::db::get_values(&tx, Self::STORE, Self::INDEX_VLOB_ID, vlob_id)
            .await
            .map(|x| x.into_iter().nth(0))
    }

    pub(super) async fn get_need_sync(conn: &IdbDatabase) -> anyhow::Result<Vec<Self>> {
        let tx = Self::read(conn)?;
        super::db::get_values(&tx, Self::STORE, Self::INDEX_NEED_SYNC, true.into()).await
    }

    pub(super) async fn get_all(conn: &IdbDatabase) -> anyhow::Result<Vec<Self>> {
        let tx = Self::read(conn)?;
        super::db::get_all(&tx, Self::STORE).await
    }

    pub(super) async fn remove(tx: &IdbTransaction<'_>, vlob_id: &Bytes) -> anyhow::Result<()> {
        let vlob_id =
            serde_wasm_bindgen::to_value(vlob_id).map_err(|e| anyhow::anyhow!("{e:?}"))?;
        super::db::remove(&tx, Self::STORE, Self::INDEX_VLOB_ID, vlob_id).await
    }

    pub(super) async fn insert(&self, tx: &IdbTransaction<'_>) -> anyhow::Result<()> {
        let value = serde_wasm_bindgen::to_value(self).map_err(|e| anyhow::anyhow!("{e:?}"))?;
        super::db::insert(&tx, Self::STORE, value).await
    }
}

pub(super) async fn initialize_model_if_needed(
    mut db_req: OpenDbRequest,
) -> anyhow::Result<IdbDatabase> {
    db_req.set_on_upgrade_needed(Some(|evt: &IdbVersionChangeEvent| {
        Vlob::create(evt)?;
        RealmCheckpoint::create(evt)?;
        PreventSyncPattern::create(evt)?;
        Chunk::create(evt)?;
        Remanence::create(evt)?;
        Certificate::create(evt)?;

        Ok(())
    }));

    let conn = db_req.await.map_err(|e| anyhow::anyhow!("{e:?}"))?;

    if PreventSyncPattern::get(&conn).await?.is_none() {
        PreventSyncPattern {
            pattern: PREVENT_SYNC_PATTERN_EMPTY_PATTERN.into(),
            fully_applied: false,
        }
        .insert(&conn)
        .await?;
    }

    Ok(conn)
}
