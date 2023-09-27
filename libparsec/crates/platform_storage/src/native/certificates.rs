// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Certificate storage relies on the upper layer to do the actual certification
// validation work and takes care of handling concurrency issues.
// Hence no unique violation should occur under normal circumstances here.
// However, as last failsafe, the certificate storage ensures an index cannot
// be inserted multiple times, as this is a simple check (using an unique index).

use diesel::{BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use std::path::Path;

use libparsec_types::prelude::*;

use crate::certificates::AddCertificateData;
use super::db::{LocalDatabase, VacuumMode};
use super::model::get_certificates_storage_db_relative_path;

#[derive(Debug)]
pub(crate) struct PlatformCertificatesStorage {
    db: LocalDatabase,
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

    pub async fn get_certificate(&self, index: IndexInt) -> anyhow::Result<Option<Vec<u8>>> {
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

    /// Returns the timestamp of creation of the considered certificate index
    /// and (if any) of the certificate index following it.
    /// If this certificate index doesn't exist yet, `(None, None)` is returned.
    pub async fn get_timestamp_bounds(
        &self,
        index: IndexInt,
    ) -> anyhow::Result<(Option<DateTime>, Option<DateTime>)> {
        let index_as_i64 = i64::try_from(index)?;

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
            .await?
            .into_iter();

        match (items.next(), items.next()) {
            (Some(start), Some(end)) => {
                Ok((Some(DateTime::from(start)), Some(DateTime::from(end))))
            }
            // Certificate index is the last one, hence it has no upper bound
            (Some(start), None) => Ok((Some(DateTime::from(start)), None)),
            // Certificate index too high
            (None, _) => Ok((None, None)),
        }
    }

    pub async fn get_last_index(&self) -> anyhow::Result<Option<(IndexInt, DateTime)>> {
        let maybe = self
            .db
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
                query.first::<(i64, super::db::DateTime)>(conn).optional()
            })
            .await?;

        match maybe {
            None => Ok(None),
            Some((last_index, last_timestamp)) => {
                let last_index = IndexInt::try_from(last_index)?;
                let last_timestamp = DateTime::from(last_timestamp);
                Ok(Some((last_index, last_timestamp)))
            }
        }
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

    pub async fn add_single_certificate(
        &self,
        data: AddCertificateData,
    ) -> anyhow::Result<()> {
        self.add_multiple_certificates(vec![data]).await
    }

    pub async fn add_multiple_certificates(
        &self,
        items: Vec<AddCertificateData>,
    ) -> anyhow::Result<()> {
        self.db
            .exec(move |conn| {
                // IMMEDIATE transaction means we start right away a write transaction
                // (instead of default DEFERRED mode of which waits until the first database
                // access and determine read/write transaction depending on the sql statement)
                conn.immediate_transaction(|conn| {
                    for data in items {
                        let index_as_i64 = i64::try_from(data.index).map_err(
                            |e| diesel::result::Error::SerializationError(anyhow::anyhow!("index overflow: {}", e).into())
                        )?;
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
                        query.execute(conn).map(|_| ())?
                    }

                    Ok(())
                })
            })
            .await
            .map_err(|err| err.into())
    }

    pub async fn get_single_certificate(
        &self,
        certificate_type: &'static str,
        filter1: Option<String>,
        filter2: Option<String>,
    ) -> anyhow::Result<Option<(IndexInt, Vec<u8>)>> {
        let maybe = self
            .db
            .exec(move |conn| {
                let filter1 = filter1.as_deref();
                let filter2 = filter2.as_deref();
                let query = {
                    use super::model::certificates;
                    let mut query = certificates::table
                        .select((certificates::certificate_index, certificates::certificate))
                        .filter(certificates::certificate_type.eq(certificate_type))
                        .into_boxed();

                    if filter1.is_some() {
                        query = query.filter(certificates::filter1.eq(filter1))
                    }

                    if filter2.is_some() {
                        query = query.filter(certificates::filter2.eq(filter2))
                    }

                    query
                };
                query.first::<(i64, Vec<u8>)>(conn).optional()
            })
            .await?;

        match maybe {
            Some((index, certif)) => {
                let index = IndexInt::try_from(index)?;
                Ok(Some((index, certif)))
            }
            None => Ok(None),
        }
    }

    pub async fn get_multiple_certificates(
        &self,
        certificate_type: &'static str,
        filter1: Option<String>,
        filter2: Option<String>,
    ) -> anyhow::Result<Vec<(IndexInt, Vec<u8>)>> {
        let items = self
            .db
            .exec(move |conn| {
                let filter1 = filter1.as_deref();
                let filter2 = filter2.as_deref();
                let query = {
                    use super::model::certificates;
                    let mut query = certificates::table
                        .select((certificates::certificate_index, certificates::certificate))
                        .filter(certificates::certificate_type.eq(certificate_type))
                        .into_boxed();

                    if filter1.is_some() {
                        query = query.filter(certificates::filter1.eq(filter1))
                    }

                    if filter2.is_some() {
                        query = query.filter(certificates::filter2.eq(filter2))
                    }

                    query
                };
                query.get_results::<(i64, Vec<u8>)>(conn)
            })
            .await?;

        items
            .into_iter()
            .map(|(index, certif)| -> anyhow::Result<_> {
                let index = IndexInt::try_from(index)?;
                Ok((index, certif))
            })
            .collect()
    }
}
