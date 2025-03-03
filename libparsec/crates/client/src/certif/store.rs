// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

//! The store is an intermediary layer between certificate ops and the storage.
//! Its goals are twofold:
//! - handle a cache for the most common operations (e.g. retrieving device's verify key)
//!   and keep it consistent with the storage
//! - supervise read/write operations on the certificates
//!
//! Certificates being ordered, they are very dependant of each-other. Hence
//! we must prevent concurrent write operations to ensure inserting multiple
//! certificates is done in a atomic way.
//! On top of that, some read operations (the validation ones) work with the assumption
//! the storage contains all certificates up to a certain timestamp. Here again we
//! need to prevent concurrent write operations (as it may remove certificates)

use std::{
    collections::HashMap,
    ops::AsyncFnOnce,
    path::Path,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::ConnectionError;
use libparsec_platform_async::lock::{Mutex as AsyncMutex, RwLock};
pub use libparsec_platform_storage::certificates::UpTo;
use libparsec_platform_storage::certificates::{
    CertificatesStorage, CertificatesStorageUpdater, PerTopicLastTimestamps,
};
pub(super) use libparsec_platform_storage::certificates::{
    GetCertificateError, GetCertificateQuery,
};
use libparsec_types::prelude::*;

use crate::certif::CertifPollServerError;

use super::{realm_keys_bundle::RealmKeys, CertificateOps, InvalidCertificateError};

/// This enum list all possible states for a realm.
///
/// Bootstrapping a user realm just correspond to uploading its initial realm role
/// manifest.
/// Bootstrapping a workspace realm (the famous "workspace bootstrap") requires two
/// additional certificates to be upload in a non-atomic way, hence the need to know
/// at what state we currently are in order to be able to achieve such bootstrap in
/// an idempotent way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealmBootstrapState {
    /// Step 0
    ///
    /// The realm has been created on our device but doesn't exist yet on the server
    /// (i.e. we haven't send the initial realm role certificate).
    LocalOnly,
    /// Step 1
    ///
    /// The realm exists on the server (i.e. initial realm role certificate has been uploaded).
    /// Notes:
    /// - The user realm doesn't go beyond this state.
    /// - For a workspace realm, this is the first step of the workspace bootstrap.
    CreatedInServer,
    /// Step 2 (workspace only)
    ///
    /// The realm exists on the server, and its initial key rotation manifest has
    /// been uploaded.
    InitialKeyRotationDone,
    /// Step 3 (workspace only)
    ///
    /// The realm exists on the server and its initial keys rotation manifest and realm
    /// name certificate has been uploaded.
    ///
    /// At this point the workspace bootstrap has been fully achieved.
    WorkspaceBootstrapped,
}

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

#[derive(Debug, thiserror::Error)]
pub enum CertifStoreError {
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CertifForReadWithRequirementsError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error("Invalid requirements")]
    InvalidRequirements,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Default, Debug)]
struct CurrentViewCache {
    pub per_topic_last_timestamps: ScalarCache<PerTopicLastTimestamps>,
    pub self_profile: ScalarCache<UserProfile>,
    pub per_user_profile: HashMap<UserID, UserProfile>,
    pub per_device_verify_key_and_user_id: HashMap<DeviceID, (VerifyKey, DateTime, UserID)>,
    // Realm keys is a special case: it stores cache on data (keys bundle
    // & keys bundle access key) that are not directly from certificates.
    // Instead those data are fetched from the server then validated against the
    // certificates.
    pub per_realm_keys: HashMap<VlobID, Arc<RealmKeys>>,
    pub per_realm_bootstrap_state: HashMap<VlobID, RealmBootstrapState>,
    /// If the entry is missing this is a cache miss, if the entry is at `None`
    /// that means the user used to be part of the given realm.
    pub per_realm_self_role: HashMap<VlobID, Option<RealmRole>>,
}

impl CurrentViewCache {
    fn clear(&mut self) {
        *self = CurrentViewCache::default();
    }
}

#[derive(Debug)]
pub(super) struct CertificatesStore {
    device: Arc<LocalDevice>,
    // Why 3 locks here ?
    // `lock` is the initial lock taken to exclude reads from write operations.
    // Once taken we first look in the `current_view_cache` and only then use `storage`
    // in case of cache miss.
    // Given accessing `storage` requires exclusive access, it is better to have it
    // under its own lock so that all cache hit operations can occur concurrently.
    // On top of that, the cache must be behind its own lock (and not behind the main
    // read-write lock) so that it can be updated on cache miss even if we are in a
    // read operation.
    lock: RwLock<()>,
    current_view_cache: Mutex<CurrentViewCache>,
    // Set to `None` once stopped
    storage: AsyncMutex<Option<CertificatesStorage>>,
}

impl CertificatesStore {
    pub async fn start(data_base_dir: &Path, device: Arc<LocalDevice>) -> anyhow::Result<Self> {
        let storage = CertificatesStorage::start(data_base_dir, &device).await?;

        Ok(Self {
            device,
            lock: RwLock::default(),
            storage: AsyncMutex::new(Some(storage)),
            current_view_cache: Mutex::default(),
        })
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        let mut mutex = self.storage.lock().await;
        let maybe_storage = mutex.take();
        // Go idempotent if the storage is already stopped
        if let Some(storage) = maybe_storage {
            // Note the cache is never ahead of storage (i.e. it strictly constains
            // a subset of what's in the DB), hence no flush before stop is needed
            storage.stop().await?
        }
        Ok(())
    }

    /// Only used for debugging tests
    #[cfg(any(test, feature = "expose-test-methods"))]
    #[allow(unused)]
    pub async fn debug_dump(&self) -> anyhow::Result<String> {
        let mut output = "{\n".to_owned();

        {
            let cache = self.current_view_cache.lock().expect("Mutex is poisoned");
            output += &format!("Cache: {:?}\n", *cache);
        }

        let mut maybe_storage = self.storage.lock().await;
        if let Some(storage) = &mut *maybe_storage {
            let dump = storage.debug_dump().await?;
            output += &format!("Storage: {dump}\n");
        } else {
            output += "{Stopped}";
        }

        output += "}\n";

        Ok(output)
    }

    /// Lock the store for writing purpose, this is an exclusive lock so no
    /// other read/write operation can occur.
    pub async fn for_write<T, E>(
        &self,
        cb: impl AsyncFnOnce(&mut CertificatesStoreWriteGuard<'_>) -> Result<T, E>,
    ) -> Result<Result<T, E>, CertifStoreError> {
        let _guard = self.lock.write().await;

        let mut maybe_storage = self.storage.lock().await;
        let storage = match &mut *maybe_storage {
            None => return Err(CertifStoreError::Stopped),
            Some(storage) => storage,
        };

        let outcome = storage
            .for_update(async |updater| {
                let mut write_guard = CertificatesStoreWriteGuard {
                    store: self,
                    storage: updater,
                };

                cb(&mut write_guard).await
            })
            .await;

        // The cache may have been updated during the write operations, and those new cache
        // entries might be for items that have been added by the current write operation.
        // If something goes wrong, `CertificateStorage::for_update` takes care of rolling back
        // the database, thus making the cache inconsistent with the database.
        // Hence we must clear the cache in case of error.

        let reset_cache = || {
            self.current_view_cache
                .lock()
                .expect("Mutex is poisoned !")
                .clear();
        };

        match outcome {
            Ok(Ok(res)) => Ok(Ok(res)),
            // Error from `cb`
            Ok(Err(err)) => {
                reset_cache();
                Ok(Err(err))
            }
            // Error from `CertificateStorage::for_update` (e.g. commit)
            Err(err) => {
                reset_cache();
                Err(err.into())
            }
        }
    }

    /// Lock the store for reading purpose, this lock guarantee the certificates
    /// won't change (this typically prevent the certificates database from being
    /// dropped when the user switch from/to OUTSIDER).
    ///
    /// Also note the lock is shared with other read operations.
    pub async fn for_read<T, E>(
        &self,
        cb: impl AsyncFnOnce(&mut CertificatesStoreReadGuard) -> Result<T, E>,
    ) -> Result<Result<T, E>, CertifStoreError> {
        let _guard = self.lock.read().await;

        let mut maybe_storage = self.storage.lock().await;
        let storage = match &mut *maybe_storage {
            None => return Err(CertifStoreError::Stopped),
            Some(storage) => storage,
        };

        let mut read_guard = CertificatesStoreReadGuard {
            store: self,
            storage,
        };

        let fut = cb(&mut read_guard);
        let outcome = fut.await;

        Ok(outcome)
    }

    /// Similar to `for_read`, but start by polling for new certificates until the
    /// requirements are met.
    ///
    /// This is typically useful when validation data received from the server (as
    /// the server provides us with the certificates requirements to do the validation).
    pub async fn for_read_with_requirements<C, T, E>(
        &self,
        ops: &CertificateOps,
        needed_timestamps: &PerTopicLastTimestamps,
        cb: C,
    ) -> Result<Result<T, E>, CertifForReadWithRequirementsError>
    where
        C: AsyncFnOnce(&mut CertificatesStoreReadGuard) -> Result<T, E>,
    {
        enum ForReadOutcome<C, T1, E1> {
            StoredTimestampOutdated(C),
            Done(Result<T1, E1>),
        }

        // Factorize the code in a closure to retry the operation after a server poll
        let try_with_requirements = |cb| {
            self.for_read(async |store: &mut CertificatesStoreReadGuard| {
                // Make sure we have all the needed certificates

                let last_stored_timestamps = store
                    .get_last_timestamps()
                    .await
                    .map_err(CertifForReadWithRequirementsError::Internal)?;
                if !last_stored_timestamps.is_up_to_date(needed_timestamps) {
                    return Result::<ForReadOutcome<C, T, E>, CertifForReadWithRequirementsError>::Ok(
                        ForReadOutcome::StoredTimestampOutdated(cb),
                    );
                }

                // Requirements are met, do the actual operation
                let res = cb(store).await;

                Result::<ForReadOutcome<C, T, E>, CertifForReadWithRequirementsError>::Ok(
                    ForReadOutcome::Done(res),
                )
            })
        };

        // Try to lock for read and run the callback
        let cb = match try_with_requirements(cb).await.map_err(|e| match e {
            CertifStoreError::Stopped => CertifForReadWithRequirementsError::Stopped,
            CertifStoreError::Internal(err) => err.context("Cannot lock storage for read").into(),
        })?? {
            // If the requirement were met, we are done
            ForReadOutcome::Done(res) => return Ok(res),
            // If the requirements were not met, we need to poll the server
            ForReadOutcome::StoredTimestampOutdated(cb) => cb,
        };

        // We were lacking some certificates, do a server poll and retry
        super::poll::poll_server_for_new_certificates(ops, Some(needed_timestamps))
            .await
            .map_err(|e| match e {
                CertifPollServerError::Offline(e) => CertifForReadWithRequirementsError::Offline(e),
                CertifPollServerError::Stopped => CertifForReadWithRequirementsError::Stopped,
                CertifPollServerError::InvalidCertificate(err) => {
                    CertifForReadWithRequirementsError::InvalidCertificate(err)
                }
                CertifPollServerError::Internal(err) => err
                    .context("Cannot poll server for new certificates")
                    .into(),
            })?;

        // Retry the operation now that we have the requirements
        match try_with_requirements(cb).await.map_err(|e| match e {
            CertifStoreError::Stopped => CertifForReadWithRequirementsError::Stopped,
            CertifStoreError::Internal(err) => err.context("Cannot lock storage for read").into(),
        })?? {
            // If the requirement were met, we are done
            ForReadOutcome::Done(res) => Ok(res),
            // If the requirements were not met, the requirements were invalid in the first place
            ForReadOutcome::StoredTimestampOutdated(_) => {
                Err(CertifForReadWithRequirementsError::InvalidRequirements)
            }
        }
    }
}

pub(super) enum LastShamirRecovery {
    NeverSetup,
    Valid(Arc<ShamirRecoveryBriefCertificate>),
    Deleted(
        Arc<ShamirRecoveryBriefCertificate>,
        Arc<ShamirRecoveryDeletionCertificate>,
    ),
}

pub(super) enum LastUserExistAndRevokedInfo {
    Unknown,
    Valid(Arc<UserCertificate>),
    #[allow(dead_code)]
    Revoked(Arc<UserCertificate>, Arc<RevokedUserCertificate>),
}

macro_rules! impl_read_methods {
    () => {
        #[allow(unused)]
        pub async fn get_current_self_profile(&mut self) -> anyhow::Result<UserProfile> {
            {
                let guard = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned !");
                if let ScalarCache::Present(self_profile) = guard.self_profile {
                    return Ok(self_profile);
                }
            }

            // Cache miss !

            let maybe_update = self
                .get_last_user_update_certificate(
                    UpTo::Current,
                    self.store.device.user_id,
                )
                .await?;
            let self_profile = match maybe_update {
                Some(update) => update.new_profile,
                None => self.store.device.initial_profile,
            };

            self.store
                .current_view_cache
                .lock()
                .expect("Mutex is poisoned !")
                .self_profile
                .set(self_profile);

            Ok(self_profile)
        }

        #[allow(unused)]
        pub async fn get_last_timestamps(&mut self) -> anyhow::Result<PerTopicLastTimestamps> {
            {
                let guard = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned !");
                if let ScalarCache::Present(last_timestamps) = &guard.per_topic_last_timestamps {
                    return Ok(last_timestamps.to_owned());
                }
            }

            // Cache miss !

            let last_timestamps = self.storage.get_last_timestamps().await?;

            let mut guard = self
                .store
                .current_view_cache
                .lock()
                .expect("Mutex is poisoned !");

            guard.per_topic_last_timestamps.set(last_timestamps.clone());

            Ok(last_timestamps)
        }

        #[allow(unused)]
        pub async fn get_device_verify_key(
            &mut self,
            up_to: UpTo,
            device_id: DeviceID,
        ) -> Result<VerifyKey, GetCertificateError> {
            self.get_device_verify_key_and_user_id(up_to, device_id)
                .await
                .map(|(verify_key, _)| verify_key)
        }

        #[allow(unused)]
        pub async fn get_device_verify_key_and_user_id(
            &mut self,
            up_to: UpTo,
            device_id: DeviceID,
        ) -> Result<(VerifyKey, UserID), GetCertificateError> {
            {
                let cache = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned !");
                if let Some((verify_key, timestamp, user_id)) = cache.per_device_verify_key_and_user_id.get(&device_id) {
                    if let UpTo::Timestamp(up_to) = up_to {
                        if *timestamp > up_to {
                            return Err(GetCertificateError::ExistButTooRecent {
                                certificate_timestamp: *timestamp,
                            });
                        }
                    }
                    return Ok((verify_key.to_owned(), *user_id));
                }
            }

            // Cache miss !

            let query = GetCertificateQuery::device_certificate(&device_id);
            let (_, encrypted) = self.storage.get_certificate_encrypted(query, up_to).await?;

            let certif = get_certificate_from_encrypted(
                self.store,
                &encrypted,
                DeviceCertificate::unsecure_load,
                UnsecureDeviceCertificate::skip_validation,
            )?;

            // Update cache before leaving
            {
                let mut cache = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned !");
                cache
                    .per_device_verify_key_and_user_id
                    .insert(device_id, (certif.verify_key.clone(), certif.timestamp, certif.user_id));
            }

            Ok((certif.verify_key.to_owned(), certif.user_id))
        }

        #[allow(unused)]
        pub async fn get_user_certificate(
            &mut self,
            up_to: UpTo,
            user_id: UserID,
        ) -> Result<Arc<UserCertificate>, GetCertificateError> {
            let query = GetCertificateQuery::user_certificate(&user_id);
            let (_, encrypted) = self.storage.get_certificate_encrypted(query, up_to).await?;

            let certif = get_certificate_from_encrypted(
                self.store,
                &encrypted,
                UserCertificate::unsecure_load,
                UnsecureUserCertificate::skip_validation,
            )?;

            Ok(certif)
        }

        #[allow(unused)]
        pub async fn get_user_certificate_from_device_id(
            &mut self,
            up_to: UpTo,
            device_id: DeviceID,
        ) -> Result<Arc<UserCertificate>, GetCertificateError> {
            let query = GetCertificateQuery::user_certificate_from_device_id(&device_id);
            let (_, encrypted) = self.storage.get_certificate_encrypted(query, up_to).await?;

            let certif = get_certificate_from_encrypted(
                self.store,
                &encrypted,
                UserCertificate::unsecure_load,
                UnsecureUserCertificate::skip_validation,
            )?;

            Ok(certif)
        }

        #[allow(unused)]
        /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
        pub async fn get_user_certificates(
            &mut self,
            up_to: UpTo,
            offset: Option<u32>,
            limit: Option<u32>,
        ) -> anyhow::Result<Vec<Arc<UserCertificate>>> {
            let query = GetCertificateQuery::users_certificates();
            let items = self
                .storage
                .get_multiple_certificates_encrypted(query, up_to, offset, limit)
                .await?;
            get_multiple_certificates_from_encrypted(
                &self.store,
                items,
                UserCertificate::unsecure_load,
                UnsecureUserCertificate::skip_validation,
            )
        }

        #[allow(unused)]
        pub async fn get_device_certificate(
            &mut self,
            up_to: UpTo,
            device_id: DeviceID,
        ) -> Result<Arc<DeviceCertificate>, GetCertificateError> {
            let query = GetCertificateQuery::device_certificate(&device_id);
            let (_, encrypted) = self.storage.get_certificate_encrypted(query, up_to).await?;

            let certif = get_certificate_from_encrypted(
                self.store,
                &encrypted,
                DeviceCertificate::unsecure_load,
                UnsecureDeviceCertificate::skip_validation,
            )?;

            Ok(certif)
        }

        #[allow(unused)]
        /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
        pub async fn get_user_devices_certificates(
            &mut self,
            up_to: UpTo,
            user_id: UserID,
        ) -> anyhow::Result<Vec<Arc<DeviceCertificate>>> {
            let query = GetCertificateQuery::user_devices_certificates(&user_id);
            let items = self
                .storage
                .get_multiple_certificates_encrypted(query, up_to, None, None)
                .await?;
            get_multiple_certificates_from_encrypted(
                &self.store,
                items,
                DeviceCertificate::unsecure_load,
                UnsecureDeviceCertificate::skip_validation,
            )
        }

        #[allow(unused)]
        pub async fn get_revoked_user_certificate(
            &mut self,
            up_to: UpTo,
            user_id: UserID,
        ) -> anyhow::Result<Option<Arc<RevokedUserCertificate>>> {
            let query = GetCertificateQuery::revoked_user_certificate(&user_id);
            let outcome = self.storage.get_certificate_encrypted(query, up_to).await;

            let encrypted = match outcome {
                Ok((_, encrypted)) => encrypted,
                Err(
                    GetCertificateError::NonExisting
                    | GetCertificateError::ExistButTooRecent { .. },
                ) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            get_certificate_from_encrypted(
                self.store,
                &encrypted,
                RevokedUserCertificate::unsecure_load,
                UnsecureRevokedUserCertificate::skip_validation,
            )
            .map(Some)
        }

        #[allow(unused)]
        pub async fn get_revoked_user_certificate_from_device_id(
            &mut self,
            up_to: UpTo,
            device_id: DeviceID,
        ) -> anyhow::Result<Option<Arc<RevokedUserCertificate>>> {
            let query = GetCertificateQuery::revoked_user_certificate_from_device_id(&device_id);
            let outcome = self.storage.get_certificate_encrypted(query, up_to).await;

            let encrypted = match outcome {
                Ok((_, encrypted)) => encrypted,
                Err(
                    GetCertificateError::NonExisting
                    | GetCertificateError::ExistButTooRecent { .. },
                ) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            get_certificate_from_encrypted(
                self.store,
                &encrypted,
                RevokedUserCertificate::unsecure_load,
                UnsecureRevokedUserCertificate::skip_validation,
            )
            .map(Some)
        }

        #[allow(unused)]
        pub async fn get_last_user_update_certificate(
            &mut self,
            up_to: UpTo,
            user_id: UserID,
        ) -> anyhow::Result<Option<Arc<UserUpdateCertificate>>> {
            let query = GetCertificateQuery::user_update_certificates(&user_id);
            // `get_certificate_encrypted` return the last certificate if multiple are available
            let outcome = self.storage.get_certificate_encrypted(query, up_to).await;
            let encrypted = match outcome {
                Ok((_, encrypted)) => encrypted,
                Err(
                    GetCertificateError::NonExisting
                    | GetCertificateError::ExistButTooRecent { .. },
                ) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            let signed = self
                .store
                .device
                .local_symkey
                .decrypt(&encrypted)
                .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

            let unsecure = UserUpdateCertificate::unsecure_load(signed.into())
                .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

            let certif =
                unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);

            Ok(Some(Arc::new(certif)))
        }

        #[allow(unused)]
        pub async fn get_last_user_update_certificate_from_device_id(
            &mut self,
            up_to: UpTo,
            device_id: DeviceID,
        ) -> anyhow::Result<Option<Arc<UserUpdateCertificate>>> {
            let query = GetCertificateQuery::user_update_certificates_from_device_id(&device_id);
            // `get_certificate_encrypted` return the last certificate if multiple are available
            let outcome = self.storage.get_certificate_encrypted(query, up_to).await;
            let encrypted = match outcome {
                Ok((_, encrypted)) => encrypted,
                Err(
                    GetCertificateError::NonExisting
                    | GetCertificateError::ExistButTooRecent { .. },
                ) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            let signed = self
                .store
                .device
                .local_symkey
                .decrypt(&encrypted)
                .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

            let unsecure = UserUpdateCertificate::unsecure_load(signed.into())
                .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

            let certif =
                unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);

            Ok(Some(Arc::new(certif)))
        }

        #[allow(unused)]
        pub async fn get_last_user_exist_and_revoked_info(
            &mut self,
            up_to: UpTo,
            user_id: UserID,
        ) -> anyhow::Result<LastUserExistAndRevokedInfo> {
            let user_certificate = match self.get_user_certificate(up_to, user_id).await {
                Ok(user_certificate) => user_certificate,
                Err(GetCertificateError::NonExisting | GetCertificateError::ExistButTooRecent { .. }) => return Ok(LastUserExistAndRevokedInfo::Unknown),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            match self.get_revoked_user_certificate(up_to, user_id).await? {
                None => Ok(LastUserExistAndRevokedInfo::Valid(user_certificate)),
                Some(revoked_certificate) => Ok(LastUserExistAndRevokedInfo::Revoked(user_certificate, revoked_certificate))
            }
        }

        #[allow(unused)]
        /// Return the last realm role certificate for the given user in the given realm, if any.
        pub async fn get_last_user_realm_role(
            &mut self,
            up_to: UpTo,
            user_id: UserID,
            realm_id: VlobID,
        ) -> anyhow::Result<Option<Arc<RealmRoleCertificate>>> {
            let query = GetCertificateQuery::realm_role_certificate(&realm_id, &user_id);
            // `get_certificate_encrypted` return the last certificate if multiple are available
            let encrypted = match self.storage.get_certificate_encrypted(query, up_to).await {
                Ok((_, encrypted)) => encrypted,
                Err(
                    GetCertificateError::NonExisting
                    | GetCertificateError::ExistButTooRecent { .. },
                ) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            get_certificate_from_encrypted(
                self.store,
                &encrypted,
                RealmRoleCertificate::unsecure_load,
                UnsecureRealmRoleCertificate::skip_validation,
            )
            .map(Some)
        }

        #[allow(unused)]
        /// Return all certificates involving the user.
        ///
        /// ⚠️ This method returns *all* certificates, including the ones that have been
        /// superseded ! (i.e. you should iter from the last to the first certificate to
        /// get the current role for a given realm).
        ///
        /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
        pub async fn get_user_realms_roles(
            &mut self,
            up_to: UpTo,
            user_id: UserID,
        ) -> anyhow::Result<Vec<Arc<RealmRoleCertificate>>> {
            let query = GetCertificateQuery::user_realm_role_certificates(&user_id);
            let items = self
                .storage
                .get_multiple_certificates_encrypted(query, up_to, None, None)
                .await?;
            get_multiple_certificates_from_encrypted(
                &self.store,
                items,
                RealmRoleCertificate::unsecure_load,
                UnsecureRealmRoleCertificate::skip_validation,
            )
        }

        // TODO: Provide a `get_user_last_realms_roles` that, for a given user, returns
        //       the last valid realm role certificate for each of its realms.

        #[allow(unused)]
        /// Get the current role of our user for a given realm, if any.
        pub async fn get_self_user_realm_role(
            &mut self,
            realm_id: VlobID,
        ) -> anyhow::Result<Option<Option<RealmRole>>> {
            {
                let cache = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned !");
                if let Some(role) = cache.per_realm_self_role.get(&realm_id) {
                    return Ok(Some(*role));
                }
            }

            // Cache miss !

            let query = GetCertificateQuery::realm_role_certificate(&realm_id, &self.store.device.user_id);
            // `get_certificate_encrypted` return the last certificate if multiple are available
            let outcome = self.storage.get_certificate_encrypted(query, UpTo::Current).await;
            let encrypted = match outcome {
                Ok((_, encrypted)) => encrypted,
                Err(
                    GetCertificateError::NonExisting
                    | GetCertificateError::ExistButTooRecent { .. },
                ) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            let certif = get_certificate_from_encrypted(
                self.store,
                &encrypted,
                RealmRoleCertificate::unsecure_load,
                UnsecureRealmRoleCertificate::skip_validation,
            )?;

            // Update cache before leaving
            {
                let mut cache = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned !");
                cache
                    .per_realm_self_role
                    .insert(realm_id, certif.role);
            }

            Ok(Some(certif.role))
        }

        #[allow(unused)]
        /// Return the current state of the bootstrap for a given realm.
        pub async fn get_realm_bootstrap_state(
            &mut self,
            realm_id: VlobID,
        ) -> anyhow::Result<RealmBootstrapState> {
            {
                let guard = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned !");

                if let Some(state) = guard.per_realm_bootstrap_state.get(&realm_id) {
                    return Ok(*state)
                }
            }

            // Cache miss !

            let mut state =  'get_state: {
                // Most likely case is the realm is a bootstrapped workspace, so start by
                // looking for the realm name certificate (i.e. last things done during
                // bootstrap).

                // 1) Realm name certificate ? (i.e. last workspace bootstrap step)

                let query = GetCertificateQuery::realm_name_certificates(&realm_id);
                let outcome = self.storage.get_certificate_encrypted(query, UpTo::Current).await;
                match outcome {
                    Ok(_) => {
                        break 'get_state RealmBootstrapState::WorkspaceBootstrapped;
                    },
                    Err(GetCertificateError::NonExisting)
                    | Err(GetCertificateError::ExistButTooRecent { .. }) => (),
                    Err(GetCertificateError::Internal(err)) => return Err(err),
                }

                // 2) Realm key rotation ? (i.e. first workspace bootstrap step)

                let query = GetCertificateQuery::realm_key_rotation_certificates(&realm_id);
                let outcome = self.storage.get_certificate_encrypted(query, UpTo::Current).await;
                match outcome {
                    Ok(_) => {
                        break 'get_state RealmBootstrapState::InitialKeyRotationDone;
                    },
                    Err(GetCertificateError::NonExisting)
                    | Err(GetCertificateError::ExistButTooRecent { .. }) => (),
                    Err(GetCertificateError::Internal(err)) => return Err(err),
                }

                // 3) Realm role certificate ?

                let query = GetCertificateQuery::realm_role_certificates(&realm_id);
                let outcome = self.storage.get_certificate_encrypted(query, UpTo::Current).await;
                match outcome {
                    Ok(_) => {
                        break 'get_state RealmBootstrapState::CreatedInServer;
                    },
                    Err(GetCertificateError::NonExisting)
                    | Err(GetCertificateError::ExistButTooRecent { .. }) => (),
                    Err(GetCertificateError::Internal(err)) => return Err(err),
                }

                RealmBootstrapState::LocalOnly
            };

            self.store
                .current_view_cache
                .lock()
                .expect("Mutex is poisoned !")
                .per_realm_bootstrap_state
                .insert(realm_id, state);

            Ok(state)
        }

        #[allow(unused)]
        /// Return all certificates involving the realm.
        ///
        /// ⚠️ This method returns *all* certificates, including the ones that have been
        /// superseded ! (i.e. you should iter from the last to the first certificate to
        /// get the current role of a given user).
        ///
        /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
        pub async fn get_realm_roles(
            &mut self,
            up_to: UpTo,
            realm_id: VlobID,
        ) -> anyhow::Result<Vec<Arc<RealmRoleCertificate>>> {
            let query = GetCertificateQuery::realm_role_certificates(&realm_id);
            let items = self
                .storage
                .get_multiple_certificates_encrypted(query, up_to, None, None)
                .await?;
            get_multiple_certificates_from_encrypted(
                &self.store,
                items,
                RealmRoleCertificate::unsecure_load,
                UnsecureRealmRoleCertificate::skip_validation,
            )
        }

        /// For the given realm, return the last role certificate for each user still
        /// part of it (i.e. if the last role certificate for a given user is an
        /// unsharing one, then the user will be discarded).
        #[allow(unused)]
        pub async fn get_realm_current_users_roles(
            &mut self,
            up_to: UpTo,
            realm_id: VlobID,
        ) -> anyhow::Result<HashMap<UserID, Arc<RealmRoleCertificate>>> {
            let all_certifs = self.get_realm_roles(up_to, realm_id).await?;
            let mut per_user_role = HashMap::new();
            for certif in all_certifs {
                match certif.role {
                    Some(_) => {
                        per_user_role.insert(certif.user_id, certif);
                    }
                    None => {
                        per_user_role.remove(&certif.user_id);
                    }
                }
            }
            Ok(per_user_role)
        }

        #[allow(unused)]
        pub async fn get_realm_last_name_certificate(
            &mut self,
            up_to: UpTo,
            realm_id: VlobID,
        ) -> anyhow::Result<Option<Arc<RealmNameCertificate>>> {
            let query = GetCertificateQuery::realm_name_certificates(&realm_id);
            let outcome = self.storage.get_certificate_encrypted(query, up_to).await;
            let encrypted = match outcome {
                Ok((_, encrypted)) => encrypted,
                Err(GetCertificateError::NonExisting)
                | Err(GetCertificateError::ExistButTooRecent { .. }) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            let certif = get_certificate_from_encrypted(
                self.store,
                &encrypted,
                RealmNameCertificate::unsecure_load,
                UnsecureRealmNameCertificate::skip_validation,
            )?;

            Ok(Some(certif))
        }

        #[allow(unused)]
        /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
        pub async fn get_realm_name_certificates(
            &mut self,
            up_to: UpTo,
            realm_id: VlobID,
        ) -> anyhow::Result<Vec<Arc<RealmNameCertificate>>> {
            let query = GetCertificateQuery::realm_name_certificates(&realm_id);
            let items = self
                .storage
                .get_multiple_certificates_encrypted(query, up_to, None, None)
                .await?;
            get_multiple_certificates_from_encrypted(
                &self.store,
                items,
                RealmNameCertificate::unsecure_load,
                UnsecureRealmNameCertificate::skip_validation,
            )
        }

        #[allow(unused)]
        pub async fn get_realm_last_key_rotation_certificate(
            &mut self,
            up_to: UpTo,
            realm_id: VlobID,
        ) -> anyhow::Result<Option<Arc<RealmKeyRotationCertificate>>> {
            let query = GetCertificateQuery::realm_key_rotation_certificates(&realm_id);
            let outcome = self.storage.get_certificate_encrypted(query, up_to).await;
            let encrypted = match outcome {
                Ok((_, encrypted)) => encrypted,
                Err(GetCertificateError::NonExisting)
                | Err(GetCertificateError::ExistButTooRecent { .. }) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            let certif = get_certificate_from_encrypted(
                self.store,
                &encrypted,
                RealmKeyRotationCertificate::unsecure_load,
                UnsecureRealmKeyRotationCertificate::skip_validation,
            )?;

            Ok(Some(certif))
        }

        #[allow(unused)]
        /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
        pub async fn get_realm_key_rotation_certificate(
            &mut self,
            up_to: UpTo,
            realm_id: VlobID,
            key_index: IndexInt,
        ) -> anyhow::Result<Option<Arc<RealmKeyRotationCertificate>>> {
            let query = GetCertificateQuery::realm_key_rotation_certificate(&realm_id, key_index);
            let outcome = self
                .storage
                .get_certificate_encrypted(query, UpTo::Current)
                .await;
            let encrypted = match outcome {
                Ok((_, encrypted)) => encrypted,
                Err(GetCertificateError::NonExisting) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
                // Cannot get this error with `UpTo::Current`
                Err(GetCertificateError::ExistButTooRecent { .. }) => unreachable!(),
            };

            let certif = get_certificate_from_encrypted(
                self.store,
                &encrypted,
                RealmKeyRotationCertificate::unsecure_load,
                UnsecureRealmKeyRotationCertificate::skip_validation,
            )?;

            Ok(Some(certif))
        }

        #[allow(unused)]
        /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
        pub async fn get_realm_key_rotation_certificates(
            &mut self,
            up_to: UpTo,
            realm_id: VlobID,
        ) -> anyhow::Result<Vec<Arc<RealmKeyRotationCertificate>>> {
            let query = GetCertificateQuery::realm_key_rotation_certificates(&realm_id);
            let items = self
                .storage
                .get_multiple_certificates_encrypted(query, up_to, None, None)
                .await?;
            let certifs = get_multiple_certificates_from_encrypted(
                &self.store,
                items,
                RealmKeyRotationCertificate::unsecure_load,
                UnsecureRealmKeyRotationCertificate::skip_validation,
            )?;

            // Sanity check to make sure the order of the certificates correspond to their key index
            for (certif, expected_key_index) in certifs.iter().zip(1..) {
                if certif.key_index != expected_key_index {
                    return Err(anyhow::anyhow!(
                        "Certificates local storage seems corrupted: some realm key rotation certificates are missing for realm {}", realm_id
                    ));
                }
            }

            Ok(certifs)
        }

        #[allow(unused)]
        pub async fn get_sequester_authority_certificate(
            &mut self,
        ) -> anyhow::Result<Option<Arc<SequesterAuthorityCertificate>>> {
            let query = GetCertificateQuery::sequester_authority_certificate();
            let outcome = self
                .storage
                .get_certificate_encrypted(query, UpTo::Current)
                .await;
            let encrypted = match outcome {
                Ok((_, encrypted)) => encrypted,
                Err(GetCertificateError::NonExisting) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
                // Cannot get this error with `UpTo::Current`
                Err(GetCertificateError::ExistButTooRecent { .. }) => unreachable!(),
            };

            let certif = get_certificate_from_encrypted(
                self.store,
                &encrypted,
                SequesterAuthorityCertificate::unsecure_load,
                UnsecureSequesterAuthorityCertificate::skip_validation,
            )?;

            Ok(Some(certif))
        }

        #[allow(unused)]
        /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
        pub async fn get_sequester_service_certificates(
            &mut self,
            up_to: UpTo,
        ) -> anyhow::Result<Vec<Arc<SequesterServiceCertificate>>> {
            let query = GetCertificateQuery::sequester_service_certificates();
            let maybe_authority = self.get_sequester_authority_certificate().await?;
            let authority = match maybe_authority {
                Some(authority) => authority,
                // Not a sequestered organization
                None => return Ok(vec![]),
            };
            let items = self
                .storage
                .get_multiple_certificates_encrypted(query, up_to, None, None)
                .await?;
            get_multiple_sequester_authority_signed_certificates_from_encrypted(
                &self.store,
                &authority.verify_key_der,
                items,
                SequesterServiceCertificate::verify_and_load,
            )
            .await
        }

        #[allow(unused)]
        pub async fn get_sequester_revoked_service_certificates(
            &mut self,
            up_to: UpTo,
        ) -> anyhow::Result<Vec<Arc<SequesterRevokedServiceCertificate>>> {
            let query = GetCertificateQuery::sequester_revoked_service_certificates();
            let maybe_authority = self.get_sequester_authority_certificate().await?;
            let authority = match maybe_authority {
                Some(authority) => authority,
                // Not a sequestered organization
                None => return Ok(vec![]),
            };
            let items = self
                .storage
                .get_multiple_certificates_encrypted(query, up_to, None, None)
                .await?;
            get_multiple_sequester_authority_signed_certificates_from_encrypted(
                &self.store,
                &authority.verify_key_der,
                items,
                SequesterRevokedServiceCertificate::verify_and_load,
            )
            .await
        }

        #[allow(unused)]
        pub async fn get_sequester_revoked_service_certificate(
            &mut self,
            up_to: UpTo,
            service_id: SequesterServiceID,
        ) -> anyhow::Result<Option<Arc<SequesterRevokedServiceCertificate>>> {
            let query = GetCertificateQuery::sequester_revoked_service_certificate(&service_id);
            let outcome = self.storage.get_certificate_encrypted(query, up_to).await;

            let encrypted = match outcome {
                Ok((_, encrypted)) => encrypted,
                Err(
                    GetCertificateError::NonExisting
                    | GetCertificateError::ExistButTooRecent { .. },
                ) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            let maybe_authority = self.get_sequester_authority_certificate().await?;
            let authority = match maybe_authority {
                Some(authority) => authority,
                // Not a sequestered organization
                None => return Ok(None),
            };

            get_sequester_authority_signed_certificate_from_encrypted(
                self.store,
                &authority.verify_key_der,
                &encrypted,
                SequesterRevokedServiceCertificate::verify_and_load,
            )
            .map(Some)
        }

        #[allow(unused)]
        /// Return all shamir recovery brief certificates.
        ///
        /// ⚠️ This method returns *all* certificates, including the ones that have been
        /// superseded ! (i.e. you should iter from the last to the first certificate to
        /// get the current role of a given user).
        ///
        /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
        pub async fn get_shamir_recovery_briefs(
            &mut self,
            up_to: UpTo,
        ) -> anyhow::Result<Vec<Arc<ShamirRecoveryBriefCertificate>>> {
            let query = GetCertificateQuery::shamir_recovery_brief_certificates();
            let items = self
                .storage
                .get_multiple_certificates_encrypted(query, up_to, None, None)
                .await?;
            get_multiple_certificates_from_encrypted(
                &self.store,
                items,
                ShamirRecoveryBriefCertificate::unsecure_load,
                UnsecureShamirRecoveryBriefCertificate::skip_validation,
            )
        }

        #[allow(unused)]
        /// Return all shamir recovery deletion certificates.
        ///
        /// ⚠️ This method returns *all* certificates, including the ones that have been
        /// superseded ! (i.e. you should iter from the last to the first certificate to
        /// get the current role of a given user).
        ///
        /// Certificates are returned ordered by timestamp in increasing order (i.e. oldest first)
        pub async fn get_shamir_recovery_deletions(
            &mut self,
            up_to: UpTo,
        ) -> anyhow::Result<Vec<Arc<ShamirRecoveryDeletionCertificate>>> {
            let query = GetCertificateQuery::shamir_recovery_deletion_certificates();
            let items = self
                .storage
                .get_multiple_certificates_encrypted(query, up_to, None, None)
                .await?;
            get_multiple_certificates_from_encrypted(
                &self.store,
                items,
                ShamirRecoveryDeletionCertificate::unsecure_load,
                UnsecureShamirRecoveryDeletionCertificate::skip_validation,
            )
        }

        #[allow(unused)]
        pub async fn get_last_shamir_recovery_brief_certificate(
            &mut self,
            up_to: UpTo,
        ) -> anyhow::Result<Option<Arc<ShamirRecoveryBriefCertificate>>> {
            let query = GetCertificateQuery::shamir_recovery_brief_certificates();
            // `get_certificate_encrypted` return the last certificate if multiple are available
            let outcome = self.storage.get_certificate_encrypted(query, up_to).await;
            let encrypted = match outcome {
                Ok((_, encrypted)) => encrypted,
                Err(
                    GetCertificateError::NonExisting
                    | GetCertificateError::ExistButTooRecent { .. },
                ) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            let signed = self
                .store
                .device
                .local_symkey
                .decrypt(&encrypted)
                .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

            let unsecure = ShamirRecoveryBriefCertificate::unsecure_load(signed.into())
                .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

            let certif =
                unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);

            Ok(Some(Arc::new(certif)))
        }

        #[allow(unused)]
        pub async fn get_last_shamir_recovery_brief_certificate_for_author(
            &mut self,
            up_to: UpTo,
            author: &UserID,
        ) -> anyhow::Result<Option<Arc<ShamirRecoveryBriefCertificate>>> {
            let query = GetCertificateQuery::user_shamir_recovery_brief_certificates(author);
            // `get_certificate_encrypted` returns the last certificate if multiple are available
            let outcome = self.storage.get_certificate_encrypted(query, up_to).await;
            let encrypted = match outcome {
                Ok((_, encrypted)) => encrypted,
                Err(
                    GetCertificateError::NonExisting
                    | GetCertificateError::ExistButTooRecent { .. },
                ) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            let signed = self
                .store
                .device
                .local_symkey
                .decrypt(&encrypted)
                .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

            let unsecure = ShamirRecoveryBriefCertificate::unsecure_load(signed.into())
                .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

            let certif =
                unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);

            Ok(Some(Arc::new(certif)))
        }

        #[allow(unused)]
        pub async fn get_last_shamir_recovery_share_certificate_for_recipient(
            &mut self,
            up_to: UpTo,
            author: UserID,
            recipient: UserID,
        ) -> anyhow::Result<Option<Arc<ShamirRecoveryShareCertificate>>> {
            let query = GetCertificateQuery::user_recipient_shamir_recovery_share_certificates(
                &author, &recipient,
            );
            // `get_certificate_encrypted` return the last certificate if multiple are available
            let outcome = self.storage.get_certificate_encrypted(query, up_to).await;
            let encrypted = match outcome {
                Ok((_, encrypted)) => encrypted,
                Err(
                    GetCertificateError::NonExisting
                    | GetCertificateError::ExistButTooRecent { .. },
                ) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            let signed = self
                .store
                .device
                .local_symkey
                .decrypt(&encrypted)
                .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

            let unsecure = ShamirRecoveryShareCertificate::unsecure_load(signed.into())
                .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

            let certif =
                unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);

            Ok(Some(Arc::new(certif)))
        }

        #[allow(unused)]
        pub async fn get_last_shamir_recovery_deletion_certificate_for_author(
            &mut self,
            up_to: UpTo,
            author: &UserID,
        ) -> anyhow::Result<Option<Arc<ShamirRecoveryDeletionCertificate>>> {
            let query = GetCertificateQuery::user_shamir_recovery_deletion_certificates(author);
            // `get_certificate_encrypted` returns the last certificate if multiple are available
            let outcome = self.storage.get_certificate_encrypted(query, up_to).await;
            let encrypted = match outcome {
                Ok((_, encrypted)) => encrypted,
                Err(
                    GetCertificateError::NonExisting
                    | GetCertificateError::ExistButTooRecent { .. },
                ) => return Ok(None),
                Err(GetCertificateError::Internal(err)) => return Err(err),
            };

            let signed = self
                .store
                .device
                .local_symkey
                .decrypt(&encrypted)
                .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

            let unsecure = ShamirRecoveryDeletionCertificate::unsecure_load(signed.into())
                .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

            let certif =
                unsecure.skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);

            Ok(Some(Arc::new(certif)))
        }

        #[allow(unused)]
        pub async fn get_last_shamir_recovery_for_author(
            &mut self,
            up_to: UpTo,
            user_id: UserID,
        ) -> anyhow::Result<LastShamirRecovery> {
            let last_brief_certif = match self
                .get_last_shamir_recovery_brief_certificate_for_author(up_to, &user_id)
                .await?
            {
                Some(certif) => certif,
                // The user never had any shamir recovery configured
                None => return Ok(LastShamirRecovery::NeverSetup),
            };

            let last_shamir_recovery = match self
                .get_last_shamir_recovery_deletion_certificate_for_author(up_to, &user_id)
                .await?
            {
                // The user has never deleted any shamir recovery, so the one we have is valid !
                None => LastShamirRecovery::Valid(last_brief_certif),
                Some(last_deletion_certif) => {
                    if last_deletion_certif.setup_to_delete_timestamp == last_brief_certif.timestamp {
                        // The last shamir recovery has been deleted, so the user currently has no shamir recovery
                        LastShamirRecovery::Deleted(last_brief_certif, last_deletion_certif)
                    } else {
                        // The last removal was about an older shamir recovery, we can ignore it
                        LastShamirRecovery::Valid(last_brief_certif)
                    }
                }
            };

            Ok(last_shamir_recovery)
        }

        #[allow(unused)]
        pub fn get_realm_keys(
            &self,
            realm_id: VlobID,
        ) -> Option<Arc<RealmKeys>> {
            let guard = self
                .store
                .current_view_cache
                .lock()
                .expect("Mutex is poisoned !");
            guard.per_realm_keys.get(&realm_id).cloned()
        }

        #[allow(unused)]
        pub fn update_cache_for_realm_keys(
            &self,
            realm_id: VlobID,
            info: Arc<RealmKeys>,
        ) -> Arc<RealmKeys> {
            let mut guard = self
                .store
                .current_view_cache
                .lock()
                .expect("Mutex is poisoned");
            match guard.per_realm_keys.entry(realm_id) {
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(info.clone());
                    info
                }
                // Outdated entry
                std::collections::hash_map::Entry::Occupied(mut entry)
                    if entry.get().key_index() < info.key_index() =>
                {
                    *entry.get_mut() = info.clone();
                    info
                }
                // Up-to-date entry, nothing to do
                std::collections::hash_map::Entry::Occupied(entry) => entry.get().clone(),
            }
        }
    };
}

pub(super) struct CertificatesStoreReadGuard<'a> {
    store: &'a CertificatesStore,
    storage: &'a mut CertificatesStorage,
}

impl CertificatesStoreReadGuard<'_> {
    impl_read_methods!();
}

#[clippy::has_significant_drop]
pub(super) struct CertificatesStoreWriteGuard<'a> {
    store: &'a CertificatesStore,
    storage: CertificatesStorageUpdater<'a>,
}

impl CertificatesStoreWriteGuard<'_> {
    pub async fn forget_all_certificates(&mut self) -> anyhow::Result<()> {
        self.storage.forget_all_certificates().await?;
        self.store
            .current_view_cache
            .lock()
            .expect("Mutex is poisoned !")
            .clear();
        Ok(())
    }

    pub async fn add_next_common_certificate(
        &mut self,
        certif: CommonTopicArcCertificate,
        signed: &[u8],
    ) -> anyhow::Result<()> {
        let encrypted = self.store.device.local_symkey.encrypt(signed);

        let update_timestamp_cache =
            |cache: &mut CurrentViewCache, timestamp| match &mut cache.per_topic_last_timestamps {
                ScalarCache::Present(last_timestamps) => last_timestamps.common = Some(timestamp),
                cache @ ScalarCache::Miss => {
                    cache.set(PerTopicLastTimestamps {
                        common: Some(timestamp),
                        sequester: None,
                        realm: HashMap::default(),
                        shamir_recovery: None,
                    });
                }
            };

        match certif {
            CommonTopicArcCertificate::User(certif) => {
                self.storage.add_certificate(&*certif, encrypted).await?;

                // Update cache

                let mut cache = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned");
                update_timestamp_cache(&mut cache, certif.timestamp);
                cache
                    .per_user_profile
                    .insert(certif.user_id, certif.profile);
                if certif.user_id == self.store.device.user_id {
                    cache.self_profile.set(certif.profile);
                }
            }
            CommonTopicArcCertificate::Device(certif) => {
                self.storage.add_certificate(&*certif, encrypted).await?;

                // Update cache

                let mut cache = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned");
                update_timestamp_cache(&mut cache, certif.timestamp);
                cache.per_device_verify_key_and_user_id.insert(
                    certif.device_id,
                    (certif.verify_key.clone(), certif.timestamp, certif.user_id),
                );
            }
            CommonTopicArcCertificate::UserUpdate(certif) => {
                self.storage.add_certificate(&*certif, encrypted).await?;

                // Update cache

                let mut cache = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned");
                update_timestamp_cache(&mut cache, certif.timestamp);
                cache
                    .per_user_profile
                    .insert(certif.user_id, certif.new_profile);
                if certif.user_id == self.store.device.user_id {
                    cache.self_profile.set(certif.new_profile);
                }
            }
            CommonTopicArcCertificate::RevokedUser(certif) => {
                self.storage.add_certificate(&*certif, encrypted).await?;

                // Update cache

                let mut cache = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned");
                update_timestamp_cache(&mut cache, certif.timestamp);
            }
        }

        Ok(())
    }

    pub async fn add_next_realm_x_certificate(
        &mut self,
        certif: RealmTopicArcCertificate,
        signed: &[u8],
    ) -> anyhow::Result<()> {
        let encrypted = self.store.device.local_symkey.encrypt(signed);
        let (realm_id, timestamp, new_keys_bundle) = match certif {
            RealmTopicArcCertificate::RealmRole(certif) => {
                self.storage.add_certificate(&*certif, encrypted).await?;
                (certif.realm_id, certif.timestamp, false)
            }
            RealmTopicArcCertificate::RealmName(certif) => {
                self.storage.add_certificate(&*certif, encrypted).await?;
                (certif.realm_id, certif.timestamp, false)
            }
            RealmTopicArcCertificate::RealmKeyRotation(certif) => {
                self.storage.add_certificate(&*certif, encrypted).await?;
                (certif.realm_id, certif.timestamp, true)
            }
            RealmTopicArcCertificate::RealmArchiving(certif) => {
                self.storage.add_certificate(&*certif, encrypted).await?;
                (certif.realm_id, certif.timestamp, false)
            }
        };

        // Update the cache

        let mut guard = self
            .store
            .current_view_cache
            .lock()
            .expect("Mutex is poisoned");

        match &mut guard.per_topic_last_timestamps {
            ScalarCache::Present(last_timestamps) => {
                last_timestamps.realm.insert(realm_id, timestamp);
            }
            cache @ ScalarCache::Miss => {
                cache.set(PerTopicLastTimestamps {
                    common: None,
                    sequester: None,
                    realm: HashMap::from([(realm_id, timestamp)]),
                    shamir_recovery: None,
                });
            }
        }

        // By always discarding those caches, we trade simplicity for performances,
        // however it is no big deal given new certificates within a given realm
        // shouldn't occur that often.
        guard.per_realm_self_role.remove(&realm_id);
        guard.per_realm_bootstrap_state.remove(&realm_id);

        // Discard proactively the keys bundle cache in case of a key rotation.
        // This is to ensure we won't try to encrypt data with the wrong key
        // (i.e. not the very last one).
        if new_keys_bundle {
            guard.per_realm_keys.remove(&realm_id);
        }

        Ok(())
    }

    pub async fn add_next_shamir_recovery_certificate(
        &mut self,
        certif: ShamirRecoveryTopicArcCertificate,
        signed: &[u8],
    ) -> anyhow::Result<()> {
        let encrypted = self.store.device.local_symkey.encrypt(signed);
        let timestamp = match certif {
            ShamirRecoveryTopicArcCertificate::ShamirRecoveryBrief(certif) => {
                self.storage.add_certificate(&*certif, encrypted).await?;
                certif.timestamp
            }
            ShamirRecoveryTopicArcCertificate::ShamirRecoveryShare(certif) => {
                self.storage.add_certificate(&*certif, encrypted).await?;
                certif.timestamp
            }
            ShamirRecoveryTopicArcCertificate::ShamirRecoveryDeletion(certif) => {
                self.storage.add_certificate(&*certif, encrypted).await?;
                certif.timestamp
            }
        };

        let mut guard = self
            .store
            .current_view_cache
            .lock()
            .expect("Mutex is poisoned");
        match &mut guard.per_topic_last_timestamps {
            ScalarCache::Present(last_timestamps) => {
                last_timestamps.shamir_recovery = Some(timestamp)
            }
            cache @ ScalarCache::Miss => {
                cache.set(PerTopicLastTimestamps {
                    common: None,
                    sequester: None,
                    realm: HashMap::default(),
                    shamir_recovery: Some(timestamp),
                });
            }
        }

        Ok(())
    }

    pub async fn add_next_sequester_certificate(
        &mut self,
        certif: SequesterTopicArcCertificate,
        signed: &[u8],
    ) -> anyhow::Result<()> {
        let encrypted = self.store.device.local_symkey.encrypt(signed);
        let timestamp = match certif {
            SequesterTopicArcCertificate::SequesterAuthority(certif) => {
                self.storage.add_certificate(&*certif, encrypted).await?;
                certif.timestamp
            }
            SequesterTopicArcCertificate::SequesterService(certif) => {
                self.storage.add_certificate(&*certif, encrypted).await?;
                certif.timestamp
            }
            SequesterTopicArcCertificate::SequesterRevokedService(certif) => {
                self.storage.add_certificate(&*certif, encrypted).await?;
                certif.timestamp
            }
        };

        let mut guard = self
            .store
            .current_view_cache
            .lock()
            .expect("Mutex is poisoned");
        match &mut guard.per_topic_last_timestamps {
            ScalarCache::Present(last_timestamps) => last_timestamps.sequester = Some(timestamp),
            cache @ ScalarCache::Miss => {
                cache.set(PerTopicLastTimestamps {
                    common: None,
                    sequester: Some(timestamp),
                    realm: HashMap::default(),
                    shamir_recovery: None,
                });
            }
        }

        Ok(())
    }

    impl_read_methods!();
}

fn get_multiple_certificates_from_encrypted<T, U>(
    store: &CertificatesStore,
    items: Vec<(DateTime, Vec<u8>)>,
    unsecure_load: fn(Bytes) -> DataResult<U>,
    skip_validation: fn(U, UnsecureSkipValidationReason) -> T,
) -> anyhow::Result<Vec<Arc<T>>> {
    let mut certifs = Vec::with_capacity(items.len());

    for (_, encrypted) in items {
        let certif =
            get_certificate_from_encrypted(store, &encrypted, unsecure_load, skip_validation)?;
        certifs.push(certif);
    }

    Ok(certifs)
}

fn get_certificate_from_encrypted<T, U>(
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

async fn get_multiple_sequester_authority_signed_certificates_from_encrypted<T>(
    store: &CertificatesStore,
    authority_verify_key: &SequesterVerifyKeyDer,
    items: Vec<(DateTime, Vec<u8>)>,
    verify_and_load: fn(&[u8], &SequesterVerifyKeyDer) -> DataResult<T>,
) -> anyhow::Result<Vec<Arc<T>>> {
    let mut certifs = Vec::with_capacity(items.len());

    for (_, encrypted) in items {
        let certif = get_sequester_authority_signed_certificate_from_encrypted(
            store,
            authority_verify_key,
            &encrypted,
            verify_and_load,
        )?;
        certifs.push(certif);
    }

    Ok(certifs)
}

fn get_sequester_authority_signed_certificate_from_encrypted<T>(
    store: &CertificatesStore,
    authority_verify_key: &SequesterVerifyKeyDer,
    encrypted: &[u8],
    verify_and_load: fn(&[u8], &SequesterVerifyKeyDer) -> DataResult<T>,
) -> anyhow::Result<Arc<T>> {
    let signed = store
        .device
        .local_symkey
        .decrypt(encrypted)
        .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

    let certif = verify_and_load(&signed, authority_verify_key)
        .map_err(|e| anyhow::anyhow!("Local database contains invalid data: {}", e))?;

    Ok(Arc::new(certif))
}
