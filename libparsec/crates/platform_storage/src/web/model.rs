// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use indexed_db_futures::{
    database::{Database, VersionChangeEvent},
    factory::OpenDbRequestBuilder,
    transaction::Transaction,
    Build, KeyPath, KeyPathSeq,
};
use js_sys::Array;
use js_sys::{wasm_bindgen::JsValue, Uint8Array};
use serde::{Deserialize, Serialize};

use libparsec_types::prelude::*;

use crate::certificates::{FilterKind, GetCertificateQuery};
use crate::workspace::{RawEncryptedChunk, RawEncryptedManifest};
use crate::PREVENT_SYNC_PATTERN_EMPTY_PATTERN;

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

    fn to_query(&self) -> Array {
        match &self.0 {
            GetCertificateQuery::NoFilter { certificate_type } => {
                let array = Array::new_with_length(1);
                array.set(0, JsValue::from(*certificate_type));
                array
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
                        array.set(1, Uint8Array::from(filter.as_ref()).into())
                    }
                    FilterKind::U64(filter) => {
                        array.set(1, Uint8Array::from(filter.as_ref()).into())
                    }
                }
                array
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
                        array.set(1, Uint8Array::from(filter.as_ref()).into())
                    }
                    FilterKind::U64(filter) => {
                        array.set(1, Uint8Array::from(filter.as_ref()).into())
                    }
                }
                array
            }
            GetCertificateQuery::BothFilters {
                certificate_type,
                filter1,
                filter2,
            } => {
                let array = Array::new_with_length(3);
                let mut index = 1;
                array.set(0, JsValue::from(*certificate_type));
                match filter1 {
                    FilterKind::Null => (),
                    FilterKind::Bytes(filter) => {
                        array.set(index, Uint8Array::from(filter.as_ref()).into());
                        index += 1;
                    }
                    FilterKind::U64(filter) => {
                        array.set(index, Uint8Array::from(filter.as_ref()).into());
                        index += 1;
                    }
                }
                match filter2 {
                    FilterKind::Null => (),
                    FilterKind::Bytes(filter) => {
                        array.set(index, Uint8Array::from(filter.as_ref()).into())
                    }
                    FilterKind::U64(filter) => {
                        array.set(index, Uint8Array::from(filter.as_ref()).into())
                    }
                }
                array
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

    fn create(
        _evt: &VersionChangeEvent,
        db: &Database,
    ) -> Result<(), indexed_db_futures::error::Error> {
        if !db.object_store_names().any(|n| n == Self::STORE) {
            let store = db
                .create_object_store(Self::STORE)
                .with_auto_increment(true)
                .build()?;
            store
                .create_index(
                    Self::INDEX_CERTIFICATE_TYPE,
                    KeyPath::Sequence(KeyPathSeq::from_slice(&[Self::INDEX_CERTIFICATE_TYPE])),
                )
                .build()?;
            store
                .create_index(
                    Self::INDEX_FILTER1,
                    KeyPath::Sequence(KeyPathSeq::from_slice(&[
                        Self::INDEX_CERTIFICATE_TYPE,
                        Self::INDEX_FILTER1,
                    ])),
                )
                .build()?;
            store
                .create_index(
                    Self::INDEX_FILTER2,
                    KeyPath::Sequence(KeyPathSeq::from_slice(&[
                        Self::INDEX_CERTIFICATE_TYPE,
                        Self::INDEX_FILTER2,
                    ])),
                )
                .build()?;
            store
                .create_index(
                    Self::INDEX_FILTERS,
                    KeyPath::Sequence(KeyPathSeq::from_slice(&[
                        Self::INDEX_CERTIFICATE_TYPE,
                        Self::INDEX_FILTER1,
                        Self::INDEX_FILTER2,
                    ])),
                )
                .build()?;
        }

        Ok(())
    }

    pub(super) fn write(conn: &Database) -> anyhow::Result<Transaction> {
        log::debug!("Create write transaction for certificates");
        super::db::write(conn, Self::STORE)
    }

    pub(super) async fn clear(tx: &Transaction<'_>) -> anyhow::Result<()> {
        log::debug!("Clear the certificate store");
        super::db::clear(tx, Self::STORE).await
    }

    pub(super) async fn get_values(
        conn: &Transaction<'_>,
        filter: CertificateFilter<'_>,
    ) -> anyhow::Result<Vec<Self>> {
        match &filter.0 {
            GetCertificateQuery::NoFilter { .. }
            | GetCertificateQuery::Filter1 { .. }
            | GetCertificateQuery::Filter2 { .. }
            | GetCertificateQuery::BothFilters { .. } => {
                super::db::get_values(conn, Self::STORE, filter.index(), filter.to_query()).await
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
                    subquery_filter.to_query(),
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
                    main_filter.to_query(),
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
                    subquery_filter.to_query(),
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
                    main_filter.to_query(),
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
                    subquery_filter.to_query(),
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
                    main_filter.to_query(),
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
                    subquery_filter.to_query(),
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
                    main_filter.to_query(),
                )
                .await
            }
        }
    }

    pub(super) async fn insert(&self, tx: &Transaction<'_>) -> anyhow::Result<()> {
        super::db::insert(tx, Self::STORE, self).await
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

    fn create(
        _evt: &VersionChangeEvent,
        db: &Database,
    ) -> Result<(), indexed_db_futures::error::Error> {
        if !db.object_store_names().any(|n| n == Self::STORE) {
            let store = db
                .create_object_store(Self::STORE)
                .with_auto_increment(true)
                .build()?;
            store
                .create_index(Self::INDEX_CHUNK_ID, KeyPath::One(Self::INDEX_CHUNK_ID))
                .with_unique(true)
                .build()?;
            store
                .create_index(Self::INDEX_IS_BLOCK, KeyPath::One(Self::INDEX_IS_BLOCK))
                .build()?;
        }

        Ok(())
    }

    pub(super) fn read(conn: &Database) -> anyhow::Result<Transaction> {
        super::db::read(conn, Self::STORE)
    }

    pub(super) fn write(conn: &Database) -> anyhow::Result<Transaction> {
        super::db::write(conn, Self::STORE)
    }

    pub(super) async fn count_blocks(tx: &Transaction<'_>) -> anyhow::Result<IndexInt> {
        super::db::count(tx, Self::STORE, Self::INDEX_IS_BLOCK, 1_u32)
            .await
            .map(|x| x as IndexInt)
    }

    pub(super) async fn get(
        tx: &Transaction<'_>,
        chunk_id: &Bytes,
    ) -> anyhow::Result<Option<Self>> {
        super::db::get_values(
            tx,
            Self::STORE,
            Self::INDEX_CHUNK_ID,
            Uint8Array::from(chunk_id.as_ref()),
        )
        .await
        .map(|x| x.into_iter().nth(0))
    }

    #[cfg(any(test, feature = "expose-test-methods"))]
    pub(super) async fn get_chunks(tx: &Transaction<'_>) -> anyhow::Result<Vec<Self>> {
        super::db::get_values(tx, Self::STORE, Self::INDEX_IS_BLOCK, 0_u32).await
    }

    pub(super) async fn get_blocks(tx: &Transaction<'_>) -> anyhow::Result<Vec<Self>> {
        super::db::get_values(tx, Self::STORE, Self::INDEX_IS_BLOCK, 1_u32).await
    }

    pub(super) async fn insert(&self, tx: &Transaction<'_>) -> anyhow::Result<()> {
        super::db::insert(tx, Self::STORE, self).await
    }

    pub(super) async fn remove(tx: &Transaction<'_>, chunk_id: &Bytes) -> anyhow::Result<()> {
        super::db::remove(tx, Self::STORE, Self::INDEX_CHUNK_ID, chunk_id.as_ref()).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct PreventSyncPattern {
    pub pattern: String,
    pub fully_applied: bool,
}

impl PreventSyncPattern {
    const STORE: &'static str = "prevent_sync_pattern";

    fn create(
        _evt: &VersionChangeEvent,
        db: &Database,
    ) -> Result<(), indexed_db_futures::error::Error> {
        if !db.object_store_names().any(|n| n == Self::STORE) {
            db.create_object_store(Self::STORE).build()?;
        }

        Ok(())
    }

    pub(super) fn read(conn: &Database) -> anyhow::Result<Transaction> {
        super::db::read(conn, Self::STORE)
    }

    pub(super) fn write(conn: &Database) -> anyhow::Result<Transaction> {
        super::db::write(conn, Self::STORE)
    }

    /// Retrieve the prevent sync pattern with `id = 0` from the database.
    pub(super) async fn get(tx: &Transaction<'_>) -> anyhow::Result<Option<Self>> {
        super::db::get_value(&tx, Self::STORE, 0_u32).await
    }

    /// Insert the current sync pattern into the database.
    pub(super) async fn insert(&self, tx: &Transaction<'_>) -> anyhow::Result<()> {
        super::db::insert_with_key(&tx, Self::STORE, 0_u32, self).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct RealmCheckpoint {
    pub checkpoint: IndexInt,
}

impl RealmCheckpoint {
    const STORE: &'static str = "realm_checkpoint";

    fn create(
        _evt: &VersionChangeEvent,
        db: &Database,
    ) -> Result<(), indexed_db_futures::error::Error> {
        if !db.object_store_names().any(|n| n == Self::STORE) {
            db.create_object_store(Self::STORE)
                .with_auto_increment(true)
                .build()?;
        }

        Ok(())
    }

    pub(super) fn read(conn: &Database) -> anyhow::Result<Transaction> {
        super::db::read(conn, Self::STORE)
    }

    pub(super) fn write(conn: &Database) -> anyhow::Result<Transaction> {
        super::db::write(conn, Self::STORE)
    }

    pub(super) async fn get(tx: &Transaction<'_>) -> anyhow::Result<Option<Self>> {
        super::db::get_value(&tx, Self::STORE, 0_u32).await
    }

    pub(super) async fn insert(&self, tx: &Transaction<'_>) -> anyhow::Result<()> {
        super::db::insert_with_key(&tx, Self::STORE, 0_u32, self).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Remanence {
    pub block_remanent: bool,
}

impl Remanence {
    const STORE: &'static str = "remanence";

    fn create(
        _evt: &VersionChangeEvent,
        db: &Database,
    ) -> Result<(), indexed_db_futures::error::Error> {
        if !db.object_store_names().any(|n| n == Self::STORE) {
            db.create_object_store(Self::STORE)
                .with_auto_increment(true)
                .build()?;
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

    fn create(
        _evt: &VersionChangeEvent,
        db: &Database,
    ) -> Result<(), indexed_db_futures::error::Error> {
        if !db.object_store_names().any(|n| n == Self::STORE) {
            let store = db
                .create_object_store(Self::STORE)
                .with_auto_increment(true)
                .build()?;
            store
                .create_index(Self::INDEX_VLOB_ID, KeyPath::One(Self::INDEX_VLOB_ID))
                .with_unique(true)
                .build()?;
            store
                .create_index(Self::INDEX_NEED_SYNC, KeyPath::One(Self::INDEX_NEED_SYNC))
                .build()?;
        }

        Ok(())
    }

    pub(super) fn read(conn: &Database) -> anyhow::Result<Transaction> {
        super::db::read(conn, Self::STORE)
    }

    pub(super) fn write(conn: &Database) -> anyhow::Result<Transaction> {
        super::db::write(conn, Self::STORE)
    }

    pub(super) async fn get(tx: &Transaction<'_>, vlob_id: &Bytes) -> anyhow::Result<Option<Self>> {
        super::db::get_values(
            &tx,
            Self::STORE,
            Self::INDEX_VLOB_ID,
            Uint8Array::from(vlob_id.as_ref()),
        )
        .await
        .map(|x| x.into_iter().nth(0))
    }

    pub(super) async fn get_need_sync(conn: &Database) -> anyhow::Result<Vec<Self>> {
        let tx = Self::read(conn)?;
        super::db::get_values(&tx, Self::STORE, Self::INDEX_NEED_SYNC, true).await
    }

    pub(super) async fn get_all(conn: &Database) -> anyhow::Result<Vec<Self>> {
        let tx = Self::read(conn)?;
        super::db::get_all(&tx, Self::STORE).await
    }

    pub(super) async fn list(
        conn: &Database,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Vec<Self>> {
        let tx = Self::read(conn)?;
        super::db::list(&tx, Self::STORE, offset, limit).await
    }

    pub(super) async fn remove(tx: &Transaction<'_>, vlob_id: &Bytes) -> anyhow::Result<()> {
        super::db::remove(&tx, Self::STORE, Self::INDEX_VLOB_ID, vlob_id.as_ref()).await
    }

    pub(super) async fn insert(&self, tx: &Transaction<'_>) -> anyhow::Result<()> {
        super::db::insert(&tx, Self::STORE, self).await
    }
}

pub(super) async fn initialize_model_if_needed(
    db_req: OpenDbRequestBuilder<&'_ String, u32>,
) -> anyhow::Result<Database> {
    let db_req = db_req.with_on_upgrade_needed(|evt: VersionChangeEvent, db: Database| {
        Vlob::create(&evt, &db)?;
        RealmCheckpoint::create(&evt, &db)?;
        PreventSyncPattern::create(&evt, &db)?;
        Chunk::create(&evt, &db)?;
        Remanence::create(&evt, &db)?;
        Certificate::create(&evt, &db)?;

        Ok(())
    });

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
