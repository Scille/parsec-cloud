// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AnonymousCmds, ConnectionError};
use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum AsyncEnrollementInfoError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Enrollment not found")]
    EnrollmentNotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum AsyncEnrollmentVerifyAcceptPayloadError {
    #[error("Invalid payload serialization")]
    BadPayload,
    /// The signature couldn't be verified
    #[error("Invalid payload signature")]
    BadSignature,
}

pub trait AsyncEnrollmentSubmitIdentityStrategy {
    fn verify_accept_payload(
        &self,
        payload: &[u8],
        payload_signature: libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_info::PayloadSignature,
    ) -> Result<AsyncEnrollmentSubmitPayload, AsyncEnrollmentVerifyAcceptPayloadError> {
        let submit_payload = AsyncEnrollmentSubmitPayload::load(payload)
            .map_err(|_| AsyncEnrollmentVerifyAcceptPayloadError::BadPayload)?;
        self._verify_accept_payload(payload, payload_signature)?;
        Ok(submit_payload)
    }
    fn _verify_accept_payload(
        &self,
        payload: &[u8],
        payload_signature: libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_info::PayloadSignature,
    ) -> Result<AsyncEnrollmentSubmitPayload, AsyncEnrollmentVerifyAcceptPayloadError>;
}

pub async fn async_enrollment_info(
    cmds: &AnonymousCmds,
    enrollment_id: AsyncEnrollmentID,
) -> Result<(AsyncEnrollmentID, DateTime), AsyncEnrollementInfoError> {
    let status = {
        use libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_info::{Req, Rep};

        let req = Req { enrollment_id };
        let rep = cmds.send(req).await?;
        match rep {
            Rep::Ok(status) => status,
            Rep::EnrollmentNotFound => return Err(AsyncEnrollementInfoError::EnrollmentNotFound),
            bad_rep @ Rep::UnknownStatus { .. } => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        }
    };

    {
        use libparsec_protocol::anonymous_cmds::v5::async_enrollment_info::InfoStatus;
        match status {
            InfoStatus::Submitted { submitted_on } => Ok(Info::Submitted { submitted_on }),
            InfoStatus::Rejected {
                rejected_on,
                submitted_on,
            } => Ok(Info::Rejected {
                rejected_on,
                submitted_on,
            }),
            InfoStatus::Cancelled {
                cancelled_on,
                submitted_on,
            } => Ok(Info::Cancelled {
                cancelled_on,
                submitted_on,
            }),
            InfoStatus::Accepted {
                accepted_on,
                payload,
                payload_signature,
                submitted_on,
            } => Ok(Info::Accepted {
                accepted_on,
                payload,
                payload_signature,
                submitted_on,
            }),
        }
    }
}
