// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Certificate storage relies on the upper layer to do the actual certification
// validation work and takes care of handling concurrency issues.
// Hence no unique violation should occur under normal circumstances here.
// However, as last failsafe, the certificate storage ensures an index cannot
// be inserted multiple times, as this is a simple check (using an unique index).

use diesel::{BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use std::path::Path;

use libparsec_types::prelude::*;

use super::db::{LocalDatabase, VacuumMode};
use super::model::get_certificates_storage_db_relative_path;
use crate::certificates::{
    AddCertificateData, GetAnyCertificateData, GetCertificateError, GetCertificateQuery,
    GetTimestampBoundsError, UpTo,
};

#[derive(Debug)]
pub(crate) struct PlatformCertificatesStorage {
    db: LocalDatabase,
}

fn up_to_into_option_i64(up_to: UpTo) -> anyhow::Result<Option<i64>> {
    match up_to {
        UpTo::Current => Ok(None),
        UpTo::Index(index) => {
            let index_as_i64 = i64::try_from(index)?;
            Ok(Some(index_as_i64))
        }
    }
}

fn i64_into_index_int(x: i64) -> Result<IndexInt, diesel::result::Error> {
    IndexInt::try_from(x).map_err(|e| {
        diesel::result::Error::SerializationError(
            anyhow::anyhow!("invalid index from DB: {}", e).into(),
        )
    })
}

impl PlatformCertificatesStorage {
    pub async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
    ) -> anyhow::Result<Self> {
        // 1) Open the database

        let db_relative_path = get_certificates_storage_db_relative_path(device);
        let db = LocalDatabase::from_path(data_base_dir, &db_relative_path, VacuumMode::default())
            .await?;

        // 2) Initialize the database (if needed)

        super::model::initialize_model_if_needed(&db).await?;

        // 3) All done !

        let storage = Self { db };
        Ok(storage)
    }

    pub async fn stop(&self) {
        self.db.close().await
    }

    #[cfg(test)]
    pub async fn test_get_raw_certificate(
        &self,
        index: IndexInt,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        let index_as_i64 = i64::try_from(index)?;
        let maybe = self
            .db
            .exec(move |conn| {
                let query = {
                    use super::model::certificates;
                    certificates::table
                        .select(certificates::certificate)
                        .filter(certificates::certificate_index.eq(index_as_i64))
                };
                query.first::<Vec<u8>>(conn).optional()
            })
            .await?;

        Ok(maybe)
    }

    /// Return the timestamp of creation of the considered certificate index
    /// and (if any) of the certificate index following it.
    /// If this certificate index doesn't exist yet, `(None, None)` is returned.
    pub async fn get_timestamp_bounds(
        &self,
        index: IndexInt,
    ) -> Result<(DateTime, Option<DateTime>), GetTimestampBoundsError> {
        let index_as_i64 =
            i64::try_from(index).map_err(|err| GetTimestampBoundsError::Internal(err.into()))?;

        let mut items = self
            .db
            .exec(move |conn| {
                let query = {
                    use super::model::certificates;
                    certificates::table
                        .select(certificates::certificate_timestamp)
                        .filter(
                            certificates::certificate_index
                                .eq(index_as_i64)
                                .or(certificates::certificate_index.eq(index_as_i64 + 1)),
                        )
                };
                query.limit(2).get_results::<super::db::DateTime>(conn)
            })
            .await
            .map_err(|err| GetTimestampBoundsError::Internal(err.into()))?
            .into_iter();

        match (items.next(), items.next()) {
            (Some(start), Some(end)) => Ok((DateTime::from(start), Some(DateTime::from(end)))),
            // Certificate index is the last one, hence it has no upper bound
            (Some(start), None) => Ok((DateTime::from(start), None)),
            // Certificate index too high
            (None, _) => Err(GetTimestampBoundsError::NonExisting),
        }
    }

    pub async fn get_last_index(&self) -> anyhow::Result<Option<(IndexInt, DateTime)>> {
        self.db
            .exec(move |conn| {
                let query = {
                    use super::model::certificates;
                    certificates::table
                        .select((
                            certificates::certificate_index,
                            certificates::certificate_timestamp,
                        ))
                        .order(certificates::certificate_index.desc())
                };
                query
                    .first::<(i64, super::db::DateTime)>(conn)
                    .optional()
                    .and_then(|maybe_found| match maybe_found {
                        None => Ok(None),
                        Some((last_index, last_timestamp)) => {
                            let last_index = i64_into_index_int(last_index)?;
                            let last_timestamp = DateTime::from(last_timestamp);
                            Ok(Some((last_index, last_timestamp)))
                        }
                    })
            })
            .await
            .map_err(|err| err.into())
    }

    /// Remove all certificates from the database
    /// There is no data loss from this as certificates can be re-obtained from
    /// the server, however it is only needed when switching from/to redacted
    /// certificates
    pub async fn forget_all_certificates(&self) -> anyhow::Result<()> {
        self.db
            .exec(move |conn| {
                // IMMEDIATE transaction means we start right away a write transaction
                // (instead of default DEFERRED mode of which waits until the first database
                // access and determine read/write transaction depending on the sql statement)
                conn.immediate_transaction(|conn| {
                    let query = {
                        use super::model::certificates;
                        diesel::delete(certificates::table)
                    };
                    query.execute(conn).map(|_| ())
                })
            })
            .await
            .map_err(|err| err.into())
    }

    pub async fn add_certificate(
        &self,
        index: IndexInt,
        data: AddCertificateData,
    ) -> anyhow::Result<()> {
        let index_as_i64 = i64::try_from(index)?;

        self.db
            .exec(move |conn| {
                // IMMEDIATE transaction means we start right away a write transaction
                // (instead of default DEFERRED mode of which waits until the first database
                // access and determine read/write transaction depending on the sql statement)
                conn.immediate_transaction(|conn| {
                    let index_as_i64 = index_as_i64;
                    let new_certificate = super::model::NewCertificate {
                        certificate_index: index_as_i64,
                        certificate: &data.encrypted,
                        certificate_timestamp: data.timestamp.into(),
                        certificate_type: data.certificate_type,
                        filter1: data.filter1.as_deref(),
                        filter2: data.filter2.as_deref(),
                    };

                    let query = {
                        use super::model::certificates::dsl::*;
                        diesel::insert_into(certificates).values(new_certificate)
                    };
                    query.execute(conn).map(|_| ())
                })
            })
            .await
            .map_err(|err| err.into())
    }

    pub async fn get_any_certificate_encrypted(
        &self,
        index: IndexInt,
    ) -> Result<GetAnyCertificateData, GetCertificateError> {
        let index_as_i64 =
            i64::try_from(index).map_err(|err| GetCertificateError::Internal(err.into()))?;

        self.db
            .exec(move |conn| {
                use super::model::certificates;
                certificates::table
                    .select((
                        certificates::certificate_type,
                        certificates::certificate_index,
                        certificates::certificate,
                    ))
                    .filter(certificates::certificate_index.eq(index_as_i64))
                    .first::<(String, i64, Vec<u8>)>(conn)
                    .optional()
                    .and_then(|maybe_found| match maybe_found {
                        None => Ok(None),
                        Some((certificate_type, index, encrypted)) => {
                            let index = i64_into_index_int(index)?;
                            Ok(Some(GetAnyCertificateData {
                                certificate_type,
                                index,
                                encrypted,
                            }))
                        }
                    })
            })
            .await
            .map_err(|e| GetCertificateError::Internal(e.into()))
            .and_then(|maybe_found| match maybe_found {
                None => Err(GetCertificateError::NonExisting),
                Some(found) => Ok(found),
            })
    }

    /// Not if multiple results are possible, the highest index is kept (i.e. latest certificate)
    pub async fn get_certificate_encrypted(
        &self,
        query: GetCertificateQuery,
        up_to: UpTo,
    ) -> Result<(IndexInt, Vec<u8>), GetCertificateError> {
        // Handling `up_to` is a bit tricky:
        // 1) we want the last result up to the given index (included)
        // 2) BUT ! if 1) fails we want we want the first result if any
        //    in order to correctly return NonExisting vs ExistButTooRecent
        //
        // So to do that we act in three steps:
        //  1) do the SQL query with a filter on `up_to`
        //  2) if the previous query returned nothing, do another SQL query with `up_to`
        //  3) compare the index of the result with the on provided by `up_to`: if it's
        //     too high it's a ExistButTooRecent error otherwise it's a valid result

        let up_to_i64 = up_to_into_option_i64(up_to)?;
        let maybe_found = self
            .db
            .exec(move |conn| {
                let filter1 = query.filter1.as_deref();
                let filter2 = query.filter2.as_deref();

                let build_sql_query = |up_to_i64| {
                    use super::model::certificates;
                    let mut sql_query = certificates::table
                        .select((
                            certificates::certificate_index,
                            certificates::certificate_timestamp,
                            certificates::certificate,
                        ))
                        .filter(certificates::certificate_type.eq(query.certificate_type))
                        .order(certificates::certificate_index.desc())
                        .into_boxed();

                    if filter1.is_some() {
                        sql_query = sql_query.filter(certificates::filter1.eq(filter1))
                    }

                    if filter2.is_some() {
                        sql_query = sql_query.filter(certificates::filter2.eq(filter2))
                    }

                    if let Some(up_to_index) = up_to_i64 {
                        sql_query =
                            sql_query.filter(certificates::certificate_index.le(up_to_index))
                    }

                    sql_query
                };

                // Step 1: do the regular SQL query
                let sql_query = build_sql_query(up_to_i64);
                sql_query
                    .first::<(i64, super::db::DateTime, Vec<u8>)>(conn)
                    .optional()
                    .and_then(|maybe_found| match (up_to_i64, maybe_found) {
                        // Step 2: do another SQL query without `up_to` to look for an
                        // "existing but too recent" item
                        (Some(_), None) => {
                            let sql_query = build_sql_query(None);
                            sql_query
                                .first::<(i64, super::db::DateTime, Vec<u8>)>(conn)
                                .optional()
                        }
                        (_, maybe_found) => Ok(maybe_found),
                    })
                    .and_then(|maybe_found| match maybe_found {
                        None => Ok(None),
                        Some((index, timestamp, encrypted)) => {
                            let index = i64_into_index_int(index)?;
                            Ok(Some((index, timestamp, encrypted)))
                        }
                    })
            })
            .await
            .map_err(|e| GetCertificateError::Internal(e.into()))?;

        // Step 3: determine if the result is an actual success or a ExistButTooRecent error
        match (up_to, maybe_found) {
            (_, None) => Err(GetCertificateError::NonExisting),
            (UpTo::Index(up_to_index), Some((index, timestamp, _))) if index > up_to_index => {
                Err(GetCertificateError::ExistButTooRecent {
                    certificate_index: index,
                    certificate_timestamp: DateTime::from(timestamp),
                })
            }
            (_, Some((index, _, encrypted))) => Ok((index, encrypted)),
        }
    }

    /// Certificates are returned ordered by index in increasing order
    pub async fn get_multiple_certificates_encrypted(
        &self,
        query: GetCertificateQuery,
        up_to: UpTo,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> anyhow::Result<Vec<(IndexInt, Vec<u8>)>> {
        let offset = offset.map(i64::try_from).transpose()?;
        let limit = limit.map(i64::try_from).transpose()?;
        let up_to = up_to_into_option_i64(up_to)?;
        let items = self
            .db
            .exec(move |conn| {
                let filter1 = query.filter1.as_deref();
                let filter2 = query.filter2.as_deref();
                let query = {
                    use super::model::certificates;
                    let mut query = certificates::table
                        .select((certificates::certificate_index, certificates::certificate))
                        .filter(certificates::certificate_type.eq(query.certificate_type))
                        .order(certificates::certificate_index.asc())
                        .into_boxed();

                    if filter1.is_some() {
                        query = query.filter(certificates::filter1.eq(filter1))
                    }

                    if filter2.is_some() {
                        query = query.filter(certificates::filter2.eq(filter2))
                    }

                    if let Some(offset) = offset {
                        query = query.offset(offset);
                    }

                    if let Some(limit) = limit {
                        query = query.limit(limit);
                    }

                    if let Some(index) = up_to {
                        query = query.filter(certificates::certificate_index.le(index));
                    }

                    query
                };
                query.get_results::<(i64, Vec<u8>)>(conn)
            })
            .await?;

        items
            .into_iter()
            .map(|(index, certif)| -> anyhow::Result<_> {
                let index = i64_into_index_int(index)?;
                Ok((index, certif))
            })
            .collect()
    }
}
