// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::{
    store::CertifStoreError, CertifOps, CertificateBasedActionOutcome, InvalidCertificateError,
    InvalidKeysBundleError,
};
use crate::{
    certif::realm_keys_bundle::EncryptRealmKeysBundleAccessForUserError,
    EventTooMuchDriftWithServerClock,
};

#[derive(Debug, thiserror::Error)]
pub enum CertifShareRealmError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot share with oneself")]
    RecipientIsSelf,
    #[error("Recipient user not found")]
    RecipientNotFound,
    #[error("Workspace realm not found")]
    RealmNotFound,
    #[error("Cannot share with a revoked user")]
    RecipientRevoked,
    #[error("Author not allowed")]
    AuthorNotAllowed,
    #[error("Role incompatible with the user, as it has profile OUTSIDER")]
    RoleIncompatibleWithOutsider,
    #[error("Cannot reach the server")]
    Offline,
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error("There is no key available in this realm for encryption")]
    NoKey,
    #[error(transparent)]
    InvalidKeysBundle(#[from] InvalidKeysBundleError),
    #[error(transparent)]
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for CertifShareRealmError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            // TODO: handle organization expired and user revoked here ?
            err => Self::Internal(err.into()),
        }
    }
}

impl From<CertifStoreError> for CertifShareRealmError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

pub(super) async fn share_realm(
    ops: &CertifOps,
    realm_id: VlobID,
    recipient: UserID,
    role: Option<RealmRole>,
) -> Result<CertificateBasedActionOutcome, CertifShareRealmError> {
    if *ops.device.device_id.user_id() == recipient {
        return Err(CertifShareRealmError::RecipientIsSelf);
    }

    // Realm sharing involves multiple checks:
    // - Recipient should not be revoked (unless we are unsharing).
    // - OWNER/MANAGER role cannot be given to an OUTSIDER recipient.
    // - Our user's current role determines which role (or even if !) we can
    //   give to the recipient.
    //
    // In theory we should not trust the server on those checks, and hence we should:
    // 1) do the checks in local
    // 2) if ok, then send the command to the server
    // 3) if the server returned us an error, load the newer certificates from the
    //    server and re-do the checks in local.
    //
    // However this is cumbersome, so here instead we rely entirely on the server for
    // the checks.
    // Of course this also gives room for the server to lie to us (e.g. it can pretend
    // the recipient has been revoked).
    // This is considered ok-enough given the only thing the server can do here is to
    // prevent us from sharing a workspace (it cannot trick us to alter the realm role
    // certificate), which it can also achieve by just pretending to be not available.

    // Loop is needed to deal with server requiring greater timestamp
    let mut timestamp = ops.device.now();
    loop {
        let outcome = match role {
            Some(role) => {
                share_do_server_command(ops, realm_id, recipient.clone(), role, timestamp).await
            }
            None => unshare_do_server_command(ops, realm_id, recipient.clone(), timestamp).await,
        }?;

        match outcome {
            DoServerCommandOutcome::Done(outcome) => return Ok(outcome),
            DoServerCommandOutcome::RequireGreaterTimestamp(strictly_greater_than) => {
                // TODO: handle `strictly_greater_than` out of the client ballpark by
                // returning an error
                timestamp = std::cmp::max(ops.device.time_provider.now(), strictly_greater_than);
            }
        }
    }
}

enum DoServerCommandOutcome {
    Done(CertificateBasedActionOutcome),
    RequireGreaterTimestamp(DateTime),
}

async fn unshare_do_server_command(
    ops: &CertifOps,
    realm_id: VlobID,
    recipient: UserID,
    timestamp: DateTime,
) -> Result<DoServerCommandOutcome, CertifShareRealmError> {
    // 0) Sanity check to prevent generating and invalid certificate

    if recipient == *ops.device.device_id.user_id() {
        return Err(CertifShareRealmError::RecipientIsSelf);
    }

    // 1) Build role certificate

    let signed_certificate = RealmRoleCertificate {
        author: CertificateSignerOwned::User(ops.device.device_id.clone()),
        timestamp,
        realm_id,
        user_id: recipient,
        role: None,
    }
    .dump_and_sign(&ops.device.signing_key);

    // 2) Actually send the command to the server

    use authenticated_cmds::latest::realm_unshare::{Rep, Req};

    let req = Req {
        realm_role_certificate: signed_certificate.into(),
    };
    let rep = ops.cmds.send(req).await?;
    match rep {
        Rep::Ok => Ok(DoServerCommandOutcome::Done(
            CertificateBasedActionOutcome::Uploaded {
                certificate_timestamp: timestamp,
            },
        )),
        Rep::RecipientAlreadyUnshared => Ok(DoServerCommandOutcome::Done(
            CertificateBasedActionOutcome::RemoteIdempotent,
        )),
        Rep::RequireGreaterTimestamp {
            strictly_greater_than,
        } => {
            // The retry is handled by the caller
            Ok(DoServerCommandOutcome::RequireGreaterTimestamp(
                strictly_greater_than,
            ))
        }
        Rep::AuthorNotAllowed { .. } => Err(CertifShareRealmError::AuthorNotAllowed),
        Rep::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let event = EventTooMuchDriftWithServerClock {
                server_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
                client_timestamp,
            };
            ops.event_bus.send(&event);

            Err(CertifShareRealmError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            })
        }
        Rep::RealmNotFound { .. } => Err(CertifShareRealmError::RealmNotFound),
        // Unlike for the sharing, we didn't have retrieved the user on our side,
        // so this error can actually occur (this is only theorical though, as the
        // user ID is supposed to have been obtained from certificates).
        Rep::RecipientNotFound { .. } => Err(CertifShareRealmError::RecipientNotFound),
        bad_rep @ (Rep::InvalidCertificate { .. } | Rep::UnknownStatus { .. }) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

async fn share_do_server_command(
    ops: &CertifOps,
    realm_id: VlobID,
    recipient: UserID,
    role: RealmRole,
    timestamp: DateTime,
) -> Result<DoServerCommandOutcome, CertifShareRealmError> {
    // 0) Sanity check to prevent generating and invalid certificate

    if recipient == *ops.device.device_id.user_id() {
        return Err(CertifShareRealmError::RecipientIsSelf);
    }

    // 1) Retrieve & encrypt keys bundle access for recipient

    let (encrypted_keys_bundle_access, key_index) = ops
        .store
        .for_read({
            let recipient = recipient.clone();
            |store| async move {
                super::realm_keys_bundle::encrypt_realm_keys_bundle_access_for_user(
                    ops, store, realm_id, recipient,
                )
                .await
            }
        })
        .await?
        .map_err(|e| match e {
            EncryptRealmKeysBundleAccessForUserError::Stopped => CertifShareRealmError::Stopped,
            EncryptRealmKeysBundleAccessForUserError::Offline => CertifShareRealmError::Offline,
            EncryptRealmKeysBundleAccessForUserError::NotAllowed => {
                CertifShareRealmError::AuthorNotAllowed
            }
            EncryptRealmKeysBundleAccessForUserError::UserNotFound => {
                CertifShareRealmError::RecipientNotFound
            }
            EncryptRealmKeysBundleAccessForUserError::NoKey => CertifShareRealmError::NoKey,
            EncryptRealmKeysBundleAccessForUserError::InvalidKeysBundle(err) => {
                CertifShareRealmError::InvalidKeysBundle(err)
            }
            EncryptRealmKeysBundleAccessForUserError::Internal(err) => err
                .context("Cannot encrypt realm keys bundle access for recipient")
                .into(),
        })?;

    // 2) Build role certificate

    let signed_certificate = RealmRoleCertificate {
        author: CertificateSignerOwned::User(ops.device.device_id.clone()),
        timestamp,
        realm_id,
        user_id: recipient,
        role: Some(role),
    }
    .dump_and_sign(&ops.device.signing_key);

    // 3) Actually send the command to the server

    use authenticated_cmds::latest::realm_share::{Rep, Req};

    let req = Req {
        key_index,
        recipient_keys_bundle_access: encrypted_keys_bundle_access.into(),
        realm_role_certificate: signed_certificate.into(),
    };
    let rep = ops.cmds.send(req).await?;
    match rep {
        Rep::Ok => Ok(DoServerCommandOutcome::Done(CertificateBasedActionOutcome::Uploaded { certificate_timestamp: timestamp })),
        Rep::RoleAlreadyGranted => Ok(DoServerCommandOutcome::Done(CertificateBasedActionOutcome::RemoteIdempotent)),
        Rep::RequireGreaterTimestamp {
            strictly_greater_than,
        } => {
            // The retry is handled by the caller
            Ok(
                DoServerCommandOutcome::RequireGreaterTimestamp(
                    strictly_greater_than,
                ),
            )
        }
        Rep::AuthorNotAllowed { .. } => Err(CertifShareRealmError::AuthorNotAllowed),
        Rep::RoleIncompatibleWithOutsider { .. } => {
            Err(CertifShareRealmError::RoleIncompatibleWithOutsider)
        }
        Rep::RecipientRevoked => Err(CertifShareRealmError::RecipientRevoked),
        Rep::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let event = EventTooMuchDriftWithServerClock {
                server_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
                client_timestamp,
            };
            ops.event_bus.send(&event);

            Err(CertifShareRealmError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            })
        }
        // Note this error should never occur in practice given we have already
        // made sure the workspace exists in the server.
        Rep::RealmNotFound { .. } => Err(CertifShareRealmError::RealmNotFound),
        // Note this error should never occur in practice given we have already
        // retrieve the user on our side.
        Rep::RecipientNotFound { .. } => Err(CertifShareRealmError::RecipientNotFound),
        bad_rep @ (
            Rep::BadKeyIndex  // We got the key index from a certificate, so it must be valid
            | Rep::InvalidCertificate { .. }
            | Rep::UnknownStatus { .. }
        ) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
