// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_platform_storage::certificates::{GetCertificateError, UpTo};
use libparsec_types::prelude::*;

use super::{encrypt::CertifEncryptForUserError, store::CertificatesStoreReadGuard, CertifOps};

#[derive(Debug)]
enum ValidatedKey {
    Valid {
        key: SecretKey,
        timestamp: DateTime,
    },
    /// Key has failed to decrypt the canary of the corresponding key rotation certificate.
    /// As its name implies, the corrupted key should not be used. But we need to keep it
    /// nevertheless to be able to generate the next keys bundle.
    Corrupted {
        corrupted_key_dont_use_me: SecretKey,
    },
}

#[derive(Debug)]
pub(super) struct RealmKeys {
    pub realm_id: VlobID,
    // Not public field: use `last_valid_key()` & `key_from_index()` methods to access
    // the keys (as this prevent us from accidentally using a corrupted key or writing
    // off-by-one bug when converting key index into vec index).
    keys: Vec<ValidatedKey>,
    // Once keys bundle decrypted, keys bundle access key is no longer needed.
    // However it is still useful to keep it around given we need it to transmit
    // it when sharing the workspace (and validating the keys bundle access fetched
    // from server is cumbersome and error prone, so better to do it once !).
    pub keys_bundle_access_key: SecretKey,
}

#[derive(Debug, thiserror::Error)]
pub(super) enum KeyFromIndexError {
    #[error("Key exists but is corrupted")]
    CorruptedKey,
    #[error("Key doesn't exist at this time")]
    KeyNotFound,
}

impl RealmKeys {
    // TODO: finish non-initial key rotation !
    #[allow(dead_code)]
    fn keys_with_indexes(&self) -> impl Iterator<Item = (&ValidatedKey, IndexInt)> {
        self.keys.iter().zip(1..)
    }

    pub fn key_index(&self) -> IndexInt {
        self.keys.len() as IndexInt
    }

    /// Return `None` if no valid key exists :(
    pub fn last_valid_key(&self) -> Option<(&SecretKey, IndexInt)> {
        self.keys
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, key)| match key {
                ValidatedKey::Valid { key, .. } => Some((key, (index + 1) as IndexInt)),
                ValidatedKey::Corrupted { .. } => None,
            })
    }

    pub fn key_from_index(
        &self,
        key_index: IndexInt,
        up_to: DateTime,
    ) -> Result<&SecretKey, KeyFromIndexError> {
        let key = (key_index as usize)
            .checked_sub(1)
            .and_then(|index| self.keys.get(index))
            .ok_or(KeyFromIndexError::KeyNotFound)?;

        match key {
            ValidatedKey::Valid { key, timestamp } if *timestamp <= up_to => Ok(key),
            ValidatedKey::Valid { .. } => Err(KeyFromIndexError::KeyNotFound),
            ValidatedKey::Corrupted { .. } => Err(KeyFromIndexError::CorruptedKey),
        }
    }

    // TODO: finish non-initial key rotation !
    #[allow(dead_code)]
    pub fn generate_next_key_bundle(
        &self,
        author: DeviceID,
        timestamp: DateTime,
        next_key: SecretKey,
    ) -> RealmKeysBundle {
        let keys = {
            let mut keys = Vec::with_capacity(self.keys.len() + 1);
            for validated_key in &self.keys {
                let key = match validated_key {
                    ValidatedKey::Valid { key, .. } => key,
                    // Keep the invalid keys as is. Who knows, maybe we are the buggy
                    // one and they are valid after all !
                    ValidatedKey::Corrupted {
                        corrupted_key_dont_use_me,
                    } => corrupted_key_dont_use_me,
                }
                .to_owned();
                keys.push(key);
            }
            keys.push(next_key);
            keys
        };
        RealmKeysBundle::new(author, timestamp, self.realm_id, keys)
    }
}

#[derive(Debug, thiserror::Error)]
pub(super) enum LoadLastKeysBundleError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error("The realm doesn't have any key yet")]
    NoKey,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for LoadLastKeysBundleError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            // TODO: handle organization expired and user revoked here ?
            err => Self::Internal(err.into()),
        }
    }
}

/// Load the keys from the last keys bundle for the given realm.
///
/// Note that the keys bundle might contains corrupted keys, or even might itself be
/// corrupted.
/// The idea here is to trust the last keys bundle author of having done his best to
/// provide us with valid keys (and if something is corrupted, an OWNER should do
/// a healing key rotation soon enough in theory).
pub(super) async fn load_last_realm_keys_bundle(
    ops: &CertifOps,
    store: &mut CertificatesStoreReadGuard<'_>,
    realm_id: VlobID,
) -> Result<Arc<RealmKeys>, LoadLastKeysBundleError> {
    if let Some(cache) = store.get_realm_keys(realm_id) {
        return Ok(cache);
    }

    // Cache miss !

    // 1) Retrieve the last key rotation certificate we know about

    let key_rotation_certificates = store
        .get_realm_key_rotation_certificates(UpTo::Current, realm_id)
        .await?;
    let key_rotation_certificate = key_rotation_certificates
        .last()
        .ok_or(LoadLastKeysBundleError::NoKey)?;
    if key_rotation_certificate.key_index as usize != key_rotation_certificates.len() {
        return Err(anyhow::anyhow!(
            "Local storage of certificates seems corrupted: expected {} certificates, but only got {}\nRetrieved certificates: {:?}",
            key_rotation_certificate.key_index,
            key_rotation_certificates.len(),
            key_rotation_certificates,
        ).into());
    }

    // 2) Fetch the corresponding keys bundle from the server

    let (keys_bundle, keys_bundle_access) = {
        use libparsec_protocol::authenticated_cmds::latest::realm_get_keys_bundle::{Rep, Req};

        let req = Req {
            realm_id,
            key_index: key_rotation_certificate.key_index,
        };
        let rep = ops.cmds.send(req).await?;
        match rep {
            Rep::Ok {
                keys_bundle,
                keys_bundle_access,
            } => (keys_bundle, keys_bundle_access),
            Rep::AccessNotAvailableForAuthor
            | Rep::AuthorNotAllowed => return Err(LoadLastKeysBundleError::NotAllowed),
            // Unexpected errors :(
            bad_rep @ (
                // We didn't specify a key index
                Rep::BadKeyIndex
                // Don't know what to do with this status :/
                | Rep::UnknownStatus { .. }
            ) => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
            }
        }
    };

    // 2) Validate it against the corresponding key rotation certificate
    let realm_keys = validate_keys_bundle(
        ops,
        store,
        realm_id,
        &keys_bundle,
        &keys_bundle_access,
        key_rotation_certificate,
        key_rotation_certificates
            .iter()
            .map(|certif| (certif.key_canary.as_ref(), certif.timestamp)),
    )
    .await
    .map_err(|e| match e {
        ValidateKeysBundleError::InvalidKeysBundle(err) => {
            LoadLastKeysBundleError::InvalidKeysBundle(err)
        }
        ValidateKeysBundleError::Internal(err) => err.into(),
    })?;

    // Don't forget to update the cache in the store (this also takes care of
    // concurrent update with more recent info, hence the return value).
    let realm_keys = store.update_cache_for_realm_keys(realm_id, realm_keys);

    Ok(realm_keys)
}

// TODO: finish key bundle healing !
#[allow(dead_code)]
pub enum KeysBundleHealingOutcome {
    /// Current last keys bundle is valid and contains only valid keys.
    NotNeeded,
    /// Current last keys bundle has some corruption, but the previous keys bundle
    /// we have access to aren't enough to fix it.
    NotPossible,
    /// Current last keys bundle had some corruption, that we were able to partially
    /// fix (i.e. there is still some invalid keys in the new keys bundle).
    PartialSuccess { still_broken_keys: Vec<IndexInt> },
    /// Current last keys bundle had some corruption, we fixed everything \o/
    TotalSuccess,
}

#[derive(Debug, thiserror::Error)]
pub(super) enum AttemptRealmKeysBundleHealingError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Not allowed to access this realm")]
    NotAllowed,
    // TODO: finish key bundle healing !
    #[allow(dead_code)]
    #[error("The realm doesn't have any key yet")]
    NoKey,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for AttemptRealmKeysBundleHealingError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            // TODO: handle organization expired and user revoked here ?
            err => Self::Internal(err.into()),
        }
    }
}

// /// Recursively load all keys bundle the user has access to in order to recover
// /// as much keys as possible.
// /// If the number of keys retrieved doesn't account for the number of key
// /// rotations, this function complete with dummy zeroized keys.
// ///
// /// This method is useful when the last keys bundle is corrupted, or if it contains
// /// corrupted keys.
// ///
// /// Note a user only have access to the keys bundles starting from the one that was
// /// the current one when he joined the realm. Hence the capacity to recover keys
// /// depend on the user doing the recovery.
// ///
// /// This method is typically used by a monitor.
// pub(super) async fn attempt_realm_keys_bundle_healing<'a>(
//     ops: &CertifOps,
//     realm_id: VlobID,
// ) -> Result<KeysBundleHealingOutcome, AttemptRealmKeysBundleHealingError> {
//     loop {
//         let outcome = ops.store.for_read(|store| async move {
//             recover_realm_keys_from_previous_bundles(ops, realm_id).await
//         }).await??;

//         if matches!(outcome, KeysBundleHealingOutcome::NotNeeded | KeysBundleHealingOutcome::NotPossible) {
//             return Ok(outcome);
//         }

//     }
// }

// TODO: finish key bundle healing !
#[allow(dead_code)]
async fn recover_realm_keys_from_previous_bundles(
    ops: &CertifOps,
    store: &mut CertificatesStoreReadGuard<'_>,
    realm_id: VlobID,
) -> Result<KeysBundleHealingOutcome, AttemptRealmKeysBundleHealingError> {
    // 1) Retrieve all key rotation certificates, we will need them all given
    //    each key has to be validated

    // Note `get_realm_key_rotation_certificates` already guarantees the certificates
    // are sorted according to their key index.
    let key_rotation_certificates = store
        .get_realm_key_rotation_certificates(UpTo::Current, realm_id)
        .await?;

    // 2) Start with all keys unknown, and then progressively recover them from the
    //    keys bundles.

    let mut recovered_keys = (0..key_rotation_certificates.len())
        .map(|_| None)
        .collect::<Vec<_>>();

    // At the end we will compare our healing to the last realm keys bundle to determine
    // if we have been able to recover some keys (and in this case we will proceed with
    // a key rotation).
    enum FetchLastKeysBundle {
        NotYet,
        Done(Arc<RealmKeys>),
        Corrupted,
    }
    let mut last_realm_keys_bundle = FetchLastKeysBundle::NotYet;

    // Note we progress backward (last certificate first) given 1) it's most likely the
    // last certificate contains most key (if not all !), and 2) we only have access to
    // certificates starting from the one that was the current one when we joined.
    for key_rotation_certificate in key_rotation_certificates.iter().rev() {
        // 3) Fetch the corresponding keys bundle

        let (keys_bundle, keys_bundle_access) = {
            use libparsec_protocol::authenticated_cmds::latest::realm_get_keys_bundle::{Rep, Req};

            let req = Req {
                realm_id,
                key_index: key_rotation_certificate.key_index,
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok {
                    keys_bundle,
                    keys_bundle_access,
                    ..
                } => (keys_bundle, keys_bundle_access),

                Rep::AuthorNotAllowed => {
                    return Err(AttemptRealmKeysBundleHealingError::NotAllowed)
                }
                // This key rotation has been done while we were not part of the realm,
                // so we have no choice but to skip it.
                // Not we still continue to fetch the previous keys bundle, as we might
                // have been temporary part of the realm in the past.
                Rep::AccessNotAvailableForAuthor => continue,
                Rep::BadKeyIndex => {
                    return Err(anyhow::anyhow!(
                        "Unexpected server response: server considers the key index invalid while we got it from an actual certificate !"
                    ).into());
                }
                // Nothing much we can do with this status :/
                bad_rep @ Rep::UnknownStatus { .. } => {
                    return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
                }
            }
        };

        // 4) Validate the keys bundle against the corresponding key rotation certificate

        let key_canaries = key_rotation_certificates
            .iter()
            .take(key_rotation_certificate.key_index as usize)
            .map(|certif| (certif.key_canary.as_ref(), certif.timestamp));

        let outcome = validate_keys_bundle(
            ops,
            store,
            realm_id,
            &keys_bundle,
            &keys_bundle_access,
            key_rotation_certificate,
            key_canaries,
        )
        .await;

        match outcome {
            // The keys bundle is valid, we have recovered all the valid keys it contains !
            Ok(realm_keys) => {
                if matches!(last_realm_keys_bundle, FetchLastKeysBundle::NotYet) {
                    last_realm_keys_bundle = FetchLastKeysBundle::Done(realm_keys.clone());
                }

                for (recovered_key, current_bundle_keys) in
                    recovered_keys.iter_mut().zip(&realm_keys.keys)
                {
                    if let ValidatedKey::Valid { key, .. } = current_bundle_keys {
                        *recovered_key = Some(key.to_owned());
                    }
                }
            }
            Err(err) => match err {
                // Keys bundle is itself corrupted, so we cannot recover any key from it :(
                ValidateKeysBundleError::InvalidKeysBundle(_) => {
                    if matches!(last_realm_keys_bundle, FetchLastKeysBundle::NotYet) {
                        last_realm_keys_bundle = FetchLastKeysBundle::Corrupted;
                    }
                }
                ValidateKeysBundleError::Internal(err) => {
                    return Err(err.context("Cannot validate realm keys bundle").into())
                }
            },
        }

        // 5) Finally stop there once we have recovered all keys

        // Of course, we cannot recover the invalid keys that have been added by the
        // key rotation certificates that we have already processed !
        let still_recoverable_keys = recovered_keys
            .iter()
            .take((key_rotation_certificate.key_index - 1) as usize);

        if still_recoverable_keys.into_iter().all(|key| key.is_some()) {
            break;
        }
    }

    // 6) The recovery is done, let's compare our result with the last keys bundle to see
    //    if we have been able to recover some keys.

    let recovery_better_than_last_keys_bundle = match last_realm_keys_bundle {
        // This realm doesn't have any key yet, so there is nothing to improve
        FetchLastKeysBundle::NotYet => false,
        // If the last keys bundle is corrupted, indeed we want to replace it !
        FetchLastKeysBundle::Corrupted => true,
        FetchLastKeysBundle::Done(realm_keys) => {
            realm_keys
                .keys
                .iter()
                .zip(recovered_keys)
                .any(|(key, recovered_key)| match (key, recovered_key) {
                    // We have successfully recovered a key that was corrupted in the last keys bundle !
                    (ValidatedKey::Corrupted { .. }, Some(_)) => true,
                    // Our recovery uses the last keys bundle, so it must be at least as good as it !
                    (ValidatedKey::Valid { .. }, None) => unreachable!(),
                    // Both valid or both corrupted, nothing new here
                    _ => false,
                })
        }
    };

    if !recovery_better_than_last_keys_bundle {
        return Ok(KeysBundleHealingOutcome::NotNeeded);
    }

    // 7) Final step: do a key rotation with our recovered keys !

    todo!()
}

#[derive(Debug, thiserror::Error)]
pub enum EncryptRealmKeysBundleAccessForUserError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error("User not found")]
    UserNotFound,
    #[error("The realm doesn't have any key yet")]
    NoKey,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn encrypt_realm_keys_bundle_access_for_user(
    ops: &CertifOps,
    store: &mut CertificatesStoreReadGuard<'_>,
    realm_id: VlobID,
    user_id: UserID,
) -> Result<(Vec<u8>, IndexInt), EncryptRealmKeysBundleAccessForUserError> {
    let realm_keys = load_last_realm_keys_bundle(ops, store, realm_id)
        .await
        .map_err(|e| match e {
            LoadLastKeysBundleError::Offline => EncryptRealmKeysBundleAccessForUserError::Offline,
            LoadLastKeysBundleError::NotAllowed => {
                EncryptRealmKeysBundleAccessForUserError::NotAllowed
            }
            LoadLastKeysBundleError::NoKey => EncryptRealmKeysBundleAccessForUserError::NoKey,
            LoadLastKeysBundleError::InvalidKeysBundle(err) => {
                EncryptRealmKeysBundleAccessForUserError::InvalidKeysBundle(err)
            }
            LoadLastKeysBundleError::Internal(err) => {
                EncryptRealmKeysBundleAccessForUserError::Internal(err)
            }
        })?;

    let cleartext_keys_bundle_access = RealmKeysBundleAccess {
        keys_bundle_key: realm_keys.keys_bundle_access_key.to_owned(),
    }
    .dump();

    let recipient_keys_bundle_access =
        super::encrypt::encrypt_for_user(store, user_id, &cleartext_keys_bundle_access)
            .await
            .map_err(|e| match e {
                CertifEncryptForUserError::UserNotFound => {
                    EncryptRealmKeysBundleAccessForUserError::UserNotFound
                }
                CertifEncryptForUserError::Internal(err) => err.into(),
            })?;

    Ok((recipient_keys_bundle_access, realm_keys.key_index()))
}

#[derive(Debug, thiserror::Error)]
pub enum InvalidKeysBundleError {
    #[error("`{recipient}`'s access for keys bundle with index {key_index} (key rotation by `{key_rotation_author}` on {key_rotation_timestamp}) for realm `{realm}` is corrupted: {error}")]
    CorruptedAccess {
        realm: VlobID,
        key_index: IndexInt,
        // Note the keys bundle access may have been created by another user than the author
        // of the key rotation (if the realm has been shared after the key rotation).
        key_rotation_author: DeviceID,
        key_rotation_timestamp: DateTime,
        recipient: UserID,
        error: DataError,
    },
    /// Note the keys bundle is signed before being encrypted. So with this error we
    /// don't know which one of the key or data is at fault.
    #[error("`{recipient}`'s access for keys bundle with index {key_index} (key rotation by `{key_rotation_author}` on {key_rotation_timestamp}) for realm `{realm}` is not able to decrypt the keys bundle")]
    Decryption {
        realm: VlobID,
        key_index: IndexInt,
        key_rotation_author: DeviceID,
        key_rotation_timestamp: DateTime,
        recipient: UserID,
    },
    #[error("Keys bundle with index {key_index} (key rotation by `{author}` on {timestamp}) for realm `{realm}` is corrupted: {error}")]
    Corrupted {
        realm: VlobID,
        key_index: IndexInt,
        author: DeviceID,
        timestamp: DateTime,
        error: DataError,
    },
    #[error("Keys bundle with index {key_index} (key rotation by `{author}` on {timestamp}) for realm `{realm}`: at that time author didn't exist !")]
    NonExistentAuthor {
        realm: VlobID,
        key_index: IndexInt,
        author: DeviceID,
        timestamp: DateTime,
    },
    #[error("Keys bundle with index {key_index} (key rotation by `{author}` on {timestamp}) for realm `{realm}`: at that time author was already revoked !")]
    RevokedAuthor {
        realm: VlobID,
        key_index: IndexInt,
        author: DeviceID,
        timestamp: DateTime,
    },
    #[error("Keys bundle with index {expected_key_index} (key rotation by `{author}` on {timestamp}) for realm `{realm}`: wrong key index specified ({bad_key_index})")]
    KeyIndexMismatch {
        realm: VlobID,
        expected_key_index: IndexInt,
        bad_key_index: IndexInt,
        author: DeviceID,
        timestamp: DateTime,
    },
    #[error("Keys bundle with index {key_index} (key rotation by `{author}` on {timestamp}) for realm `{expected_realm_id}`: wrong realm ID specified ({bad_realm_id})")]
    RealmIDMismatch {
        expected_realm_id: VlobID,
        bad_realm_id: VlobID,
        key_index: IndexInt,
        author: DeviceID,
        timestamp: DateTime,
    },
    #[error("Keys bundle with index {key_index} (key rotation by `{expected_author}` on {timestamp}) for realm `{realm}`: wrong author specified ({bad_author})")]
    AuthorMismatch {
        realm: VlobID,
        key_index: IndexInt,
        expected_author: DeviceID,
        bad_author: DeviceID,
        timestamp: DateTime,
    },
    #[error("Keys bundle with index {key_index} (key rotation by `{author}` on {expected_timestamp}) for realm `{realm}`: wrong timestamp specified ({bad_timestamp})")]
    TimestampMismatch {
        realm: VlobID,
        key_index: IndexInt,
        author: DeviceID,
        expected_timestamp: DateTime,
        bad_timestamp: DateTime,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ValidateKeysBundleError {
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[allow(clippy::too_many_arguments)]
async fn validate_keys_bundle(
    ops: &CertifOps,
    store: &mut CertificatesStoreReadGuard<'_>,
    realm_id: VlobID,
    keys_bundle: &[u8],
    keys_bundle_access: &[u8],
    certif: &RealmKeyRotationCertificate,
    key_canaries: impl Iterator<Item = (&[u8], DateTime)>,
) -> Result<Arc<RealmKeys>, ValidateKeysBundleError> {
    // 1) Decrypt & load keys bundle access

    let keys_bundle_access_key = {
        let cleartext_keys_bundle_access = ops
            .device
            .private_key
            .decrypt_from_self(keys_bundle_access)
            .map_err(|_| {
                ValidateKeysBundleError::InvalidKeysBundle(Box::new(
                    InvalidKeysBundleError::CorruptedAccess {
                        realm: realm_id,
                        key_index: certif.key_index,
                        key_rotation_author: certif.author.to_owned(),
                        key_rotation_timestamp: certif.timestamp,
                        recipient: ops.device.user_id().to_owned(),
                        error: DataError::Decryption,
                    },
                ))
            })?;

        let keys_bundle_access = RealmKeysBundleAccess::load(&cleartext_keys_bundle_access)
            .map_err(|_| {
                ValidateKeysBundleError::InvalidKeysBundle(Box::new(
                    InvalidKeysBundleError::CorruptedAccess {
                        realm: realm_id,
                        key_index: certif.key_index,
                        key_rotation_author: certif.author.to_owned(),
                        key_rotation_timestamp: certif.timestamp,
                        recipient: ops.device.user_id().to_owned(),
                        error: DataError::Serialization,
                    },
                ))
            })?;

        keys_bundle_access.keys_bundle_key
    };

    // 2) Decrypt & load keys bundle

    let keys_bundle = {
        let cleartext_keys_bundle = keys_bundle_access_key.decrypt(keys_bundle).map_err(|_| {
            ValidateKeysBundleError::InvalidKeysBundle(Box::new(
                InvalidKeysBundleError::Decryption {
                    realm: realm_id,
                    key_index: certif.key_index,
                    key_rotation_author: certif.author.to_owned(),
                    key_rotation_timestamp: certif.timestamp,
                    recipient: ops.device.user_id().to_owned(),
                },
            ))
        })?;

        let unsecure =
            RealmKeysBundle::unsecure_load(cleartext_keys_bundle.into()).map_err(|error| {
                ValidateKeysBundleError::InvalidKeysBundle(Box::new(
                    InvalidKeysBundleError::Corrupted {
                        realm: realm_id,
                        key_index: certif.key_index,
                        author: certif.author.to_owned(),
                        timestamp: certif.timestamp,
                        error,
                    },
                ))
            })?;
        let author_verify_key = store
            .get_device_verify_key(
                UpTo::Timestamp(*unsecure.timestamp()),
                unsecure.author().to_owned(),
            )
            .await
            .map_err(|e| match e {
                GetCertificateError::NonExisting
                | GetCertificateError::ExistButTooRecent { .. } => {
                    ValidateKeysBundleError::InvalidKeysBundle(Box::new(
                        InvalidKeysBundleError::NonExistentAuthor {
                            realm: realm_id,
                            key_index: certif.key_index,
                            author: certif.author.to_owned(),
                            timestamp: certif.timestamp,
                        },
                    ))
                }
                GetCertificateError::Internal(err) => err.into(),
            })?;
        let (keys_bundle, _) =
            unsecure
                .verify_signature(&author_verify_key)
                .map_err(|(_, error)| {
                    Box::new(InvalidKeysBundleError::Corrupted {
                        realm: realm_id,
                        key_index: certif.key_index,
                        author: certif.author.to_owned(),
                        timestamp: certif.timestamp,
                        error,
                    })
                })?;

        keys_bundle
    };

    // 3) Validate keys bundle correspond to the last key rotation certificate.
    // Note we don't need to check consistency with any other certificate given this
    // has already be done during corresponding key rotation certificate validation.

    if keys_bundle.realm_id != certif.realm_id {
        return Err(ValidateKeysBundleError::InvalidKeysBundle(Box::new(
            InvalidKeysBundleError::RealmIDMismatch {
                expected_realm_id: certif.realm_id,
                bad_realm_id: keys_bundle.realm_id,
                key_index: keys_bundle.key_index(),
                author: certif.author.to_owned(),
                timestamp: certif.timestamp,
            },
        )));
    }

    // Note that by checking keys bundle's key_index, we also ensure the keys bundle
    // has the correct number of keys.
    if keys_bundle.key_index() != certif.key_index {
        return Err(ValidateKeysBundleError::InvalidKeysBundle(Box::new(
            InvalidKeysBundleError::KeyIndexMismatch {
                realm: realm_id,
                expected_key_index: certif.key_index,
                bad_key_index: keys_bundle.key_index(),
                author: certif.author.to_owned(),
                timestamp: certif.timestamp,
            },
        )));
    }

    if keys_bundle.author != certif.author {
        return Err(ValidateKeysBundleError::InvalidKeysBundle(Box::new(
            InvalidKeysBundleError::AuthorMismatch {
                realm: realm_id,
                key_index: keys_bundle.key_index(),
                expected_author: certif.author.to_owned(),
                bad_author: keys_bundle.author.to_owned(),
                timestamp: certif.timestamp,
            },
        )));
    }

    if keys_bundle.author != certif.author {
        return Err(ValidateKeysBundleError::InvalidKeysBundle(Box::new(
            InvalidKeysBundleError::TimestampMismatch {
                realm: realm_id,
                key_index: keys_bundle.key_index(),
                author: certif.author.to_owned(),
                expected_timestamp: certif.timestamp,
                bad_timestamp: keys_bundle.timestamp,
            },
        )));
    }

    // 4) Finally check each key in the bundle against it corresponding canary.
    // Note this check can fail without invalidating the key rotation: a key
    // can be invalid due to a bug during a key rotation, so we trust the
    // certificate author that he did his best to provide us the valid keys.
    // Hence we just ignore the invalid keys (and will return an error if we
    // are required later on to decrypt data with them).

    let validated_keys: Vec<_> = keys_bundle
        .keys()
        .iter()
        .zip(key_canaries)
        .map(
            |(key, (canary, certif_timestamp))| match key.decrypt(canary) {
                Ok(_) => ValidatedKey::Valid {
                    key: key.to_owned(),
                    timestamp: certif_timestamp,
                },
                Err(_) => ValidatedKey::Corrupted {
                    corrupted_key_dont_use_me: key.to_owned(),
                },
            },
        )
        .collect();

    // Sanity check to ensure the correct number of canaries has been provided
    // (in practice, it is the caller's responsibility)
    assert_eq!(validated_keys.len(), keys_bundle.keys().len());

    Ok(Arc::new(RealmKeys {
        realm_id,
        keys_bundle_access_key,
        keys: validated_keys,
    }))
}

#[derive(Debug, thiserror::Error)]
pub enum CertifEncryptForRealmError {
    /// Stopped is not used by `encrypt_for_realm`, but is convenient anyways given
    /// it is needed by the wrapper `CertificateOps::encrypt_for_realm`.
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot reach the server")]
    Offline,
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error("There is no key available in this realm for encryption")]
    NoKey,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn encrypt_for_realm(
    ops: &CertifOps,
    store: &mut CertificatesStoreReadGuard<'_>,
    realm_id: VlobID,
    data: &[u8],
) -> Result<(Vec<u8>, IndexInt), CertifEncryptForRealmError> {
    let realm_keys = load_last_realm_keys_bundle(ops, store, realm_id)
        .await
        .map_err(|e| match e {
            LoadLastKeysBundleError::Offline => CertifEncryptForRealmError::Offline,
            LoadLastKeysBundleError::NotAllowed => CertifEncryptForRealmError::NotAllowed,
            LoadLastKeysBundleError::NoKey => CertifEncryptForRealmError::NoKey,
            LoadLastKeysBundleError::InvalidKeysBundle(err) => {
                CertifEncryptForRealmError::InvalidKeysBundle(err)
            }
            LoadLastKeysBundleError::Internal(err) => {
                err.context("Cannot retrieve realm encryption info").into()
            }
        })?;

    let (key, key_index) = realm_keys
        .last_valid_key()
        .ok_or(CertifEncryptForRealmError::NoKey)?;

    let encrypted = key.encrypt(data);

    Ok((encrypted, key_index))
}
