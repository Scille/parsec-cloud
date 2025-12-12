// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AnonymousCmds, ConnectionError};
use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum SubmitterGetAsyncEnrollmentInfoError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Enrollment not found")]
    EnrollmentNotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub enum PendingAsyncEnrollmentInfo {
    Submitted {
        submitted_on: DateTime,
    },
    Cancelled {
        submitted_on: DateTime,
        cancelled_on: DateTime,
    },
    Rejected {
        submitted_on: DateTime,
        rejected_on: DateTime,
    },
    Accepted {
        submitted_on: DateTime,
        accepted_on: DateTime,
    },
}

pub async fn submitter_get_async_enrollment_info(
    cmds: &AnonymousCmds,
    enrollment_id: AsyncEnrollmentID,
) -> Result<PendingAsyncEnrollmentInfo, SubmitterGetAsyncEnrollmentInfoError> {
    let status = {
        use libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_info::{Req, Rep};

        let req = Req { enrollment_id };
        let rep = cmds.send(req).await?;
        match rep {
            Rep::Ok(status) => status,
            Rep::EnrollmentNotFound => {
                return Err(SubmitterGetAsyncEnrollmentInfoError::EnrollmentNotFound)
            }
            bad_rep @ Rep::UnknownStatus { .. } => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        }
    };

    let info = {
        use libparsec_protocol::anonymous_cmds::v5::async_enrollment_info::InfoStatus;
        match status {
            InfoStatus::Submitted { submitted_on } => {
                PendingAsyncEnrollmentInfo::Submitted { submitted_on }
            }
            InfoStatus::Cancelled {
                submitted_on,
                cancelled_on,
            } => PendingAsyncEnrollmentInfo::Cancelled {
                submitted_on,
                cancelled_on,
            },
            InfoStatus::Rejected {
                submitted_on,
                rejected_on,
            } => PendingAsyncEnrollmentInfo::Rejected {
                submitted_on,
                rejected_on,
            },
            InfoStatus::Accepted {
                submitted_on,
                accepted_on,
                ..
            } => PendingAsyncEnrollmentInfo::Accepted {
                submitted_on,
                accepted_on,
            },
        }
    };

    Ok(info)
}
