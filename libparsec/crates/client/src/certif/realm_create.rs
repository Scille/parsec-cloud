// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::{
    store::{CertifStoreError, RealmBootstrapState},
    CertificateBasedActionOutcome, CertificateOps,
};
use crate::{greater_timestamp, EventTooMuchDriftWithServerClock, GreaterTimestampOffset};

#[derive(Debug, thiserror::Error)]
pub enum CertifEnsureRealmCreatedError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Author not allowed")]
    AuthorNotAllowed,
    // Note `InvalidManifest` here, this is because we self-repair in case of invalid
    // user manifest (given otherwise the client would be stuck for good !)
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

impl From<ConnectionError> for CertifEnsureRealmCreatedError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            // TODO: handle organization expired and user revoked here ?
            err => Self::Internal(err.into()),
        }
    }
}

impl From<CertifStoreError> for CertifEnsureRealmCreatedError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

pub(super) async fn ensure_realm_created(
    ops: &CertificateOps,
    realm_id: VlobID,
) -> Result<CertificateBasedActionOutcome, CertifEnsureRealmCreatedError> {
    let realm_state = ops
        .store
        .for_read(|store| async move { store.get_realm_bootstrap_state(realm_id).await })
        .await??;
    if matches!(realm_state, RealmBootstrapState::CreatedInServer) {
        return Ok(CertificateBasedActionOutcome::LocalIdempotent);
    }

    // The realm is not created, it can mean two things:
    // 1. The initial realm role is very new and we haven't processed the corresponding
    //   certificate yet (i.e. the monitor is lagging behind).
    // 2. There is no role certificate because we are the creator this realm and
    //   so far haven't do the corresponding server-side creation yet.
    //
    // In practice case 1 is unlikely given we are informed about new realms by
    // fetching certificates. If we end up there, is most likely due backward
    // compatibility with Parsec <v3 local user manifest: at that time we process
    // dedicated messages to retrieve new realms, and didn't store certificates on disk.
    //
    // Anyway, given we cannot tell in which case we are, it is now time to try to create
    // the realm.
    create_realm_idempotent(ops, realm_id).await
}

async fn create_realm_idempotent(
    ops: &CertificateOps,
    realm_id: VlobID,
) -> Result<CertificateBasedActionOutcome, CertifEnsureRealmCreatedError> {
    let mut timestamp = ops.device.now();
    loop {
        let certif = RealmRoleCertificate::new_root(
            ops.device.user_id,
            ops.device.device_id,
            timestamp,
            realm_id,
        )
        .dump_and_sign(&ops.device.signing_key);

        use authenticated_cmds::latest::realm_create::{Rep, Req};

        let req = Req {
            realm_role_certificate: certif.into(),
        };

        let rep = ops.cmds.send(req).await?;

        return match rep {
            Rep::Ok => Ok(CertificateBasedActionOutcome::Uploaded {
                certificate_timestamp: timestamp,
            }),
            // It's possible a previous attempt to create this realm
            // succeeded but we didn't receive the confirmation, hence
            // we play idempotent here.
            Rep::RealmAlreadyExists {
                last_realm_certificate_timestamp,
            } => Ok(CertificateBasedActionOutcome::RemoteIdempotent {
                certificate_timestamp: last_realm_certificate_timestamp,
            }),
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
            // A concurrent operation must has modified our profile to OUTSIDER
            Rep::AuthorNotAllowed => Err(CertifEnsureRealmCreatedError::AuthorNotAllowed),
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

                Err(CertifEnsureRealmCreatedError::TimestampOutOfBallpark {
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
