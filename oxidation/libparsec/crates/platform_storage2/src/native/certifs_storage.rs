// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::RunQueryDsl;
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};

use libparsec_platform_async::Mutex as AsyncMutex;
use libparsec_types::prelude::*;

use super::db::{DatabaseResult, LocalDatabase, VacuumMode};
use super::model::get_certificates_storage_db_relative_path;

pub type DynError = Box<dyn std::error::Error + Send + Sync>;

// Values for `certificate_type` column in `certificates` table
// Note the fact their value is similar to the `type` field in certificates, this
// is purely for simplicity as the two are totally decorelated.
const USER_CERTIFICATE_TYPE: &str = "user_certificate";
const DEVICE_CERTIFICATE_TYPE: &str = "device_certificate";
const REVOKED_USER_CERTIFICATE_TYPE: &str = "revoked_user_certificate";
const REALM_ROLE_CERTIFICATE_TYPE: &str = "realm_role_certificate";
const SEQUESTER_AUTHORITY_CERTIFICATE_TYPE: &str = "sequester_authority_certificate";
const SEQUESTER_SERVICE_CERTIFICATE_TYPE: &str = "sequester_service_certificate";

#[derive(Debug, Default)]
struct CertifsCache {
    users: HashMap<UserID, UserCertificate>,
    devices: HashMap<DeviceID, DeviceCertificate>,
    revoked_users: HashMap<UserID, RevokedUserCertificate>,
    realm_roles: HashMap<RealmID, RealmRoleCertificate>,
    sequester_authority: Option<SequesterAuthorityCertificate>,
    sequester_services: HashMap<SequesterServiceID, SequesterServiceCertificate>,
}

#[derive(Debug)]
pub struct CertifsStorage {
    pub device: Arc<LocalDevice>,
    db: LocalDatabase,
    cache: Mutex<CertifsCache>,
    /// Lock to prevent concurrent update between SQLite DB and the cache
    lock_update: AsyncMutex<()>,
}

#[derive(Debug, thiserror::Error)]
pub enum CertifsStorageStartError {
    #[error("Cannot open database: {0}")]
    Open(DynError),
    #[error("Cannot initialize database: {0}")]
    Initialization(DynError),
}

#[derive(Debug, thiserror::Error)]
pub enum CertifsStorageNonSpeculativeInitError {
    #[error("Cannot open database: {0}")]
    Open(DynError),
    #[error("Cannot initialize database: {0}")]
    Operation(DynError),
}

pub enum AnyCertif {
    User(UserCertificate),
    Device(DeviceCertificate),
    RevokedUser(RevokedUserCertificate),
    RealmRole(RealmRoleCertificate),
    SequesterAuthority(SequesterAuthorityCertificate),
    SequesterService(SequesterServiceCertificate),
}

impl CertifsStorage {
    pub async fn start(
        data_base_dir: &Path,
        device: Arc<LocalDevice>,
    ) -> Result<Self, CertifsStorageStartError> {
        // 1) Open the database

        let db_relative_path = get_certificates_storage_db_relative_path(&device);
        let db = LocalDatabase::from_path(data_base_dir, &db_relative_path, VacuumMode::default())
            .await
            .map_err(|err| CertifsStorageStartError::Open(Box::new(err)))?;

        // 2) Initialize the database (if needed)

        super::model::initialize_model_if_needed(&db)
            .await
            .map_err(|err| CertifsStorageStartError::Initialization(Box::new(err)))?;

        // 3) All done !

        let user_storage = Self {
            device,
            db,
            cache: Mutex::new(CertifsCache::default()),
            lock_update: AsyncMutex::new(()),
        };
        Ok(user_storage)
    }

    pub async fn stop(&self) {
        self.db.close().await
    }

    /// We never remove nor replace certificates, as a certifacet is immutable (a new
    /// certificate should be issued to represent a modification, e.g. for realm role)
    /// So if the server start sending certificates that doesn't match with the one we
    /// currently have stored, it means somebody is acting fishy !
    pub async fn add_new_certificates(
        &self,
        certificates: Vec<(IndexInt, AnyCertif)>,
    ) -> Result<(), DynError> {
        let update_guard = self.lock_update.lock().await;

        for (index, certif) in certificates.iter() {
            let index = i64::try_from(*index)?;
            db_set_certificate(&self.db, &self.device, index, certif).await?;
        }

        // TODO: replace by `HashMap::try_insert` once it is stabilized
        macro_rules! try_insert {
            ($hashmap: expr, $key: expr, $value: expr) => {
                if let std::collections::hash_map::Entry::Vacant(entry) = $hashmap.entry($key) {
                    entry.insert($value);
                }
            };
        }
        // To avoid taking and releasing the mutex multiple times we update the cache
        // once in a batch.
        // However we skip this step entirely in case a certificate insert failed.
        // This is okay given it's only a cache: accessing a certificate that has been
        // inserted but not put in the cache will trigger a cache miss and be fetched
        // from sqlite
        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        for (_, certif) in certificates {
            match certif {
                AnyCertif::User(certif) => {
                    try_insert!(guard.users, certif.user_id.to_owned(), certif);
                }
                AnyCertif::Device(certif) => {
                    try_insert!(guard.devices, certif.device_id.to_owned(), certif);
                }
                AnyCertif::RevokedUser(certif) => {
                    try_insert!(guard.revoked_users, certif.user_id.to_owned(), certif);
                }
                AnyCertif::RealmRole(certif) => {
                    try_insert!(guard.realm_roles, certif.realm_id.to_owned(), certif);
                }
                AnyCertif::SequesterAuthority(certif) => {
                    if guard.sequester_authority.is_none() {
                        guard.sequester_authority = Some(certif);
                    }
                }
                AnyCertif::SequesterService(certif) => {
                    try_insert!(
                        guard.sequester_services,
                        certif.service_id.to_owned(),
                        certif
                    );
                }
            }
        }

        drop(update_guard);
        Ok(())
    }
}

async fn db_set_certificate(
    db: &LocalDatabase,
    device: &LocalDevice,
    index: i64,
    certif: &AnyCertif,
) -> DatabaseResult<()> {
    let (certificate_type, hint, dumped) = match certif {
        AnyCertif::User(certif) => {
            let hint = format!("user_id:{}", certif.user_id.as_ref());
            let dumped = certif.unsecure_dump();
            (USER_CERTIFICATE_TYPE, hint, dumped)
        }
        AnyCertif::Device(certif) => {
            let hint = format!("device_id:{}", certif.device_id.as_ref());
            let dumped = certif.unsecure_dump();
            (DEVICE_CERTIFICATE_TYPE, hint, dumped)
        }
        AnyCertif::RevokedUser(certif) => {
            let hint = format!("user_id:{}", certif.user_id.as_ref());
            let dumped = certif.unsecure_dump();
            (REVOKED_USER_CERTIFICATE_TYPE, hint, dumped)
        }
        AnyCertif::RealmRole(certif) => {
            let hint = format!(
                "realm_id:{} user_id:{}",
                certif.realm_id.hex(),
                certif.user_id.as_ref()
            );
            let dumped = certif.unsecure_dump();
            (REALM_ROLE_CERTIFICATE_TYPE, hint, dumped)
        }
        AnyCertif::SequesterAuthority(certif) => {
            // No need for hint given there is only (at most) a single sequester
            // authority certificate per organization
            let hint = "".to_owned();
            let dumped = certif.unsecure_dump();
            (SEQUESTER_AUTHORITY_CERTIFICATE_TYPE, hint, dumped)
        }
        AnyCertif::SequesterService(certif) => {
            let hint = format!("service_id:{}", certif.service_id);
            let dumped = certif.dump();
            (SEQUESTER_SERVICE_CERTIFICATE_TYPE, hint, dumped)
        }
    };
    let encrypted = device.local_symkey.encrypt(&dumped);

    db.exec(move |conn| {
        conn.immediate_transaction(|conn| {
            let new_certificate = super::model::NewCertificate {
                index_: index,
                certificate: &encrypted,
                hint: &hint,
                certificate_type,
            };

            {
                use super::model::certificates::dsl::*;

                diesel::insert_into(certificates)
                    .values(new_certificate)
                    .on_conflict(index_)
                    // In theory they should never be any conflict given the index index
                    // used to ask the new certificates from the server. However in case
                    // it occurs we can just ignore the conflict as an index should always
                    // correspond to the same certificate.
                    .do_nothing()
                    .execute(conn)?;
            }

            Ok(())
        })
    })
    .await
}
