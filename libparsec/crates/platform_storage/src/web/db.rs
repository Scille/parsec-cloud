// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use indexed_db_futures::{
    database::Database,
    prelude::BuildSerde,
    query_source::QuerySource,
    transaction::{Transaction, TransactionMode},
    Build, BuildPrimitive, DeserialiseFromJs, KeyRange, SerialiseToJs,
};
use js_sys::wasm_bindgen::JsValue;
use libparsec_platform_async::stream::TryStreamExt;
use libparsec_types::anyhow;
use serde::de::DeserializeOwned;

pub(super) fn read<'a>(conn: &'a Database, store: &str) -> anyhow::Result<Transaction<'a>> {
    conn.transaction(store)
        .with_mode(TransactionMode::Readonly)
        .build()
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) fn write<'a>(conn: &'a Database, store: &str) -> anyhow::Result<Transaction<'a>> {
    conn.transaction(store)
        .with_mode(TransactionMode::Readwrite)
        .build()
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn commit(tx: Transaction<'_>) -> anyhow::Result<()> {
    tx.commit().await.map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn get_value<K, I, V>(
    tx: &Transaction<'_>,
    store: &str,
    key: I,
) -> anyhow::Result<Option<V>>
where
    V: DeserializeOwned,
    I: Into<KeyRange<K>>,
    KeyRange<K>: SerialiseToJs,
{
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    store
        .get(key)
        .serde()
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn get_values<Q, V>(
    tx: &Transaction<'_>,
    store: &str,
    index: &str,
    query: Q,
) -> anyhow::Result<Vec<V>>
where
    V: DeserializeOwned,
    Q: Into<JsValue>,
{
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    let indexed_store = store.index(index).map_err(|e| anyhow::anyhow!("{e:?}"))?;

    let get_all = indexed_store.get_all::<V>().with_raw_query(query.into());
    let values = get_all
        .serde()
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    values
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn get_all<V>(tx: &Transaction<'_>, store: &str) -> anyhow::Result<Vec<V>>
where
    V: DeserializeOwned,
{
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    let values = store
        .get_all::<V>()
        .serde()
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    values
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn list<V>(
    tx: &Transaction<'_>,
    store: &str,
    offset: u32,
    limit: u32,
) -> anyhow::Result<Vec<V>>
where
    V: DeserializeOwned,
{
    use indexed_db_futures::KeyRange;

    // Index start at 1
    let start = offset + 1;
    let end = start + limit;

    let range = KeyRange::Bound(start, false, end, true);

    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    let Some(cursor) = store
        .open_cursor()
        .with_query::<u32, KeyRange<u32>>(range)
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
    else {
        return Ok(Vec::new());
    };

    cursor
        .stream_ser::<V>()
        .try_collect()
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn count<I, K>(
    tx: &Transaction<'_>,
    store: &str,
    index: &str,
    key: I,
) -> anyhow::Result<u32>
where
    I: Into<KeyRange<K>>,
    KeyRange<K>: SerialiseToJs,
{
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    let indexed_store = store.index(index).map_err(|e| anyhow::anyhow!("{e:?}"))?;

    indexed_store
        .count()
        .with_query(key)
        .serde()
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn clear(tx: &Transaction<'_>, store: &str) -> anyhow::Result<()> {
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    store.clear().map_err(|e| anyhow::anyhow!("{e:?}"))?;

    Ok(())
}

pub(super) async fn insert_with_key<K, V>(
    tx: &Transaction<'_>,
    store: &str,
    key: K,
    value: V,
) -> anyhow::Result<()>
where
    K: SerialiseToJs + DeserialiseFromJs,
    V: SerialiseToJs,
{
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    store
        .put(value)
        .with_key(key)
        .serde()
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
        .await
        .and(Ok(()))
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn insert<V>(tx: &Transaction<'_>, store: &str, value: V) -> anyhow::Result<()>
where
    V: SerialiseToJs,
{
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    store
        .put(value)
        .serde()
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn remove<I, K>(
    tx: &Transaction<'_>,
    store: &str,
    index: &str,
    key: I,
) -> anyhow::Result<()>
where
    I: Into<KeyRange<K>> + std::fmt::Debug,
    KeyRange<K>: indexed_db_futures::primitive::TryToJs,
{
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    let indexed_store = store.index(index).map_err(|e| anyhow::anyhow!("{e:?}"))?;

    if let Some(primary_key) = indexed_store
        .get_key(key)
        .with_key_type::<JsValue>()
        .primitive()
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
    {
        store
            .delete(&primary_key)
            .primitive()
            .map_err(|e| anyhow::anyhow!("{e:?}"))?
            .await
            .map_err(|e| anyhow::anyhow!("{e:?}"))
    } else {
        Err(anyhow::anyhow!("Entry not found"))
    }
}
