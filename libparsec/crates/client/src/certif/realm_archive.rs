// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::{
    greater_timestamp, store::CertifStoreError, CertificateBasedActionOutcome, CertificateOps,
    GreaterTimestampOffset,
};
use crate::EventTooMuchDriftWithServerClock;

/// Cannot simply use `RealmArchivingConfiguration` to specify the
/// archiving configuration since this type contains a datetime for
/// the deletion date.
///
/// Indeed: the minimal archiving period before deletion is checked
/// by subtracting the deletion date with the realm archiving
/// certificate's timestamp, which is currently not yet decided.
///
/// Hence passing the deletion date here would create invalid certificate,
/// typically when the caller want to use the minimal archiving period (which
/// would always be too short since the certificate timestamp is obtained
/// later on).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestedRealmArchivingConfiguration {
    Available,
    Archived,
    DeletionPlanned {
        // Don't use `Duration` since it is cumbersome to expose to the bindings
        archiving_period_in_seconds: u32,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum CertifArchiveRealmError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Unknown realm ID")]
    UnknownRealm,
    #[error("Realm has been deleted")]
    RealmDeleted,
    #[error("Not allowed")]
    AuthorNotAllowed,
    #[error("Archiving period is too short")]
    ArchivingPeriodTooShort,
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

impl From<CertifStoreError> for CertifArchiveRealmError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

pub(super) async fn archive_realm(
    ops: &CertificateOps,
    realm_id: VlobID,
    configuration: RequestedRealmArchivingConfiguration,
) -> Result<CertificateBasedActionOutcome, CertifArchiveRealmError> {
    let mut timestamp = ops.device.now();
    // Loop is needed to deal with server requiring greater timestamp
    loop {
        // 1) Generate the certificate

        let configuration = match configuration {
            RequestedRealmArchivingConfiguration::Available => {
                RealmArchivingConfiguration::Available
            }
            RequestedRealmArchivingConfiguration::Archived => RealmArchivingConfiguration::Archived,
            RequestedRealmArchivingConfiguration::DeletionPlanned {
                archiving_period_in_seconds,
            } => RealmArchivingConfiguration::DeletionPlanned {
                deletion_date: timestamp + Duration::seconds(archiving_period_in_seconds as i64),
            },
        };

        let certif = RealmArchivingCertificate {
            author: ops.device.device_id,
            timestamp,
            realm_id,
            configuration,
        }
        .dump_and_sign(&ops.device.signing_key)
        .into();

        // 2) Send the certificate to the server

        use authenticated_cmds::latest::realm_update_archiving::{Rep, Req};

        let req = Req {
            archiving_certificate: certif,
        };

        let rep = ops.cmds.send(req).await?;

        return match rep {
            Rep::Ok => Ok(CertificateBasedActionOutcome::Uploaded {
                certificate_timestamp: timestamp,
            }),
            Rep::RequireGreaterTimestamp {
                strictly_greater_than,
            } => {
                timestamp = greater_timestamp(
                    &ops.device.time_provider,
                    GreaterTimestampOffset::Archive,
                    strictly_greater_than,
                );
                continue;
            }
            Rep::RealmNotFound => Err(CertifArchiveRealmError::UnknownRealm),
            Rep::RealmDeleted => Err(CertifArchiveRealmError::RealmDeleted),
            Rep::AuthorNotAllowed => Err(CertifArchiveRealmError::AuthorNotAllowed),
            Rep::ArchivingPeriodTooShort => Err(CertifArchiveRealmError::ArchivingPeriodTooShort),
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

                Err(CertifArchiveRealmError::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                })
            }
            bad_rep @ (Rep::InvalidCertificate | Rep::UnknownStatus { .. }) => {
                Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        };
    }
}
