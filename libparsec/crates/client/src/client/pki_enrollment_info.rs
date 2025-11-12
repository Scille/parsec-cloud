// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::ClientConfig;
pub use anonymous_cmds::latest::pki_enrollment_info::PkiEnrollmentInfoStatus;
use libparsec_client_connection::{protocol::anonymous_cmds, AnonymousCmds, ConnectionError};
use libparsec_platform_pki::{load_answer_payload, SignedMessage};
use libparsec_types::prelude::*;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum PkiEnrollmentInfoError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("No enrollment found with that id")]
    EnrollmentNotFound,
    #[error("Invalid accept payload")]
    InvalidAcceptPayload,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub enum PKIInfoItem {
    Accepted {
        // Deserialized version of the provided payload
        // signature should have been checked before loading it
        answer: PkiEnrollmentAnswerPayload,
        accepted_on: DateTime,
        submitted_on: DateTime,
    },
    Submitted {
        submitted_on: DateTime,
    },
    Rejected {
        rejected_on: DateTime,
        submitted_on: DateTime,
    },
    Cancelled {
        submitted_on: DateTime,
        cancelled_on: DateTime,
    },
}

pub async fn info(
    config: Arc<ClientConfig>,
    addr: ParsecPkiEnrollmentAddr,
    enrollment_id: PKIEnrollmentID,
) -> Result<PKIInfoItem, PkiEnrollmentInfoError> {
    use anonymous_cmds::latest::pki_enrollment_info::{Rep, Req};
    let cmds = AnonymousCmds::new(
        &config.config_dir,
        ParsecAnonymousAddr::ParsecPkiEnrollmentAddr(addr.clone()),
        config.proxy.clone(),
    )?;
    let rep = cmds.send(Req { enrollment_id }).await?;

    let status = match rep {
        Rep::Ok(status) => status,
        Rep::EnrollmentNotFound => return Err(PkiEnrollmentInfoError::EnrollmentNotFound),
        rep @ Rep::UnknownStatus { .. } => {
            return Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    };

    // Check that the payload is valid
    let answer = match status {
        PkiEnrollmentInfoStatus::Submitted { submitted_on } => {
            PKIInfoItem::Submitted { submitted_on }
        }
        PkiEnrollmentInfoStatus::Rejected {
            rejected_on,
            submitted_on,
        } => PKIInfoItem::Rejected {
            rejected_on,
            submitted_on,
        },
        PkiEnrollmentInfoStatus::Cancelled {
            submitted_on,
            cancelled_on,
        } => PKIInfoItem::Cancelled {
            submitted_on,
            cancelled_on,
        },
        PkiEnrollmentInfoStatus::Accepted {
            accept_payload,
            accept_payload_signature,
            accepted_on,
            accepter_der_x509_certificate,
            submitted_on,
            accept_payload_signature_algorithm,
        } => {
            let message = SignedMessage {
                algo: accept_payload_signature_algorithm,
                signature: accept_payload_signature,
                message: accept_payload,
            };
            let answer = load_answer_payload(&accepter_der_x509_certificate, &message, accepted_on)
                .map_err(|_| PkiEnrollmentInfoError::InvalidAcceptPayload)?;
            PKIInfoItem::Accepted {
                answer,
                accepted_on,
                submitted_on,
            }
        }
    };
    Ok(answer)
}
