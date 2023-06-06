// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex, MutexGuard},
};

use libparsec_platform_storage2::certificates::{self as storage};
use libparsec_types::prelude::*;

#[derive(Debug)]
enum ScalarCache<T> {
    Miss,
    Present(T),
}

impl<T> Default for ScalarCache<T> {
    fn default() -> Self {
        Self::Miss
    }
}

impl<T> ScalarCache<T> {
    fn set(&mut self, new: T) -> Option<T> {
        match std::mem::replace(self, Self::Present(new)) {
            Self::Present(old) => Some(old),
            Self::Miss => None,
        }
    }
}

#[derive(Debug, Default)]
struct Cache {
    current_self_profile: ScalarCache<UserProfile>,
    user_certificates: HashMap<UserID, (IndexInt, Arc<UserCertificate>)>,
    user_update_certificates: HashMap<UserID, Arc<Vec<(IndexInt, Arc<UserUpdateCertificate>)>>>,
    revoked_user_certificates: HashMap<UserID, Option<(IndexInt, Arc<RevokedUserCertificate>)>>,
    device_certificates: HashMap<DeviceID, (IndexInt, Arc<DeviceCertificate>)>,
    realm_certificates: HashMap<RealmID, Arc<Vec<(IndexInt, Arc<RealmRoleCertificate>)>>>,
    sequester_service_certificates: ScalarCache<Vec<(IndexInt, Arc<SequesterServiceCertificate>)>>,
    sequester_authority_certificate: ScalarCache<(IndexInt, Arc<SequesterAuthorityCertificate>)>,
    per_certificate_index_timestamp_bounds: HashMap<IndexInt, (DateTime, DateTime)>,
    last_certificate_lower_timestamp_bound: ScalarCache<(IndexInt, DateTime)>,
}

#[derive(Debug)]
pub(super) struct CertificatesCachedStorage {
    device: Arc<LocalDevice>,
    cache: Mutex<Cache>,
    storage: storage::CertificatesStorage,
}

#[derive(Debug, thiserror::Error)]
pub(super) enum GetCertificateError {
    #[error("Certificate doesn't exist")]
    NonExisting,
    #[error("Certificate exists but is more recent than the provided index")]
    ExistButTooRecent {
        certificate_index: IndexInt,
        certificate_timestamp: DateTime,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub(super) enum GetTimestampBoundsError {
    #[error("Certificate index doesn't exist")]
    NonExisting,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub(super) enum AnyArcCertificate {
    User(Arc<UserCertificate>),
    Device(Arc<DeviceCertificate>),
    UserUpdate(Arc<UserUpdateCertificate>),
    RevokedUser(Arc<RevokedUserCertificate>),
    RealmRole(Arc<RealmRoleCertificate>),
    SequesterAuthority(Arc<SequesterAuthorityCertificate>),
    SequesterService(Arc<SequesterServiceCertificate>),
}

pub(super) enum UpTo {
    Current,
    Index(IndexInt),
}

impl CertificatesCachedStorage {
    pub async fn start(data_base_dir: &Path, device: Arc<LocalDevice>) -> anyhow::Result<Self> {
        let storage = storage::CertificatesStorage::start(data_base_dir, &device).await?;
        // let current_profile = storage.get_
        Ok(Self {
            device,
            cache: Default::default(),
            storage,
        })
    }

    pub async fn stop(&self) {
        self.storage.stop().await
    }

    fn update_last_certificate_lower_timestamp_bound(
        &self,
        guard: &mut MutexGuard<Cache>,
        new_last_index: IndexInt,
        new_last_timestamp: DateTime,
    ) {
        if let Some((previous_last_index, previous_last_timestamp)) = guard
            .last_certificate_lower_timestamp_bound
            .set((new_last_index, new_last_timestamp))
        {
            guard.per_certificate_index_timestamp_bounds.insert(
                previous_last_index,
                (previous_last_timestamp, new_last_timestamp),
            );
        }
    }

    pub async fn forget_all_certificates(&mut self) -> anyhow::Result<()> {
        // Clear the database
        self.storage.forget_all_certificates().await?;

        // Clear the cache (done last to makes sure it has not been concurrently
        // repopulated while we were clearing the database)
        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        *guard = Default::default();

        Ok(())
    }

    pub async fn get_current_self_profile(&mut self) -> anyhow::Result<UserProfile> {
        {
            let guard = self.cache.lock().expect("Mutex is poisoned");
            if let ScalarCache::Present(profile) = guard.current_self_profile {
                return Ok(profile);
            }
        }

        // Cache miss !

        // Updates are ordered by index, so last one contains the current profile
        let updates = self
            .get_user_update_certificates(UpTo::Current, self.device.user_id())
            .await?;
        let profile = if let Some(last_update) = updates.last() {
            last_update.new_profile
        } else {
            // No update, so our current role is still the one we got at enrollment
            self.device.initial_profile
        };

        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        guard.current_self_profile.set(profile);

        return Ok(profile);
    }

    pub async fn add_user_certificate(
        &mut self,
        certificate_index: IndexInt,
        cooked: Arc<UserCertificate>,
        serialized: Bytes,
    ) -> anyhow::Result<()> {
        let encrypted = self.device.local_symkey.encrypt(&serialized);

        self.storage
            .add_user_certificate(
                certificate_index,
                cooked.timestamp,
                cooked.user_id.clone(),
                encrypted,
            )
            .await?;

        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        self.update_last_certificate_lower_timestamp_bound(
            &mut guard,
            certificate_index,
            cooked.timestamp,
        );
        guard
            .user_certificates
            .insert(cooked.user_id.clone(), (certificate_index, cooked));

        Ok(())
    }

    pub async fn add_revoked_user_certificate(
        &mut self,
        certificate_index: IndexInt,
        cooked: Arc<RevokedUserCertificate>,
        serialized: Bytes,
    ) -> anyhow::Result<()> {
        let encrypted = self.device.local_symkey.encrypt(&serialized);

        self.storage
            .add_revoked_user_certificate(
                certificate_index,
                cooked.timestamp,
                cooked.user_id.clone(),
                encrypted,
            )
            .await?;

        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        self.update_last_certificate_lower_timestamp_bound(
            &mut guard,
            certificate_index,
            cooked.timestamp,
        );
        guard
            .revoked_user_certificates
            .insert(cooked.user_id.clone(), Some((certificate_index, cooked)));

        Ok(())
    }

    pub async fn add_user_update_certificate(
        &mut self,
        certificate_index: IndexInt,
        cooked: Arc<UserUpdateCertificate>,
        serialized: Bytes,
    ) -> anyhow::Result<()> {
        let encrypted = self.device.local_symkey.encrypt(&serialized);

        self.storage
            .add_user_update_certificate(
                certificate_index,
                cooked.timestamp,
                cooked.user_id.clone(),
                encrypted,
            )
            .await?;

        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        self.update_last_certificate_lower_timestamp_bound(
            &mut guard,
            certificate_index,
            cooked.timestamp,
        );

        // Cache keeps all updates for a given user
        match guard
            .user_update_certificates
            .entry(cooked.user_id.to_owned())
        {
            std::collections::hash_map::Entry::Occupied(mut updates) => {
                let new = Arc::make_mut(updates.get_mut());
                new.push((certificate_index, cooked));
            }
            // No cache for this user, don't add our certificate otherwise the cache
            // will appear as if there is only this certificate for the user !
            std::collections::hash_map::Entry::Vacant(_) => (),
        }

        Ok(())
    }

    pub async fn add_device_certificate(
        &mut self,
        certificate_index: IndexInt,
        cooked: Arc<DeviceCertificate>,
        serialized: Bytes,
    ) -> anyhow::Result<()> {
        let encrypted = self.device.local_symkey.encrypt(&serialized);

        self.storage
            .add_device_certificate(
                certificate_index,
                cooked.timestamp,
                cooked.device_id.clone(),
                encrypted,
            )
            .await?;

        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        self.update_last_certificate_lower_timestamp_bound(
            &mut guard,
            certificate_index,
            cooked.timestamp,
        );
        guard
            .device_certificates
            .insert(cooked.device_id.clone(), (certificate_index, cooked));

        Ok(())
    }

    pub async fn add_realm_role_certificate(
        &mut self,
        certificate_index: IndexInt,
        cooked: Arc<RealmRoleCertificate>,
        serialized: Bytes,
    ) -> anyhow::Result<()> {
        let encrypted = self.device.local_symkey.encrypt(&serialized);

        self.storage
            .add_realm_role_certificate(
                certificate_index,
                cooked.timestamp,
                cooked.realm_id,
                cooked.user_id.clone(),
                encrypted,
            )
            .await?;

        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        self.update_last_certificate_lower_timestamp_bound(
            &mut guard,
            certificate_index,
            cooked.timestamp,
        );

        // Cache keeps all roles for a given realm
        match guard.realm_certificates.entry(cooked.realm_id) {
            std::collections::hash_map::Entry::Occupied(mut roles) => {
                let new = Arc::make_mut(roles.get_mut());
                new.push((certificate_index, cooked));
            }
            // No cache for this realm, don't add our certificate otherwise the cache
            // will appear as if the realm only have a single role !
            std::collections::hash_map::Entry::Vacant(_) => (),
        }

        Ok(())
    }

    pub async fn add_sequester_authority_certificate(
        &mut self,
        certificate_index: IndexInt,
        cooked: Arc<SequesterAuthorityCertificate>,
        serialized: Bytes,
    ) -> anyhow::Result<()> {
        let encrypted = self.device.local_symkey.encrypt(&serialized);

        self.storage
            .add_sequester_authority_certificate(certificate_index, cooked.timestamp, encrypted)
            .await?;

        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        self.update_last_certificate_lower_timestamp_bound(
            &mut guard,
            certificate_index,
            cooked.timestamp,
        );
        guard
            .sequester_authority_certificate
            .set((certificate_index, cooked));

        Ok(())
    }

    pub async fn add_sequester_service_certificate(
        &mut self,
        certificate_index: IndexInt,
        cooked: Arc<SequesterServiceCertificate>,
        serialized: Bytes,
    ) -> anyhow::Result<()> {
        let encrypted = self.device.local_symkey.encrypt(&serialized);

        self.storage
            .add_sequester_service_certificate(
                certificate_index,
                cooked.timestamp,
                cooked.service_id,
                encrypted,
            )
            .await?;

        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        self.update_last_certificate_lower_timestamp_bound(
            &mut guard,
            certificate_index,
            cooked.timestamp,
        );

        if let ScalarCache::Present(sequester_service_certificates) =
            &mut guard.sequester_service_certificates
        {
            sequester_service_certificates.push((certificate_index, cooked));
        }

        Ok(())
    }

    pub async fn get_certificate(
        &self,
        index: IndexInt,
    ) -> Result<AnyArcCertificate, GetCertificateError> {
        match self.storage.get_certificate(index).await? {
            None => Err(GetCertificateError::NonExisting),
            Some(encrypted) => {
                let signed = self
                    .device
                    .local_symkey
                    .decrypt(&encrypted)
                    .map_err(|e| anyhow::anyhow!(e))?;
                let unsecure =
                    AnyCertificate::unsecure_load(signed.into()).map_err(|e| anyhow::anyhow!(e))?;
                match unsecure {
                    UnsecureAnyCertificate::User(unsecure) => {
                        let certif = unsecure
                            .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                        Ok(AnyArcCertificate::User(Arc::new(certif)))
                    }
                    UnsecureAnyCertificate::Device(unsecure) => {
                        let certif = unsecure
                            .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                        Ok(AnyArcCertificate::Device(Arc::new(certif)))
                    }
                    UnsecureAnyCertificate::UserUpdate(unsecure) => {
                        let certif = unsecure
                            .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                        Ok(AnyArcCertificate::UserUpdate(Arc::new(certif)))
                    }
                    UnsecureAnyCertificate::RevokedUser(unsecure) => {
                        let certif = unsecure
                            .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                        Ok(AnyArcCertificate::RevokedUser(Arc::new(certif)))
                    }
                    UnsecureAnyCertificate::RealmRole(unsecure) => {
                        let certif = unsecure
                            .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                        Ok(AnyArcCertificate::RealmRole(Arc::new(certif)))
                    }
                    UnsecureAnyCertificate::SequesterAuthority(unsecure) => {
                        let certif = unsecure
                            .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                        Ok(AnyArcCertificate::SequesterAuthority(Arc::new(certif)))
                    }
                    UnsecureAnyCertificate::SequesterService(unsecure) => {
                        let certif = unsecure
                            .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                        Ok(AnyArcCertificate::SequesterService(Arc::new(certif)))
                    }
                }
            }
        }
    }

    pub async fn get_user_certificate(
        &self,
        up_to: UpTo,
        user_id: &UserID,
    ) -> Result<Arc<UserCertificate>, GetCertificateError> {
        let maybe = {
            let guard = self.cache.lock().expect("Mutex is poisoned");
            guard.user_certificates.get(user_id).map(|e| e.to_owned())
        };

        let (index, certif) = match maybe {
            Some(hit) => hit,
            // Cache miss !
            None => {
                let maybe = self
                    .storage
                    .get_user_certificate(user_id.to_owned())
                    .await?;
                match maybe {
                    None => return Err(GetCertificateError::NonExisting),
                    Some((index, encrypted)) => {
                        let signed = self
                            .device
                            .local_symkey
                            .decrypt(&encrypted)
                            .map_err(|e| anyhow::anyhow!(e))?;
                        let certif = UserCertificate::unsecure_load(signed.into())
                            .map_err(|e| anyhow::anyhow!(e))?
                            .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                        let certif = Arc::new(certif);

                        let mut guard = self.cache.lock().expect("Mutex is poisoned");
                        guard
                            .user_certificates
                            .insert(certif.user_id.clone(), (index, certif.clone()));

                        (index, certif)
                    }
                }
            }
        };

        match up_to {
            UpTo::Index(up_to_index) if index > up_to_index => {
                Err(GetCertificateError::ExistButTooRecent {
                    certificate_index: index,
                    certificate_timestamp: certif.timestamp,
                })
            }
            _ => Ok(certif),
        }
    }

    pub async fn get_revoked_user_certificate(
        &self,
        up_to: UpTo,
        user_id: &UserID,
    ) -> Result<Option<Arc<RevokedUserCertificate>>, anyhow::Error> {
        let maybe = {
            let guard = self.cache.lock().expect("Mutex is poisoned");
            guard
                .revoked_user_certificates
                .get(user_id)
                .map(|e| e.to_owned())
        };

        let info = match maybe {
            Some(hit) => hit,
            // Cache miss !
            None => {
                let info = self
                    .storage
                    .get_revoked_user_certificate(user_id.to_owned())
                    .await?;
                match info {
                    None => {
                        // User is not revoked
                        let mut guard = self.cache.lock().expect("Mutex is poisoned");
                        guard
                            .revoked_user_certificates
                            .insert(user_id.clone(), None);

                        None
                    }
                    Some((index, encrypted)) => {
                        let signed = self
                            .device
                            .local_symkey
                            .decrypt(&encrypted)
                            .map_err(|e| anyhow::anyhow!(e))?;
                        let certif = RevokedUserCertificate::unsecure_load(signed.into())
                            .map_err(|e| anyhow::anyhow!(e))?
                            .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                        let certif = Arc::new(certif);

                        let mut guard = self.cache.lock().expect("Mutex is poisoned");
                        guard
                            .revoked_user_certificates
                            .insert(user_id.clone(), Some((index, certif.clone())));

                        Some((index, certif))
                    }
                }
            }
        };

        match info {
            None => {
                // User is not revoked
                Ok(None)
            }
            Some((index, certif)) => {
                match up_to {
                    // User is currently revoked, but it wasn't the case at the considered index
                    UpTo::Index(up_to_index) if index > up_to_index => Ok(None),
                    // User revoked at the considered index
                    _ => Ok(Some(certif)),
                }
            }
        }
    }

    pub async fn get_user_update_certificates(
        &self,
        up_to: UpTo,
        user_id: &UserID,
    ) -> Result<Vec<Arc<UserUpdateCertificate>>, anyhow::Error> {
        let maybe = {
            let guard = self.cache.lock().expect("Mutex is poisoned");
            guard
                .user_update_certificates
                .get(user_id)
                .map(|e| e.to_owned())
        };

        let items = match maybe {
            Some(hit) => hit,
            // Cache miss !
            None => {
                let items = self
                    .storage
                    .get_user_update_certificates(user_id.clone())
                    .await?;
                let items = items
                    .into_iter()
                    .map(|(index, encrypted)| {
                        let signed = self
                            .device
                            .local_symkey
                            .decrypt(&encrypted)
                            .map_err(|e| anyhow::anyhow!(e))?;
                        let certif = UserUpdateCertificate::unsecure_load(signed.into())
                            .map_err(|e| anyhow::anyhow!(e))?
                            .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                        let certif = Arc::new(certif);
                        Ok((index, certif))
                    })
                    .collect::<Result<Vec<_>, anyhow::Error>>()?;
                let items = Arc::new(items);

                let mut guard = self.cache.lock().expect("Mutex is poisoned");
                guard
                    .user_update_certificates
                    .insert(user_id.clone(), items.clone());

                items
            }
        };

        let items = items
            .iter()
            .filter_map(|(index, certif)| match up_to {
                UpTo::Index(up_to_index) if *index > up_to_index => None,
                _ => Some(certif.clone()),
            })
            .collect();

        Ok(items)
    }

    pub async fn get_device_certificate(
        &self,
        up_to: UpTo,
        device_id: &DeviceID,
    ) -> Result<Arc<DeviceCertificate>, GetCertificateError> {
        let maybe = {
            let guard = self.cache.lock().expect("Mutex is poisoned");
            guard
                .device_certificates
                .get(device_id)
                .map(|e| e.to_owned())
        };

        let (index, certif) = match maybe {
            Some(hit) => hit,
            // Cache miss !
            None => {
                let maybe = self
                    .storage
                    .get_device_certificate(device_id.to_owned())
                    .await?;
                match maybe {
                    None => return Err(GetCertificateError::NonExisting),
                    Some((index, encrypted)) => {
                        let signed = self
                            .device
                            .local_symkey
                            .decrypt(&encrypted)
                            .map_err(|e| anyhow::anyhow!(e))?;
                        let certif = DeviceCertificate::unsecure_load(signed.into())
                            .map_err(|e| anyhow::anyhow!(e))?
                            .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                        let certif = Arc::new(certif);

                        let mut guard = self.cache.lock().expect("Mutex is poisoned");
                        guard
                            .device_certificates
                            .insert(certif.device_id.clone(), (index, certif.clone()));

                        (index, certif)
                    }
                }
            }
        };

        match up_to {
            UpTo::Index(up_to_index) if index > up_to_index => {
                Err(GetCertificateError::ExistButTooRecent {
                    certificate_index: index,
                    certificate_timestamp: certif.timestamp,
                })
            }
            _ => Ok(certif),
        }
    }

    pub async fn get_realm_certificates(
        &self,
        up_to: UpTo,
        realm_id: RealmID,
    ) -> Result<Vec<Arc<RealmRoleCertificate>>, anyhow::Error> {
        let maybe = {
            let guard = self.cache.lock().expect("Mutex is poisoned");
            guard
                .realm_certificates
                .get(&realm_id)
                .map(|e| e.to_owned())
        };

        let items = match maybe {
            Some(hit) => hit,
            // Cache miss !
            None => {
                let items = self.storage.get_realm_certificates(realm_id).await?;
                let items = items
                    .into_iter()
                    .map(|(index, encrypted)| {
                        let signed = self
                            .device
                            .local_symkey
                            .decrypt(&encrypted)
                            .map_err(|e| anyhow::anyhow!(e))?;
                        let certif = RealmRoleCertificate::unsecure_load(signed.into())
                            .map_err(|e| anyhow::anyhow!(e))?
                            .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                        let certif = Arc::new(certif);
                        Ok((index, certif))
                    })
                    .collect::<Result<Vec<_>, anyhow::Error>>()?;
                let items = Arc::new(items);

                let mut guard = self.cache.lock().expect("Mutex is poisoned");
                guard.realm_certificates.insert(realm_id, items.clone());

                items
            }
        };

        let items = items
            .iter()
            .filter_map(|(index, certif)| match up_to {
                UpTo::Index(up_to_index) if *index > up_to_index => None,
                _ => Some(certif.clone()),
            })
            .collect();

        Ok(items)
    }

    /// Get the current (i.e. at the given index) role for this user in this realm
    pub async fn get_user_realm_role(
        &self,
        up_to: UpTo,
        user_id: &UserID,
        realm_id: RealmID,
    ) -> Result<Option<Arc<RealmRoleCertificate>>, anyhow::Error> {
        let roles = self.get_realm_certificates(up_to, realm_id).await?;
        let user_role = roles.into_iter().find(|certif| certif.user_id == *user_id);
        Ok(user_role)
    }

    /// Get the current (i.e. at the given index) role for this user in each
    /// realm it is part of
    pub async fn get_user_realms_roles(
        &self,
        up_to: UpTo,
        user_id: UserID,
    ) -> Result<Vec<Arc<RealmRoleCertificate>>, anyhow::Error> {
        self.storage
            .get_user_realms_certificates(user_id)
            .await?
            .into_iter()
            .filter_map(|(index, encrypted)| match up_to {
                UpTo::Index(up_to_index) if index > up_to_index => None,
                _ => Some(encrypted),
            })
            .map(|encrypted| -> Result<_, _> {
                let signed = self
                    .device
                    .local_symkey
                    .decrypt(&encrypted)
                    .map_err(|e| anyhow::anyhow!(e))?;
                let certif = RealmRoleCertificate::unsecure_load(signed.into())
                    .map_err(|e| anyhow::anyhow!(e))?
                    .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                let certif = Arc::new(certif);
                Ok(certif)
            })
            .collect()
    }

    pub async fn get_sequester_authority_certificate(
        &self,
        up_to: UpTo,
    ) -> Result<Arc<SequesterAuthorityCertificate>, GetCertificateError> {
        let maybe = {
            let guard = self.cache.lock().expect("Mutex is poisoned");
            match &guard.sequester_authority_certificate {
                ScalarCache::Present(hit) => Some(hit.to_owned()),
                ScalarCache::Miss => None,
            }
        };

        let (index, certif) = match maybe {
            Some(hit) => hit,
            // Cache miss !
            None => {
                let maybe = self.storage.get_sequester_authority_certificate().await?;
                match maybe {
                    None => return Err(GetCertificateError::NonExisting),
                    Some((index, encrypted)) => {
                        let signed = self
                            .device
                            .local_symkey
                            .decrypt(&encrypted)
                            .map_err(|e| anyhow::anyhow!(e))?;
                        let certif = SequesterAuthorityCertificate::unsecure_load(signed.into())
                            .map_err(|e| anyhow::anyhow!(e))?
                            .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                        let certif = Arc::new(certif);

                        let mut guard = self.cache.lock().expect("Mutex is poisoned");
                        guard
                            .sequester_authority_certificate
                            .set((index, certif.clone()));

                        (index, certif)
                    }
                }
            }
        };

        match up_to {
            UpTo::Index(up_to_index) if index > up_to_index => {
                Err(GetCertificateError::ExistButTooRecent {
                    certificate_index: index,
                    certificate_timestamp: certif.timestamp,
                })
            }
            _ => Ok(certif),
        }
    }

    pub async fn get_sequester_service_certificates(
        &self,
        up_to: UpTo,
    ) -> Result<Vec<Arc<SequesterServiceCertificate>>, anyhow::Error> {
        let maybe = {
            let guard = self.cache.lock().expect("Mutex is poisoned");
            match &guard.sequester_service_certificates {
                ScalarCache::Present(hit) => Some(hit.to_owned()),
                ScalarCache::Miss => None,
            }
        };

        let items = match maybe {
            Some(hit) => hit,
            // Cache miss !
            None => {
                let items = self.storage.get_sequester_service_certificates().await?;
                let items = items
                    .into_iter()
                    .map(|(index, encrypted)| {
                        let signed = self
                            .device
                            .local_symkey
                            .decrypt(&encrypted)
                            .map_err(|e| anyhow::anyhow!(e))?;
                        let certif = SequesterServiceCertificate::unsecure_load(signed.into())
                            .map_err(|e| anyhow::anyhow!(e))?
                            .skip_validation(UnsecureSkipValidationReason::DataFromLocalStorage);
                        let certif = Arc::new(certif);
                        Ok((index, certif))
                    })
                    .collect::<Result<Vec<_>, anyhow::Error>>()?;

                let mut guard = self.cache.lock().expect("Mutex is poisoned");
                guard.sequester_service_certificates.set(items.clone());

                items
            }
        };

        let items = items
            .into_iter()
            .filter_map(|(index, certif)| match up_to {
                UpTo::Index(up_to_index) if index > up_to_index => None,
                _ => Some(certif),
            })
            .collect();

        Ok(items)
    }

    pub async fn get_timestamp_bounds(
        &self,
        certificate_index: IndexInt,
    ) -> Result<(DateTime, Option<DateTime>), GetTimestampBoundsError> {
        {
            let guard = self.cache.lock().expect("Mutex is poisoned");
            if let ScalarCache::Present((last_index, last_timestamp)) =
                &guard.last_certificate_lower_timestamp_bound
            {
                if certificate_index == *last_index {
                    return Ok((last_timestamp.to_owned(), None));
                }
            }
            if let Some((lower, upper)) = guard
                .per_certificate_index_timestamp_bounds
                .get(&certificate_index)
            {
                return Ok((lower.to_owned(), Some(upper.to_owned())));
            }
        }

        // Cache miss !
        let bounds = self.storage.get_timestamp_bounds(certificate_index).await?;
        let mut guard = self.cache.lock().expect("Mutex is poisoned");
        match bounds {
            (Some(lower), None) => {
                guard
                    .last_certificate_lower_timestamp_bound
                    .set((certificate_index, lower));
                Ok((lower, None))
            }
            (Some(lower), Some(upper)) => {
                guard
                    .per_certificate_index_timestamp_bounds
                    .insert(certificate_index, (lower, upper));
                Ok((lower, Some(upper)))
            }
            // Provided certificate index is too high !
            (None, _) => Err(GetTimestampBoundsError::NonExisting),
        }
    }

    pub async fn get_last_certificate_index(&self) -> anyhow::Result<Option<IndexInt>> {
        {
            let guard = self.cache.lock().expect("Mutex is poisoned");
            if let ScalarCache::Present((last_index, _)) =
                guard.last_certificate_lower_timestamp_bound
            {
                return Ok(Some(last_index));
            }
        }

        // Cache miss !
        let maybe = self.storage.get_last_index().await?;
        match maybe {
            None => {
                // Special case: we don't have a single certificate stored yet
                Ok(None)
            }
            Some((last_index, last_timestamp)) => {
                let mut guard = self.cache.lock().expect("Mutex is poisoned");
                guard
                    .last_certificate_lower_timestamp_bound
                    .set((last_index, last_timestamp));
                Ok(Some(last_index))
            }
        }
    }
}
