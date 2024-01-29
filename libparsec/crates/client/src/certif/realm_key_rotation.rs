// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_client_connection::ConnectionError;
use libparsec_platform_storage::certificates::{GetCertificateError, UpTo};
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::{
    store::CertifStoreError, CertifOps, CertificateBasedActionOutcome, InvalidCertificateError,
};
use crate::{certif::CertifPollServerError, EventTooMuchDriftWithServerClock};

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

pub(super) async fn ensure_realm_intial_key_rotation(
    ops: &CertifOps,
    realm_id: VlobID,
) -> Result<CertificateBasedActionOutcome, CertifRotateRealmKeyError> {
    // First look into our local certificates...

    let has_initial_key_rotation = ops
        .store
        .for_read(|store| async move {
            store
                .get_realm_last_key_rotation_certificate(UpTo::Current, realm_id)
                .await
                .map_err(CertifRotateRealmKeyError::Internal)
                .map(|certif| certif.is_some())
        })
        .await??;

    if has_initial_key_rotation {
        return Ok(CertificateBasedActionOutcome::LocalIdempotent);
    }

    // ...then try to do the key rotation.

    let outcome = realm_initial_key_rotation_idempotent(
        ops,
        realm_id,
        WorkspaceKeyOrigin::NewlyGenerated(SecretKey::generate()),
    )
    .await
    .map_err(|e| match e {
        CertifRotateRealmKeyError::UnknownRealm => CertifRotateRealmKeyError::UnknownRealm,
        CertifRotateRealmKeyError::AuthorNotAllowed => CertifRotateRealmKeyError::AuthorNotAllowed,
        CertifRotateRealmKeyError::Offline => CertifRotateRealmKeyError::Offline,
        CertifRotateRealmKeyError::Stopped => CertifRotateRealmKeyError::Stopped,
        CertifRotateRealmKeyError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
        } => CertifRotateRealmKeyError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
        },
        CertifRotateRealmKeyError::InvalidCertificate(err) => {
            CertifRotateRealmKeyError::InvalidCertificate(err)
        }
        CertifRotateRealmKeyError::Internal(err) => err.into(),
    })?;

    // Backward compatibility: if the realm has been created with Parsec < v3, the
    // initial key rotation needs to be done with the key the existing realm's data
    // are encrypted with !
    match outcome {
        RealmInitialKeyRotationOutcome::Done(outcome) => Ok(outcome),
        RealmInitialKeyRotationOutcome::LegacyRealmWithNewlyGeneratedKey {
            encryption_revision: _,
        } => {
            // TODO: finish me !
            todo!();
            // 1) Crawl the user manifests to find the legacy key
            // 2) Re-do the key rotation

            // let outcome = realm_initial_key_rotation_idempotent(ops, realm_id, WorkspaceKeyOrigin::LegacyFromUserManifest(key)).await?;
            // if matches!(outcome, RealmInitialKeyRotationOutcome::Done) {
            //     return Err(anyhow::anyhow!("Unexpected outcome"));
            // }
        }
    }
}

enum WorkspaceKeyOrigin {
    NewlyGenerated(SecretKey),
    // TODO: finish self-heal !
    #[allow(dead_code)]
    LegacyFromUserManifest(SecretKey),
}

enum RealmInitialKeyRotationOutcome {
    /// All when smoothly ;-)
    Done(CertificateBasedActionOutcome),
    /// We try to do the initial key rotation on a legacy realm with a newly
    /// generated key :-(
    // TODO: finish self-heal !
    #[allow(dead_code)]
    LegacyRealmWithNewlyGeneratedKey { encryption_revision: IndexInt },
}

/// Note the fact this function is idempotent means the provided key may
/// not be part of the realm's keys in the end !
async fn realm_initial_key_rotation_idempotent(
    ops: &CertifOps,
    realm_id: VlobID,
    key: WorkspaceKeyOrigin,
) -> Result<RealmInitialKeyRotationOutcome, CertifRotateRealmKeyError> {
    // `never_legacy_reencrypted_or_fail` is a flag to ask the server to reject
    // key rotation if the realm is a legacy one: in that case the intial key
    // rotation must provide the current realm's key (wich must be retrieved
    // from the user manifest history) instead of a newly generated one.
    let (key, never_legacy_reencrypted_or_fail) = match key {
        WorkspaceKeyOrigin::NewlyGenerated(key) => (key, true),
        WorkspaceKeyOrigin::LegacyFromUserManifest(key) => (key, false),
    };
    let key_canary = key.encrypt(b"");
    let mut timestamp = ops.device.now();
    loop {
        let certif = RealmKeyRotationCertificate {
            author: ops.device.device_id.to_owned(),
            timestamp,
            realm_id,
            key_index: 1,
            encryption_algorithm: SecretKeyAlgorithm::Xsalsa20Poly1305,
            hash_algorithm: HashAlgorithm::Sha256,
            key_canary: key_canary.clone(),
        }
        .dump_and_sign(&ops.device.signing_key);

        // Given there is no other key rotation, we can create the keys bundle ex nihilo
        let (keys_bundle, keys_bundle_access_cleartext) = {
            let keys_bundle_access_key = SecretKey::generate();
            let keys_bundle = RealmKeysBundle::new(
                ops.device.device_id.clone(),
                timestamp,
                realm_id,
                vec![key.clone()],
            );
            let keys_bundle_encrypted =
                keys_bundle_access_key.encrypt(&keys_bundle.dump_and_sign(&ops.device.signing_key));
            let keys_bundle_access_cleartext = RealmKeysBundleAccess {
                keys_bundle_key: keys_bundle_access_key,
            }
            .dump();
            (keys_bundle_encrypted.into(), keys_bundle_access_cleartext)
        };

        // Sharing cannot be done before the initial key rotation, so in theory
        // `per_participant_keys_bundle_access` should only contains a single
        // access for ourself (aka the initial creator of the workspace).
        //
        // However in Parsec < v3 key rotation didn't exist, and so the
        // legacy workspaces may have been shared before any initial key rotation.
        //
        // Hence why here we must look into the existing certificates to retrieve
        // who have access to the workspace.

        let per_participant_keys_bundle_access = ops
            .store
            .for_read(|store| async move {
                let roles = store.get_realm_roles(UpTo::Current, realm_id).await?;
                let mut per_participant_keys_bundle_access =
                    HashMap::with_capacity(roles.len() + 1);

                // The most common case is when the workspace has just been created,
                // and hence our certificates storage hasn't stored the initial realm
                // role certificate.
                // So we always add ourself to the list of participants, this way we
                // avoid the server returning a error telling us to poll for new
                // certificates and retry.
                // Note that if we are not part of the realm, the server will return a
                // dedicated error that will stop this function, so there is no risk
                // of inifinite loop here.
                per_participant_keys_bundle_access.insert(
                    ops.device.user_id().to_owned(),
                    ops.device
                        .public_key()
                        .encrypt_for_self(&keys_bundle_access_cleartext)
                        .into(),
                );

                for role in roles {
                    if role.role.is_none() {
                        per_participant_keys_bundle_access.remove(&role.user_id);
                        continue;
                    }
                    let user = store
                        .get_user_certificate(UpTo::Current, role.user_id.clone())
                        .await
                        .map_err(|e| match e {
                            GetCertificateError::Internal(e) => {
                                CertifRotateRealmKeyError::Internal(e)
                            }
                            // Given we held the store read lock, use must exists given it was
                            // referenced from a certificate coming from the store !
                            GetCertificateError::NonExisting
                            | GetCertificateError::ExistButTooRecent { .. } => unreachable!(),
                        })?;
                    per_participant_keys_bundle_access.insert(
                        user.user_id.clone(),
                        user.public_key
                            .encrypt_for_self(&keys_bundle_access_cleartext)
                            .into(),
                    );
                }
                Result::<_, CertifRotateRealmKeyError>::Ok(per_participant_keys_bundle_access)
            })
            .await??;

        use authenticated_cmds::latest::realm_rotate_key::{Rep, Req};

        let req = Req {
            realm_key_rotation_certificate: certif.into(),
            keys_bundle,
            per_participant_keys_bundle_access,
            never_legacy_reencrypted_or_fail,
        };

        let rep = ops.cmds.send(req).await?;

        return match rep {
            Rep::Ok => Ok(RealmInitialKeyRotationOutcome::Done(
                CertificateBasedActionOutcome::Uploaded {
                    certificate_timestamp: timestamp,
                },
            )),
            // Bad key index means another key rotation occured, hence our job is done here !
            Rep::BadKeyIndex {
                last_realm_certificate_timestamp,
            } => Ok(RealmInitialKeyRotationOutcome::Done(
                CertificateBasedActionOutcome::RemoteIdempotent {
                    certificate_timestamp: last_realm_certificate_timestamp,
                },
            )),
            Rep::LegacyReencryptedRealm {
                encryption_revision,
            } => Ok(
                RealmInitialKeyRotationOutcome::LegacyRealmWithNewlyGeneratedKey {
                    encryption_revision,
                },
            ),
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
                continue;
            }
            Rep::RequireGreaterTimestamp {
                strictly_greater_than,
            } => {
                timestamp = std::cmp::max(strictly_greater_than, ops.device.time_provider.now());
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

// // TODO:
// // - rotate_realm_key
// // - keys_bundle_heal

// pub(super) async fn rotate_realm_key(
//     ops: &CertifOps,
//     realm_id: VlobID,
// ) -> Result<(), CertifRotateRealmKeyError> {
//     todo!()
// }

// pub(super) async fn force_rotate_realm_key_for_healing(
//     ops: &CertifOps,
//     realm_id: VlobID,
// ) -> Result<(), CertifRotateRealmKeyError> {
//     todo!()
// }
