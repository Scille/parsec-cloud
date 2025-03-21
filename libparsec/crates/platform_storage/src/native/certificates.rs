// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Certificate storage relies on the upper layer to do the actual certification
// validation work and takes care of handling concurrency issues.
// Hence no unique violation should occur under normal circumstances here.

use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous},
    ConnectOptions, Connection, Row, Sqlite, SqliteConnection, Transaction,
};
use std::{collections::HashMap, path::Path};

use libparsec_types::prelude::*;

use super::model::get_certificates_storage_db_relative_path;
use crate::certificates::{
    FilterKind, GetCertificateError, GetCertificateQuery, PerTopicLastTimestamps,
    StorableCertificateTopic, UpTo,
};

// `concat!` macro only works with literal (i.e. `concat!("foo", "bar")`), here we support
// constant identifiers.
// This is useful here so that we can build our query using the `StorableCertificate::TYPE`
// to avoid typos.
macro_rules! ident_concat {
    ($first:expr $(, $remain:expr)* $(,)? ) => {{
        const LEN: usize = $first.len() $( + $remain.len() )*;
        const fn combine(parts: &'static [&'static str]) -> [u8; LEN] {
            let mut out = [0u8; LEN];
            let mut offset = 0;
            let mut i = 0;
            while i < parts.len() {
                let part = parts[i];
                out = copy_slice(part.as_bytes(), out, offset);
                offset += part.len();
                i += 1;
            }
            out
        }
        const RAW: &[u8] = &combine(&[$first $(, $remain )*]);
        // SAFETY: `RAW` is a concatenation strings, hence it is a valid UTF-8 string
        const COOKED: &str = unsafe { std::str::from_utf8_unchecked(RAW) };
        COOKED
    }}
}

/// e.g. `FooBarTopic` (containing `FooCertificate` & `BarCertificate`) -> `'foo_certificate', 'bar_certificate'`
macro_rules! build_sql_in_fragment_types {
    ($topic_struct:ident) => {{
        const fn compute_len() -> usize {
            let types = <$topic_struct as StorableCertificateTopic>::TYPES;
            let mut l: usize = 0;
            let mut i: usize = 0;
            while i < types.len() {
                l += types[i].len();
                l += 3; // Two quotes plus coma between items
                i += 1;
            }
            if l != 0 {
                // Last item doesn't have coma
                l -= 1;
            }
            l
        }
        const LEN: usize = compute_len();

        const fn combine() -> [u8; LEN] {
            let types = <$topic_struct as StorableCertificateTopic>::TYPES;
            let mut out = [0u8; LEN];
            let mut offset = 0;
            let mut i = 0;
            while i < types.len() {
                let t = types[i];
                out = copy_slice(b"'", out, offset);
                offset += 1;
                out = copy_slice(t.as_bytes(), out, offset);
                offset += t.len();
                out = copy_slice(b"'", out, offset);
                offset += 1;
                if i + 1 != types.len() {
                    out = copy_slice(b",", out, offset);
                    offset += 1;
                }
                i += 1;
            }
            out
        }
        const RAW: &[u8] = &combine();
        // SAFETY: `RAW` is a concatenation strings, hence it is a valid UTF-8 string
        const COOKED: &str = unsafe { std::str::from_utf8_unchecked(RAW) };
        COOKED
    }};
}

const fn copy_slice<const LEN: usize>(
    input: &[u8],
    mut output: [u8; LEN],
    offset: usize,
) -> [u8; LEN] {
    let mut i = 0;
    while i < input.len() {
        output[offset + i] = input[i];
        i += 1;
    }
    output
}

// It is too error prone to specify the `certificate_type` by hand when writing
// a SQL query filtering on a specific certificate topic (given it will silently
// fail whenever the topic gets a new certificate !). Hence those pre-made fragments.
const SQL_IN_FRAGMENT_COMMON_TYPES: &str = build_sql_in_fragment_types!(CommonTopicArcCertificate);
const SQL_IN_FRAGMENT_SEQUESTER_TYPES: &str =
    build_sql_in_fragment_types!(SequesterTopicArcCertificate);
const SQL_IN_FRAGMENT_REALM_TYPES: &str = build_sql_in_fragment_types!(RealmTopicArcCertificate);
const SQL_IN_FRAGMENT_SHAMIR_RECOVERY_TYPES: &str =
    build_sql_in_fragment_types!(ShamirRecoveryTopicArcCertificate);

fn build_get_certificate_query<'a: 'b, 'b>(
    query: &'a GetCertificateQuery<'b>,
    up_to: UpTo,
) -> sqlx::QueryBuilder<'a, sqlx::Sqlite> {
    let mut builder = sqlx::QueryBuilder::new(
        "\
        SELECT \
            certificate_timestamp, \
            certificate \
        FROM certificates \
        WHERE \
            certificate_type = \
        ",
    );

    macro_rules! bind_filter_kind {
        ($builder: expr, $filter: expr) => {
            match $filter {
                FilterKind::Null => $builder.push_bind(Option::<&[u8]>::None),
                FilterKind::Bytes(filter) => $builder.push_bind(filter),
                FilterKind::U64(filter) => $builder.push_bind(filter.as_ref()),
            }
        };
    }

    match query {
        GetCertificateQuery::NoFilter { certificate_type } => {
            builder.push_bind(certificate_type);
        }
        GetCertificateQuery::Filter1 {
            certificate_type,
            filter1,
        } => {
            builder.push_bind(certificate_type);
            builder.push(" AND filter1 = ");
            bind_filter_kind!(builder, filter1);
        }
        GetCertificateQuery::Filter2 {
            certificate_type,
            filter2,
        } => {
            builder.push_bind(certificate_type);
            builder.push(" AND filter2 = ");
            bind_filter_kind!(builder, filter2);
        }
        GetCertificateQuery::BothFilters {
            certificate_type,
            filter1,
            filter2,
        } => {
            builder.push_bind(certificate_type);
            builder.push(" AND filter1 = ");
            bind_filter_kind!(builder, filter1);
            builder.push(" AND filter2 = ");
            bind_filter_kind!(builder, filter2);
        }
        GetCertificateQuery::Filter1EqFilter2WhereFilter1 {
            certificate_type,
            subquery_certificate_type,
            filter1,
        } => {
            builder.push_bind(certificate_type);
            builder.push(
                " AND filter1 = (\
                SELECT filter2 \
                FROM certificates \
                WHERE certificate_type = \
            ",
            );
            builder.push_bind(subquery_certificate_type);
            builder.push(" AND filter1 = ");
            bind_filter_kind!(builder, filter1);
            builder.push(")");
        }
        GetCertificateQuery::Filter1EqFilter1WhereFilter2 {
            certificate_type,
            subquery_certificate_type,
            filter2,
        } => {
            builder.push_bind(certificate_type);
            builder.push(
                " AND filter1 = (\
                SELECT filter1 \
                FROM certificates \
                WHERE certificate_type = \
            ",
            );
            builder.push_bind(subquery_certificate_type);
            builder.push(" AND filter2 = ");
            bind_filter_kind!(builder, filter2);
            builder.push(")");
        }
        GetCertificateQuery::Filter2EqFilter1WhereFilter2 {
            certificate_type,
            subquery_certificate_type,
            filter2,
        } => {
            builder.push_bind(certificate_type);
            builder.push(
                " AND filter2 = (\
                SELECT filter1 \
                FROM certificates \
                WHERE certificate_type = \
            ",
            );
            builder.push_bind(subquery_certificate_type);
            builder.push(" AND filter2 = ");
            bind_filter_kind!(builder, filter2);
            builder.push(")");
        }
        GetCertificateQuery::Filter2EqFilter2WhereFilter1 {
            certificate_type,
            subquery_certificate_type,
            filter1,
        } => {
            builder.push_bind(certificate_type);
            builder.push(
                " AND filter2 = (\
                SELECT filter2 \
                FROM certificates \
                WHERE certificate_type = \
            ",
            );
            builder.push_bind(subquery_certificate_type);
            builder.push(" AND filter1 = ");
            bind_filter_kind!(builder, filter1);
            builder.push(")");
        }
    }

    if let UpTo::Timestamp(up_to) = up_to {
        builder.push(" AND certificate_timestamp <= ");
        builder.push_bind(up_to.as_timestamp_micros());
    }

    builder
}

#[derive(Debug)]
pub(crate) struct PlatformCertificatesStorageForUpdateGuard<'a> {
    /// Don't trust the `'static` lifetimes, it should be `&'b mut Transaction<'a, Sqlite>` here !
    ///
    /// This means you should under no circumstances make this field public or
    /// copy it outside of the struct !
    transaction: &'static mut Transaction<'static, Sqlite>,
    /// Capture the `'a` lifetime that should in theory parametrize the `Transaction` type.
    ///
    /// We cannot do that due to a limitation in the Rust compiler when inferring if
    /// a closure containing a reference is `Send`.
    ///
    /// See https://users.rust-lang.org/t/implementation-of-fnonce-is-not-general-enough/78006/3
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl PlatformCertificatesStorageForUpdateGuard<'_> {
    pub async fn get_certificate_encrypted(
        &mut self,
        query: GetCertificateQuery<'_>,
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
        //      by `up_to`: too means it's an ExistButTooRecent error, otherwise it's a valid
        //      result that got concurrently added between steps 1 and 2 (highly unlikely though).
        let mut query_builder = build_get_certificate_query(&query, up_to);
        query_builder.push(" ORDER BY certificate_timestamp DESC");

        // Step 1: do the regular SQL query

        let maybe_row = query_builder
            .build()
            .fetch_optional(&mut **self.transaction)
            .await
            .map_err(|err| GetCertificateError::Internal(err.into()))?;

        if let Some(row) = maybe_row {
            let certificate_timestamp = {
                let ts = row
                    .try_get::<i64, _>(0)
                    .map_err(|err| GetCertificateError::Internal(err.into()))?;
                DateTime::from_timestamp_micros(ts)
                    .map_err(|err| GetCertificateError::Internal(err.into()))?
            };
            let certificate = row
                .try_get::<Vec<u8>, _>(1)
                .map_err(|err| GetCertificateError::Internal(err.into()))?;

            return Ok((certificate_timestamp, certificate));
        }

        let up_to = if let UpTo::Timestamp(up_to) = up_to {
            up_to
        } else {
            return Err(GetCertificateError::NonExisting);
        };

        // Step 2: do another SQL query without `up_to` to look for an
        // "existing but too recent" item

        let mut query_builder = build_get_certificate_query(&query, UpTo::Current);
        let maybe_row = query_builder
            .build()
            .fetch_optional(&mut **self.transaction)
            .await
            .map_err(|err| GetCertificateError::Internal(err.into()))?;

        // Step 3: determine if the result is an actual success or a ExistButTooRecent error

        if let Some(row) = maybe_row {
            let certificate_timestamp = {
                let ts = row
                    .try_get::<i64, _>(0)
                    .map_err(|err| GetCertificateError::Internal(err.into()))?;
                DateTime::from_timestamp_micros(ts)
                    .map_err(|err| GetCertificateError::Internal(err.into()))?
            };
            if certificate_timestamp > up_to {
                Err(GetCertificateError::ExistButTooRecent {
                    certificate_timestamp,
                })
            } else {
                // If we are here, a concurrent insertion has created the certificate
                // we are looking for between steps 1 and 2... this is highly unlikely though
                // (especially given we don't do concurrent operation on the certificate
                // database !)
                let certificate = row
                    .try_get::<Vec<u8>, _>(1)
                    .map_err(|err| GetCertificateError::Internal(err.into()))?;
                Ok((certificate_timestamp, certificate))
            }
        } else {
            Err(GetCertificateError::NonExisting)
        }
    }

    /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
    pub async fn get_multiple_certificates_encrypted(
        &mut self,
        query: GetCertificateQuery<'_>,
        up_to: UpTo,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<(DateTime, Vec<u8>)>> {
        let mut query_builder = build_get_certificate_query(&query, up_to);
        query_builder.push(" ORDER BY certificate_timestamp ASC");
        // In SQLite, OFFSET clause requires a LIMIT (if there is no actual limit, -1 must be used)
        match (limit, offset) {
            (Some(limit), Some(offset)) => {
                query_builder.push(" LIMIT ");
                query_builder.push_bind(limit);
                query_builder.push(" OFFSET ");
                query_builder.push_bind(offset);
            }
            (Some(limit), None) => {
                query_builder.push(" LIMIT ");
                query_builder.push_bind(limit);
            }
            (None, Some(offset)) => {
                query_builder.push(" LIMIT ");
                query_builder.push_bind(-1);
                query_builder.push(" OFFSET ");
                query_builder.push_bind(offset);
            }
            _ => (),
        }

        let rows = query_builder
            .build()
            .fetch_all(&mut **self.transaction)
            .await?;

        let mut items = Vec::with_capacity(rows.len());
        for row in rows {
            let raw_timestamp = row.try_get::<i64, _>(0)?;
            let certificate = row.try_get::<Vec<u8>, _>(1)?;
            let timestamp = DateTime::from_timestamp_micros(raw_timestamp)?;
            items.push((timestamp, certificate))
        }
        Ok(items)
    }

    pub async fn forget_all_certificates(&mut self) -> anyhow::Result<()> {
        sqlx::query(
            // No WHERE clause, we delete every certificate !
            "DELETE FROM certificates",
        )
        .execute(&mut **self.transaction)
        .await
        .map(|_| ())
        .map_err(|err| err.into())
    }

    pub async fn add_certificate(
        &mut self,
        certificate_type: &'static str,
        filter1: FilterKind<'_>,
        filter2: FilterKind<'_>,
        timestamp: DateTime,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        let builder = sqlx::query(
            "INSERT INTO certificates( \
                certificate_timestamp, \
                certificate, \
                certificate_type, \
                filter1, \
                filter2 \
            ) \
            VALUES(
                ?1,
                ?2,
                ?3,
                ?4,
                ?5
            ) \
            ",
        )
        .bind(timestamp.as_timestamp_micros())
        .bind(&encrypted)
        .bind(certificate_type);

        let builder = match &filter1 {
            FilterKind::Null => builder.bind(Option::<&[u8]>::None),
            FilterKind::Bytes(filter) => builder.bind(filter),
            FilterKind::U64(filter) => builder.bind(filter.as_ref()),
        };

        let builder = match &filter2 {
            FilterKind::Null => builder.bind(Option::<&[u8]>::None),
            FilterKind::Bytes(filter) => builder.bind(filter),
            FilterKind::U64(filter) => builder.bind(filter.as_ref()),
        };

        builder
            .execute(&mut **self.transaction)
            .await
            .map(|_| ())
            .map_err(|err| err.into())
    }

    pub async fn get_last_timestamps(&mut self) -> anyhow::Result<PerTopicLastTimestamps> {
        const SQL_COMMON_LAST_TIMESTAMP: &str = ident_concat!(
            "SELECT certificate_timestamp \
            FROM certificates \
            WHERE certificate_type IN ( \
            ",
            SQL_IN_FRAGMENT_COMMON_TYPES,
            ") \
            ORDER BY certificate_timestamp DESC \
            LIMIT 1 \
            "
        );
        let maybe_row = sqlx::query(SQL_COMMON_LAST_TIMESTAMP)
            .fetch_optional(&mut **self.transaction)
            .await?;
        let common_last_timestamp = if let Some(row) = maybe_row {
            let raw_dt = row.try_get::<i64, _>(0)?;
            Some(DateTime::from_timestamp_micros(raw_dt)?)
        } else {
            None
        };

        const SQL_SEQUESTER_LAST_TIMESTAMP: &str = ident_concat!(
            "SELECT certificate_timestamp \
            FROM certificates \
            WHERE certificate_type IN ( \
            ",
            SQL_IN_FRAGMENT_SEQUESTER_TYPES,
            ") \
            ORDER BY certificate_timestamp DESC \
            LIMIT 1 \
            "
        );
        let maybe_row = sqlx::query(SQL_SEQUESTER_LAST_TIMESTAMP)
            .fetch_optional(&mut **self.transaction)
            .await?;
        let sequester_last_timestamp = if let Some(row) = maybe_row {
            let raw_dt = row.try_get::<i64, _>(0)?;
            Some(DateTime::from_timestamp_micros(raw_dt)?)
        } else {
            None
        };

        const SQL_PER_REALM_LAST_TIMESTAMPS: &str = ident_concat!(
            // Filter 1 is the realm ID for `RealmRoleCertificate`
            "SELECT certificate_timestamp, filter1 \
            FROM ( \
                SELECT * FROM certificates \
                ORDER BY certificate_timestamp DESC \
            ) \
            WHERE certificate_type IN ( \
            ",
            SQL_IN_FRAGMENT_REALM_TYPES,
            ") \
            GROUP BY filter1 \
            "
        );
        let rows = sqlx::query(SQL_PER_REALM_LAST_TIMESTAMPS)
            .fetch_all(&mut **self.transaction)
            .await?;
        let mut per_realm_last_timestamps = HashMap::with_capacity(rows.len());
        for row in rows {
            let raw_dt = row.try_get::<i64, _>(0)?;
            let timestamp = DateTime::from_timestamp_micros(raw_dt)?;
            let raw_realm_id = row.try_get::<&[u8], _>(1)?;
            let realm_id = VlobID::try_from(raw_realm_id).map_err(|err| anyhow::anyhow!(err))?;
            per_realm_last_timestamps.insert(realm_id, timestamp);
        }

        const SQL_SHAMIR_RECOVERY_LAST_TIMESTAMP: &str = ident_concat!(
            "SELECT certificate_timestamp \
            FROM certificates \
            WHERE certificate_type IN ( \
            ",
            SQL_IN_FRAGMENT_SHAMIR_RECOVERY_TYPES,
            ") \
            ORDER BY certificate_timestamp DESC \
            LIMIT 1 \
            "
        );
        let maybe_row = sqlx::query(SQL_SHAMIR_RECOVERY_LAST_TIMESTAMP)
            .fetch_optional(&mut **self.transaction)
            .await?;
        let shamir_recovery_last_timestamp = if let Some(row) = maybe_row {
            let raw_dt = row.try_get::<i64, _>(0)?;
            Some(DateTime::from_timestamp_micros(raw_dt)?)
        } else {
            None
        };

        Ok(PerTopicLastTimestamps {
            common: common_last_timestamp,
            sequester: sequester_last_timestamp,
            realm: per_realm_last_timestamps,
            shamir_recovery: shamir_recovery_last_timestamp,
        })
    }

    /// Only used for debugging tests
    #[cfg(any(test, feature = "expose-test-methods"))]
    #[allow(unused)]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        let rows = sqlx::query(
            "SELECT \
                certificate_timestamp, \
                certificate_type, \
                filter1, \
                filter2 \
            FROM certificates \
            ",
        )
        .fetch_all(&mut **self.transaction)
        .await?;

        let mut output = "[\n".to_owned();
        for row in rows {
            let timestamp = {
                let raw_dt = row.try_get::<i64, _>(0)?;
                DateTime::from_timestamp_micros(raw_dt)
            };
            let type_ = row.try_get::<&str, _>(1)?;
            let filter1 = row.try_get::<&[u8], _>(2)?;
            let filter2 = row.try_get::<&[u8], _>(3)?;
            output += &format!(
                "{{\n\
                \ttimestamp: {timestamp:?}\n\
                \ttype: {type_:?}\n\
                \tfilter1: {filter1:?}\n\
                \tfilter2: {filter2:?}\n\
            }},\n",
            );
        }
        output += "]\n";

        Ok(output)
    }
}

#[derive(Debug)]
pub(crate) struct PlatformCertificatesStorage {
    conn: SqliteConnection,
    #[cfg(feature = "test-with-testbed")]
    path_info: super::testbed::DBPathInfo,
}

impl PlatformCertificatesStorage {
    pub async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
    ) -> anyhow::Result<Self> {
        // 1) Open the database

        let db_relative_path = get_certificates_storage_db_relative_path(device);
        let db_path = data_base_dir.join(&db_relative_path);

        #[cfg(feature = "test-with-testbed")]
        let path_info = super::testbed::DBPathInfo {
            data_base_dir: data_base_dir.to_owned(),
            db_relative_path: db_relative_path.to_owned(),
        };

        #[cfg(feature = "test-with-testbed")]
        let conn = super::testbed::maybe_open_sqlite_in_memory(&path_info).await;

        #[cfg(not(feature = "test-with-testbed"))]
        let conn: Option<SqliteConnection> = None;

        let mut conn = match conn {
            // In-memory database for testing
            Some(conn) => conn,
            // Actual production code: open the connection on disk
            None => {
                if let Some(parent) = db_path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }

                SqliteConnectOptions::new()
                    .filename(&db_path)
                    .create_if_missing(true)
                    .journal_mode(SqliteJournalMode::Wal)
                    .synchronous(SqliteSynchronous::Normal)
                    .connect()
                    .await?
            }
        };

        // 2) Initialize the database (if needed)

        super::model::sqlx_initialize_model_if_needed(&mut conn).await?;

        // 3) All done !

        let storage = Self {
            conn,
            #[cfg(feature = "test-with-testbed")]
            path_info,
        };
        Ok(storage)
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        #[cfg(feature = "test-with-testbed")]
        {
            if let Err(conn) =
                super::testbed::maybe_return_sqlite_in_memory_conn(&self.path_info, self.conn).await
            {
                // Testbed don't want to keep this connection, so we should close it
                conn.close().await.map_err(|e| e.into())
            } else {
                Ok(())
            }
        }

        #[cfg(not(feature = "test-with-testbed"))]
        self.conn.close().await.map_err(|e| e.into())
    }

    pub async fn for_update<R, E>(
        &mut self,
        cb: impl AsyncFnOnce(PlatformCertificatesStorageForUpdateGuard) -> Result<R, E>,
    ) -> anyhow::Result<Result<R, E>> {
        // The idea here is to provide the transaction object to the callback, then
        // use the transaction to commit or rollback the changes according to the
        // callback outcome.
        //
        // To do that there is two approaches:
        // 1. Pass the ownership of the transaction to the callback, then have the
        //    callback returning the transaction back.
        // 2. Pass a reference to the transaction.
        //
        // The trick is this `PlatformCertificatesStorage::for_update` method is itself
        // used by another `CertificatesStorage::for_update`... which in turn is used
        // in a similar fashioned `CertificatesStore::for_write` !
        // So any of the two approaches would have a cascading effect.
        //
        // In theory the right thing to do would be to go with approach 2, but we would
        // end up with a hard to read soup of lifetimes (i.e. structs with fields like
        // `updater: &'a mut Updater<'b, 'c>`) that the Rust compiler fail to understand
        // when used in a `async` closure required `Send` (see
        // https://users.rust-lang.org/t/implementation-of-fnonce-is-not-general-enough/78006/3)
        //
        // So we rely on a hack to tweak the lifetime of the transaction to be `'static`,
        // which avoid the lifetime soup, make the compiler happy and save us from the
        // extensive modifications approach 1 would have required in the higher layers.

        let mut transaction = self.conn.begin().await?;
        // SAFETY: `transaction` is kept private and has a longer lifetime than `updater`,
        // so we can pretend its lifetime is static.
        let transaction_mut_ref: &'static mut _ = unsafe { std::mem::transmute(&mut transaction) };
        let updater = PlatformCertificatesStorageForUpdateGuard {
            transaction: transaction_mut_ref,
            _phantom: std::marker::PhantomData,
        };
        match cb(updater).await {
            Ok(result) => {
                transaction.commit().await?;
                Ok(Ok(result))
            }
            Err(err) => {
                transaction.rollback().await?;
                Ok(Err(err))
            }
        }
    }

    pub async fn get_last_timestamps(&mut self) -> anyhow::Result<PerTopicLastTimestamps> {
        // TODO: transaction shouldn't be needed here (but it's currently easier to implement this way)
        self.for_update(async |mut updater| updater.get_last_timestamps().await)
            .await?
    }

    pub async fn get_certificate_encrypted(
        &mut self,
        query: GetCertificateQuery<'_>,
        up_to: UpTo,
    ) -> Result<(DateTime, Vec<u8>), GetCertificateError> {
        // TODO: transaction shouldn't be needed here (but it's currently easier to implement this way)
        self.for_update(async |mut updater| updater.get_certificate_encrypted(query, up_to).await)
            .await?
    }

    pub async fn get_multiple_certificates_encrypted(
        &mut self,
        query: GetCertificateQuery<'_>,
        up_to: UpTo,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<(DateTime, Vec<u8>)>> {
        // TODO: transaction shouldn't be needed here (but it's currently easier to implement this way)
        self.for_update(async |mut updater| {
            updater
                .get_multiple_certificates_encrypted(query, up_to, offset, limit)
                .await
        })
        .await?
    }

    /// Only used for debugging tests
    #[cfg(any(test, feature = "expose-test-methods"))]
    #[allow(unused)]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        self.for_update(async |mut updater| updater.debug_dump().await)
            .await?
    }
}

#[cfg(test)]
#[path = "../../tests/unit/native/sqlite_db_creation.rs"]
mod test;
