// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::{
    greater_timestamp,
    store::{CertifStoreError, GetCertificateError},
    CertificateBasedActionOutcome, CertificateOps, GreaterTimestampOffset, InvalidCertificateError,
    UpTo,
};
use crate::EventTooMuchDriftWithServerClock;

pub type CertifGetRealmCanSelfPromoteToOwnerError = CertifStoreError;

pub(super) async fn get_realm_can_self_promote_to_owner(
    ops: &CertificateOps,
    realm_id: VlobID,
) -> Result<bool, CertifGetRealmCanSelfPromoteToOwnerError> {
    ops.store
        .for_read(async |store| {
            // OUTSIDER cannot become OWNER, hence cannot self-promote
            if store.get_current_self_profile().await? == UserProfile::Outsider {
                return Ok(false);
            }

            // The following code is very similar to what is in `add.rs`'s `check_realm_role_certificate_consistency`.
            // However we cannot factorize them easily since the `store` object is of different types
            // (`CertificatesStoreReadGuard` vs `CertificatesStoreWriteGuard`).

            let realm_roles = store.get_realm_roles(UpTo::Current, realm_id).await?;

            let mut current_roles = HashMap::new();
            for certif in realm_roles {
                match certif.role {
                    Some(role) => {
                        current_roles.insert(certif.user_id, role);
                    }
                    None => {
                        current_roles.remove(&certif.user_id);
                    }
                }
            }

            let role_to_priority = |role: RealmRole| match role {
                RealmRole::Reader => 1u8,
                RealmRole::Contributor => 2,
                RealmRole::Manager => 3,
                RealmRole::Owner => 4,
            };

            let author_role_priority = match current_roles.get(&ops.device.user_id) {
                // There is nothing to promote if we are already OWNER, and nothing we can do
                // if we are not part of the realm!
                None | Some(RealmRole::Owner) => return Ok(false),
                Some(role) => role_to_priority(*role),
            };

            let more_senior_members = current_roles.into_iter().filter_map(|(user_id, role)| {
                if role_to_priority(role) > author_role_priority {
                    Some(user_id)
                } else {
                    None
                }
            });
            for more_senior_member in more_senior_members {
                let is_revoked = store
                    .get_revoked_user_certificate(UpTo::Current, more_senior_member)
                    .await?
                    .is_some();
                if is_revoked {
                    continue;
                }

                // Must also exclude OUTSIDERs, as they are not allowed to self-promote
                let more_senior_member_profile = {
                    // Updates overwrite each others, so last one contains the current profile
                    let maybe_update = store
                        .get_last_user_update_certificate(UpTo::Current, more_senior_member)
                        .await?;
                    if let Some(last_update) = maybe_update {
                        last_update.new_profile
                    } else {
                        match store
                            .get_user_certificate(UpTo::Current, more_senior_member)
                            .await {
                                Ok(certif) => Ok(certif.profile),
                                Err(GetCertificateError::NonExisting | GetCertificateError::ExistButTooRecent { .. }) => {
                                    Err(CertifGetRealmCanSelfPromoteToOwnerError::Internal(anyhow::anyhow!(
                                        "Local storage of certificates seems corrupted: User {} is member of workspace {}, but its user certificate doesn't exist", more_senior_member, realm_id,
                                    )))
                                },
                                // D'oh :/
                                Err(err @ GetCertificateError::Internal(_)) => Err(CertifGetRealmCanSelfPromoteToOwnerError::Internal(err.into())),
                            }?
                    }
                };
                if more_senior_member_profile == UserProfile::Outsider {
                    continue;
                }

                // The workspace still contains a non-revoked member with a higher role than ours
                return Ok(false);
            }

            Ok(true)
        })
        .await?
        .map_err(|err: anyhow::Error| CertifGetRealmCanSelfPromoteToOwnerError::Internal(err))
}

#[derive(Debug, thiserror::Error)]
pub enum CertifSelfPromoteToOwnerError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Author not allowed")]
    AuthorNotAllowed,
    #[error("Workspace realm not found")]
    RealmNotFound,
    #[error("The workspace's realm has been deleted on the server")]
    RealmDeleted,
    #[error("An active user is already OWNER of this workspace")]
    ActiveOwnerAlreadyExists,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
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

impl From<CertifStoreError> for CertifSelfPromoteToOwnerError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

pub(super) async fn self_promote_to_owner(
    ops: &CertificateOps,
    realm_id: VlobID,
) -> Result<CertificateBasedActionOutcome, CertifSelfPromoteToOwnerError> {
    // Loop is needed to deal with server requiring greater timestamp
    let mut timestamp = ops.device.now();
    loop {
        let outcome = do_server_command(ops, realm_id, timestamp).await?;

        match outcome {
            DoServerCommandOutcome::Done(outcome) => return Ok(outcome),
            DoServerCommandOutcome::RequireGreaterTimestamp(strictly_greater_than) => {
                // TODO: handle `strictly_greater_than` out of the client ballpark by
                // returning an error
                timestamp = greater_timestamp(
                    &ops.device.time_provider,
                    GreaterTimestampOffset::Realm,
                    strictly_greater_than,
                );
            }
        }
    }
}

enum DoServerCommandOutcome {
    Done(CertificateBasedActionOutcome),
    RequireGreaterTimestamp(DateTime),
}

async fn do_server_command(
    ops: &CertificateOps,
    realm_id: VlobID,
    timestamp: DateTime,
) -> Result<DoServerCommandOutcome, CertifSelfPromoteToOwnerError> {
    // Build role certificate with author promoting themselves to OWNER
    let signed_certificate = RealmRoleCertificate {
        author: ops.device.device_id,
        timestamp,
        realm_id,
        user_id: ops.device.user_id,
        role: Some(RealmRole::Owner),
    }
    .dump_and_sign(&ops.device.signing_key);

    use authenticated_cmds::latest::realm_self_promote_to_owner::{Rep, Req};

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
        Rep::RequireGreaterTimestamp {
            strictly_greater_than,
        } => {
            // The retry is handled by the caller
            Ok(DoServerCommandOutcome::RequireGreaterTimestamp(
                strictly_greater_than,
            ))
        }
        Rep::ActiveOwnerAlreadyExists { .. } => {
            Err(CertifSelfPromoteToOwnerError::ActiveOwnerAlreadyExists)
        }
        Rep::AuthorNotAllowed => Err(CertifSelfPromoteToOwnerError::AuthorNotAllowed),
        Rep::RealmNotFound => Err(CertifSelfPromoteToOwnerError::RealmNotFound),
        Rep::RealmDeleted => Err(CertifSelfPromoteToOwnerError::RealmDeleted),
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

            Err(CertifSelfPromoteToOwnerError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            })
        }
        bad_rep @ (Rep::InvalidCertificate | Rep::UnknownStatus { .. }) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
