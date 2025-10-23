// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_protocol::authenticated_cmds::latest::pki_enrollment_reject::{Rep, Req};
use libparsec_types::prelude::*;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum ClientPkiEnrollmentRejectError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("author not allowed: must be admin to reject pki enrollment")]
    AuthorNotAllowed,
    #[error("enrollment is no longer pending")]
    EnrollmentNoLongerAvailable,
    #[error("enrollment not found")]
    EnrollmentNotFound,
}

pub async fn reject(
    cmds: &Arc<AuthenticatedCmds>,
    enrollment_id: EnrollmentID,
) -> Result<(), ClientPkiEnrollmentRejectError> {
    match cmds.send(Req { enrollment_id }).await? {
        Rep::Ok => Ok(()),
        Rep::AuthorNotAllowed => Err(ClientPkiEnrollmentRejectError::AuthorNotAllowed),
        Rep::EnrollmentNoLongerAvailable => {
            Err(ClientPkiEnrollmentRejectError::EnrollmentNoLongerAvailable)
        }
        Rep::EnrollmentNotFound => Err(ClientPkiEnrollmentRejectError::EnrollmentNotFound),
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
