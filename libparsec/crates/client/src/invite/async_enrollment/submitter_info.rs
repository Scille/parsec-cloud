// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{AnonymousCmds, ConnectionError};
use libparsec_types::prelude::*;

use crate::ClientConfig;

#[derive(Debug, thiserror::Error)]
pub enum SubmitterGetAsyncEnrollmentInfoError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Server doesn't know this enrollment")]
    EnrollmentNotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
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
    config: Arc<ClientConfig>,
    addr: ParsecAsyncEnrollmentAddr,
    enrollment_id: AsyncEnrollmentID,
) -> Result<PendingAsyncEnrollmentInfo, SubmitterGetAsyncEnrollmentInfoError> {
    let cmds = Arc::new(
        AnonymousCmds::new(&config.config_dir, addr.into(), config.proxy.clone())
            .map_err(|e| anyhow::anyhow!("Error while configuring connection to server: {e}"))?,
    );

    let status = {
        use libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_info::{Req, Rep};

        let req = Req { enrollment_id };
        let rep = cmds.send(req).await?;
        match rep {
            Rep::Ok(status) => status,
            Rep::EnrollmentNotFound => {
                // Note the server might not know about our valid enrollment if
                // it is a very old one (think of a periodic database cleanup
                // run by the database admin).
                return Err(SubmitterGetAsyncEnrollmentInfoError::EnrollmentNotFound);
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
