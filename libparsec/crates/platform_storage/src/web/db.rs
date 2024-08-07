// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use indexed_db_futures::{
    prelude::IdbTransaction,
    web_sys::{wasm_bindgen::JsValue, IdbTransactionMode},
    IdbDatabase, IdbQuerySource,
};
use libparsec_types::anyhow;
use serde::de::DeserializeOwned;

pub(super) fn read<'a>(conn: &'a IdbDatabase, store: &str) -> anyhow::Result<IdbTransaction<'a>> {
    conn.transaction_on_one_with_mode(store, IdbTransactionMode::Readonly)
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) fn write<'a>(conn: &'a IdbDatabase, store: &str) -> anyhow::Result<IdbTransaction<'a>> {
    conn.transaction_on_one_with_mode(store, IdbTransactionMode::Readwrite)
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn commit(tx: IdbTransaction<'_>) -> anyhow::Result<()> {
    tx.await.into_result().map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn get_value<V>(
    tx: &IdbTransaction<'_>,
    store: &str,
    key: JsValue,
) -> anyhow::Result<Option<V>>
where
    V: DeserializeOwned,
{
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    store
        .get(&key)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
        .map(|x| serde_wasm_bindgen::from_value(x))
        .transpose()
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn get_values<V>(
    tx: &IdbTransaction<'_>,
    store: &str,
    index: &str,
    key: JsValue,
) -> anyhow::Result<Vec<V>>
where
    V: DeserializeOwned,
{
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    let indexed_store = store.index(index).map_err(|e| anyhow::anyhow!("{e:?}"))?;

    let values = indexed_store
        .get_all_with_key(&key)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    values
        .into_iter()
        .map(|x| serde_wasm_bindgen::from_value(x))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn get_all<V>(tx: &IdbTransaction<'_>, store: &str) -> anyhow::Result<Vec<V>>
where
    V: DeserializeOwned,
{
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    let values = store
        .get_all()
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    values
        .into_iter()
        .map(|x| serde_wasm_bindgen::from_value(x))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn list<V>(
    tx: &IdbTransaction<'_>,
    store: &str,
    offset: u32,
    limit: u32,
) -> anyhow::Result<Vec<V>>
where
    V: DeserializeOwned,
{
    use js_sys::Number;
    use web_sys::IdbKeyRange;

    // Index start at 1
    let start = offset + 1;
    let end = start + limit;

    let range = IdbKeyRange::bound_with_lower_open_and_upper_open(
        &Number::from(start),
        &Number::from(end),
        false,
        true,
    )
    .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    let Some(cursor) = store
        .open_cursor_with_range_owned(range)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
    else {
        return Ok(Vec::new());
    };

    cursor
        .into_vec(0)
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))
        .and_then(|v: Vec<_>| {
            v.into_iter()
                .map(|key_val|
                    // TODO: Sad that KeyVal does not provide it's internal value without a reference.
                    // That reference force us to clone the value, which is not optimal.
                    // Could be improved if https://github.com/Alorel/rust-indexed-db/issues/39 is fixed
                    serde_wasm_bindgen::from_value(key_val.value().clone())
                        .map_err(|e| anyhow::anyhow!("{e:?}")))
                .collect::<anyhow::Result<Vec<V>>>()
        })
}

pub(super) async fn count(
    tx: &IdbTransaction<'_>,
    store: &str,
    index: &str,
    key: JsValue,
) -> anyhow::Result<u32> {
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    let indexed_store = store.index(index).map_err(|e| anyhow::anyhow!("{e:?}"))?;

    indexed_store
        .count_with_key(&key)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn clear(tx: &IdbTransaction<'_>, store: &str) -> anyhow::Result<()> {
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    store.clear().map_err(|e| anyhow::anyhow!("{e:?}"))?;

    Ok(())
}

pub(super) async fn insert_with_key(
    tx: &IdbTransaction<'_>,
    store: &str,
    key: JsValue,
    value: JsValue,
) -> anyhow::Result<()> {
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    store
        .put_key_val(&key, &value)
        .map_err(|e| anyhow::anyhow!("{e:?} ({value:?}) is invalid"))?
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn insert(
    tx: &IdbTransaction<'_>,
    store: &str,
    value: JsValue,
) -> anyhow::Result<()> {
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    store
        .put_val(&value)
        .map_err(|e| anyhow::anyhow!("{e:?} ({value:?}) is invalid"))?
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))
}

pub(super) async fn remove(
    tx: &IdbTransaction<'_>,
    store: &str,
    index: &str,
    key: JsValue,
) -> anyhow::Result<()> {
    let store = tx
        .object_store(store)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    let indexed_store = store.index(index).map_err(|e| anyhow::anyhow!("{e:?}"))?;

    if let Some(primary_key) = indexed_store
        .get_key(&key)
        .map_err(|e| anyhow::anyhow!("{e:?} ({key:?}) is invalid"))?
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))?
    {
        store
            .delete(&primary_key)
            .map_err(|e| anyhow::anyhow!("{e:?}"))?
            .await
            .map_err(|e| anyhow::anyhow!("{e:?}"))
    } else {
        Err(anyhow::anyhow!("Entry with {key:?} not found"))
    }
}
