// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::CertificatesOps;
use crate::EventTooMuchDriftWithServerClock;

#[derive(Debug, thiserror::Error)]
pub enum EnsureRealmsCreatedError {
    #[error("Cannot reach the server")]
    Offline,
    // Note `InvalidManifest` here, this is because we self-repair in case of invalid
    // user manifest (given otherwise the client would be stuck for good !)
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    BadTimestamp {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for EnsureRealmsCreatedError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub async fn ensure_realms_created(
    ops: &CertificatesOps,
    realms_ids: &[VlobID],
) -> Result<(), EnsureRealmsCreatedError> {
    // Note that `roles` also mention the realm we used to be part of
    let roles = ops.get_current_self_realms_roles().await?;
    for realm_id in realms_ids {
        if roles.contains_key(realm_id) {
            // If we have a role certificate, then the realm must exist on the server
            continue;
        }
        // We never had a role for the given realm, it can mean two things:
        // - The role is very new and we haven't processed the corresponding certificate
        //   yet
        // - There is no role certificate because we are the creator of a workspace and
        //   so far haven't synced
        // Given we cannot tell in which case we are, it is now time to try to create
        // the realm.
        create_realm_idempotent(ops, *realm_id).await?;
    }
    Ok(())
}

async fn create_realm_idempotent(
    ops: &CertificatesOps,
    realm_id: VlobID,
) -> Result<(), EnsureRealmsCreatedError> {
    let mut timestamp = ops.device.now();
    loop {
        let certif =
            RealmRoleCertificate::new_root(ops.device.device_id.to_owned(), timestamp, realm_id)
                .dump_and_sign(&ops.device.signing_key);

        use authenticated_cmds::latest::realm_create::{Rep, Req};

        let req = Req {
            role_certificate: certif.into(),
        };

        let rep = ops.cmds.send(req).await?;

        return match rep {
            Rep::Ok => Ok(()),
            // It's possible a previous attempt to create this realm
            // succeeded but we didn't receive the confirmation, hence
            // we play idempotent here.
            Rep::AlreadyExists => Ok(()),
            Rep::RequireGreaterTimestamp {
                strictly_greater_than,
            } => {
                timestamp = std::cmp::max(strictly_greater_than, ops.device.time_provider.now());
                continue;
            }
            Rep::BadTimestamp {
                backend_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
                client_timestamp,
                ..
            } => {
                let event = EventTooMuchDriftWithServerClock {
                    backend_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                    client_timestamp,
                };
                ops.event_bus.send(&event);

                Err(EnsureRealmsCreatedError::BadTimestamp {
                    server_timestamp: backend_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                })
            }
            bad_rep @ (Rep::InvalidData { .. }
            | Rep::InvalidCertification { .. }
            | Rep::NotFound { .. }
            | Rep::UnknownStatus { .. }) => {
                Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        };
    }
}
