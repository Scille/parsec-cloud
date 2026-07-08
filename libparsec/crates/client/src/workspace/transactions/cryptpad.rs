// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::protocol::authenticated_cmds;
use libparsec_client_connection::ConnectionError;
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

pub use crate::CryptpadSessionKeys;
use crate::{
    workspace::WorkspaceOps, CertifGetLatestRealmKeyForEncryptionError, CertifPollServerError,
    CertifValidateCryptpadSessionKeysError, EncryptionUsage, InvalidCertificateError,
    InvalidCryptpadSessionKeysError, InvalidKeysBundleError,
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceRegisterCryptpadSessionError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Cryptpad is unavailable on the server")]
    CryptpadUnavailable,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("The workspace's realm has been deleted on the server")]
    RealmDeleted,
    #[error("There is no key available in this realm for encryption")]
    NoKey,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidCryptpadSessionKeys(#[from] Box<InvalidCryptpadSessionKeysError>),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(crate) async fn register_cryptpad_session(
    ops: &WorkspaceOps,
    document_id: VlobID,
    candidate_edit_key: String,
    candidate_view_key: String,
) -> Result<CryptpadSessionKeys, WorkspaceRegisterCryptpadSessionError> {
    loop {
        let edit_allowed = ops
            .workspace_external_info
            .lock()
            .expect("Mutex is poisoned")
            .entry
            .role
            .can_write();

        let req = {
            use authenticated_cmds::latest::cryptpad_register_session::Req;

            // Encrypt candidate view & edit keys

            let (encrypt_key, encrypt_key_index) = ops
                .certificates_ops
                .get_latest_realm_key_for_encryption(
                    EncryptionUsage::CryptpadSessionKey(document_id),
                    ops.realm_id,
                )
                .await
                .map_err(|err| match err {
                    CertifGetLatestRealmKeyForEncryptionError::Stopped => {
                        WorkspaceRegisterCryptpadSessionError::Stopped
                    }
                    CertifGetLatestRealmKeyForEncryptionError::Offline(err) => {
                        WorkspaceRegisterCryptpadSessionError::Offline(err)
                    }
                    CertifGetLatestRealmKeyForEncryptionError::NotAllowed => {
                        WorkspaceRegisterCryptpadSessionError::NoRealmAccess
                    }
                    CertifGetLatestRealmKeyForEncryptionError::RealmDeleted => {
                        WorkspaceRegisterCryptpadSessionError::RealmDeleted
                    }
                    CertifGetLatestRealmKeyForEncryptionError::NoKey => {
                        WorkspaceRegisterCryptpadSessionError::NoKey
                    }
                    CertifGetLatestRealmKeyForEncryptionError::InvalidKeysBundle(err) => {
                        WorkspaceRegisterCryptpadSessionError::InvalidKeysBundle(err)
                    }
                    CertifGetLatestRealmKeyForEncryptionError::Internal(err) => err
                        .context("Cannot encrypt Cryptpad session keys for realm")
                        .into(),
                })?;

            let timestamp = ops.device.now();

            let encrypted_candidate_view_key = CryptpadSessionKey {
                author: ops.device.device_id,
                timestamp,
                document_id,
                can_edit: false,
                key: candidate_view_key.clone(),
            }
            .dump_sign_and_encrypt(&ops.device.signing_key, &encrypt_key)
            .into();

            let encrypted_candidate_edit_key = if edit_allowed {
                Some(
                    CryptpadSessionKey {
                        author: ops.device.device_id,
                        timestamp: ops.device.now(),
                        document_id,
                        can_edit: true,
                        key: candidate_edit_key.clone(),
                    }
                    .dump_sign_and_encrypt(&ops.device.signing_key, &encrypt_key)
                    .into(),
                )
            } else {
                None
            };

            Req {
                realm_id: ops.realm_id,
                document_id,
                key_index: encrypt_key_index,
                timestamp,
                encrypted_candidate_view_key,
                encrypted_candidate_edit_key,
            }
        };

        // Send the request to the server

        let (
            author,
            timestamp,
            key_index,
            encrypted_view_key,
            encrypted_edit_key,
            needed_common_certificate_timestamp,
            needed_realm_certificate_timestamp,
        ) = {
            use authenticated_cmds::latest::cryptpad_register_session::Rep;

            // Send the request to the server
            let rep = ops
                .cmds
                .send(req)
                .await
                .map_err(WorkspaceRegisterCryptpadSessionError::Offline)?;

            match rep {
                Rep::Ok {
                    author,
                    timestamp,
                    key_index,
                    encrypted_view_key,
                    encrypted_edit_key,
                    needed_common_certificate_timestamp,
                    needed_realm_certificate_timestamp,
                } => (
                    author,
                    timestamp,
                    key_index,
                    encrypted_view_key,
                    encrypted_edit_key,
                    needed_common_certificate_timestamp,
                    needed_realm_certificate_timestamp,
                ),
                Rep::RealmDeleted => return Err(WorkspaceRegisterCryptpadSessionError::RealmDeleted),
                Rep::AuthorNotAllowed => return Err(WorkspaceRegisterCryptpadSessionError::NoRealmAccess),
                Rep::CryptpadUnavailable => return Err(WorkspaceRegisterCryptpadSessionError::CryptpadUnavailable),
                Rep::TimestampOutOfBallpark { ballpark_client_early_offset, ballpark_client_late_offset, client_timestamp, server_timestamp } => {
                    return Err(WorkspaceRegisterCryptpadSessionError::TimestampOutOfBallpark {
                        server_timestamp,
                        client_timestamp,
                        ballpark_client_early_offset,
                        ballpark_client_late_offset,
                    })
                },

                // A key rotation occurred concurrently, should poll for new certificates and retry
                Rep::BadKeyIndex { last_realm_certificate_timestamp } => {
                    let latest_known_timestamps = PerTopicLastTimestamps::new_for_realm(ops.realm_id, last_realm_certificate_timestamp);
                    ops.certificates_ops
                        .poll_server_for_new_certificates(Some(&latest_known_timestamps))
                        .await
                        .map_err(|err| match err {
                            CertifPollServerError::Stopped => WorkspaceRegisterCryptpadSessionError::Stopped,
                            CertifPollServerError::Offline(e) => WorkspaceRegisterCryptpadSessionError::Offline(e),
                            CertifPollServerError::InvalidCertificate(err) => WorkspaceRegisterCryptpadSessionError::InvalidCertificate(err),
                            CertifPollServerError::Internal(err) => err.context("Cannot poll server for new certificates").into(),
                        })?;
                    continue;
                }

                // Unexpected errors :(
                bad_rep @ (
                    // Already checked the realm exists when we called `CertificateOps::encrypt_for_realm`
                    | Rep::RealmNotFound
                    // Don't know what to do with this status :/
                    | Rep::UnknownStatus { .. }
                ) => {
                    return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                }
            }
        };

        // Decrypt the edit & view keys to actually use

        return ops
            .certificates_ops
            .validate_cryptpad_session_keys(
                needed_realm_certificate_timestamp,
                needed_common_certificate_timestamp,
                ops.realm_id,
                key_index,
                document_id,
                author,
                timestamp,
                &encrypted_view_key,
                encrypted_edit_key.as_deref(),
            )
            .await
            .map_err(|err| match err {
                CertifValidateCryptpadSessionKeysError::Offline(err) => {
                    WorkspaceRegisterCryptpadSessionError::Offline(err)
                }
                CertifValidateCryptpadSessionKeysError::Stopped => {
                    WorkspaceRegisterCryptpadSessionError::Stopped
                }
                CertifValidateCryptpadSessionKeysError::NotAllowed => {
                    WorkspaceRegisterCryptpadSessionError::NoRealmAccess
                }
                CertifValidateCryptpadSessionKeysError::RealmDeleted => {
                    WorkspaceRegisterCryptpadSessionError::RealmDeleted
                }
                CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(err) => {
                    WorkspaceRegisterCryptpadSessionError::InvalidCryptpadSessionKeys(err)
                }
                CertifValidateCryptpadSessionKeysError::InvalidCertificate(err) => {
                    WorkspaceRegisterCryptpadSessionError::InvalidCertificate(err)
                }
                CertifValidateCryptpadSessionKeysError::InvalidKeysBundle(err) => {
                    WorkspaceRegisterCryptpadSessionError::InvalidKeysBundle(err)
                }
                CertifValidateCryptpadSessionKeysError::Internal(err) => err
                    .context("Cannot decrypt Cryptpad session key for realm")
                    .into(),
            });
    }
}
