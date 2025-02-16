// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::{
    realm_keys_bundle::CertifEncryptForRealmError, store::CertifStoreError,
    CertificateBasedActionOutcome, CertificateOps, InvalidCertificateError, InvalidKeysBundleError,
    UpTo,
};
use crate::{
    certif::CertifPollServerError, greater_timestamp, EncrytionUsage,
    EventTooMuchDriftWithServerClock, GreaterTimestampOffset,
};

#[derive(Debug, thiserror::Error)]
pub enum CertifRenameRealmError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
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
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
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
    ops: &CertificateOps,
    realm_id: VlobID,
    new_name: EntryName,
) -> Result<CertificateBasedActionOutcome, CertifRenameRealmError> {
    rename_realm_internal(ops, realm_id, new_name, false).await
}

async fn rename_realm_internal(
    ops: &CertificateOps,
    realm_id: VlobID,
    new_name: EntryName,
    initial_rename: bool,
) -> Result<CertificateBasedActionOutcome, CertifRenameRealmError> {
    let mut timestamp = ops.device.now();
    // Loop is needed to deal with server requiring greater timestamp
    loop {
        // 1) Encrypt the new name

        let (encrypted_name, key_index) = ops
            .encrypt_for_realm(
                EncrytionUsage::RealmRename,
                realm_id,
                new_name.as_ref().as_bytes(),
            )
            .await
            .map_err(|e| match e {
                CertifEncryptForRealmError::Stopped => CertifRenameRealmError::Stopped,
                CertifEncryptForRealmError::Offline(e) => CertifRenameRealmError::Offline(e),
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
            Rep::InitialNameAlreadyExists {
                last_realm_certificate_timestamp,
            } => Ok(CertificateBasedActionOutcome::RemoteIdempotent {
                certificate_timestamp: last_realm_certificate_timestamp,
            }),
            // There is a new key rotation we don't know about, let's fetch it and retry
            Rep::BadKeyIndex {
                last_realm_certificate_timestamp,
            } => {
                let latest_known_timestamps = PerTopicLastTimestamps::new_for_realm(
                    realm_id,
                    last_realm_certificate_timestamp,
                );
                ops.poll_server_for_new_certificates(Some(&latest_known_timestamps))
                    .await
                    .map_err(|e| match e {
                        CertifPollServerError::Stopped => CertifRenameRealmError::Stopped,
                        CertifPollServerError::Offline(e) => CertifRenameRealmError::Offline(e),
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
                timestamp = greater_timestamp(
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
    ops: &CertificateOps,
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
                .map_err(CertifRenameRealmError::Internal)
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
