// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;
use std::path::Path;
use wasm_bindgen::{JsCast, JsValue};

use indexed_db::{Database, Factory, Transaction};
use libparsec_types::prelude::*;

use crate::certificates::{
    FilterKind, GetCertificateError, GetCertificateQuery, PerTopicLastTimestamps,
    StorableCertificateTopic, UpTo,
};

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

// Note each database (certificates, workspace etc.) has its own version.
const DB_VERSION: u32 = 1;
// Store contains: {
//   certificate_type: string,
//   filter1: Uint8Array,
//   filter2: Uint8Array,
//   certificate_timestamp: number,
//   certificate: Uint8Array
// }
const STORE: &'static str = "certificates";
const FIELD_CERTIFICATE_TYPE: &'static str = "certificate_type";
const FIELD_FILTER1: &'static str = "filter1";
const FIELD_FILTER2: &'static str = "filter2";
const FIELD_CERTIFICATE_TIMESTAMP: &'static str = "certificate_timestamp";
const FIELD_CERTIFICATE: &'static str = "certificate";
const INDEX_CERTIFICATE_TYPE: &'static str = "_idx_certificate_type";
const INDEX_FILTER1: &'static str = "_idx_filter1";
const INDEX_FILTER2: &'static str = "_idx_filter2";
const INDEX_FILTERS: &'static str = "_idx_filters";

async fn initialize_database(
    db: &Database<CustomErrMarker>,
) -> indexed_db::Result<(), CustomErrMarker> {
    let store = db.build_object_store(STORE).auto_increment().create()?;

    store
        .build_compound_index(
            INDEX_CERTIFICATE_TYPE,
            &[FIELD_CERTIFICATE_TYPE, FIELD_CERTIFICATE_TIMESTAMP],
        )
        .create()?;

    store
        .build_compound_index(
            INDEX_FILTER1,
            &[
                FIELD_CERTIFICATE_TYPE,
                FIELD_FILTER1,
                FIELD_CERTIFICATE_TIMESTAMP,
            ],
        )
        .create()?;

    store
        .build_compound_index(
            INDEX_FILTER2,
            &[
                FIELD_CERTIFICATE_TYPE,
                FIELD_FILTER2,
                FIELD_CERTIFICATE_TIMESTAMP,
            ],
        )
        .create()?;

    store
        .build_compound_index(
            INDEX_FILTERS,
            &[
                FIELD_CERTIFICATE_TYPE,
                FIELD_FILTER1,
                FIELD_FILTER2,
                FIELD_CERTIFICATE_TIMESTAMP,
            ],
        )
        .create()?;

    Ok(())
}

// TODO: update doc
// `indexed_db::Database` force us to specify a custom error type off the bat.
// This is an issue since `PlatformCertificatesStorage::for_update` is generic
// over the error type !
// Hence our only solution is to rely on type erasing to make the error go
// through the `indexed_db` API, only to downcast it back to the original error.
//
// See https://github.com/Ekleog/indexed-db/issues/4
#[derive(Debug, thiserror::Error)]
struct CustomErrMarker;
impl std::fmt::Display for CustomErrMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CustomErrMarker")
    }
}

#[derive(Debug)]
pub(crate) struct PlatformCertificatesStorageForUpdateGuard<'a> {
    transaction: &'a Transaction<CustomErrMarker>,
}

fn rs_to_js_filter(filter: &FilterKind<'_>) -> JsValue {
    match filter {
        FilterKind::Bytes(x) => js_sys::Uint8Array::from(x.as_ref()).into(),
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
fn rs_to_js_timestamp(ts: DateTime) -> anyhow::Result<JsValue> {
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

fn js_to_rs_timestamp(raw_js: JsValue) -> anyhow::Result<DateTime> {
    raw_js
        .as_f64()
        .and_then(|raw| DateTime::from_timestamp_micros(raw as i64).ok())
        .ok_or_else(|| anyhow::anyhow!("Invalid timestamp, expected f64, got {raw_js:?}"))
}

fn js_to_rs_realm_id(raw_js: JsValue) -> anyhow::Result<VlobID> {
    // TODO: We were expecting `raw_js` to be a `Uint8Array` (since it is what we
    // store in the first place, and also what we get when using `indexed_db` API
    // from Javascript...), but it seems it is an `ArrayBuffer` instead.

    raw_js
        .dyn_ref::<js_sys::ArrayBuffer>()
        .and_then(|array| {
            let array = js_sys::Uint8Array::new(array);
            if array.length() as usize != size_of::<uuid::Bytes>() {
                return None;
            }
            let mut raw: uuid::Bytes = Default::default();
            array.copy_to(&mut raw);
            Some(VlobID::from(raw))
        })
        .ok_or_else(|| anyhow::anyhow!("Invalid realm ID, got {raw_js:?}"))
}

fn js_to_rs_bytes(raw_js: JsValue) -> anyhow::Result<Vec<u8>> {
    raw_js
        .dyn_ref::<js_sys::Uint8Array>()
        .map(|array| array.to_vec())
        .ok_or_else(|| anyhow::anyhow!("Invalid bytes, expected Uint8Array, got {raw_js:?}"))
}

fn extract_datetime_field(obj: &JsValue, field: &str) -> anyhow::Result<DateTime> {
    let raw_js = js_sys::Reflect::get(obj, &field.into())
        .map_err(|e| anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}"))?;

    js_to_rs_timestamp(raw_js)
}

fn extract_bytes_field(obj: &JsValue, field: &str) -> anyhow::Result<Vec<u8>> {
    let raw_js = js_sys::Reflect::get(obj, &field.into())
        .map_err(|e| anyhow::anyhow!("Invalid entry, got {obj:?}: error {e:?}"))?;

    js_to_rs_bytes(raw_js)
}

impl<'a> PlatformCertificatesStorageForUpdateGuard<'a> {
    pub async fn get_certificate_encrypted<'b>(
        &mut self,
        query: GetCertificateQuery<'b>,
        up_to: UpTo,
    ) -> Result<(DateTime, Vec<u8>), GetCertificateError> {
        get_certificate_encrypted(&self.transaction, query, up_to).await
    }

    /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
    pub async fn get_multiple_certificates_encrypted<'b>(
        &mut self,
        query: GetCertificateQuery<'b>,
        up_to: UpTo,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<(DateTime, Vec<u8>)>> {
        get_multiple_certificates_encrypted(&self.transaction, query, up_to, offset, limit).await
    }

    pub async fn forget_all_certificates(&mut self) -> anyhow::Result<()> {
        let store = self.transaction.object_store(STORE)?;
        store.clear().await?;
        Ok(())
    }

    pub async fn add_certificate(
        &mut self,
        certificate_type: &'static str,
        filter1: FilterKind<'_>,
        filter2: FilterKind<'_>,
        timestamp: DateTime,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        // 1) Convert data to Javascript types

        let data = js_sys::Object::new();

        js_sys::Reflect::set(&data, &"certificate_type".into(), &certificate_type.into())
            .map_err(|e| anyhow::anyhow!("{e:?}"))?;

        // `certificate_timestamp` field
        {
            js_sys::Reflect::set(
                &data,
                &"certificate_timestamp".into(),
                &rs_to_js_timestamp(timestamp)?,
            )
            .map_err(|e| anyhow::anyhow!("{e:?}"))?;
        }

        // `certificate` field
        {
            let encrypted: JsValue = js_sys::Uint8Array::from(encrypted.as_ref()).into();
            js_sys::Reflect::set(&data, &"certificate".into(), &encrypted)
                .map_err(|e| anyhow::anyhow!("{e:?}"))?;
        }

        // `filter1` field
        {
            let filter1 = rs_to_js_filter(&filter1);
            js_sys::Reflect::set(&data, &"filter1".into(), &filter1)
                .map_err(|e| anyhow::anyhow!("{e:?}"))?;
        }

        // `filter2` field
        {
            let filter2 = rs_to_js_filter(&filter2);
            js_sys::Reflect::set(&data, &"filter2".into(), &filter2)
                .map_err(|e| anyhow::anyhow!("{e:?}"))?;
        }

        // 2) Actual insertion

        let store = self.transaction.object_store(STORE)?;
        store.put(&data.into()).await?;

        Ok(())
    }

    pub async fn get_last_timestamps(&mut self) -> anyhow::Result<PerTopicLastTimestamps> {
        get_last_timestamps(&self.transaction).await
    }

    /// Only used for debugging tests
    #[cfg(any(test, feature = "expose-test-methods"))]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        debug_dump(&self.transaction).await
    }
}

async fn get_last_timestamps(
    transaction: &Transaction<CustomErrMarker>,
) -> anyhow::Result<PerTopicLastTimestamps> {
    let store = transaction
        .object_store(STORE)
        .map_err(|e| anyhow::anyhow!(e))?;

    // Unlike SQL, IndexedDb doesn't support grouping or aggregation.
    //
    // In practice it means we cannot get a last timestamp for a set of certificate type
    // (i.e. `SELECT timestamp FROM certificates WHERE type IN 'user_certificate', 'device_certificate' ORDER BY timestamp DESC`),
    // but only for a single certificate type at a time (i.e. `SELECT ... FROM ... WHERE type = 'user_certificate`).
    //
    // On top of that we cannot group per realm ID for realm certificates, which means
    // we have no choice but to iterate over all realm certificates.
    //
    // So in the end simply iterate over all certificates to compute the last timestamp for each topic.

    let mut per_realm_last_timestamps = HashMap::new();
    let mut common_last_timestamp = None;
    let mut shamir_recovery_last_timestamp = None;
    let mut sequester_last_timestamp = None;

    let mut cursor = store
        .index(&INDEX_FILTER1)
        .map_err(|e| anyhow::anyhow!(e))?
        .cursor()
        .open_key()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    macro_rules! build_types_list {
        ($topic:ty) => {{
            let types = js_sys::Array::new_with_length(
                <$topic as StorableCertificateTopic>::TYPES.len() as u32,
            );
            for (i, ty) in <$topic as StorableCertificateTopic>::TYPES
                .iter()
                .enumerate()
            {
                types.set(i as u32, (*ty).into());
            }
            types
        }};
    }
    let realm_types = build_types_list!(RealmTopicArcCertificate);
    let common_types = build_types_list!(CommonTopicArcCertificate);
    let shamir_recovery_types = build_types_list!(ShamirRecoveryTopicArcCertificate);
    let sequester_types = build_types_list!(SequesterTopicArcCertificate);

    while let Some(key_js) = cursor.key() {
        // Since we use the index `INDEX_FILTER1`, the key has for layout `[certificate_type, filter1, timestamp]`
        let key_as_array = key_js
            .dyn_ref::<js_sys::Array>()
            .ok_or_else(|| anyhow::anyhow!("Invalid key, expected array, got {key_js:?}"))?;
        let certificate_type = key_as_array.get(0);
        let timestamp = js_to_rs_timestamp(key_as_array.get(2))?;

        if realm_types.includes(&certificate_type, 0) {
            // In case of realm certificates, filter1 (i.e. second item in the index) is the realm ID
            let realm_id = js_to_rs_realm_id(key_as_array.get(1))?;
            match per_realm_last_timestamps.entry(realm_id) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.insert(std::cmp::max(*entry.get(), timestamp));
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(timestamp);
                }
            }
        } else if common_types.includes(&certificate_type, 0) {
            common_last_timestamp = match common_last_timestamp {
                None => Some(timestamp),
                Some(last_timestamp) => Some(std::cmp::max(last_timestamp, timestamp)),
            };
        } else if shamir_recovery_types.includes(&certificate_type, 0) {
            shamir_recovery_last_timestamp = match shamir_recovery_last_timestamp {
                None => Some(timestamp),
                Some(last_timestamp) => Some(std::cmp::max(last_timestamp, timestamp)),
            };
        } else if sequester_types.includes(&certificate_type, 0) {
            sequester_last_timestamp = match sequester_last_timestamp {
                None => Some(timestamp),
                Some(last_timestamp) => Some(std::cmp::max(last_timestamp, timestamp)),
            };
        }

        cursor.advance(1).await.map_err(|e| anyhow::anyhow!(e))?;
    }

    Ok(PerTopicLastTimestamps {
        realm: per_realm_last_timestamps,
        common: common_last_timestamp,
        shamir_recovery: shamir_recovery_last_timestamp,
        sequester: sequester_last_timestamp,
    })
}

macro_rules! count_tts {
    () => {0u32};
    ($_head:tt $($tail:tt)*) => {1u32 + count_tts!($($tail)*)};
}

/// The idea here is to obtain an IndexedDb cursor from `query` & `up_to`,
/// for this we:
/// 1. Select an index, note they all have the `timestamp` as field.
/// 2. Generate the lower bound for the range by filling the fields of the index
///    according to `query`. Note we set the `timestamp` field the lowest
///    possible value (i.e. NEGATIVE_INFINITY) as a way to ignore it.
/// 3. If `up_to` is provided, we generate the upper bound for the range by
///    cloning the lower bound and updating its `timestamp` field to the
///    provided value.
///
/// Finally we configure the direction of the cursor in previous mode.
/// Since the index always end with the `timestamp` field, this makes our
/// results ordered by timestamp in decreasing order (i.e. newest first).
macro_rules! configure_cursor_range {
    ($up_to: expr, $cursor: expr, [ $( $bound_fields:expr ),* $(,)? ] $(,)?) => {
        {
            const INDEX_FIELDS_MINUS_TIMESTAMP: u32 = count_tts!( $( $bound_fields )* );
            const INDEX_FIELDS: u32 = INDEX_FIELDS_MINUS_TIMESTAMP + 1;

            let lower_bound = {
                let lower_bound = js_sys::Array::new_with_length(INDEX_FIELDS);
                let mut i: u32 = 0;
                $(
                    lower_bound.set(i, $bound_fields.into());
                    i += 1;
                )*
                // Last item in the index is always the timestamp, set it
                // to the lowest possible value to ignore it
                lower_bound.set(i, js_sys::Number::NEGATIVE_INFINITY.into());
                lower_bound
            };

            // Clone lower bound, and update the timestamp to the upper bound
            let upper_bound = {
                let upper_bound = lower_bound.slice(0, INDEX_FIELDS);
                match $up_to {
                    UpTo::Current => {
                        upper_bound.set(INDEX_FIELDS - 1, js_sys::Number::POSITIVE_INFINITY.into());
                    }
                    UpTo::Timestamp(up_to) => {
                        let up_to_js = rs_to_js_timestamp(up_to).context("Invalid `up_to` parameter")?;
                        upper_bound.set(INDEX_FIELDS - 1, up_to_js);
                    }
                }
                upper_bound
            };

            $cursor.range(JsValue::from(lower_bound)..=JsValue::from(upper_bound))
            .map_err(|e| anyhow::anyhow!(e))?
        }
    };
}

async fn cursor_factory_fx_eq_fy_where_fz(
    store: &indexed_db::ObjectStore<CustomErrMarker>,
    up_to: UpTo,
    query_certificate_type: &str,
    query_index: &str,
    subquery_certificate_type: &str,
    subquery_index: &str,
    subquery_filter_value: &FilterKind<'_>,
    subquery_selected_field: &str,
) -> anyhow::Result<Option<indexed_db::CursorBuilder<CustomErrMarker>>> {
    // Do the subquery first

    let subquery_index = store
        .index(subquery_index)
        .map_err(|e| anyhow::anyhow!(e))?;

    let subquery_cursor = configure_cursor_range!(
        up_to,
        subquery_index.cursor(),
        [
            subquery_certificate_type,
            rs_to_js_filter(subquery_filter_value),
        ],
    );

    let subquery_result = subquery_cursor
        // Order by timestamp in decreasing order (i.e. newest first)
        .direction(indexed_db::CursorDirection::Prev)
        .open()
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .value();

    let filter_value = match subquery_result {
        Some(obj) => js_sys::Reflect::get(&obj, &subquery_selected_field.into()).map_err(|_| {
            anyhow::anyhow!(
                "Invalid entry, got {obj:?}: field {subquery_selected_field:?} is missing"
            )
        })?,
        None => return Ok(None),
    };

    // Now the main query

    let index = store.index(&query_index).map_err(|e| anyhow::anyhow!(e))?;

    let cursor = configure_cursor_range!(
        up_to,
        index.cursor(),
        [query_certificate_type, filter_value,],
    );

    Ok(Some(cursor))
}

async fn cursor_builder_factory<'a>(
    transaction: &Transaction<CustomErrMarker>,
    query: &GetCertificateQuery<'a>,
    up_to: UpTo,
) -> anyhow::Result<Option<indexed_db::CursorBuilder<CustomErrMarker>>> {
    let store = transaction
        .object_store(STORE)
        .map_err(|e| anyhow::anyhow!(e))?;

    let cursor_builder = match query {
        GetCertificateQuery::NoFilter { certificate_type } => {
            let index = store
                .index(&INDEX_CERTIFICATE_TYPE)
                .map_err(|e| anyhow::anyhow!(e))?;

            configure_cursor_range!(up_to, index.cursor(), [*certificate_type],)
        }

        GetCertificateQuery::Filter1 {
            certificate_type,
            filter1,
        } => {
            let index = store
                .index(&INDEX_FILTER1)
                .map_err(|e| anyhow::anyhow!(e))?;

            configure_cursor_range!(
                up_to,
                index.cursor(),
                [*certificate_type, rs_to_js_filter(filter1),],
            )
        }

        GetCertificateQuery::Filter2 {
            certificate_type,
            filter2,
        } => {
            let index = store
                .index(&INDEX_FILTER2)
                .map_err(|e| anyhow::anyhow!(e))?;

            configure_cursor_range!(
                up_to,
                index.cursor(),
                [*certificate_type, rs_to_js_filter(filter2),],
            )
        }

        GetCertificateQuery::BothFilters {
            certificate_type,
            filter1,
            filter2,
        } => {
            let index = store
                .index(&INDEX_FILTERS)
                .map_err(|e| anyhow::anyhow!(e))?;

            configure_cursor_range!(
                up_to,
                index.cursor(),
                [
                    *certificate_type,
                    rs_to_js_filter(filter1),
                    rs_to_js_filter(filter2),
                ],
            )
        }

        GetCertificateQuery::Filter1EqFilter2WhereFilter1 {
            certificate_type,
            subquery_certificate_type,
            filter1,
        } => {
            // SELECT * FROM certificates WHERE type = ? AND filter1 = (SELECT filter2 FROM certificates WHERE type = ? AND filter1 = ?)
            match cursor_factory_fx_eq_fy_where_fz(
                &store,
                up_to,
                certificate_type,
                INDEX_FILTER1,
                &subquery_certificate_type,
                INDEX_FILTER1,
                filter1,
                FIELD_FILTER2,
            )
            .await?
            {
                Some(cursor) => cursor,
                None => return Ok(None),
            }
        }

        GetCertificateQuery::Filter1EqFilter1WhereFilter2 {
            certificate_type,
            subquery_certificate_type,
            filter2,
        } => {
            // SELECT * FROM certificates WHERE type = ? AND filter1 = (SELECT filter1 FROM certificates WHERE type = ? AND filter2 = ?)
            match cursor_factory_fx_eq_fy_where_fz(
                &store,
                up_to,
                certificate_type,
                INDEX_FILTER1,
                &subquery_certificate_type,
                INDEX_FILTER2,
                filter2,
                FIELD_FILTER1,
            )
            .await?
            {
                Some(cursor) => cursor,
                None => return Ok(None),
            }
        }

        GetCertificateQuery::Filter2EqFilter1WhereFilter2 {
            certificate_type,
            subquery_certificate_type,
            filter2,
        } => {
            // SELECT * FROM certificates WHERE type = ? AND filter2 = (SELECT filter1 FROM certificates WHERE type = ? AND filter2 = ?)
            match cursor_factory_fx_eq_fy_where_fz(
                &store,
                up_to,
                certificate_type,
                INDEX_FILTER2,
                &subquery_certificate_type,
                INDEX_FILTER2,
                filter2,
                FIELD_FILTER1,
            )
            .await?
            {
                Some(cursor) => cursor,
                None => return Ok(None),
            }
        }

        GetCertificateQuery::Filter2EqFilter2WhereFilter1 {
            certificate_type,
            subquery_certificate_type,
            filter1,
        } => {
            // SELECT * FROM certificates WHERE type = ? AND filter2 = (SELECT filter2 FROM certificates WHERE type = ? AND filter1 = ?)
            match cursor_factory_fx_eq_fy_where_fz(
                &store,
                up_to,
                certificate_type,
                INDEX_FILTER2,
                &subquery_certificate_type,
                INDEX_FILTER1,
                filter1,
                FIELD_FILTER2,
            )
            .await?
            {
                Some(cursor) => cursor,
                None => return Ok(None),
            }
        }
    };

    Ok(Some(cursor_builder))
}

async fn get_certificate_encrypted<'a>(
    transaction: &Transaction<CustomErrMarker>,
    query: GetCertificateQuery<'a>,
    up_to: UpTo,
) -> Result<(DateTime, Vec<u8>), GetCertificateError> {
    // Handling `up_to` with a timestamp is a bit tricky:
    // 1) We want the last result up to the given timestamp (included).
    // 2) BUT ! If 1) fails we want the first result in order
    //    to correctly return NonExisting vs ExistButTooRecent.
    //
    // So to do that we act in three steps:
    //  1) Do the SQL query with the `up_to` filter.
    //  2) If the previous query returned nothing, do another SQL query without `up_to`.
    //  3a) If this new query return no result, it's obviously a NonExisting error.
    //  3b) If this new query returns a result, compare its timestamp with the one provided
    //      by `up_to`: higher means it's an ExistButTooRecent error, otherwise it's a valid
    //      result that got concurrently added between steps 1 and 2 (highly unlikely though).

    // Step 1: do the regular SQL query

    if let Some(cursor_builder) = cursor_builder_factory(transaction, &query, up_to).await? {
        let cursor = cursor_builder
            // Last item in the index is always the timestamp, so here we order by
            // timestamp in decreasing order (i.e. newest first)
            .direction(indexed_db::CursorDirection::Prev)
            .open()
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        if let Some(row_js) = cursor.value() {
            // Huzza we got something !
            let certificate_timestamp =
                extract_datetime_field(&row_js, FIELD_CERTIFICATE_TIMESTAMP)?;
            let certificate = extract_bytes_field(&row_js, FIELD_CERTIFICATE)?;

            return Ok((certificate_timestamp, certificate));
        }
    }

    // If we are here, it means the first query returned nothing :(

    let up_to = if let UpTo::Timestamp(up_to) = up_to {
        up_to
    } else {
        return Err(GetCertificateError::NonExisting);
    };

    // Step 2: do another SQL query without `up_to` to look for an
    // "existing but too recent" item

    let maybe_row = match cursor_builder_factory(transaction, &query, UpTo::Current).await? {
        Some(cursor_builder) => {
            let cursor = cursor_builder
                // Last item in the index is always the timestamp, so here we order by
                // timestamp in decreasing order (i.e. newest first)
                .direction(indexed_db::CursorDirection::Prev)
                .open()
                .await
                .map_err(|e| anyhow::anyhow!(e))?;

            cursor.value()
        }
        None => None,
    };

    // Step 3: determine if the result is an actual success or a ExistButTooRecent error

    if let Some(row_js) = maybe_row {
        let certificate_timestamp = extract_datetime_field(&row_js, FIELD_CERTIFICATE_TIMESTAMP)?;

        if certificate_timestamp > up_to {
            Err(GetCertificateError::ExistButTooRecent {
                certificate_timestamp,
            })
        } else {
            // If we are here, a concurrent insertion has created the certificate
            // we are looking for between steps 1 and 2... this is highly unlikely though
            // (especially given we don't do concurrent operation on the certificate
            // database !)
            let certificate = extract_bytes_field(&row_js, FIELD_CERTIFICATE)?;

            Ok((certificate_timestamp, certificate))
        }
    } else {
        Err(GetCertificateError::NonExisting)
    }
}

async fn get_multiple_certificates_encrypted<'a>(
    transaction: &Transaction<CustomErrMarker>,
    query: GetCertificateQuery<'a>,
    up_to: UpTo,
    offset: Option<u32>,
    limit: Option<u32>,
) -> anyhow::Result<Vec<(DateTime, Vec<u8>)>> {
    let mut cursor = match cursor_builder_factory(transaction, &query, up_to).await? {
        Some(cursor) => cursor
            .direction(indexed_db::CursorDirection::Next)
            .open()
            .await
            .map_err(|e| anyhow::anyhow!(e))?,
        None => return Ok(vec![]),
    };

    // Note cursor already takes care of handling the higher timestamp bound and
    // ordering by timestamp in increasing order (i.e. oldest first), so we just
    // have to apply offset & limit and collect the results.

    if let Some(offset) = offset {
        cursor.advance(offset).await?;
    }

    let mut res = vec![];
    while limit
        .map(|limit| res.len() < limit as usize)
        .unwrap_or(true)
    {
        let obj = match cursor.value() {
            Some(val) => val,
            None => break,
        };

        let certificate_timestamp = extract_datetime_field(&obj, FIELD_CERTIFICATE_TIMESTAMP)?;
        let certificate = extract_bytes_field(&obj, FIELD_CERTIFICATE)?;
        res.push((certificate_timestamp, certificate));

        cursor.advance(1).await?;
    }

    Ok(res)
}

#[cfg(any(test, feature = "expose-test-methods"))]
async fn debug_dump(transaction: &Transaction<CustomErrMarker>) -> anyhow::Result<String> {
    let mut output = "[\n".to_owned();

    let store = transaction
        .object_store(STORE)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    let cursor = store
        .cursor()
        .open()
        .await
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;

    loop {
        let obj = match cursor.value() {
            Some(val) => val,
            None => break,
        };

        let certificate_timestamp = extract_datetime_field(&obj, FIELD_CERTIFICATE_TIMESTAMP)?;
        let certificate_type = extract_bytes_field(&obj, FIELD_CERTIFICATE_TYPE)?;
        let filter1 = extract_bytes_field(&obj, FIELD_FILTER1)?;
        let filter2 = extract_bytes_field(&obj, FIELD_FILTER2)?;

        output += &format!(
            "{{\n\
            \ttimestamp: {certificate_timestamp:?}\n\
            \ttype: {certificate_type:?}\n\
            \tfilter1: {filter1:?}\n\
            \tfilter2: {filter2:?}\n\
        }},\n",
        );
    }
    output += "]\n";

    Ok(output)
}

#[derive(Debug)]
pub(crate) struct PlatformCertificatesStorage {
    conn: Database<CustomErrMarker>,
}

// SAFETY: see `pretend_future_is_send_on_web`'s documentation for the full explanation.
// `PlatformCertificatesStorage` contains `indexed_db::Database` which is `!Send` since
// it interfaces with the browser's IndexedDB API that is not thread-safe.
// Since are always mono-threaded in web, we can safely pretend to be `Send`, which is
// handy for our platform-agnostic code.
unsafe impl Send for PlatformCertificatesStorage {}
// SAFETY: see `pretend_future_is_send_on_web`'s documentation for the full explanation.
unsafe impl Sync for PlatformCertificatesStorage {}

impl Drop for PlatformCertificatesStorage {
    fn drop(&mut self) {
        self.conn.close();
    }
}

impl PlatformCertificatesStorage {
    pub async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
    ) -> anyhow::Result<Self> {
        // 1) Open the database

        let name = get_certificates_storage_db_name(data_base_dir, device.device_id);

        let factory = Factory::<CustomErrMarker>::get().map_err(|e| anyhow::anyhow!("{e:?}"))?;
        let conn = factory
            .open(
                &name,
                DB_VERSION,
                |evt: indexed_db::VersionChangeEvent<CustomErrMarker>| async move {
                    // 2) Initialize the database (if needed)

                    let db = evt.database();
                    initialize_database(&db).await
                },
            )
            .await
            .map_err(|e| anyhow::anyhow!("{e:?}"))?;

        // 3) All done !

        Ok(Self { conn })
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        // Nothing to do here since the database is closed automatically on drop
        Ok(())
    }

    pub async fn for_update<R, E>(
        &mut self,
        cb: impl AsyncFnOnce(PlatformCertificatesStorageForUpdateGuard) -> Result<R, E>,
    ) -> anyhow::Result<Result<R, E>> {
        let custom_err = std::cell::Cell::new(None);
        let custom_err_ref = &custom_err;

        let outcome = self
            .conn
            .transaction(&[STORE])
            .rw()
            .run(async move |transaction| {
                let updater = PlatformCertificatesStorageForUpdateGuard {
                    transaction: &transaction,
                };
                cb(updater).await.map_err(|e| {
                    custom_err_ref.set(Some(e));
                    CustomErrMarker.into()
                })
            })
            .await;

        match outcome {
            Ok(ok) => Ok(Ok(ok)),
            Err(err) => match err {
                indexed_db::Error::User(_) => {
                    Ok(Err(custom_err.take().expect("error must have been set")))
                }
                err => Err(anyhow::anyhow!("{err:?}")),
            },
        }
    }

    pub async fn get_last_timestamps(&mut self) -> anyhow::Result<PerTopicLastTimestamps> {
        self.with_ro_transaction(async move |transaction| get_last_timestamps(&transaction).await)
            .await
    }

    pub async fn get_certificate_encrypted<'a>(
        &mut self,
        query: GetCertificateQuery<'a>,
        up_to: UpTo,
    ) -> Result<(DateTime, Vec<u8>), GetCertificateError> {
        self.with_ro_transaction(async move |transaction| {
            get_certificate_encrypted(&transaction, query, up_to).await
        })
        .await
    }

    pub async fn get_multiple_certificates_encrypted<'a>(
        &mut self,
        query: GetCertificateQuery<'a>,
        up_to: UpTo,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<(DateTime, Vec<u8>)>> {
        self.with_ro_transaction(async move |transaction| {
            get_multiple_certificates_encrypted(&transaction, query, up_to, offset, limit).await
        })
        .await
    }

    /// Only used for debugging tests
    #[cfg(any(test, feature = "expose-test-methods"))]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        self.with_ro_transaction(async |transaction| debug_dump(&transaction).await)
            .await
    }

    async fn with_ro_transaction<R, E: From<anyhow::Error>>(
        &self,
        cb: impl AsyncFnOnce(Transaction<CustomErrMarker>) -> Result<R, E>,
    ) -> Result<R, E> {
        let custom_err = std::cell::Cell::new(None);
        let custom_err_ref = &custom_err;

        let outcome = self
            .conn
            .transaction(&[STORE])
            .run(async move |transaction| {
                cb(transaction).await.map_err(|e| {
                    custom_err_ref.set(Some(e));
                    CustomErrMarker.into()
                })
            })
            .await;

        match outcome {
            Ok(ok) => Ok(ok),
            Err(err) => match err {
                indexed_db::Error::User(_) => {
                    Err(custom_err.take().expect("error must have been set"))
                }
                err => Err(anyhow::anyhow!("{err:?}").into()),
            },
        }
    }
}
