// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::cell::RefCell;
use std::collections::HashMap;

use crate::{build_signature_path, TrustchainError, TrustchainResult};
use libparsec_crypto::VerifyKey;
use libparsec_protocol::authenticated_cmds::v2::user_get::Trustchain;
use libparsec_types::{
    CertificateSignerOwned, CertificateSignerRef, DateTime, DeviceCertificate, DeviceID,
    RevokedUserCertificate, TimeProvider, UserCertificate, UserID, UserProfile,
};

#[derive(Debug)]
struct CertifState<'a, T> {
    certif: &'a [u8],
    content: T,
    verified: bool,
}

pub struct TrustchainContext {
    root_verify_key: VerifyKey,
    time_provider: TimeProvider,
    cache_validity: i64,
    _users_cache: HashMap<UserID, (DateTime, UserCertificate)>,
    _devices_cache: HashMap<DeviceID, (DateTime, DeviceCertificate)>,
    _revoked_users_cache: HashMap<UserID, (DateTime, RevokedUserCertificate)>,
}

impl TrustchainContext {
    fn verify_created_by_device(
        &self,
        users_states: &HashMap<UserID, RefCell<CertifState<UserCertificate>>>,
        revoked_users_states: &HashMap<UserID, RefCell<CertifState<RevokedUserCertificate>>>,
        author_device: &DeviceCertificate,
        verified_data: (&UserID, DateTime),
        author: &DeviceID,
        sign_chain: &[String],
    ) -> TrustchainResult<()> {
        // Author is either admin or signing one of it own devices
        if author_device.device_id.user_id() != verified_data.0 {
            match users_states.get(author.user_id()) {
                Some(certif_state)
                    if certif_state.borrow().content.profile != UserProfile::Admin =>
                {
                    return Err(TrustchainError::InvalidSignatureGiven {
                        path: build_signature_path(sign_chain),
                        user_id: author.user_id().clone(),
                    })
                }
                None => {
                    return Err(TrustchainError::MissingUserCertificate {
                        path: build_signature_path(sign_chain),
                        user_id: author.user_id().clone(),
                    })
                }
                _ => (),
            }
        }

        // Also make sure author wasn't revoked at creation time
        if let Some(author_revoked_user) = revoked_users_states.get(author.user_id()) {
            let author_revoked_user = &author_revoked_user.borrow().content;

            if verified_data.1 > author_revoked_user.timestamp {
                return Err(TrustchainError::SignaturePosteriorUserRevocation {
                    path: build_signature_path(sign_chain),
                    verified_timestamp: verified_data.1,
                    user_timestamp: author_revoked_user.timestamp,
                });
            }
        }

        Ok(())
    }

    fn recursive_verify_device(
        &self,
        users_states: &HashMap<UserID, RefCell<CertifState<UserCertificate>>>,
        devices_states: &HashMap<DeviceID, RefCell<CertifState<DeviceCertificate>>>,
        revoked_users_states: &HashMap<UserID, RefCell<CertifState<RevokedUserCertificate>>>,
        device_id: &DeviceID,
        sign_chain: &mut Vec<String>,
    ) -> TrustchainResult<DeviceCertificate> {
        let device_id_str = device_id.to_string();

        if sign_chain.contains(&device_id_str) {
            sign_chain.push(device_id_str);
            return Err(TrustchainError::InvalidSignatureLoopDetected {
                path: build_signature_path(sign_chain),
            });
        }

        sign_chain.push(device_id_str);

        let state = devices_states
            .get(device_id)
            .ok_or(TrustchainError::MissingDeviceCertificate {
                path: build_signature_path(sign_chain),
                device_id: device_id.clone(),
            })?
            .borrow();

        match &state.content.author {
            CertificateSignerOwned::Root => DeviceCertificate::verify_and_load(
                state.certif,
                &self.root_verify_key,
                CertificateSignerRef::Root,
                None,
            )
            .map_err(|exc| TrustchainError::InvalidCertificate {
                path: build_signature_path(sign_chain),
                exc: exc.to_string(),
            }),
            CertificateSignerOwned::User(author) => {
                let author_device = self.recursive_verify_device(
                    users_states,
                    devices_states,
                    revoked_users_states,
                    author,
                    sign_chain,
                )?;

                let verified = DeviceCertificate::verify_and_load(
                    state.certif,
                    &author_device.verify_key,
                    CertificateSignerRef::User(&author_device.device_id),
                    None,
                )
                .map_err(|exc| TrustchainError::InvalidCertificate {
                    path: build_signature_path(sign_chain),
                    exc: exc.to_string(),
                })?;

                self.verify_created_by_device(
                    users_states,
                    revoked_users_states,
                    &author_device,
                    (verified.device_id.user_id(), verified.timestamp),
                    author,
                    sign_chain,
                )?;

                Ok(verified)
            }
        }
    }

    fn verify_user(
        &self,
        users_states: &HashMap<UserID, RefCell<CertifState<UserCertificate>>>,
        devices_states: &HashMap<DeviceID, RefCell<CertifState<DeviceCertificate>>>,
        revoked_users_states: &HashMap<UserID, RefCell<CertifState<RevokedUserCertificate>>>,
        unverified_content: &UserCertificate,
        certif: &[u8],
    ) -> TrustchainResult<UserCertificate> {
        let user_id = &unverified_content.user_id;

        match &unverified_content.author {
            CertificateSignerOwned::Root => UserCertificate::verify_and_load(
                certif,
                &self.root_verify_key,
                CertificateSignerRef::Root,
                None,
                None,
            )
            .map_err(|exc| TrustchainError::InvalidCertificate {
                path: build_signature_path(&[format!("{user_id}'s creation")]),
                exc: exc.to_string(),
            }),
            CertificateSignerOwned::User(author) if author.user_id() == user_id => {
                Err(TrustchainError::InvalidSelfSignedUserCertificate {
                    user_id: user_id.clone(),
                })
            }
            CertificateSignerOwned::User(author) => {
                let mut sign_chain = vec![format!("{user_id}'s creation")];

                let author_device = self.recursive_verify_device(
                    users_states,
                    devices_states,
                    revoked_users_states,
                    author,
                    &mut sign_chain,
                )?;

                let verified = UserCertificate::verify_and_load(
                    certif,
                    &author_device.verify_key,
                    CertificateSignerRef::User(&author_device.device_id),
                    None,
                    None,
                )
                .map_err(|exc| TrustchainError::InvalidCertificate {
                    path: build_signature_path(&sign_chain),
                    exc: exc.to_string(),
                })?;

                self.verify_created_by_device(
                    users_states,
                    revoked_users_states,
                    &author_device,
                    (&verified.user_id, verified.timestamp),
                    author,
                    &sign_chain,
                )?;

                Ok(verified)
            }
        }
    }

    fn verify_revoked_user(
        &self,
        users_states: &HashMap<UserID, RefCell<CertifState<UserCertificate>>>,
        devices_states: &HashMap<DeviceID, RefCell<CertifState<DeviceCertificate>>>,
        revoked_users_states: &HashMap<UserID, RefCell<CertifState<RevokedUserCertificate>>>,
        unverified_content: &RevokedUserCertificate,
        certif: &[u8],
    ) -> TrustchainResult<RevokedUserCertificate> {
        let author = &unverified_content.author;
        let user_id = &unverified_content.user_id;

        if author.user_id() == user_id {
            Err(
                TrustchainError::InvalidSelfSignedUserRevocationCertificate {
                    user_id: user_id.clone(),
                },
            )
        } else {
            let mut sign_chain = vec![format!("{user_id}'s revocation")];

            let author_device = self.recursive_verify_device(
                users_states,
                devices_states,
                revoked_users_states,
                author,
                &mut sign_chain,
            )?;

            let verified = RevokedUserCertificate::verify_and_load(
                certif,
                &author_device.verify_key,
                &author_device.device_id,
                None,
            )
            .map_err(|exc| TrustchainError::InvalidCertificate {
                path: build_signature_path(&sign_chain),
                exc: exc.to_string(),
            })?;

            self.verify_created_by_device(
                users_states,
                revoked_users_states,
                &author_device,
                (&verified.user_id, verified.timestamp),
                author,
                &sign_chain,
            )?;

            Ok(verified)
        }
    }
}

impl TrustchainContext {
    pub fn new(
        root_verify_key: VerifyKey,
        time_provider: TimeProvider,
        cache_validity: i64,
    ) -> Self {
        Self {
            root_verify_key,
            time_provider,
            cache_validity,
            _users_cache: HashMap::new(),
            _devices_cache: HashMap::new(),
            _revoked_users_cache: HashMap::new(),
        }
    }

    pub fn cache_validity(&self) -> i64 {
        self.cache_validity
    }

    pub fn invalidate_user_cache(&mut self, user_id: &UserID) {
        self._users_cache.remove(user_id);
    }

    pub fn get_user(&self, user_id: &UserID) -> Option<&UserCertificate> {
        let now = self.time_provider.now();

        match self._users_cache.get(user_id) {
            Some((cached_on, verified_user))
                if (now - *cached_on).num_seconds() < self.cache_validity =>
            {
                Some(verified_user)
            }
            _ => None,
        }
    }

    pub fn get_revoked_user(&self, user_id: &UserID) -> Option<&RevokedUserCertificate> {
        let now = self.time_provider.now();

        match self._revoked_users_cache.get(user_id) {
            Some((cached_on, verified_revoked_user))
                if (now - *cached_on).num_seconds() < self.cache_validity =>
            {
                Some(verified_revoked_user)
            }
            _ => None,
        }
    }

    pub fn get_device(&self, device_id: &DeviceID) -> Option<&DeviceCertificate> {
        let now = self.time_provider.now();

        match self._devices_cache.get(device_id) {
            Some((cached_on, verified_device))
                if (now - *cached_on).num_seconds() < self.cache_validity =>
            {
                Some(verified_device)
            }
            _ => None,
        }
    }

    pub fn load_trustchain(
        &mut self,
        users: &[Vec<u8>],
        revoked_users: &[Vec<u8>],
        devices: &[Vec<u8>],
    ) -> TrustchainResult<(
        Vec<UserCertificate>,
        Vec<RevokedUserCertificate>,
        Vec<DeviceCertificate>,
    )> {
        let mut users_states = HashMap::new();
        let mut devices_states = HashMap::new();
        let mut revoked_users_states = HashMap::new();

        // Deserialize the certificates and filter the ones we already have in cache
        for certif in devices {
            let unverified_device =
                DeviceCertificate::unsecure_load(certif).unwrap_or_else(|_| unreachable!());
            match self.get_device(&unverified_device.device_id) {
                Some(verified_device) => {
                    devices_states.insert(
                        verified_device.device_id.clone(),
                        RefCell::new(CertifState {
                            certif,
                            content: verified_device.clone(),
                            verified: true,
                        }),
                    );
                }
                _ => {
                    devices_states.insert(
                        unverified_device.device_id.clone(),
                        RefCell::new(CertifState {
                            certif,
                            content: unverified_device,
                            verified: false,
                        }),
                    );
                }
            }
        }

        for certif in users {
            let unverified_user =
                UserCertificate::unsecure_load(certif).unwrap_or_else(|_| unreachable!());
            match self.get_user(&unverified_user.user_id) {
                Some(verified_user) => {
                    users_states.insert(
                        verified_user.user_id.clone(),
                        RefCell::new(CertifState {
                            certif,
                            content: verified_user.clone(),
                            verified: true,
                        }),
                    );
                }
                _ => {
                    users_states.insert(
                        unverified_user.user_id.clone(),
                        RefCell::new(CertifState {
                            certif,
                            content: unverified_user,
                            verified: false,
                        }),
                    );
                }
            }
        }

        for certif in revoked_users {
            let unverified_revoked_user =
                RevokedUserCertificate::unsecure_load(certif).unwrap_or_else(|_| unreachable!());
            match self.get_revoked_user(&unverified_revoked_user.user_id) {
                Some(verified_revoked_user) => {
                    revoked_users_states.insert(
                        verified_revoked_user.user_id.clone(),
                        RefCell::new(CertifState {
                            certif,
                            content: verified_revoked_user.clone(),
                            verified: true,
                        }),
                    );
                }
                _ => {
                    revoked_users_states.insert(
                        unverified_revoked_user.user_id.clone(),
                        RefCell::new(CertifState {
                            certif,
                            content: unverified_revoked_user,
                            verified: false,
                        }),
                    );
                }
            }
        }

        // Verified what need to be and populate the cache with them
        // RefCell is intended to be used here because it is borrowed twice
        for certif_state in devices_states.values() {
            let verified = certif_state.borrow().verified;
            if !verified {
                let content = self.recursive_verify_device(
                    &users_states,
                    &devices_states,
                    &revoked_users_states,
                    &certif_state.borrow().content.device_id,
                    &mut vec![],
                )?;
                certif_state.borrow_mut().content = content;
            }
        }
        for certif_state in users_states.values() {
            let verified = certif_state.borrow().verified;
            if !verified {
                let content = self.verify_user(
                    &users_states,
                    &devices_states,
                    &revoked_users_states,
                    &certif_state.borrow().content,
                    certif_state.borrow().certif,
                )?;
                certif_state.borrow_mut().content = content;
            }
        }
        for certif_state in revoked_users_states.values() {
            let verified = certif_state.borrow().verified;
            if !verified {
                let content = self.verify_revoked_user(
                    &users_states,
                    &devices_states,
                    &revoked_users_states,
                    &certif_state.borrow().content,
                    certif_state.borrow().certif,
                )?;
                certif_state.borrow_mut().content = content;
            }
        }

        // Finally populate the cache
        let now = self.time_provider.now();
        devices_states
            .values()
            .map(|state| state.borrow())
            .filter(|state| !state.verified)
            .for_each(|state| {
                self._devices_cache.insert(
                    state.content.device_id.clone(),
                    (now, state.content.clone()),
                );
            });
        users_states
            .values()
            .map(|state| state.borrow())
            .filter(|state| !state.verified)
            .for_each(|state| {
                self._users_cache
                    .insert(state.content.user_id.clone(), (now, state.content.clone()));
            });
        revoked_users_states
            .values()
            .map(|state| state.borrow())
            .filter(|state| !state.verified)
            .for_each(|state| {
                self._revoked_users_cache
                    .insert(state.content.user_id.clone(), (now, state.content.clone()));
            });

        let verified_users = users_states
            .into_values()
            .map(|state| state.into_inner().content)
            .collect();
        let verified_revoked_users = revoked_users_states
            .into_values()
            .map(|state| state.into_inner().content)
            .collect();
        let verified_devices = devices_states
            .into_values()
            .map(|state| state.into_inner().content)
            .collect();

        Ok((verified_users, verified_revoked_users, verified_devices))
    }

    pub fn load_user_and_devices(
        &mut self,
        mut trustchain: Trustchain,
        user_certif: Vec<u8>,
        revoked_user_certif: Option<Vec<u8>>,
        devices_certifs: Vec<Vec<u8>>,
        expected_user_id: Option<&UserID>,
    ) -> TrustchainResult<(
        UserCertificate,
        Option<RevokedUserCertificate>,
        Vec<DeviceCertificate>,
    )> {
        let UserCertificate { user_id, .. } =
            UserCertificate::unsecure_load(&user_certif).unwrap_or_else(|_| unreachable!());

        if let Some(expected_user_id) = expected_user_id {
            if expected_user_id != &user_id {
                return Err(TrustchainError::UnexpectedCertificate {
                    expected: expected_user_id.clone(),
                    got: user_id,
                });
            }
        }

        trustchain.users.push(user_certif);
        trustchain.devices.extend(devices_certifs);

        let revoked_user_certif_is_some = match revoked_user_certif {
            Some(revoked_user_certif) => {
                trustchain.revoked_users.push(revoked_user_certif);
                true
            }
            None => false,
        };

        let (verified_users, verified_revoked_users, verified_devices) = self.load_trustchain(
            &trustchain.users,
            &trustchain.revoked_users,
            &trustchain.devices,
        )?;

        Ok((
            verified_users
                .into_iter()
                .find(|user| user.user_id == user_id)
                .unwrap_or_else(|| unreachable!()),
            if revoked_user_certif_is_some {
                verified_revoked_users
                    .into_iter()
                    .find(|user| user.user_id == user_id)
            } else {
                None
            },
            verified_devices
                .into_iter()
                .filter(|device| device.device_id.user_id() == &user_id)
                .collect(),
        ))
    }
}
