// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use paste::paste;
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};

use libparsec_platform_async::Mutex as AsyncMutex;
use libparsec_types::prelude::*;

use super::db::{DatabaseError, LocalDatabase, VacuumMode};
use super::model::get_certificates_storage_db_relative_path;

// Values for `certificate_type` column in `certificates` table
// Note the fact their value is similar to the `type` field in certificates, this
// is purely for simplicity as the two are totally decorelated.
const USER_CERTIFICATE_TYPE: &str = "user_certificate";
const DEVICE_CERTIFICATE_TYPE: &str = "device_certificate";
const REVOKED_USER_CERTIFICATE_TYPE: &str = "revoked_user_certificate";
const REALM_ROLE_CERTIFICATE_TYPE: &str = "realm_role_certificate";
const SEQUESTER_AUTHORITY_CERTIFICATE_TYPE: &str = "sequester_authority_certificate";
const SEQUESTER_SERVICE_CERTIFICATE_TYPE: &str = "sequester_service_certificate";

macro_rules! mk_hint {
    (UserCertificate, certificate=$certif:ident) => {
        mk_hint!(UserCertificate, user_id = $certif.user_id)
    };
    (DeviceCertificate, certificate=$certif:ident) => {
        mk_hint!(DeviceCertificate, device_id = $certif.device_id)
    };
    (RevokedUserCertificate, certificate=$certif:ident) => {
        mk_hint!(RevokedUserCertificate, user_id = $certif.user_id)
    };
    (RealmRoleCertificate, certificate=$certif:ident) => {
        mk_hint!(
            RealmRoleCertificate,
            realm_id = $certif.realm_id,
            user_id = $certif.user_id
        )
    };
    (SequesterAuthorityCertificate, certificate=$certif:ident) => {
        mk_hint!(SequesterAuthorityCertificate)
    };
    (SequesterServiceCertificate, certificate=$certif:ident) => {
        mk_hint!(SequesterServiceCertificate, service_id = $certif.service_id)
    };

    (UserCertificate, user_id=$user_id:expr) => {
        format!("user_id:{}", $user_id)
    };
    (DeviceCertificate, device_id=$device_id:expr) => {
        format!(
            "user_id:{} device_name:{}",
            $device_id.user_id(),
            $device_id.device_name()
        )
    };
    (RevokedUserCertificate, user_id=$user_id:expr) => {
        format!("user_id:{}", $user_id)
    };
    (RealmRoleCertificate, realm_id=$realm_id:expr, user_id=$user_id:expr) => {
        format!("realm_id:{} user_id:{}", $realm_id.hex(), $user_id)
    };
    (SequesterAuthorityCertificate) => {
        // No need for hint given there is only (at most) a single sequester
        // authority certificate per organization
        "".to_owned()
    };
    (SequesterServiceCertificate, service_id=$service_id:expr) => {
        format!("service_id:{}", $service_id)
    };
}

#[derive(Debug, Default)]
struct CertificatesCache {
    users: HashMap<UserID, Arc<UserCertificate>>,
    devices: HashMap<DeviceID, Arc<DeviceCertificate>>,
    revoked_users: HashMap<UserID, Arc<RevokedUserCertificate>>,
    realm_roles: HashMap<RealmID, Vec<Arc<RealmRoleCertificate>>>,
    sequester_authority: Option<Arc<SequesterAuthorityCertificate>>,
    sequester_services: HashMap<SequesterServiceID, Arc<SequesterServiceCertificate>>,
}

impl CertificatesCache {
    fn add_new_user_certificate(&mut self, certif: Arc<UserCertificate>) {
        self.users.insert(certif.user_id.clone(), certif);
    }
    fn add_new_device_certificate(&mut self, certif: Arc<DeviceCertificate>) {
        self.devices.insert(certif.device_id.clone(), certif);
    }
    fn add_new_revoked_user_certificate(&mut self, certif: Arc<RevokedUserCertificate>) {
        self.revoked_users.insert(certif.user_id.clone(), certif);
    }
    fn add_new_realm_role_certificate(&mut self, certif: Arc<RealmRoleCertificate>) {
        match self.realm_roles.entry(certif.realm_id) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().push(certif);
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(vec![certif]);
            }
        }
    }
    fn add_new_sequester_authority_certificate(
        &mut self,
        certif: Arc<SequesterAuthorityCertificate>,
    ) {
        self.sequester_authority.replace(certif);
    }
    fn add_new_sequester_service_certificate(&mut self, certif: Arc<SequesterServiceCertificate>) {
        self.sequester_services.insert(certif.service_id, certif);
    }

    fn get_user_certificate(&self, user_id: &UserID) -> Option<&Arc<UserCertificate>> {
        self.users.get(user_id)
    }
    fn get_device_certificate(&self, device_id: &DeviceID) -> Option<&Arc<DeviceCertificate>> {
        self.devices.get(device_id)
    }
    fn get_revoked_user_certificate(
        &self,
        user_id: &UserID,
    ) -> Option<&Arc<RevokedUserCertificate>> {
        self.revoked_users.get(user_id)
    }
    fn get_realm_role_certificate(
        &self,
        realm_id: &RealmID,
        user_id: &UserID,
    ) -> Option<&Arc<RealmRoleCertificate>> {
        self.realm_roles
            .get(realm_id)
            .and_then(|roles| roles.iter().find(|role| role.user_id == *user_id))
    }
    fn get_sequester_authority_certificate(&self) -> Option<&Arc<SequesterAuthorityCertificate>> {
        self.sequester_authority.as_ref()
    }
    fn get_sequester_service_certificate(
        &self,
        service_id: &SequesterServiceID,
    ) -> Option<&Arc<SequesterServiceCertificate>> {
        self.sequester_services.get(service_id)
    }
}

#[derive(Debug)]
pub struct CertificatesStorage {
    pub device: Arc<LocalDevice>,
    db: LocalDatabase,
    cache: Mutex<CertificatesCache>,
    /// Lock to prevent concurrent update between SQLite DB and the cache
    lock_update: AsyncMutex<()>,
}

#[derive(Debug, thiserror::Error)]
pub enum StorageStartError {
    #[error("Cannot open database: {0}")]
    Open(DatabaseError),
    #[error("Cannot initialize database: {0}")]
    Initialization(DatabaseError),
}

#[derive(Debug, thiserror::Error)]
pub enum GetCertificateError {
    #[error("Cannot retreive the certificate from database: {0}")]
    Operation(DatabaseError),
}

enum AddCertificateOutcome {
    Inserted,
    AlreadyPresent {
        in_db_encrypted: Bytes,
        incoming: Bytes,
        hint: String,
    },
}

macro_rules! impl_add_certificate_error {
    ($certificate_type:ident) => {
        paste!{
            #[derive(Debug, thiserror::Error)]
            pub enum [< Add $certificate_type Error >] {
                #[error("We already know this certificate, but with a different content: `{ours:?}` vs `{incoming:?}`")]
                AlreadyKnownButMismatch {ours: Arc<$certificate_type>, incoming: Arc<$certificate_type>},
                #[error("Cannot store the certificate in database: {0}")]
                Operation(DatabaseError),
            }
        }
    };
}

impl_add_certificate_error!(UserCertificate);
impl_add_certificate_error!(DeviceCertificate);
impl_add_certificate_error!(RevokedUserCertificate);
impl_add_certificate_error!(RealmRoleCertificate);
impl_add_certificate_error!(SequesterAuthorityCertificate);
impl_add_certificate_error!(SequesterServiceCertificate);

macro_rules! load_certificate_from_local_storage {
    (SequesterServiceCertificate, $raw:expr) => {
        SequesterServiceCertificate::load(&$raw).map(Arc::new)
    };
    ($certificate_type:ident, $raw:expr) => {
        $certificate_type::unsecure_load($raw.into())
            .map(|to_validate| {
                to_validate.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage)
            })
            .map(Arc::new)
    };
}
macro_rules! impl_add_certificate_fn {
    ($certificate_type:ident) => {

    paste!{
        pub async fn [< add_new_ $certificate_type:snake >](&self, cooked: Arc<$certificate_type>, certificate: Bytes) -> Result<(), [< Add $certificate_type Error >] > {
            let hint = mk_hint!($certificate_type, certificate=cooked);

            let certificate_type_name = [< $certificate_type:snake:upper _TYPE >];
            type AddCertificateError = [< Add $certificate_type Error >];

            let outcome = self.add_new_certificate_in_db(certificate_type_name, hint, cooked.timestamp, certificate, false).await
            .map_err(|err| AddCertificateError::Operation(err))?;

            // In theory they should never be any conflict given the we should only receive
            // new certificates from the server.
            // However in case it occurs there is two possibilities:
            // - Our certificate and the incoming one are the same. In that case we can
            //   go idempotent and pretent everything went right
            // - The two certificates differs. This means something fishy is going on, so
            //   we return an error.
            //
            // Note this is not necessarily the proof of a malicious attack: the server may
            // have be subjected to rollback. In any way we must notify the user and let
            // him decide what should be done.
            //
            // Another possible reason for such conflict would be in case of bug on our side
            // when our user has his role switched from/to OUTSIDER given the certificates
            // are redacted for this role.
            if let AddCertificateOutcome::AlreadyPresent { in_db_encrypted, incoming, hint } = outcome {
                if let Ok(decrypted) = self.device.local_symkey.decrypt(&in_db_encrypted) {
                    if let Ok(ours) = load_certificate_from_local_storage!($certificate_type, decrypted) {
                        return Err(AddCertificateError::AlreadyKnownButMismatch { ours, incoming: cooked });
                    }
                }
                // Cannot load the existing certificate, so just overwrite it for self-healing
                log::warn!("{} ({}) appears to be corrupted, overwritting it", stringify!($certificate_type), &hint);
                let outcome = self.add_new_certificate_in_db(certificate_type_name, hint, cooked.timestamp, incoming, true).await
                .map_err(|err| AddCertificateError::Operation(err.into()))?;
                assert!(matches!(outcome, AddCertificateOutcome::Inserted));
            }

            // Finally update the cache
            let mut guard = self.cache.lock().expect("Mutex is poisoned");
            guard.[< add_new_ $certificate_type:snake >](cooked);

            Ok(())
        }
    }

    };
}

macro_rules! impl_get_certificate_fn {
    ($certificate_type:ident $(, $field_name:ident: $field_type:ty)* $(,)?) => {
    paste! {
        pub async fn  [< get_ $certificate_type:snake >](
            &self,
            $(
                $field_name: &$field_type
            ),*
        ) -> Result<Option<Arc<$certificate_type>>, GetCertificateError> {
            {
                let guard = self.cache.lock().expect("Mutex is poisoned");
                if let Some(found) = guard.[< get_ $certificate_type:snake >]($($field_name),*) {
                    return Ok(Some(found.clone()));
                }
            }

            // Certif not in cache, look into the db...
            let hint = mk_hint!($certificate_type $(, $field_name=$field_name)*);
            let encrypted = self.get_certificate_from_db(USER_CERTIFICATE_TYPE, hint).await.map_err(|err| GetCertificateError::Operation(err))?;

            if let Ok(decrypted) = self.device.local_symkey.decrypt(&encrypted) {
                if let Ok(cooked) = load_certificate_from_local_storage!($certificate_type, decrypted) {

                    // Populate the cache before returing
                    {
                        let _update_guard = self.lock_update.lock().await;
                        let mut guard = self.cache.lock().expect("Mutex is poisoned");
                        guard.[< add_new_ $certificate_type:snake >](cooked.clone());
                    }

                    return Ok(Some(cooked));
                }
            }

            // The certificate in database appear to be corrupted... just pretend it doesn't exist
            let hint = mk_hint!($certificate_type $(, $field_name=$field_name)*);
            log::warn!("{} ({}) appears to be corrupted, ignoring it", stringify!($certificate_type), &hint);
            Ok(None)
        }
    }
    };
}

impl CertificatesStorage {
    pub async fn start(
        data_base_dir: &Path,
        device: Arc<LocalDevice>,
    ) -> Result<Self, StorageStartError> {
        // 1) Open the database

        let db_relative_path = get_certificates_storage_db_relative_path(&device);
        let db = LocalDatabase::from_path(data_base_dir, &db_relative_path, VacuumMode::default())
            .await
            .map_err(StorageStartError::Open)?;

        // 2) Initialize the database (if needed)

        super::model::initialize_model_if_needed(&db)
            .await
            .map_err(StorageStartError::Initialization)?;

        // 3) All done !

        let user_storage = Self {
            device,
            db,
            cache: Mutex::new(CertificatesCache::default()),
            lock_update: AsyncMutex::new(()),
        };
        Ok(user_storage)
    }

    pub async fn stop(&self) {
        self.db.close().await
    }

    pub async fn get_last_certificate_timestamp(
        &self,
    ) -> Result<Option<DateTime>, GetCertificateError> {
        self.db
            .exec(move |conn| {
                conn.immediate_transaction(|conn| {
                    let query = {
                        use super::model::certificates::dsl::*;
                        certificates
                            .select(certificate_timestamp)
                            .order_by(certificate_timestamp.desc())
                    };
                    query
                        .first::<super::db::DateTime>(conn)
                        .optional()
                        .map(|maybe_dt| maybe_dt.map(|dt| dt.into()))
                })
            })
            .await
            .map_err(GetCertificateError::Operation)
    }

    impl_add_certificate_fn!(UserCertificate);
    impl_add_certificate_fn!(DeviceCertificate);
    impl_add_certificate_fn!(RevokedUserCertificate);
    impl_add_certificate_fn!(RealmRoleCertificate);
    impl_add_certificate_fn!(SequesterAuthorityCertificate);
    impl_add_certificate_fn!(SequesterServiceCertificate);

    impl_get_certificate_fn!(UserCertificate, user_id: UserID);
    impl_get_certificate_fn!(DeviceCertificate, device_id: DeviceID);
    impl_get_certificate_fn!(RevokedUserCertificate, user_id: UserID);
    impl_get_certificate_fn!(RealmRoleCertificate, realm_id: RealmID, user_id: UserID);
    impl_get_certificate_fn!(SequesterAuthorityCertificate);
    impl_get_certificate_fn!(SequesterServiceCertificate, service_id: SequesterServiceID);

    async fn get_certificate_from_db(
        &self,
        certificate_type: &'static str,
        hint: String,
    ) -> Result<Vec<u8>, DatabaseError> {
        self.db
            .exec(move |conn| {
                conn.immediate_transaction(|conn| {
                    let query = {
                        use super::model::certificates;
                        certificates::table
                            .select(certificates::certificate)
                            .filter(certificates::certificate_type.eq(certificate_type))
                            .filter(certificates::hint.eq(&hint))
                    };
                    query.first::<Vec<u8>>(conn)
                })
            })
            .await
    }

    /// We never remove nor replace certificates, as a certifacet is immutable (a new
    /// certificate should be issued to represent a modification, e.g. for realm role)
    /// So if the server start sending certificates that doesn't match with the one we
    /// currently have stored, it means somebody is acting fishy !
    async fn add_new_certificate_in_db(
        &self,
        certificate_type: &'static str,
        hint: String,
        certificate_timestamp: DateTime,
        certificate: Bytes,
        force: bool,
    ) -> Result<AddCertificateOutcome, DatabaseError> {
        let encrypted = self.device.local_symkey.encrypt(&certificate);
        let _update_guard = self.lock_update.lock().await;

        // It's unlikely the certificate is already present, so we don't check for it
        // existance in cache and rely instead on the unique violation error from SQlite
        self.db
            .exec(move |conn| {
                conn.immediate_transaction(|conn| {
                    let new_certificate = super::model::NewCertificate {
                        certificate: &encrypted,
                        certificate_type,
                        certificate_timestamp: certificate_timestamp.into(),
                        hint: &hint,
                    };

                    if force {
                        let query = {
                            use super::model::certificates::dsl::*;
                            use diesel::upsert::excluded;
                            diesel::insert_into(certificates)
                                .values(new_certificate)
                                .on_conflict((certificate_type, hint))
                                .do_update()
                                .set((
                                    certificate.eq(excluded(certificate)),
                                    certificate_timestamp.eq(excluded(certificate_timestamp)),
                                ))
                        };
                        query.execute(conn).map(|_| AddCertificateOutcome::Inserted)
                    } else {
                        let query = {
                            use super::model::certificates::dsl::*;
                            diesel::insert_into(certificates).values(new_certificate)
                        };
                        let insert_res = query.execute(conn);

                        if let Err(diesel::result::Error::DatabaseError(
                            diesel::result::DatabaseErrorKind::UniqueViolation,
                            _,
                        )) = insert_res
                        {
                            // Conflict !
                            let query = {
                                use super::model::certificates;
                                certificates::table
                                    .select(certificates::certificate)
                                    .filter(certificates::certificate_type.eq(certificate_type))
                                    .filter(certificates::hint.eq(&hint))
                            };
                            let select_res = query.first::<Vec<u8>>(conn);
                            match select_res {
                                Ok(in_db_encrypted) => Ok(AddCertificateOutcome::AlreadyPresent {
                                    in_db_encrypted: in_db_encrypted.into(),
                                    incoming: certificate,
                                    hint,
                                }),
                                Err(err) => Err(err),
                            }
                        } else {
                            insert_res.map(|_| AddCertificateOutcome::Inserted)
                        }
                    }
                })
            })
            .await
    }
}
