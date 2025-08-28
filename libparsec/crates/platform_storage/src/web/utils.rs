// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use indexed_db::{Database, Transaction};
use wasm_bindgen::{JsCast, JsValue};

use libparsec_types::prelude::*;

use crate::certificates::FilterKind;

// TODO: update doc
// `indexed_db::Database` force us to specify a custom error type off the bat.
// This is an issue since `PlatformCertificatesStorage::for_update` is generic
// over the error type !
// Hence our only solution is to rely on type erasing to make the error go
// through the `indexed_db` API, only to downcast it back to the original error.
//
// See https://github.com/Ekleog/indexed-db/issues/4
#[derive(Debug, thiserror::Error)]
pub(super) struct CustomErrMarker;
impl std::fmt::Display for CustomErrMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CustomErrMarker")
    }
}

pub(super) fn rs_to_js_filter(filter: &FilterKind<'_>) -> JsValue {
    match filter {
        FilterKind::Bytes(x) => js_sys::Uint8Array::from(*x).into(),
        FilterKind::U64(x) => js_sys::Uint8Array::from(x.as_ref()).into(),
        // We don't use a Javascript `null` here since it doesn't work with
        // IndexedDb's index (e.g. any item containing a `null` will be skipped
        // when iterating over an index :/).
        FilterKind::Null => js_sys::Uint8Array::new_with_length(0).into(),
    }
}

/// `DateTime` type stores the amount of microseconds since epoch on a u64.
///
/// However Javascript's number type is a f64 in a trench-coat, so it supports only
/// 53 bits integers...
///
/// So in theory we should represent our `DateTime` as a BigInt, but IndexedDb doesn't
/// support BigInt as key (which is not documented -_-'') because a Twitter poll
/// convinced the W3C this is not needed o_O (see https://github.com/w3c/IndexedDB/pull/231).
///
/// So anyway we have the choice between rolling our own encoding of `DateTime` on a
/// `Uint8Array` (and ensuring endianness doesn't cause issue with ordering...) or
/// simply stick to regular js number by truncating the timestamp to 53 bits.
///
/// We choose the latter since 53 bits allows to encode up to year 2255, which is
/// good enough since we only use datetime for current time in Parsec.
pub(super) fn rs_to_js_timestamp(ts: DateTime) -> anyhow::Result<JsValue> {
    const MIN_SAFE_INTEGER: i64 = js_sys::Number::MIN_SAFE_INTEGER as i64;
    const MAX_SAFE_INTEGER: i64 = js_sys::Number::MAX_SAFE_INTEGER as i64;

    let ts_i64 = ts.as_timestamp_micros();
    if !(MIN_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&ts_i64) {
        return Err(anyhow::anyhow!(
            "Timestamp {ts} can't be represented as a JavaScript number"
        ));
    }
    Ok((ts_i64 as f64).into())
}

/// Wasm-bindgen's i64 -> JsValue conversion relies on BigInt, which is not
/// supported by IndexedDb's index (see `rs_to_js_timestamp` for more details).
#[expect(dead_code)]
pub(super) fn rs_to_js_i64(x: i64) -> anyhow::Result<JsValue> {
    const MIN_SAFE_INTEGER: i64 = js_sys::Number::MIN_SAFE_INTEGER as i64;
    const MAX_SAFE_INTEGER: i64 = js_sys::Number::MAX_SAFE_INTEGER as i64;

    if !(MIN_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&x) {
        return Err(anyhow::anyhow!(
            "Integer {x} can't be represented as a JavaScript number"
        ));
    }
    Ok((x as f64).into())
}

/// Wasm-bindgen's u64 -> JsValue conversion relies on BigInt, which is not
/// supported by IndexedDb's index (see `rs_to_js_timestamp` for more details).
pub(super) fn rs_to_js_u64(x: u64) -> anyhow::Result<JsValue> {
    const MAX_SAFE_INTEGER: u64 = js_sys::Number::MAX_SAFE_INTEGER as u64;

    if x > MAX_SAFE_INTEGER {
        return Err(anyhow::anyhow!(
            "Integer {x} can't be represented as a JavaScript number"
        ));
    }
    Ok((x as f64).into())
}

pub(super) fn js_to_rs_u64(raw_js: JsValue) -> anyhow::Result<u64> {
    raw_js
        .as_f64()
        .map(|raw| raw as u64)
        .ok_or_else(|| anyhow::anyhow!("Invalid integer, expected f64, got {raw_js:?}"))
}

pub(super) fn js_to_rs_u32(raw_js: JsValue) -> anyhow::Result<u32> {
    raw_js
        .as_f64()
        .map(|raw| raw as u32)
        .ok_or_else(|| anyhow::anyhow!("Invalid integer, expected f64, got {raw_js:?}"))
}

pub(super) fn js_to_rs_timestamp(raw_js: JsValue) -> anyhow::Result<DateTime> {
    raw_js
        .as_f64()
        .and_then(|raw| DateTime::from_timestamp_micros(raw as i64).ok())
        .ok_or_else(|| anyhow::anyhow!("Invalid timestamp, expected f64, got {raw_js:?}"))
}

macro_rules! bytes_based_js_to_rs_conversion {
    ($raw_js: expr, $final_type:tt) => {
        // TODO: We were expecting `raw_js` to be a `Uint8Array` (since it is what we
        // store in the first place, and also what we get when using `indexed_db` API
        // from Javascript...), but it seems it is an `ArrayBuffer` instead.
        $raw_js
            .dyn_ref::<js_sys::ArrayBuffer>()
            .and_then(|array| {
                let array = js_sys::Uint8Array::new(array);
                if array.length() as usize != size_of::<uuid::Bytes>() {
                    return None;
                }
                let mut raw: uuid::Bytes = Default::default();
                array.copy_to(&mut raw);
                Some($final_type::from(raw))
            })
            .ok_or_else(|| {
                anyhow::anyhow!(concat!(
                    "Invalid",
                    stringify!($final_type),
                    ", got {raw_js:?}"
                ))
            })
    };
}

pub(super) fn js_to_rs_vlob_id(raw_js: JsValue) -> anyhow::Result<VlobID> {
    bytes_based_js_to_rs_conversion!(raw_js, VlobID)
}

#[cfg_attr(not(any(test, feature = "expose-test-methods")), expect(dead_code))]
pub(super) fn js_to_rs_block_id(raw_js: JsValue) -> anyhow::Result<BlockID> {
    bytes_based_js_to_rs_conversion!(raw_js, BlockID)
}

#[cfg_attr(not(any(test, feature = "expose-test-methods")), expect(dead_code))]
pub(super) fn js_to_rs_chunk_id(raw_js: JsValue) -> anyhow::Result<ChunkID> {
    bytes_based_js_to_rs_conversion!(raw_js, ChunkID)
}

pub(super) fn js_to_rs_bytes(raw_js: JsValue) -> anyhow::Result<Vec<u8>> {
    raw_js
        .dyn_ref::<js_sys::Uint8Array>()
        .map(|array| array.to_vec())
        .ok_or_else(|| anyhow::anyhow!("Invalid bytes, expected Uint8Array, got {raw_js:?}"))
}

pub(super) fn js_to_rs_string(raw_js: JsValue) -> anyhow::Result<String> {
    raw_js
        .as_string()
        .ok_or_else(|| anyhow::anyhow!("Invalid string, got {raw_js:?}"))
}

// We use a macro here to be able to log the file and line where the transaction
// is started, otherwise internal errors are just a nightmare to debug (since web
// doesn't support backtrace, we otherwise only get a generic error message).
macro_rules! with_transaction {
    ($conn: expr, $stores: expr, $rw: expr, $cb: expr $(,)?) => {
        $crate::platform::utils::with_transaction_internal(
            $conn,
            $stores,
            $rw,
            $cb,
            file!(),
            line!(),
        )
    };
}
pub(super) use with_transaction;

// TODO: `cb` signature should be modified so that it returns an `indexed_db::Error`,
// this way we can add file/line info to the IndexedDb errors.
// However this is not yet possible since for the moment `indexed_db` API requires
// a single error type for all operations (see https://github.com/Ekleog/indexed-db/issues/4).
pub(super) async fn with_transaction_internal<R, E>(
    conn: &Database,
    stores: &[&str],
    rw: bool,
    cb: impl AsyncFnOnce(Transaction<CustomErrMarker>) -> Result<R, E>,
    file: &str,
    line: u32,
) -> anyhow::Result<Result<R, E>> {
    // log::debug!("{file}:{line} IndexedDb transaction started stores={stores:?}, write={rw}");
    let custom_err = std::cell::Cell::new(None);
    let custom_err_ref = &custom_err;

    let transaction_builder = conn.transaction(stores);
    let transaction_builder = if rw {
        transaction_builder.rw()
    } else {
        transaction_builder
    };

    let outcome = transaction_builder
        .run(async move |transaction| {
            cb(transaction).await.map_err(|e| {
                custom_err_ref.set(Some(e));
                CustomErrMarker.into()
            })
        })
        .await;

    match outcome {
        Ok(ok) => {
            // log::debug!("{file}:{line} IndexedDb transaction ok stores={stores:?}");
            Ok(Ok(ok))
        }
        Err(err) => match err {
            indexed_db::Error::User(_) => {
                Ok(Err(custom_err.take().expect("error must have been set")))
            }
            err => {
                let err = anyhow::anyhow!(
                    "{file}:{line} IndexedDb error (stores={stores:?}, rw={rw}): {err}"
                );
                // log::debug!("{file}:{line} IndexedDb transaction stores={stores:?} internal error: {err:?}");
                Err(err)
            }
        },
    }
}
