// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum AsyncEnrollementRejectError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Author not allowed")]
    AuthorNotAllowed,
    #[error("Enrollment not found")]
    EnrollmentNotFound,
    #[error("Enrollment no longer available (either already accepted or rejected)")]
    EnrollmentNoLongerAvailable,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn async_enrollment_reject(
    cmds: &AuthenticatedCmds,
    enrollment_id: AsyncEnrollmentID,
) -> Result<(), AsyncEnrollementRejectError> {
    use libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_reject::{Req, Rep};

    let rep = cmds.send(Req { enrollment_id }).await?;
    match rep {
        Rep::Ok => Ok(()),
        Rep::AuthorNotAllowed => Err(AsyncEnrollementRejectError::AuthorNotAllowed),
        Rep::EnrollmentNotFound => Err(AsyncEnrollementRejectError::EnrollmentNotFound),
        Rep::EnrollmentNoLongerAvailable => {
            Err(AsyncEnrollementRejectError::EnrollmentNoLongerAvailable)
        }
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
