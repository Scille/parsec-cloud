// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_client_connection::ConnectionError;
use libparsec_platform_storage::certificates::{GetCertificateError, UpTo};
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::{
    store::CertifStoreError, CertificateBasedActionOutcome, CertificateOps, InvalidCertificateError,
};
use crate::{
    certif::{
        realm_keys_bundle::{self, GenerateNextKeyBundleForRealmError},
        CertifPollServerError,
    },
    manage_require_greater_timestamp, EventTooMuchDriftWithServerClock, GreaterTimestampOffset,
    InvalidKeysBundleError,
};

#[derive(Debug, thiserror::Error)]
pub enum CertifRotateRealmKeyError {
    #[error("Unknown realm ID")]
    UnknownRealm,
    #[error("Not allowed")]
    AuthorNotAllowed,
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    // TODO: This error should be `CurrentKeysBundleCorruptedAndUnrecoverable`, given
    //       we should attempt a self-healing by recursively getting older keys bundles;
    //       and only returning this error if our user has reached the last keys bundle
    //       he has access to without a single one being valid.
    #[error("Cannot achieve a key rotation if the current keys bundle is corrupted: {0}")]
    CurrentKeysBundleCorrupted(Box<InvalidKeysBundleError>),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for CertifRotateRealmKeyError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            // TODO: handle organization expired and user revoked here ?
            err => Self::Internal(err.into()),
        }
    }
}

impl From<CertifStoreError> for CertifRotateRealmKeyError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

pub(super) async fn rotate_realm_key_idempotent(
    ops: &CertificateOps,
    realm_id: VlobID,
    target_key_index: IndexInt,
) -> Result<CertificateBasedActionOutcome, CertifRotateRealmKeyError> {
    let mut timestamp = ops.device.now();

    loop {
        // 1) Generate the request

        let (req, key_index) = generate_realm_rotate_key_req(ops, realm_id, timestamp).await?;

        // If the key index mismatch, it means a concurrent modification occurred in
        // our local storage. In this case there is two possibilities:
        // - key_index > target_key_index: a new key rotation occured, our job is done here !
        // - key_index < target_key_index: some (all ?) key rotation certificates disappeared
        //   from the storage (e.g. our user has been switch from/to OUTSIDER profile).
        //   This is a corner case that we can just ignore: when the missing certificates will
        //   will eventually be re-fetched from the server, the key rotation will be re-attempted.
        if key_index != target_key_index {
            return Ok(CertificateBasedActionOutcome::LocalIdempotent);
        }

        // 2) Send the request

        use authenticated_cmds::latest::realm_rotate_key::Rep;

        let rep = ops.cmds.send(req).await?;

        return match rep {
            Rep::Ok => Ok(CertificateBasedActionOutcome::Uploaded {
                certificate_timestamp: timestamp,
            }),
            // Bad key index means another key rotation occured, hence our job is done here !
            Rep::BadKeyIndex {
                last_realm_certificate_timestamp,
            } => Ok(CertificateBasedActionOutcome::RemoteIdempotent {
                certificate_timestamp: last_realm_certificate_timestamp,
            }),
            Rep::RealmNotFound => Err(CertifRotateRealmKeyError::UnknownRealm),
            Rep::AuthorNotAllowed => Err(CertifRotateRealmKeyError::AuthorNotAllowed),
            Rep::ParticipantMismatch => {
                // List of participants got updated in our back, refresh and retry
                ops.poll_server_for_new_certificates(None)
                    .await
                    .map_err(|e| match e {
                        CertifPollServerError::Offline => CertifRotateRealmKeyError::Offline,
                        CertifPollServerError::Stopped => CertifRotateRealmKeyError::Stopped,
                        CertifPollServerError::InvalidCertificate(err) => {
                            CertifRotateRealmKeyError::InvalidCertificate(err)
                        }
                        CertifPollServerError::Internal(err) => err
                            .context("Cannot poll server for new certificates")
                            .into(),
                    })?;
                timestamp = ops.device.now();
                continue;
            }
            Rep::RequireGreaterTimestamp {
                strictly_greater_than,
            } => {
                timestamp = manage_require_greater_timestamp(
                    &ops.device.time_provider,
                    GreaterTimestampOffset::Realm,
                    strictly_greater_than,
                );
                continue;
            }
            Rep::TimestampOutOfBallpark {
                server_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
                client_timestamp,
                ..
            } => {
                let event = EventTooMuchDriftWithServerClock {
                    server_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                    client_timestamp,
                };
                ops.event_bus.send(&event);

                Err(CertifRotateRealmKeyError::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                })
            }
            bad_rep @ (Rep::InvalidCertificate { .. } | Rep::UnknownStatus { .. }) => {
                Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        };
    }
}

async fn generate_realm_rotate_key_req(
    ops: &CertificateOps,
    realm_id: VlobID,
    timestamp: DateTime,
) -> Result<(authenticated_cmds::latest::realm_rotate_key::Req, IndexInt), CertifRotateRealmKeyError>
{
    ops.store
        .for_read(|store| async move {
            // 1) Generate the next keys bundle

            // Note that given this function is idempotent, this new keys bundle might get
            // discarded if another key rotation occurred concurrently !
            let (keys_bundle, keys_bundle_access) =
                realm_keys_bundle::generate_next_keys_bundle_for_realm(ops, store, realm_id)
                    .await
                    .map_err(|err| match err {
                        GenerateNextKeyBundleForRealmError::Offline => {
                            CertifRotateRealmKeyError::Offline
                        }
                        GenerateNextKeyBundleForRealmError::NotAllowed => {
                            CertifRotateRealmKeyError::AuthorNotAllowed
                        }
                        GenerateNextKeyBundleForRealmError::Internal(err) => {
                            CertifRotateRealmKeyError::Internal(err)
                        }
                        // TODO: Attempt self-healing here !
                        GenerateNextKeyBundleForRealmError::InvalidKeysBundle(err) => {
                            CertifRotateRealmKeyError::CurrentKeysBundleCorrupted(err)
                        }
                    })?;
            let key_index = keys_bundle.key_index();
            let keys_bundle_access_cleartext = keys_bundle_access.dump();
            let keys_bundle_encrypted: Bytes = keys_bundle_access
                .keys_bundle_key
                .encrypt(&keys_bundle.dump_and_sign(&ops.device.signing_key))
                .into();

            // 2) Generate the key rotation certificate

            let key_canary = {
                let key = keys_bundle
                    .last_key()
                    .derive_secret_key_from_uuid(CANARY_KEY_DERIVATION_UUID);
                key.encrypt(b"")
            };
            let certif = RealmKeyRotationCertificate {
                author: ops.device.device_id.to_owned(),
                timestamp,
                realm_id,
                key_index,
                encryption_algorithm: SecretKeyAlgorithm::Blake2bXsalsa20Poly1305,
                hash_algorithm: HashAlgorithm::Sha256,
                key_canary,
            }
            .dump_and_sign(&ops.device.signing_key);

            // 3) Encrypte keys bundle access for each participant

            let per_participant_keys_bundle_access = {
                let mut per_user_role = store
                    .get_realm_current_users_roles(UpTo::Current, realm_id)
                    .await?;

                let mut per_participant_keys_bundle_access =
                    HashMap::with_capacity(per_user_role.len());

                // Dealing with our own encryption is a special case:
                // - In case we have just created the workspace, our certificates storage might
                //   not have stored the initial realm role certificate yet.
                // - We are expected to be part of the realm (otherwise why are we here ?),
                //   but store's `get_realm_current_users_roles` doesn't give such
                //   guarantee (e.g. it will return an empty hashmap if the realm ID is unknown).
                //
                // In order to avoid those corner cases, we always add ourself to the list
                // of participants, this way we avoid the server returning a error telling
                // us to poll for new certificates and retry.
                //
                // Note that if we are not part of the realm, the server will return a
                // dedicated error that will stop this function, so there is no risk
                // of infinite loop here.
                per_user_role.remove(&ops.device.user_id);
                per_participant_keys_bundle_access.insert(
                    ops.device.user_id,
                    ops.device
                        .public_key()
                        .encrypt_for_self(&keys_bundle_access_cleartext)
                        .into(),
                );

                // Now add the remaining members
                for user_id in per_user_role.into_keys() {
                    let user = store
                        .get_user_certificate(UpTo::Current, user_id)
                        .await
                        .map_err(|e| match e {
                            GetCertificateError::Internal(e) => {
                                CertifRotateRealmKeyError::Internal(e)
                            }
                            // User must exists given it was referenced from a certificate
                            // coming from the store and we haven't released the store read
                            // lock in the meantime !
                            GetCertificateError::NonExisting
                            | GetCertificateError::ExistButTooRecent { .. } => unreachable!(),
                        })?;
                    per_participant_keys_bundle_access.insert(
                        user.user_id,
                        user.public_key
                            .encrypt_for_self(&keys_bundle_access_cleartext)
                            .into(),
                    );
                }

                per_participant_keys_bundle_access
            };

            // 4) Now we have everything we need to build the request object !

            use authenticated_cmds::latest::realm_rotate_key::Req;

            let req = Req {
                realm_key_rotation_certificate: certif.into(),
                keys_bundle: keys_bundle_encrypted,
                per_participant_keys_bundle_access,
            };

            Ok((req, key_index))
        })
        .await?
}
