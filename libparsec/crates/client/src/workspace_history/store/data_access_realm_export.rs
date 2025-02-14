// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, path::Path, sync::Arc};

use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_platform_realm_export::{
    RealmExportDB, RealmExportDBFetchBlockError, RealmExportDBFetchCertificatesError,
    RealmExportDBFetchKeysBundleAccessError, RealmExportDBFetchKeysBundleError,
    RealmExportDBFetchManifestError, RealmExportDBStartError,
};
use libparsec_types::prelude::*;

use super::{
    DataAccessFetchBlockError, DataAccessFetchManifestError, WorkspaceHistoryRealmExportDecryptor,
};
use crate::{certif::EncrytionUsage, InvalidBlockAccessError, InvalidManifestError};

pub(super) struct RealmExportDataAccess {
    db: AsyncMutex<RealmExportDB>,
    realm_id: VlobID,
    per_device_verify_key: HashMap<DeviceID, VerifyKey>,
    realm_keys: HashMap<IndexInt, KeyDerivation>,
}

pub(super) type RealmExportDataAccessStartError = RealmExportDBStartError;

async fn decrypt_keys_bundle_with_user(
    db: &mut RealmExportDB,
    key_index: IndexInt,
    encrypted_keys_bundle: &[u8],
    user_id: UserID,
    private_key: &PrivateKey,
) -> Result<Option<Vec<u8>>, RealmExportDataAccessStartError> {
    // User can have multiple keys bundle access for a given realm ID & key
    // index pair (since a new access is provided each time a sharing is done).
    let mut skip_accesses = 0;
    loop {
        let outcome = db
            .fetch_keys_bundle_accesses_for_user(key_index, user_id, skip_accesses)
            .await;
        let encrypted_access = match outcome {
            Ok(encrypted_access) => encrypted_access,
            // We have tried all the access for this user, none are valid :'(
            Err(RealmExportDBFetchKeysBundleAccessError::AccessNotFound) => return Ok(None),
            Err(RealmExportDBFetchKeysBundleAccessError::InvalidDatabase(e)) => {
                return Err(RealmExportDataAccessStartError::InvalidDatabase(e))
            }
        };

        let outcome = private_key
            .decrypt_from_self(&encrypted_access)
            .ok()
            .and_then(|raw| RealmKeysBundleAccess::load(&raw).ok());
        let access = match outcome {
            Some(access) => access,
            // Invalid access, try the next one
            None => {
                skip_accesses += 1;
                continue;
            }
        };

        // The access appears to be valid, now let's try to use it...
        match access.keys_bundle_key.decrypt(encrypted_keys_bundle) {
            // All good \o/
            Ok(decrypted_keys_bundle) => return Ok(Some(decrypted_keys_bundle)),
            // Invalid access, try the next one
            Err(_) => {
                skip_accesses += 1;
                continue;
            }
        }
    }
}

async fn decrypt_keys_bundle_with_sequester(
    db: &mut RealmExportDB,
    key_index: IndexInt,
    encrypted_keys_bundle: &[u8],
    sequester_service_id: SequesterServiceID,
    private_key: &SequesterPrivateKeyDer,
) -> Result<Option<Vec<u8>>, RealmExportDataAccessStartError> {
    let outcome = db
        .fetch_keys_bundle_accesses_for_sequester(key_index, sequester_service_id)
        .await;
    let encrypted_access = match outcome {
        Ok(encrypted_access) => encrypted_access,
        // This sequester service has no access to this key bundle
        Err(RealmExportDBFetchKeysBundleAccessError::AccessNotFound) => return Ok(None),
        Err(RealmExportDBFetchKeysBundleAccessError::InvalidDatabase(e)) => {
            return Err(RealmExportDataAccessStartError::InvalidDatabase(e))
        }
    };

    let outcome = private_key
        .decrypt(&encrypted_access)
        .ok()
        .and_then(|raw| RealmKeysBundleAccess::load(&raw).ok());
    let access = match outcome {
        Some(access) => access,
        // Invalid access :'(
        None => return Ok(None),
    };

    // The access appears to be valid, now let's try to use it...
    match access.keys_bundle_key.decrypt(encrypted_keys_bundle) {
        // All good \o/
        Ok(decrypted_keys_bundle) => Ok(Some(decrypted_keys_bundle)),
        // Invalid access :'(
        Err(_) => Ok(None),
    }
}

async fn decrypt_keys_bundle(
    db: &mut RealmExportDB,
    key_index: IndexInt,
    decryptors: &[WorkspaceHistoryRealmExportDecryptor],
) -> Result<Option<RealmKeysBundle>, RealmExportDataAccessStartError> {
    let encrypted_keys_bundle = match db.fetch_encrypted_keys_bundle(key_index).await {
        Ok(encrypted_keys_bundle) => encrypted_keys_bundle,
        Err(RealmExportDBFetchKeysBundleError::BundleNotFound) => return Ok(None),
        Err(RealmExportDBFetchKeysBundleError::InvalidDatabase(err)) => {
            return Err(RealmExportDataAccessStartError::InvalidDatabase(err))
        }
    };

    let mut decryptors_iter = decryptors.iter();
    loop {
        let decryptor = match decryptors_iter.next() {
            Some(decryptor) => decryptor,
            None => {
                // We have tried all the decryptors, none of them worked :'(
                return Ok(None);
            }
        };

        let maybe_decrypted_keys_bundle = match decryptor {
            WorkspaceHistoryRealmExportDecryptor::User {
                user_id,
                private_key,
            } => {
                decrypt_keys_bundle_with_user(
                    db,
                    key_index,
                    &encrypted_keys_bundle,
                    *user_id,
                    private_key,
                )
                .await?
            }
            WorkspaceHistoryRealmExportDecryptor::SequesterService {
                sequester_service_id,
                private_key,
            } => {
                decrypt_keys_bundle_with_sequester(
                    db,
                    key_index,
                    &encrypted_keys_bundle,
                    *sequester_service_id,
                    private_key,
                )
                .await?
            }
        };

        let decrypted_keys_bundle = match maybe_decrypted_keys_bundle {
            Some(decrypted_keys_bundle) => decrypted_keys_bundle,
            // Maybe we will have more luck with another decryptor...
            None => continue,
        };

        return match RealmKeysBundle::base_unsecure_load(&decrypted_keys_bundle) {
            Ok(keys_bundle) => Ok(Some(keys_bundle)),
            // The keys bundle itself is invalid !
            Err(_) => Ok(None),
        };
    }
}

async fn load_device_verify_keys(
    db: &mut RealmExportDB,
    root_verify_key: &VerifyKey,
) -> Result<HashMap<DeviceID, VerifyKey>, RealmExportDataAccessStartError> {
    let raw_certificates = db.fetch_common_certificates().await.map_err(|e| match e {
        RealmExportDBFetchCertificatesError::InvalidDatabase(error) => {
            RealmExportDataAccessStartError::InvalidDatabase(error)
        }
    })?;

    let mut per_device_verify_key: HashMap<DeviceID, VerifyKey> = HashMap::new();
    for raw_certificate in raw_certificates {
        let unsecure = match DeviceCertificate::unsecure_load(raw_certificate.into()) {
            Ok(unsecure) => unsecure,
            // Not a device certificate, just skip it
            Err(_) => continue,
        };

        let author_verify_key = match unsecure.author() {
            CertificateSignerOwned::User(device_id) => {
                match per_device_verify_key.get(&device_id) {
                    Some(key) => key,
                    None => {
                        log::warn!(
                            "Ignoring device certificate signed by unknown device: {:?}",
                            unsecure
                        );
                        continue;
                    }
                }
            }
            CertificateSignerOwned::Root => root_verify_key,
        };

        let certificate = match unsecure.verify_signature(author_verify_key) {
            Ok((certificate, _)) => certificate,
            Err((unsecure, _)) => {
                log::warn!(
                    "Ignoring device certificate with invalid signature: {:?}",
                    unsecure
                );
                continue;
            }
        };

        per_device_verify_key.insert(certificate.device_id, certificate.verify_key);
    }

    Ok(per_device_verify_key)
}

async fn load_realm_keys(
    db: &mut RealmExportDB,
    per_device_verify_key: &HashMap<DeviceID, VerifyKey>,
    decryptors: &[WorkspaceHistoryRealmExportDecryptor],
) -> Result<HashMap<IndexInt, KeyDerivation>, RealmExportDataAccessStartError> {
    // First we need to retrieve all the realm key rotation certificates

    let raw_certificates = db.fetch_realm_certificates().await.map_err(|e| match e {
        RealmExportDBFetchCertificatesError::InvalidDatabase(error) => {
            RealmExportDataAccessStartError::InvalidDatabase(error)
        }
    })?;
    let mut key_rotation_certificates = vec![];
    for raw_certificate in raw_certificates.into_iter() {
        let unsecure = match RealmKeyRotationCertificate::unsecure_load(raw_certificate.into()) {
            Ok(unsecure) => unsecure,
            // Not a key rotation certificate, just skip it
            Err(_) => continue,
        };

        let author_verify_key = match per_device_verify_key.get(&unsecure.author()) {
            Some(key) => key,
            None => {
                log::warn!(
                    "Ignoring key rotation certificate signed by unknown device: {:?}",
                    unsecure
                );
                continue;
            }
        };

        let certificate = match unsecure.verify_signature(author_verify_key) {
            Ok((certificate, _)) => certificate,
            Err((unsecure, _)) => {
                log::warn!(
                    "Ignoring key rotation certificate with invalid signature: {:?}",
                    unsecure
                );
                continue;
            }
        };

        key_rotation_certificates.push(certificate);
    }

    // Each key rotation add one new key, our goal here is to retrieve as much as possible.
    // Note:
    // - Thanks to the canary in each key rotation certificate, we know when a key is valid,
    //   so we can finish early if we end up with all keys.
    // - However it is not always possible to retrieve *all* keys, because some might have
    //   always been invalid (e.g. due to a buggy client doing the rotation).
    //
    // So the strategy is to start by the latest keys bundle (as it name implies, a keys
    // bundle contains all the realm's keys that exist at it creation time) which should
    // give us most (if not all) keys.
    // Then, for each missing key, we try to decrypt the keys bundle corresponding to
    // this key index (as subsequent keys bundle are supposed to simply copy the existing
    // keys from their previous keys bundle).
    // In this process, we just ignore invalid keys bundles and keys not matching their
    // corresponding canary.

    let mut per_index_key: HashMap<IndexInt, KeyDerivation> = HashMap::new();
    let mut per_index_canary: HashMap<_, _> = key_rotation_certificates
        .into_iter()
        .map(|c| (c.key_index, c.key_canary))
        .collect();

    while let Some(latest_remaining_key_index) = per_index_canary.keys().max().copied() {
        let keys_bundle =
            match decrypt_keys_bundle(db, latest_remaining_key_index, decryptors).await? {
                Some(keys_bundle) => keys_bundle,
                None => {
                    // None of our decryptors are able to access this keys bundle, or
                    // the keys bundle itself is invalid.
                    // Since we process the keys bundles in reverse order, we are
                    // guaranteed that we will never be able to obtain the key
                    // corresponding to this key index (i.e. all remaining keys bundles
                    // have been created before the key rotation corresponding to the
                    // current key index occurred).
                    per_index_canary.remove(&latest_remaining_key_index);
                    continue;
                }
            };

        // Now try to load as much keys as possible from this keys bundle
        for (key, key_index) in keys_bundle.keys().iter().zip(1u64..) {
            match per_index_canary.entry(key_index) {
                std::collections::hash_map::Entry::Occupied(entry) => {
                    let canary = entry.get();
                    // If the key in the keys bundle is invalid, ignore it and
                    // hope another keys bundle will provide a valid one...
                    if key
                        .derive_secret_key_from_uuid(CANARY_KEY_DERIVATION_UUID)
                        .decrypt(canary)
                        .is_ok()
                    {
                        per_index_key.insert(key_index, key.to_owned());
                        // Update `per_index_canary` so that it only contains the
                        // keys that remains to be retrieved.
                        entry.remove();
                    }
                }
                // The key for this key index is already known, nothing more to do
                std::collections::hash_map::Entry::Vacant(_) => (),
            }
        }
    }

    Ok(per_index_key)
}

impl RealmExportDataAccess {
    pub async fn start(
        export_db_path: &Path,
        decryptors: Vec<WorkspaceHistoryRealmExportDecryptor>,
    ) -> Result<(Self, OrganizationID, VlobID, DateTime), RealmExportDataAccessStartError> {
        let (mut db, organization_id, realm_id, root_verify_key, timestamp_higher_bound) =
            RealmExportDB::start(export_db_path).await?;

        let per_device_verify_key = load_device_verify_keys(&mut db, &root_verify_key).await?;
        let realm_keys = load_realm_keys(&mut db, &per_device_verify_key, &decryptors).await?;

        Ok((
            Self {
                db: AsyncMutex::new(db),
                realm_id,
                per_device_verify_key,
                realm_keys,
            },
            organization_id,
            realm_id,
            timestamp_higher_bound,
        ))
    }

    pub async fn fetch_manifest(
        &self,
        at: DateTime,
        entry_id: VlobID,
    ) -> Result<ArcChildManifest, DataAccessFetchManifestError> {
        let (author, timestamp, version, key_index, encrypted) = {
            let mut db = self.db.lock().await;
            db.fetch_encrypted_manifest(at, entry_id)
                .await
                .map_err(|e| match e {
                    RealmExportDBFetchManifestError::EntryNotFound => {
                        DataAccessFetchManifestError::EntryNotFound
                    }
                    error @ RealmExportDBFetchManifestError::InvalidDatabase(_) => {
                        DataAccessFetchManifestError::Internal(error.into())
                    }
                })?
        };

        let manifest = if entry_id != self.realm_id {
            let manifest = self.decrypt_and_validate_manifest(
                entry_id,
                author,
                timestamp,
                version,
                key_index,
                &encrypted,
                ChildManifest::verify_and_load,
            )?;
            match manifest {
                ChildManifest::File(manifest) => Arc::new(manifest).into(),
                ChildManifest::Folder(manifest) => Arc::new(manifest).into(),
            }
        } else {
            let manifest = self.decrypt_and_validate_manifest(
                entry_id,
                author,
                timestamp,
                version,
                key_index,
                &encrypted,
                WorkspaceManifest::verify_and_load,
            )?;
            ArcChildManifest::Folder(Arc::new(manifest.into()))
        };

        Ok(manifest)
    }

    pub async fn get_workspace_manifest_v1(
        &self,
    ) -> Result<Arc<FolderManifest>, DataAccessFetchManifestError> {
        let (author, timestamp, key_index, encrypted) = {
            let mut db = self.db.lock().await;
            db.get_encrypted_workspace_manifest_v1()
                .await
                .map_err(|e| match e {
                    RealmExportDBFetchManifestError::EntryNotFound => {
                        DataAccessFetchManifestError::EntryNotFound
                    }
                    error @ RealmExportDBFetchManifestError::InvalidDatabase(_) => {
                        DataAccessFetchManifestError::Internal(error.into())
                    }
                })?
        };
        let entry_id = self.realm_id;
        let version = 1;

        let manifest = self.decrypt_and_validate_manifest(
            entry_id,
            author,
            timestamp,
            version,
            key_index,
            &encrypted,
            WorkspaceManifest::verify_and_load,
        )?;

        Ok(Arc::new(manifest.into()))
    }

    #[allow(clippy::too_many_arguments)]
    fn decrypt_and_validate_manifest<M>(
        &self,
        entry_id: VlobID,
        author: DeviceID,
        timestamp: DateTime,
        version: VersionInt,
        key_index: IndexInt,
        encrypted: &[u8],
        verify_and_load: impl FnOnce(
            &[u8],
            &VerifyKey,
            DeviceID,
            DateTime,
            Option<VlobID>,
            Option<VersionInt>,
        ) -> DataResult<M>,
    ) -> Result<M, DataAccessFetchManifestError> {
        let key = match self.realm_keys.get(&key_index) {
            None => {
                return Err(DataAccessFetchManifestError::InvalidManifest(Box::new(
                    // Note here we don't know if the key index doesn't exist or if
                    // we couldn't retrieve it.
                    InvalidManifestError::NonExistentKeyIndex {
                        realm: self.realm_id,
                        vlob: entry_id,
                        version,
                        author,
                        timestamp,
                        key_index,
                    },
                )));
            }
            Some(key) => {
                let usage = EncrytionUsage::Vlob(entry_id);
                key.derive_secret_key_from_uuid(usage.key_derivation_uuid())
            }
        };

        let author_verify_key = match self.per_device_verify_key.get(&author) {
            None => {
                return Err(DataAccessFetchManifestError::InvalidManifest(Box::new(
                    InvalidManifestError::NonExistentAuthor {
                        realm: self.realm_id,
                        vlob: entry_id,
                        version,
                        author,
                        timestamp,
                    },
                )));
            }
            Some(key) => key,
        };

        let signed = match key.decrypt(encrypted) {
            Ok(signed) => signed,
            Err(_) => {
                return Err(DataAccessFetchManifestError::InvalidManifest(Box::new(
                    InvalidManifestError::CannotDecrypt {
                        realm: self.realm_id,
                        vlob: entry_id,
                        version,
                        author,
                        timestamp,
                        key_index,
                    },
                )));
            }
        };

        let manifest = match verify_and_load(
            &signed,
            author_verify_key,
            author,
            timestamp,
            Some(entry_id),
            Some(version),
        ) {
            Ok(manifest) => manifest,
            Err(err) => {
                return Err(DataAccessFetchManifestError::InvalidManifest(Box::new(
                    InvalidManifestError::CleartextCorrupted {
                        realm: self.realm_id,
                        vlob: entry_id,
                        version,
                        author,
                        timestamp,
                        error: err.into(),
                    },
                )));
            }
        };

        Ok(manifest)
    }

    pub async fn fetch_block(
        &self,
        manifest: &FileManifest,
        access: &BlockAccess,
    ) -> Result<Bytes, DataAccessFetchBlockError> {
        let (key_index, encrypted) = {
            let mut db = self.db.lock().await;
            db.fetch_encrypted_block(access.id)
                .await
                .map_err(|e| match e {
                    RealmExportDBFetchBlockError::BlockNotFound => {
                        DataAccessFetchBlockError::BlockNotFound
                    }
                    error @ RealmExportDBFetchBlockError::InvalidDatabase(_) => {
                        DataAccessFetchBlockError::Internal(error.into())
                    }
                })?
        };

        let key = match self.realm_keys.get(&key_index) {
            None => {
                return Err(DataAccessFetchBlockError::InvalidBlockAccess(Box::new(
                    // Note here we don't know if the key index doesn't exist or if
                    // we couldn't retrieve it.
                    InvalidBlockAccessError::NonExistentKeyIndex {
                        realm_id: self.realm_id,
                        manifest_id: manifest.id,
                        manifest_version: manifest.version,
                        manifest_timestamp: manifest.timestamp,
                        manifest_author: manifest.author,
                        block_id: access.id,
                        key_index,
                    },
                )));
            }
            Some(key) => {
                let usage = EncrytionUsage::Block(access.id);
                key.derive_secret_key_from_uuid(usage.key_derivation_uuid())
            }
        };

        let block: Bytes = match key.decrypt(&encrypted) {
            Ok(cleartext) => cleartext.into(),
            Err(_) => {
                return Err(DataAccessFetchBlockError::InvalidBlockAccess(Box::new(
                    InvalidBlockAccessError::CannotDecrypt {
                        realm_id: self.realm_id,
                        manifest_id: manifest.id,
                        manifest_version: manifest.version,
                        manifest_timestamp: manifest.timestamp,
                        manifest_author: manifest.author,
                        block_id: access.id,
                        key_index,
                    },
                )));
            }
        };

        if block.len() != access.size.get() as usize {
            return Err(DataAccessFetchBlockError::InvalidBlockAccess(Box::new(
                InvalidBlockAccessError::SizeMismatch {
                    realm_id: self.realm_id,
                    manifest_id: manifest.id,
                    manifest_version: manifest.version,
                    manifest_timestamp: manifest.timestamp,
                    manifest_author: manifest.author,
                    block_id: access.id,
                },
            )));
        }

        if HashDigest::from_data(&block) != access.digest {
            return Err(DataAccessFetchBlockError::InvalidBlockAccess(Box::new(
                InvalidBlockAccessError::HashDigestMismatch {
                    realm_id: self.realm_id,
                    manifest_id: manifest.id,
                    manifest_version: manifest.version,
                    manifest_timestamp: manifest.timestamp,
                    manifest_author: manifest.author,
                    block_id: access.id,
                },
            )));
        }

        Ok(block)
    }
}
