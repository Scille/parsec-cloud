// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::{
    realm_keys_bundle::CertifEncryptForRealmError, store::CertifStoreError, CertifOps,
    CertificateBasedActionOutcome, InvalidCertificateError, InvalidKeysBundleError, UpTo,
};
use crate::{certif::CertifPollServerError, EventTooMuchDriftWithServerClock};

#[derive(Debug, thiserror::Error)]
pub enum CertifRenameRealmError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot reach the server")]
    Offline,
    #[error("Unknown realm ID")]
    UnknownRealm,
    #[error("Not allowed")]
    AuthorNotAllowed,
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

impl From<ConnectionError> for CertifRenameRealmError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            // TODO: handle organization expired and user revoked here ?
            err => Self::Internal(err.into()),
        }
    }
}

impl From<CertifStoreError> for CertifRenameRealmError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

pub(super) async fn rename_realm(
    ops: &CertifOps,
    realm_id: VlobID,
    new_name: EntryName,
) -> Result<CertificateBasedActionOutcome, CertifRenameRealmError> {
    rename_realm_internal(ops, realm_id, new_name, false).await
}

async fn rename_realm_internal(
    ops: &CertifOps,
    realm_id: VlobID,
    new_name: EntryName,
    initial_rename: bool,
) -> Result<CertificateBasedActionOutcome, CertifRenameRealmError> {
    let mut timestamp = ops.device.now();
    // Loop is needed to deal with server requiring greater timestamp
    loop {
        // 1) Encrypt the new name

        let (encrypted_name, key_index) = ops
            .encrypt_for_realm(realm_id, new_name.as_ref().as_bytes())
            .await
            .map_err(|e| match e {
                CertifEncryptForRealmError::Stopped => CertifRenameRealmError::Stopped,
                CertifEncryptForRealmError::Offline => CertifRenameRealmError::Offline,
                CertifEncryptForRealmError::NotAllowed => CertifRenameRealmError::AuthorNotAllowed,
                CertifEncryptForRealmError::NoKey => CertifRenameRealmError::NoKey,
                CertifEncryptForRealmError::InvalidKeysBundle(err) => {
                    CertifRenameRealmError::InvalidKeysBundle(err)
                }
                CertifEncryptForRealmError::Internal(err) => {
                    err.context("Cannot encrypt name for realm").into()
                }
            })?;

        // 2) Generate the realm name certificate

        let certif = RealmNameCertificate {
            author: ops.device.device_id.to_owned(),
            timestamp,
            realm_id,
            key_index,
            encrypted_name,
        }
        .dump_and_sign(&ops.device.signing_key)
        .into();

        // 3) Send the certificate to the server

        use authenticated_cmds::latest::realm_rename::{Rep, Req};

        let req = Req {
            realm_name_certificate: certif,
            initial_name_or_fail: initial_rename,
        };

        let rep = ops.cmds.send(req).await?;

        return match rep {
            Rep::Ok => Ok(CertificateBasedActionOutcome::Uploaded {
                certificate_timestamp: timestamp,
            }),
            Rep::InitialNameAlreadyExists => Ok(CertificateBasedActionOutcome::RemoteIdempotent),
            // There is a new key rotation we don't know about, let's fetch it and retry
            Rep::BadKeyIndex => {
                ops.poll_server_for_new_certificates(None)
                    .await
                    .map_err(|e| match e {
                        CertifPollServerError::Stopped => CertifRenameRealmError::Stopped,
                        CertifPollServerError::Offline => CertifRenameRealmError::Offline,
                        CertifPollServerError::InvalidCertificate(err) => {
                            CertifRenameRealmError::InvalidCertificate(err)
                        }
                        CertifPollServerError::Internal(err) => err
                            .context("Cannot poll server for new certificates")
                            .into(),
                    })?;
                timestamp = ops.device.time_provider.now();
                continue;
            }
            Rep::RealmNotFound => Err(CertifRenameRealmError::UnknownRealm),
            Rep::AuthorNotAllowed => Err(CertifRenameRealmError::AuthorNotAllowed),
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

                Err(CertifRenameRealmError::TimestampOutOfBallpark {
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

pub(super) async fn ensure_realm_initial_rename(
    ops: &CertifOps,
    realm_id: VlobID,
    name: EntryName,
) -> Result<CertificateBasedActionOutcome, CertifRenameRealmError> {
    // First look into our local certificates...

    let has_initial_rename = ops
        .store
        .for_read(|store| async move {
            store
                .get_realm_last_name_certificate(UpTo::Current, realm_id)
                .await
                .map_err(|e| CertifRenameRealmError::Internal(e))
                .map(|certif| certif.is_some())
        })
        .await??;

    if has_initial_rename {
        return Ok(CertificateBasedActionOutcome::LocalIdempotent);
    }

    // ...then try to rename the realm.
    // Note the `initial_rename` flag that makes the query idempotent \o/
    rename_realm_internal(ops, realm_id, name, true).await
}
