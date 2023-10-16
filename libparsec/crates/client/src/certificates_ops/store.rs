// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! The store is an intermediary layer between certificate ops and the storage.
//! It goals are twofold:
//! - handle a cache for the most common operations (e.g. retrieving device's verify key)
//!   and keep it consistent with the storage
//! - supervise read/write operations on the certificates
//!
//! Certificates being ordered, they are very dependant of each-other. Hence
//! we must prevent concurrent write operations to ensure inserting multiple
//! certificates is done in a atomic way.
//! On top of that, some read operations (the validation ones) work with the assumption
//! the storage contains all certificates up to a certain index. Here again we
//! need to prevent concurrent write operations (as it may remove certificates)

use async_trait::async_trait;
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};

use libparsec_platform_async::lock::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use libparsec_platform_storage::certificates::CertificatesStorage;
pub use libparsec_platform_storage::certificates::{
    AddCertificateData, GetCertificateError, GetCertificateQuery, GetTimestampBoundsError, UpTo,
};
use libparsec_types::prelude::*;

#[derive(Debug, Default)]
enum ScalarCache<T> {
    #[default]
    Miss,
    Present(T),
}

impl<T> ScalarCache<T> {
    pub fn set(&mut self, new: T) -> Option<T> {
        match std::mem::replace(self, Self::Present(new)) {
            Self::Present(old) => Some(old),
            Self::Miss => None,
        }
    }
}

#[derive(Default, Debug)]
struct CurrentViewCache {
    pub per_user_profile: HashMap<UserID, UserProfile>,
    pub last_index: ScalarCache<IndexInt>,
    // TODO
    // pub per_device_verify_key: HashMap<DeviceID, VerifyKey>,
}

impl CurrentViewCache {
    fn clear(&mut self) {
        *self = CurrentViewCache::default();
    }
}

#[derive(Debug)]
pub(crate) struct CertificatesStore {
    device: Arc<LocalDevice>,
    lock: RwLock<()>,
    current_view_cache: Mutex<CurrentViewCache>,
    storage: CertificatesStorage,
}

impl CertificatesStore {
    pub async fn start(data_base_dir: &Path, device: Arc<LocalDevice>) -> anyhow::Result<Self> {
        let storage = CertificatesStorage::start(data_base_dir, &device).await?;

        Ok(Self {
            device,
            lock: RwLock::default(),
            current_view_cache: Mutex::default(),
            storage,
        })
    }

    pub async fn stop(&self) {
        // Note the cache is never ahead of storage (i.e. it strictly constains
        // a subset of what's in the DB), hence no flush before stop is needed
        self.storage.stop().await;
    }

    pub async fn for_read(&self) -> CertificatesStoreReadGuard {
        let guard = self.lock.read().await;
        CertificatesStoreReadGuard {
            _guard: guard,
            store: self,
        }
    }

    pub async fn for_write(&self) -> CertificatesStoreWriteGuard {
        let guard = self.lock.write().await;
        CertificatesStoreWriteGuard {
            _guard: guard,
            store: self,
        }
    }
}

#[async_trait]
pub(crate) trait CertificatesStoreReadExt {
    fn store(&self) -> &CertificatesStore;

    async fn get_last_certificate_index(&self) -> anyhow::Result<IndexInt> {
        {
            let guard = self
                .store()
                .current_view_cache
                .lock()
                .expect("Mutex is poisoned !");
            if let ScalarCache::Present(index) = &guard.last_index {
                return Ok(*index);
            }
        }

        // Cache miss !

        let index = self
            .store()
            .storage
            .get_last_index()
            .await
            .map(|maybe| match maybe {
                None => 0,
                Some((index, _)) => index,
            })?;
        let mut guard = self
            .store()
            .current_view_cache
            .lock()
            .expect("Mutex is poisoned !");

        guard.last_index.set(index);

        Ok(index)
    }

    async fn get_current_self_profile(&self) -> anyhow::Result<UserProfile> {
        let maybe_update = get_last_user_update_certificate(
            self.store(),
            UpTo::Current,
            self.store().device.user_id().to_owned(),
        )
        .await?;
        let profile = match maybe_update {
            Some(update) => update.new_profile,
            None => self.store().device.initial_profile,
        };
        Ok(profile)
    }

    async fn get_current_user_profile(
        &self,
        user_id: UserID,
    ) -> Result<UserProfile, GetCertificateError> {
        {
            let guard = self
                .store()
                .current_view_cache
                .lock()
                .expect("Mutex is poisoned !");
            if let Some(profile) = guard.per_user_profile.get(&user_id) {
                return Ok(*profile);
            }
        }

        // Cache miss !

        let profile = {
            let maybe_user_update =
                get_last_user_update_certificate(self.store(), UpTo::Current, user_id.clone())
                    .await?;
            if let Some(user_update) = maybe_user_update {
                return Ok(user_update.new_profile);
            }
            // The user has never been modified
            if user_id == *self.store().device.user_id() {
                self.store().device.initial_profile
            } else {
                let certif =
                    get_user_certificate(self.store(), UpTo::Current, user_id.clone()).await?;
                certif.profile
            }
        };
        let mut guard = self
            .store()
            .current_view_cache
            .lock()
            .expect("Mutex is poisoned !");

        guard.per_user_profile.insert(user_id, profile);

        Ok(profile)
    }

    async fn get_timestamp_bounds(
        &self,
        index: IndexInt,
    ) -> Result<(DateTime, Option<DateTime>), GetTimestampBoundsError> {
        self.store().storage.get_timestamp_bounds(index).await
    }

    async fn get_any_certificate(
        &self,
        index: IndexInt,
    ) -> Result<AnyArcCertificate, GetCertificateError> {
        let data = self
            .store()
            .storage
            .get_any_certificate_encrypted(index)
            .await?;

        let certif = data
            .decrypt_and_load(&self.store().device.local_symkey)
            .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

        Ok(certif)
    }

    async fn get_user_certificate(
        &self,
        up_to: UpTo,
        user_id: UserID,
    ) -> Result<Arc<UserCertificate>, GetCertificateError> {
        get_user_certificate(self.store(), up_to, user_id).await
    }

    async fn get_user_certificates(
        &self,
        up_to: UpTo,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> anyhow::Result<Vec<Arc<UserCertificate>>> {
        let query = GetCertificateQuery::users_certificates();
        get_certificates(
            self.store(),
            query,
            up_to,
            offset,
            limit,
            UserCertificate::unsecure_load,
            UnsecureUserCertificate::skip_validation,
        )
        .await
    }

    async fn get_last_user_update_certificate(
        &self,
        up_to: UpTo,
        user_id: UserID,
    ) -> anyhow::Result<Option<Arc<UserUpdateCertificate>>> {
        get_last_user_update_certificate(self.store(), up_to, user_id).await
    }

    async fn get_device_certificate(
        &self,
        up_to: UpTo,
        device_id: DeviceID,
    ) -> Result<Arc<DeviceCertificate>, GetCertificateError> {
        let query = GetCertificateQuery::device_certificate(device_id);
        let (_, encrypted) = self
            .store()
            .storage
            .get_certificate_encrypted(query, up_to)
            .await?;

        Ok(get_certificate_from_encrypted(
            self.store(),
            &encrypted,
            DeviceCertificate::unsecure_load,
            UnsecureDeviceCertificate::skip_validation,
        )
        .await?)
    }

    async fn get_user_devices_certificates(
        &self,
        up_to: UpTo,
        user_id: UserID,
    ) -> anyhow::Result<Vec<Arc<DeviceCertificate>>> {
        let query = GetCertificateQuery::user_device_certificates(user_id);
        get_certificates(
            self.store(),
            query,
            up_to,
            None,
            None,
            DeviceCertificate::unsecure_load,
            UnsecureDeviceCertificate::skip_validation,
        )
        .await
    }

    async fn get_revoked_user_certificate(
        &self,
        up_to: UpTo,
        user_id: UserID,
    ) -> anyhow::Result<Option<Arc<RevokedUserCertificate>>> {
        let query = GetCertificateQuery::revoked_user_certificate(user_id);
        let encrypted = match self
            .store()
            .storage
            .get_certificate_encrypted(query, up_to)
            .await
        {
            Ok((_, encrypted)) => encrypted,
            Err(
                GetCertificateError::NonExisting | GetCertificateError::ExistButTooRecent { .. },
            ) => return Ok(None),
            Err(GetCertificateError::Internal(err)) => return Err(err),
        };

        get_certificate_from_encrypted(
            self.store(),
            &encrypted,
            RevokedUserCertificate::unsecure_load,
            UnsecureRevokedUserCertificate::skip_validation,
        )
        .await
        .map(Some)
    }

    async fn get_realm_roles(
        &self,
        up_to: UpTo,
        realm_id: VlobID,
    ) -> anyhow::Result<Vec<Arc<RealmRoleCertificate>>> {
        let query = GetCertificateQuery::realm_role_certificates(realm_id);
        get_certificates(
            self.store(),
            query,
            up_to,
            None,
            None,
            RealmRoleCertificate::unsecure_load,
            UnsecureRealmRoleCertificate::skip_validation,
        )
        .await
    }

    async fn get_user_realm_role(
        &self,
        up_to: UpTo,
        user_id: UserID,
        realm_id: VlobID,
    ) -> anyhow::Result<Option<Arc<RealmRoleCertificate>>> {
        let query = GetCertificateQuery::realm_role_certificate(realm_id, user_id);
        let encrypted = match self
            .store()
            .storage
            .get_certificate_encrypted(query, up_to)
            .await
        {
            Ok((_, encrypted)) => encrypted,
            Err(
                GetCertificateError::NonExisting | GetCertificateError::ExistButTooRecent { .. },
            ) => return Ok(None),
            Err(GetCertificateError::Internal(err)) => return Err(err),
        };

        get_certificate_from_encrypted(
            self.store(),
            &encrypted,
            RealmRoleCertificate::unsecure_load,
            UnsecureRealmRoleCertificate::skip_validation,
        )
        .await
        .map(Some)
    }

    async fn get_user_realms_roles(
        &self,
        up_to: UpTo,
        user_id: UserID,
    ) -> anyhow::Result<Vec<Arc<RealmRoleCertificate>>> {
        let query = GetCertificateQuery::user_realm_role_certificates(user_id);
        get_certificates(
            self.store(),
            query,
            up_to,
            None,
            None,
            RealmRoleCertificate::unsecure_load,
            UnsecureRealmRoleCertificate::skip_validation,
        )
        .await
    }

    async fn get_sequester_authority_certificate(
        &self,
        up_to: UpTo,
    ) -> Result<Arc<SequesterAuthorityCertificate>, GetCertificateError> {
        let query = GetCertificateQuery::sequester_authority_certificate();
        let (_, encrypted) = self
            .store()
            .storage
            .get_certificate_encrypted(query, up_to)
            .await?;

        Ok(get_certificate_from_encrypted(
            self.store(),
            &encrypted,
            SequesterAuthorityCertificate::unsecure_load,
            UnsecureSequesterAuthorityCertificate::skip_validation,
        )
        .await?)
    }

    async fn get_sequester_service_certificates(
        &self,
        up_to: UpTo,
    ) -> anyhow::Result<Vec<Arc<SequesterServiceCertificate>>> {
        let query = GetCertificateQuery::sequester_service_certificates();
        get_certificates(
            self.store(),
            query,
            up_to,
            None,
            None,
            SequesterServiceCertificate::unsecure_load,
            UnsecureSequesterServiceCertificate::skip_validation,
        )
        .await
    }
}

pub(crate) struct CertificatesStoreWriteGuard<'a> {
    _guard: RwLockWriteGuard<'a, ()>,
    store: &'a CertificatesStore,
}

impl<'a> CertificatesStoreReadExt for CertificatesStoreWriteGuard<'a> {
    fn store(&self) -> &CertificatesStore {
        self.store
    }
}

impl<'a> CertificatesStoreWriteGuard<'a> {
    pub async fn forget_all_certificates(&self) -> anyhow::Result<()> {
        self.store.storage.forget_all_certificates().await?;
        self.store
            .current_view_cache
            .lock()
            .expect("Mutex is poisoned !")
            .clear();
        Ok(())
    }

    pub async fn add_next_certificate(
        &self,
        index: IndexInt,
        certif: &AnyArcCertificate,
        signed: &[u8],
    ) -> anyhow::Result<()> {
        let encrypted = self.store.device.local_symkey.encrypt(signed);
        let data = AddCertificateData::from_certif(certif, encrypted);
        self.store.storage.add_next_certificate(index, data).await?;

        // Update cache

        let mut guard = self
            .store
            .current_view_cache
            .lock()
            .expect("Mutex is poisoned !");
        guard.last_index.set(index);
        match certif {
            AnyArcCertificate::User(certif) => {
                guard
                    .per_user_profile
                    .insert(certif.user_id.clone(), certif.profile);
            }
            AnyArcCertificate::UserUpdate(certif) => {
                guard
                    .per_user_profile
                    .insert(certif.user_id.clone(), certif.new_profile);
            }
            _ => (),
        }

        Ok(())
    }
}

pub(crate) struct CertificatesStoreReadGuard<'a> {
    _guard: RwLockReadGuard<'a, ()>,
    store: &'a CertificatesStore,
}

impl<'a> CertificatesStoreReadExt for CertificatesStoreReadGuard<'a> {
    fn store(&self) -> &CertificatesStore {
        self.store
    }
}

async fn get_user_certificate(
    store: &CertificatesStore,
    up_to: UpTo,
    user_id: UserID,
) -> Result<Arc<UserCertificate>, GetCertificateError> {
    let query = GetCertificateQuery::user_certificate(user_id);
    let (_, encrypted) = store
        .storage
        .get_certificate_encrypted(query, up_to)
        .await?;

    let signed = store
        .device
        .local_symkey
        .decrypt(&encrypted)
        .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

    let unsecure = UserCertificate::unsecure_load(signed.into())
        .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

    let certif = unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);

    Ok(Arc::new(certif))
}

async fn get_last_user_update_certificate(
    store: &CertificatesStore,
    up_to: UpTo,
    user_id: UserID,
) -> anyhow::Result<Option<Arc<UserUpdateCertificate>>> {
    let query = GetCertificateQuery::user_update_certificates(user_id);
    // `get_certificate_encrypted` return the last certificate if multiple are available
    let outcome = store.storage.get_certificate_encrypted(query, up_to).await;
    let encrypted = match outcome {
        Ok((_, encrypted)) => encrypted,
        Err(GetCertificateError::NonExisting | GetCertificateError::ExistButTooRecent { .. }) => {
            return Ok(None)
        }
        Err(GetCertificateError::Internal(err)) => return Err(err),
    };

    let signed = store
        .device
        .local_symkey
        .decrypt(&encrypted)
        .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

    let unsecure = UserUpdateCertificate::unsecure_load(signed.into())
        .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

    let certif = unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);

    Ok(Some(Arc::new(certif)))
}

async fn get_certificates<T, U>(
    store: &CertificatesStore,
    query: GetCertificateQuery,
    up_to: UpTo,
    offset: Option<usize>,
    limit: Option<usize>,
    unsecure_load: fn(Bytes) -> DataResult<U>,
    skip_validation: fn(U, UnsecureSkipValidationReason) -> T,
) -> anyhow::Result<Vec<Arc<T>>> {
    let items = store
        .storage
        .get_multiple_certificates_encrypted(query, up_to, offset, limit)
        .await?;

    let mut certifs = Vec::with_capacity(items.len());
    for (_, encrypted) in items {
        let certif =
            get_certificate_from_encrypted(store, &encrypted, unsecure_load, skip_validation)
                .await?;
        certifs.push(certif);
    }

    Ok(certifs)
}

async fn get_certificate_from_encrypted<T, U>(
    store: &CertificatesStore,
    encrypted: &[u8],
    unsecure_load: fn(Bytes) -> DataResult<U>,
    skip_validation: fn(U, UnsecureSkipValidationReason) -> T,
) -> anyhow::Result<Arc<T>> {
    let signed = store
        .device
        .local_symkey
        .decrypt(encrypted)
        .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

    let unsecure = unsecure_load(signed.into())
        .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

    let certif = skip_validation(unsecure, UnsecureSkipValidationReason::DataFromLocalStorage);

    Ok(Arc::new(certif))
}
