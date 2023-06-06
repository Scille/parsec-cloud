// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{
    storage::{AnyArcCertificate, GetCertificateError, UpTo},
    CertificatesOps, PollServerError,
};
use crate::event_bus::{EventUnprocessableMessage, UnprocessableMessageReason};

#[derive(Debug, thiserror::Error)]
pub enum InvalidMessageError {
    #[error("Message #{index} from `{sender}` at {timestamp} is corrupted: {error}")]
    Corrupted {
        index: IndexInt,
        sender: DeviceID,
        timestamp: DateTime,
        error: DataError,
    },
    #[error("Message #{index} from `{sender}` at {timestamp}: at that time author didn't exist !")]
    NonExistantAuthor {
        index: IndexInt,
        sender: DeviceID,
        timestamp: DateTime,
    },
    #[error("Message #{index} from `{sender}` at {timestamp}: at that time author was already revoked !")]
    RevokedAuthor {
        index: IndexInt,
        sender: DeviceID,
        timestamp: DateTime,
    },
    #[error("Message #{index} from `{sender}` at {timestamp}: at that time author couldn't share `{realm_id}` with us given it role was `{sender_role:?}`")]
    AuthorRealmRoleCannotShare {
        index: IndexInt,
        sender: DeviceID,
        timestamp: DateTime,
        realm_id: RealmID,
        sender_role: RealmRole,
    },
    #[error("Message #{index} from `{sender}` at {timestamp}: at that time author didn't have access to realm `{realm_id}` and hence couldn't share it with us")]
    AuthorNoAccessToRealm {
        index: IndexInt,
        sender: DeviceID,
        timestamp: DateTime,
        realm_id: RealmID,
    },
    #[error("Message #{index} from `{sender}` at {timestamp}: no realm role certificate correspond to this message !")]
    NoCorrespondingRealmRoleCertificate {
        index: IndexInt,
        sender: DeviceID,
        timestamp: DateTime,
    },
    #[error("Message #{index} from `{sender}` at {timestamp}: author has not a sufficent role to reencrypt the workspace !")]
    AuthorNotAllowedToReencrypt {
        index: IndexInt,
        sender: DeviceID,
        timestamp: DateTime,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ValidateMessageError {
    #[error("Cannot reach the server")]
    Offline,
    #[error(transparent)]
    InvalidMessage(#[from] InvalidMessageError),
    #[error(transparent)]
    PollServerError(#[from] PollServerError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl CertificatesOps {
    pub async fn validate_message(
        &self,
        certificate_index: IndexInt,
        index: IndexInt,
        sender: &DeviceID,
        timestamp: DateTime,
        body: &[u8],
    ) -> Result<MessageContent, ValidateMessageError> {
        self.validate_message_internal(certificate_index, index, sender, timestamp, body)
            .await
            .map_err(|err| {
                if let ValidateMessageError::InvalidMessage(what) = err {
                    let event = EventUnprocessableMessage {
                        index,
                        sender: sender.to_owned(),
                        reason: UnprocessableMessageReason::InvalidMessage(what),
                    };
                    self.event_bus.send(&event);
                    if let UnprocessableMessageReason::InvalidMessage(what) = event.reason {
                        ValidateMessageError::InvalidMessage(what)
                    } else {
                        unreachable!();
                    }
                } else {
                    err
                }
            })
    }

    pub async fn validate_message_internal(
        &self,
        certificate_index: IndexInt,
        index: IndexInt,
        sender: &DeviceID,
        timestamp: DateTime,
        body: &[u8],
    ) -> Result<MessageContent, ValidateMessageError> {
        // 1) Make sure we have all the needed certificates

        let storage = self
            .ensure_certificates_available_and_read_lock(certificate_index)
            .await?;

        // 2) Sender must exist and not be revoked to send a message !

        // 2.1) Check device exists (this also imply the user exists)

        let sender_certif = match storage
            .get_device_certificate(UpTo::Index(certificate_index), sender)
            .await
        {
            // Exists, as expected :)
            Ok(certif) => certif,

            // Doesn't exist at the considered index :(
            Err(
                GetCertificateError::NonExisting | GetCertificateError::ExistButTooRecent { .. },
            ) => {
                let what = InvalidMessageError::NonExistantAuthor {
                    index,
                    sender: sender.to_owned(),
                    timestamp,
                };
                return Err(ValidateMessageError::InvalidMessage(what));
            }

            // D'oh :/
            Err(err @ GetCertificateError::Internal(_)) => {
                return Err(ValidateMessageError::Internal(err.into()))
            }
        };

        // 2.2) Check user is not revoked

        match storage
            .get_revoked_user_certificate(UpTo::Index(certificate_index), sender.user_id())
            .await
        {
            // Not revoked at the considered index, as we expected :)
            Ok(None) => (),

            // Revoked :(
            Ok(Some(_)) => {
                let what = InvalidMessageError::RevokedAuthor {
                    index,
                    sender: sender.to_owned(),
                    timestamp,
                };
                return Err(ValidateMessageError::InvalidMessage(what));
            }

            // D'oh :/
            Err(err) => return Err(ValidateMessageError::Internal(err.into())),
        }

        // 3) Actually validate the message

        let content = MessageContent::decrypt_verify_and_load_for(
            body,
            &self.device.private_key,
            &sender_certif.verify_key,
            sender,
            timestamp,
        )
        .map_err(|error| {
            let what = InvalidMessageError::Corrupted {
                index,
                sender: sender.to_owned(),
                timestamp,
                error,
            };
            ValidateMessageError::InvalidMessage(what)
        })?;

        // 4) Finally we have to check the message content is consistent with the system

        match &content {
            MessageContent::SharingRevoked {
                author,
                timestamp,
                id,
            }
            | MessageContent::SharingGranted {
                author,
                timestamp,
                id,
                ..
            } => {
                // There is two separate things going on here:
                // - Sender has created realm role certificate
                // - Sender has sent a sharing message
                //
                // The certificate is validated by the server poll system just like any
                // other certificate, so we don't have to handle it validation here.
                //
                // So the only check we have is to retreive this certificate.
                let certif_as_expected = match storage.get_certificate(certificate_index).await {
                    Ok(certif) => {
                        if let AnyArcCertificate::RealmRole(certif) = certif {
                            let mut as_expected = true;
                            as_expected |= matches!(&certif.author, CertificateSignerOwned::User(certif_author) if certif_author == author);
                            as_expected |= certif.timestamp == *timestamp;
                            as_expected |= certif.realm_id == *id;
                            as_expected |= certif.user_id == *self.device.user_id();
                            match &content {
                                MessageContent::SharingRevoked { .. } => {
                                    as_expected |= certif.role == None;
                                }
                                MessageContent::SharingGranted { .. } => {
                                    as_expected |= certif.role != None;
                                }
                                _ => {
                                    unreachable!()
                                }
                            }
                            as_expected
                        } else {
                            false
                        }
                    }
                    Err(GetCertificateError::ExistButTooRecent { .. }) => false,
                    Err(GetCertificateError::NonExisting) => false,
                    Err(err @ GetCertificateError::Internal(_)) => {
                        return Err(ValidateMessageError::Internal(err.into()))
                    }
                };
                if !certif_as_expected {
                    let what = InvalidMessageError::NoCorrespondingRealmRoleCertificate {
                        index,
                        sender: sender.to_owned(),
                        timestamp: *timestamp,
                    };
                    return Err(ValidateMessageError::InvalidMessage(what));
                }
            }

            MessageContent::SharingReencrypted {
                author,
                timestamp,
                id,
                ..
            } => {
                // No certificate is generated for reencryption, so we only check the
                // role of the sender
                let author_is_owner = storage
                    .get_user_realm_role(UpTo::Index(certificate_index), author.user_id(), *id)
                    .await?
                    .map(|certif| certif.role == Some(RealmRole::Owner))
                    .unwrap_or(false);
                if !author_is_owner {
                    let what = InvalidMessageError::AuthorNotAllowedToReencrypt {
                        index,
                        sender: sender.to_owned(),
                        timestamp: *timestamp,
                    };
                    return Err(ValidateMessageError::InvalidMessage(what));
                }
            }

            // Nothing more to check
            MessageContent::Ping { .. } => (),
        }

        // All done & good !

        Ok(content)
    }
}
