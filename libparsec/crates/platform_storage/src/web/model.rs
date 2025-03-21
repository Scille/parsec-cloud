// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use indexed_db_futures::{prelude::*, web_sys::DomException};
use js_sys::{wasm_bindgen::JsValue, Array, Uint8Array};
use serde::{Deserialize, Serialize};
use std::path::Path;

use libparsec_types::prelude::*;

use crate::certificates::{FilterKind, GetCertificateQuery};
use crate::workspace::{RawEncryptedChunk, RawEncryptedManifest};
use crate::PREVENT_SYNC_PATTERN_EMPTY_PATTERN;

pub(super) fn get_certificates_storage_db_name(
    data_base_dir: &Path,
    device_id: DeviceID,
) -> String {
    format!(
        "{}-{}-certificates",
        data_base_dir.display(),
        device_id.hex()
    )
}

pub(super) fn get_user_storage_db_name(data_base_dir: &Path, device_id: DeviceID) -> String {
    format!("{}-{}-user", data_base_dir.display(), device_id.hex())
}

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

#[derive(Debug, Clone)]
pub(super) struct CertificateFilter<'a>(pub GetCertificateQuery<'a>);

impl CertificateFilter<'_> {
    fn index(&self) -> &str {
        match &self.0 {
            GetCertificateQuery::NoFilter { .. } => Certificate::INDEX_CERTIFICATE_TYPE,
            GetCertificateQuery::Filter1 { .. } => Certificate::INDEX_FILTER1,
            GetCertificateQuery::Filter2 { .. } => Certificate::INDEX_FILTER2,
            GetCertificateQuery::BothFilters { .. } => Certificate::INDEX_FILTERS,
            GetCertificateQuery::Filter1EqFilter2WhereFilter1 { .. }
            | GetCertificateQuery::Filter1EqFilter1WhereFilter2 { .. }
            | GetCertificateQuery::Filter2EqFilter1WhereFilter2 { .. }
            | GetCertificateQuery::Filter2EqFilter2WhereFilter1 { .. } => unreachable!(),
        }
    }

    fn to_js_array(&self) -> JsValue {
        match &self.0 {
            GetCertificateQuery::NoFilter { certificate_type } => {
                let array = Array::new_with_length(1);
                array.set(0, JsValue::from(*certificate_type));

                array.into()
            }
            GetCertificateQuery::Filter1 {
                certificate_type,
                filter1,
            } => {
                let array = Array::new_with_length(2);
                array.set(0, JsValue::from(*certificate_type));

                match filter1 {
                    FilterKind::Null => (),
                    FilterKind::Bytes(filter) => {
                        array.set(1, JsValue::from(Uint8Array::from(filter.as_ref())));
                    }
                    FilterKind::U64(filter) => {
                        array.set(1, JsValue::from(Uint8Array::from(filter.as_ref())));
                    }
                }

                array.into()
            }
            GetCertificateQuery::Filter2 {
                certificate_type,
                filter2,
            } => {
                let array = Array::new_with_length(2);
                array.set(0, JsValue::from(*certificate_type));

                match filter2 {
                    FilterKind::Null => (),
                    FilterKind::Bytes(filter) => {
                        array.set(1, JsValue::from(Uint8Array::from(filter.as_ref())));
                    }
                    FilterKind::U64(filter) => {
                        array.set(1, JsValue::from(Uint8Array::from(filter.as_ref())));
                    }
                }

                array.into()
            }
            GetCertificateQuery::BothFilters {
                certificate_type,
                filter1,
                filter2,
            } => {
                let array = Array::new_with_length(3);
                array.set(0, JsValue::from(*certificate_type));

                let mut index = 1;

                match filter1 {
                    FilterKind::Null => (),
                    FilterKind::Bytes(filter) => {
                        array.set(index, JsValue::from(Uint8Array::from(filter.as_ref())));
                        index += 1;
                    }
                    FilterKind::U64(filter) => {
                        array.set(index, JsValue::from(Uint8Array::from(filter.as_ref())));
                        index += 1;
                    }
                }

                match filter2 {
                    FilterKind::Null => (),
                    FilterKind::Bytes(filter) => {
                        array.set(index, JsValue::from(Uint8Array::from(filter.as_ref())));
                    }
                    FilterKind::U64(filter) => {
                        array.set(index, JsValue::from(Uint8Array::from(filter.as_ref())));
                    }
                }

                array.into()
            }
            GetCertificateQuery::Filter1EqFilter2WhereFilter1 { .. }
            | GetCertificateQuery::Filter1EqFilter1WhereFilter2 { .. }
            | GetCertificateQuery::Filter2EqFilter1WhereFilter2 { .. }
            | GetCertificateQuery::Filter2EqFilter2WhereFilter1 { .. } => unreachable!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Certificate {
    pub certificate_timestamp: i64,
    pub certificate: Bytes,
    pub certificate_type: String,
    pub filter1: Option<Bytes>,
    pub filter2: Option<Bytes>,
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
        filter: CertificateFilter<'_>,
    ) -> anyhow::Result<Vec<Self>> {
        match &filter.0 {
            GetCertificateQuery::NoFilter { .. }
            | GetCertificateQuery::Filter1 { .. }
            | GetCertificateQuery::Filter2 { .. }
            | GetCertificateQuery::BothFilters { .. } => {
                super::db::get_values(conn, Self::STORE, filter.index(), filter.to_js_array()).await
            }

            GetCertificateQuery::Filter1EqFilter2WhereFilter1 {
                certificate_type,
                subquery_certificate_type,
                filter1,
            } => {
                let subquery_filter = CertificateFilter(GetCertificateQuery::Filter1 {
                    certificate_type: subquery_certificate_type,
                    filter1: filter1.to_owned(),
                });
                let sub_query_certifs: Vec<Self> = super::db::get_values(
                    conn,
                    Self::STORE,
                    subquery_filter.index(),
                    subquery_filter.to_js_array(),
                )
                .await?;
                let sub_query_filter2_result = match sub_query_certifs.get(0) {
                    Some(certif) => match &certif.filter2 {
                        Some(filter2) => FilterKind::Bytes(&filter2),
                        None => return Ok(vec![]),
                    },
                    None => return Ok(vec![]),
                };

                let main_filter = CertificateFilter(GetCertificateQuery::Filter1 {
                    certificate_type,
                    filter1: sub_query_filter2_result,
                });
                super::db::get_values(
                    conn,
                    Self::STORE,
                    main_filter.index(),
                    main_filter.to_js_array(),
                )
                .await
            }
            GetCertificateQuery::Filter1EqFilter1WhereFilter2 {
                certificate_type,
                subquery_certificate_type,
                filter2,
            } => {
                let subquery_filter = CertificateFilter(GetCertificateQuery::Filter2 {
                    certificate_type: subquery_certificate_type,
                    filter2: filter2.to_owned(),
                });
                let sub_query_certifs: Vec<Self> = super::db::get_values(
                    conn,
                    Self::STORE,
                    subquery_filter.index(),
                    subquery_filter.to_js_array(),
                )
                .await?;
                let sub_query_filter1_result = match sub_query_certifs.get(0) {
                    Some(certif) => match &certif.filter1 {
                        Some(filter1) => FilterKind::Bytes(&filter1),
                        None => return Ok(vec![]),
                    },
                    None => return Ok(vec![]),
                };

                let main_filter = CertificateFilter(GetCertificateQuery::Filter1 {
                    certificate_type,
                    filter1: sub_query_filter1_result,
                });
                super::db::get_values(
                    conn,
                    Self::STORE,
                    main_filter.index(),
                    main_filter.to_js_array(),
                )
                .await
            }
            GetCertificateQuery::Filter2EqFilter1WhereFilter2 {
                certificate_type,
                subquery_certificate_type,
                filter2,
            } => {
                let subquery_filter = CertificateFilter(GetCertificateQuery::Filter2 {
                    certificate_type: subquery_certificate_type,
                    filter2: filter2.to_owned(),
                });
                let sub_query_certifs: Vec<Self> = super::db::get_values(
                    conn,
                    Self::STORE,
                    subquery_filter.index(),
                    subquery_filter.to_js_array(),
                )
                .await?;
                let sub_query_filter1_result = match sub_query_certifs.get(0) {
                    Some(certif) => match &certif.filter1 {
                        Some(filter1) => FilterKind::Bytes(&filter1),
                        None => return Ok(vec![]),
                    },
                    None => return Ok(vec![]),
                };

                let main_filter = CertificateFilter(GetCertificateQuery::Filter2 {
                    certificate_type,
                    filter2: sub_query_filter1_result,
                });
                super::db::get_values(
                    conn,
                    Self::STORE,
                    main_filter.index(),
                    main_filter.to_js_array(),
                )
                .await
            }
            GetCertificateQuery::Filter2EqFilter2WhereFilter1 {
                certificate_type,
                subquery_certificate_type,
                filter1,
            } => {
                let subquery_filter = CertificateFilter(GetCertificateQuery::Filter1 {
                    certificate_type: subquery_certificate_type,
                    filter1: filter1.to_owned(),
                });
                let sub_query_certifs: Vec<Self> = super::db::get_values(
                    conn,
                    Self::STORE,
                    subquery_filter.index(),
                    subquery_filter.to_js_array(),
                )
                .await?;
                let sub_query_filter2_result = match sub_query_certifs.get(0) {
                    Some(certif) => match &certif.filter2 {
                        Some(filter2) => FilterKind::Bytes(&filter2),
                        None => return Ok(vec![]),
                    },
                    None => return Ok(vec![]),
                };

                let main_filter = CertificateFilter(GetCertificateQuery::Filter2 {
                    certificate_type,
                    filter2: sub_query_filter2_result,
                });
                super::db::get_values(
                    conn,
                    Self::STORE,
                    main_filter.index(),
                    main_filter.to_js_array(),
                )
                .await
            }
        }
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
    pub accessed_on: Option<i64>,
    pub data: RawEncryptedChunk,
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
            let index_params = IdbIndexParameters::new();
            index_params.set_unique(true);
            store.create_index_with_params(
                Self::INDEX_CHUNK_ID,
                &IdbKeyPath::str(Self::INDEX_CHUNK_ID),
                &index_params,
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

    #[cfg(any(test, feature = "expose-test-methods"))]
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

    /// Retrieve the prevent sync pattern with `id = 0` from the database.
    pub(super) async fn get(tx: &IdbTransaction<'_>) -> anyhow::Result<Option<Self>> {
        super::db::get_value(&tx, Self::STORE, 0.into()).await
    }

    /// Insert the current sync pattern into the database.
    pub(super) async fn insert(&self, tx: &IdbTransaction<'_>) -> anyhow::Result<()> {
        let value = serde_wasm_bindgen::to_value(self).map_err(|e| anyhow::anyhow!("{e:?}"))?;

        super::db::insert_with_key(&tx, Self::STORE, 0.into(), value).await
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
    pub blob: RawEncryptedManifest,
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
            let index_params = IdbIndexParameters::new();
            index_params.set_unique(true);
            store.create_index_with_params(
                Self::INDEX_VLOB_ID,
                &IdbKeyPath::str(Self::INDEX_VLOB_ID),
                &index_params,
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

    pub(super) async fn list(
        conn: &IdbDatabase,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Vec<Self>> {
        let tx = Self::read(conn)?;
        super::db::list(&tx, Self::STORE, offset, limit).await
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

    {
        let tx = PreventSyncPattern::write(&conn)?;

        if PreventSyncPattern::get(&tx).await?.is_none() {
            PreventSyncPattern {
                pattern: PREVENT_SYNC_PATTERN_EMPTY_PATTERN.into(),
                fully_applied: false,
            }
            .insert(&tx)
            .await?;
        }
    }

    Ok(conn)
}
