// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::Client;
use crate::{
    certif::InvalidCertificateError, CertifGetRealmNeedsError, CertifPollServerError,
    CertifRotateRealmKeyError, CertifShareRealmError, ClientGetCurrentSelfProfileError,
    EventTooMuchDriftWithServerClock, RealmNeeds,
};

#[derive(Debug, thiserror::Error)]
pub enum ClientProcessWorkspacesNeedsError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
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

pub async fn process_workspaces_needs(
    client: &Client,
) -> Result<(), ClientProcessWorkspacesNeedsError> {
    // There is a footgun to be aware of here !
    //
    // We should only use the certificate ops to get informations about what workspace
    // our user is part of.
    //
    // Indeed, it could be tempting to instead use `Client::list_workspaces` however this
    // method relies on the local user manifest that itself is updated whenever a new
    // certificate arrives.
    //
    // So the issue is we would have a new workspace event triggering two monitors (the
    // one calling `refresh_workspaces_list` and the one calling `process_workspaces_needs`),
    // while the latter depends on the output of the former...
    let realms = client
        .certificates_ops
        .get_current_self_realms_role()
        .await
        .map_err(|err| match err {
            ClientGetCurrentSelfProfileError::Stopped => ClientProcessWorkspacesNeedsError::Stopped,
            ClientGetCurrentSelfProfileError::Internal(err) => err.into(),
        })?;

    for (realm_id, role, _) in realms {
        // Only OWNER can rotate keys
        if role != Some(RealmRole::Owner) {
            continue;
        }

        process_workspace_needs(client, realm_id).await?;
    }

    Ok(())
}

async fn process_workspace_needs(
    client: &Client,
    realm_id: VlobID,
) -> Result<(), ClientProcessWorkspacesNeedsError> {
    let mut needs = client
        .certificates_ops
        .get_realm_needs(realm_id)
        .await
        .map_err(|err| match err {
            CertifGetRealmNeedsError::Stopped => ClientProcessWorkspacesNeedsError::Stopped,
            CertifGetRealmNeedsError::Internal(err) => err.into(),
        })?;

    if let RealmNeeds::UnshareThenKeyRotation {
        current_key_index,
        revoked_users,
    } = &needs
    {
        for user in revoked_users {
            let outcome = client
                .certificates_ops
                .share_realm(realm_id, *user, None)
                .await;
            match outcome {
                Ok(_) => (),
                Err(err) => match err {
                    // Valid errors

                    CertifShareRealmError::Stopped => return Err(ClientProcessWorkspacesNeedsError::Stopped),
                    CertifShareRealmError::Offline(e) => return Err(ClientProcessWorkspacesNeedsError::Offline(e)),
                    // A concurrent operation has changed our rights to the workspace,
                    // hence we can no longer process its needs !
                    CertifShareRealmError::AuthorNotAllowed => return Ok(()),
                    CertifShareRealmError::TimestampOutOfBallpark {
                        server_timestamp,
                        client_timestamp,
                        ballpark_client_early_offset,
                        ballpark_client_late_offset,
                    } => {
                        let event = EventTooMuchDriftWithServerClock {
                            server_timestamp,
                            ballpark_client_early_offset,
                            ballpark_client_late_offset,
                            client_timestamp,
                        };
                        client.event_bus.send(&event);

                        return Err(ClientProcessWorkspacesNeedsError::TimestampOutOfBallpark {
                            server_timestamp,
                            client_timestamp,
                            ballpark_client_early_offset,
                            ballpark_client_late_offset,
                        });
                    }
                    CertifShareRealmError::InvalidKeysBundle(_) => {
                        // TODO: handle self-healing here !
                        return Ok(());
                    }
                    CertifShareRealmError::InvalidCertificate(err) => return Err(ClientProcessWorkspacesNeedsError::InvalidCertificate(err)),
                    CertifShareRealmError::Internal(err) => return Err(err.into()),

                    // Invalid errors

                    bad_rep @ (
                        // If we were revoked, we wouldn't be here anyway !
                        CertifShareRealmError::RecipientIsSelf
                        // We got the recipient ID from the certificates !
                        | CertifShareRealmError::RecipientNotFound
                        // We got the realm ID from the certificates !
                        | CertifShareRealmError::RealmNotFound
                        // Unsharing is allowed on a revoked user (and that's precisely what we are doing here !)
                        | CertifShareRealmError::RecipientRevoked
                        // Role is only given when sharing, not unsharing !
                        | CertifShareRealmError::RoleIncompatibleWithOutsider
                        // Keys bundle only needed when sharing, not unsharing !
                        | CertifShareRealmError::NoKey
                    ) => return Err(anyhow::anyhow!("Unexpected server response: {}", bad_rep).into()),
                },
            }
        }

        // Given the key rotation encrypt the keys bundle access for each member of the
        // realm we must fetch back the realm role certificates we've just send to the
        // server, otherwise newly unshared user will be part of the key rotation...
        client
            .poll_server_for_new_certificates()
            .await
            .map_err(|err| match err {
                CertifPollServerError::Stopped => ClientProcessWorkspacesNeedsError::Stopped,
                CertifPollServerError::Offline(e) => ClientProcessWorkspacesNeedsError::Offline(e),
                CertifPollServerError::InvalidCertificate(err) => {
                    ClientProcessWorkspacesNeedsError::InvalidCertificate(err)
                }
                CertifPollServerError::Internal(err) => err.into(),
            })?;

        needs = RealmNeeds::KeyRotationOnly {
            current_key_index: *current_key_index,
        };
    }

    if let RealmNeeds::KeyRotationOnly { current_key_index } = &needs {
        let target_key_index = current_key_index.unwrap_or(0) + 1;
        let outcome = client
            .certificates_ops
            .rotate_realm_key_idempotent(realm_id, target_key_index)
            .await;
        match outcome {
            Ok(_) => (),
            Err(err) => match err {
                // Valid errors
                CertifRotateRealmKeyError::Offline(e) => {
                    return Err(ClientProcessWorkspacesNeedsError::Offline(e))
                }
                CertifRotateRealmKeyError::Stopped => {
                    return Err(ClientProcessWorkspacesNeedsError::Stopped)
                }
                // A concurrent operation has changed our rights to the workspace,
                // hence we can no longer process its needs !
                CertifRotateRealmKeyError::AuthorNotAllowed => return Ok(()),
                CertifRotateRealmKeyError::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                } => {
                    let event = EventTooMuchDriftWithServerClock {
                        server_timestamp,
                        ballpark_client_early_offset,
                        ballpark_client_late_offset,
                        client_timestamp,
                    };
                    client.event_bus.send(&event);

                    return Err(ClientProcessWorkspacesNeedsError::TimestampOutOfBallpark {
                        server_timestamp,
                        client_timestamp,
                        ballpark_client_early_offset,
                        ballpark_client_late_offset,
                    });
                }
                CertifRotateRealmKeyError::InvalidCertificate(err) => {
                    return Err(ClientProcessWorkspacesNeedsError::InvalidCertificate(err))
                }
                CertifRotateRealmKeyError::CurrentKeysBundleCorrupted(_) => {
                    // TODO: handle self-healing here !
                    return Ok(());
                }
                CertifRotateRealmKeyError::Internal(err) => return Err(err.into()),

                // Invalid errors

                // We got the realm ID from the certificates !
                bad_rep @ CertifRotateRealmKeyError::UnknownRealm => {
                    return Err(anyhow::anyhow!("Unexpected server response: {}", bad_rep).into())
                }
            },
        }

        needs = RealmNeeds::Nothing;
    }

    assert!(matches!(needs, RealmNeeds::Nothing));
    Ok(())
}
