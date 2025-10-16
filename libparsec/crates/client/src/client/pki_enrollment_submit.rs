// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AnonymousCmds, ConnectionError};
use libparsec_protocol::anonymous_cmds::latest::pki_enrollment_submit::{Rep, Req};
use libparsec_types::prelude::*;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum ClientPkiEnrollmentSubmitError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("User already enrolled with this certificate")]
    AlreadyEnrolled,
    #[error("Certificate already submitted on {0}")]
    AlreadySubmitted(DateTime),
    #[error("User already enrolled with this email")]
    EmailAlreadyUsed,
    #[error("EnrollmentId already used")]
    IdAlreadyUsed,
    #[error("Unable to read payload")]
    InvalidPayload,
}

pub async fn pki_enrollment_submit(
    cmds: &Arc<AnonymousCmds>,
    der_x509_certificate: Bytes,
    enrollment_id: EnrollmentID,
    force: bool,
    payload: Bytes,
    payload_signature: Bytes,
) -> Result<DateTime, ClientPkiEnrollmentSubmitError> {
    let _ = match cmds
        .send(Req {
            der_x509_certificate,
            enrollment_id,
            force,
            payload,
            payload_signature,
        })
        .await?
    {
        Rep::Ok { submitted_on } => Ok(submitted_on),
        Rep::AlreadyEnrolled => Err(ClientPkiEnrollmentSubmitError::AlreadyEnrolled),
        Rep::AlreadySubmitted { submitted_on } => Err(
            ClientPkiEnrollmentSubmitError::AlreadySubmitted(submitted_on),
        ),
        Rep::EmailAlreadyUsed => Err(ClientPkiEnrollmentSubmitError::EmailAlreadyUsed),
        Rep::IdAlreadyUsed => Err(ClientPkiEnrollmentSubmitError::IdAlreadyUsed),
        Rep::InvalidPayload => Err(ClientPkiEnrollmentSubmitError::InvalidPayload),
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    };
    todo!("#11440");
}
