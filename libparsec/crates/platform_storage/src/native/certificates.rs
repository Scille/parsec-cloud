// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

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

// Values for `certificate_type` column in `certificates` table
// Note the fact their value is similar to the `type` field in certificates, this
// is purely for simplicity as the two are totally decorelated.
const USER_CERTIFICATE_TYPE: &str = "user_certificate";
const DEVICE_CERTIFICATE_TYPE: &str = "device_certificate";
const REVOKED_USER_CERTIFICATE_TYPE: &str = "revoked_user_certificate";
const USER_UPDATE_CERTIFICATE_TYPE: &str = "user_update_certificate";
const REALM_ROLE_CERTIFICATE_TYPE: &str = "realm_role_certificate";
const SEQUESTER_AUTHORITY_CERTIFICATE_TYPE: &str = "sequester_authority_certificate";
const SEQUESTER_SERVICE_CERTIFICATE_TYPE: &str = "sequester_service_certificate";

#[derive(Debug)]
pub struct CertificatesStorage {
    db: LocalDatabase,
}

fn user_certificate_filters(user_id: UserID) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = Some(user_id.into());
    let filter2 = None;
    (USER_CERTIFICATE_TYPE, filter1, filter2)
}

fn revoked_user_certificate_filters(
    user_id: UserID,
) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = Some(user_id.into());
    let filter2 = None;
    (REVOKED_USER_CERTIFICATE_TYPE, filter1, filter2)
}

fn user_update_certificate_filters(
    user_id: UserID,
) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = Some(user_id.into());
    let filter2 = None;
    (USER_UPDATE_CERTIFICATE_TYPE, filter1, filter2)
}

fn device_certificate_filters(
    device_id: DeviceID,
) -> (&'static str, Option<String>, Option<String>) {
    let (user_id, device_name) = device_id.into();
    // DeviceName is already unique enough, so we provide it as first filter
    // to speed up database lookup
    let filter1 = Some(device_name.into());
    let filter2 = Some(user_id.into());
    (DEVICE_CERTIFICATE_TYPE, filter1, filter2)
}

fn realm_role_certificate_filters(
    realm_id: RealmID,
    user_id: UserID,
) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = Some(realm_id.hex());
    let filter2 = Some(user_id.into());
    (REALM_ROLE_CERTIFICATE_TYPE, filter1, filter2)
}

/// Get all realm role certificates for a given realm
fn get_realm_certificates_filters(
    realm_id: RealmID,
) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = Some(realm_id.hex());
    let filter2 = None;
    (REALM_ROLE_CERTIFICATE_TYPE, filter1, filter2)
}

/// Get all realm role certificates for a given user
fn get_user_realms_certificates_filters(
    user_id: UserID,
) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = None;
    let filter2 = Some(user_id.into());
    (REALM_ROLE_CERTIFICATE_TYPE, filter1, filter2)
}

fn sequester_authority_certificate_filters() -> (&'static str, Option<String>, Option<String>) {
    (SEQUESTER_AUTHORITY_CERTIFICATE_TYPE, None, None)
}

fn sequester_service_certificate_filters(
    service_id: SequesterServiceID,
) -> (&'static str, Option<String>, Option<String>) {
    let filter1 = Some(service_id.hex());
    let filter2 = None;
    (SEQUESTER_SERVICE_CERTIFICATE_TYPE, filter1, filter2)
}

// Get all sequester service certificates
fn get_sequester_service_certificates_filters() -> (&'static str, Option<String>, Option<String>) {
    let filter1 = None;
    let filter2 = None;
    (SEQUESTER_SERVICE_CERTIFICATE_TYPE, filter1, filter2)
}

impl CertificatesStorage {
    pub async fn start(data_base_dir: &Path, device: &LocalDevice) -> anyhow::Result<Self> {
        // `maybe_populate_certificate_storage` needs to start a `CertificatesStorage`,
        // leading to a recursive call which is not support for async functions.
        // Hence `no_populate_start` which breaks the recursion.
        //
        // Also note we don't try to return the `CertificatesStorage` that has been
        // use during the populate as it would change the internal state of the
        // storage (typically caches) depending of if populate has been needed or not.

        #[cfg(feature = "test-with-testbed")]
        crate::testbed::maybe_populate_certificate_storage(data_base_dir, device).await;

        Self::no_populate_start(data_base_dir, device).await
    }

    pub(crate) async fn no_populate_start(
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

    pub async fn add_user_certificate(
        &self,
        index: IndexInt,
        timestamp: DateTime,
        user_id: UserID,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        let (ty, filter1, filter2) = user_certificate_filters(user_id);
        self.add_certificate(ty, index, timestamp, filter1, filter2, encrypted)
            .await
    }

    pub async fn add_revoked_user_certificate(
        &self,
        index: IndexInt,
        timestamp: DateTime,
        user_id: UserID,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        let (ty, filter1, filter2) = revoked_user_certificate_filters(user_id);
        self.add_certificate(ty, index, timestamp, filter1, filter2, encrypted)
            .await
    }

    pub async fn add_user_update_certificate(
        &self,
        index: IndexInt,
        timestamp: DateTime,
        user_id: UserID,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        let (ty, filter1, filter2) = user_update_certificate_filters(user_id);
        self.add_certificate(ty, index, timestamp, filter1, filter2, encrypted)
            .await
    }

    pub async fn add_device_certificate(
        &self,
        index: IndexInt,
        timestamp: DateTime,
        device_id: DeviceID,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        let (ty, filter1, filter2) = device_certificate_filters(device_id);
        self.add_certificate(ty, index, timestamp, filter1, filter2, encrypted)
            .await
    }

    pub async fn add_realm_role_certificate(
        &self,
        index: IndexInt,
        timestamp: DateTime,
        realm_id: RealmID,
        user_id: UserID,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        let (ty, filter1, filter2) = realm_role_certificate_filters(realm_id, user_id);
        self.add_certificate(ty, index, timestamp, filter1, filter2, encrypted)
            .await
    }

    pub async fn add_sequester_authority_certificate(
        &self,
        index: IndexInt,
        timestamp: DateTime,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        let (ty, filter1, filter2) = sequester_authority_certificate_filters();
        self.add_certificate(ty, index, timestamp, filter1, filter2, encrypted)
            .await
    }

    pub async fn add_sequester_service_certificate(
        &self,
        index: IndexInt,
        timestamp: DateTime,
        service_id: SequesterServiceID,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        let (ty, filter1, filter2) = sequester_service_certificate_filters(service_id);
        self.add_certificate(ty, index, timestamp, filter1, filter2, encrypted)
            .await
    }

    pub async fn get_user_certificate(
        &self,
        user_id: UserID,
    ) -> anyhow::Result<Option<(IndexInt, Vec<u8>)>> {
        let (ty, filter1, filter2) = user_certificate_filters(user_id);
        self.get_single_certificate(ty, filter1, filter2).await
    }

    pub async fn get_revoked_user_certificate(
        &self,
        user_id: UserID,
    ) -> anyhow::Result<Option<(IndexInt, Vec<u8>)>> {
        let (ty, filter1, filter2) = revoked_user_certificate_filters(user_id);
        self.get_single_certificate(ty, filter1, filter2).await
    }

    pub async fn get_user_update_certificates(
        &self,
        user_id: UserID,
    ) -> anyhow::Result<Vec<(IndexInt, Vec<u8>)>> {
        let (ty, filter1, filter2) = user_update_certificate_filters(user_id);
        self.get_multiple_certificates(ty, filter1, filter2).await
    }

    pub async fn get_device_certificate(
        &self,
        device_id: DeviceID,
    ) -> anyhow::Result<Option<(IndexInt, Vec<u8>)>> {
        let (ty, filter1, filter2) = device_certificate_filters(device_id);
        self.get_single_certificate(ty, filter1, filter2).await
    }

    // Get all realm role certificates for a given realm
    pub async fn get_realm_certificates(
        &self,
        realm_id: RealmID,
    ) -> anyhow::Result<Vec<(IndexInt, Vec<u8>)>> {
        let (ty, filter1, filter2) = get_realm_certificates_filters(realm_id);
        self.get_multiple_certificates(ty, filter1, filter2).await
    }

    // Get all realm role certificates for a given user
    pub async fn get_user_realms_certificates(
        &self,
        user_id: UserID,
    ) -> anyhow::Result<Vec<(IndexInt, Vec<u8>)>> {
        let (ty, filter1, filter2) = get_user_realms_certificates_filters(user_id);
        self.get_multiple_certificates(ty, filter1, filter2).await
    }

    pub async fn get_sequester_authority_certificate(
        &self,
    ) -> anyhow::Result<Option<(IndexInt, Vec<u8>)>> {
        let (ty, filter1, filter2) = sequester_authority_certificate_filters();
        self.get_single_certificate(ty, filter1, filter2).await
    }

    pub async fn get_sequester_service_certificates(
        &self,
    ) -> anyhow::Result<Vec<(IndexInt, Vec<u8>)>> {
        let (ty, filter1, filter2) = get_sequester_service_certificates_filters();
        self.get_multiple_certificates(ty, filter1, filter2).await
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

    async fn add_certificate(
        &self,
        certificate_type: &'static str,
        index: IndexInt,
        timestamp: DateTime,
        filter1: Option<String>,
        filter2: Option<String>,
        encrypted: Vec<u8>,
    ) -> anyhow::Result<()> {
        let index_as_i64 = i64::try_from(index)?;

        self.db
            .exec(move |conn| {
                let filter1 = filter1.as_deref();
                let filter2 = filter2.as_deref();

                // IMMEDIATE transaction means we start right away a write transaction
                // (instead of default DEFERRED mode of which waits until the first database
                // access and determine read/write transaction depending on the sql statement)
                conn.immediate_transaction(|conn| {
                    let new_certificate = super::model::NewCertificate {
                        certificate_index: index_as_i64,
                        certificate: &encrypted,
                        certificate_timestamp: timestamp.into(),
                        certificate_type,
                        filter1,
                        filter2,
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

    async fn get_single_certificate(
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

    async fn get_multiple_certificates(
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
