// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{AnonymousCmds, ConnectionError};
use libparsec_types::prelude::*;

use crate::ClientConfig;

#[derive(Debug, thiserror::Error)]
pub enum SubmitterCancelAsyncEnrollmentError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Enrollment not found")]
    NotFound,
    #[error("Cannot access local storage")]
    StorageNotAvailable,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub async fn submitter_cancel_async_enrollment(
    config: Arc<ClientConfig>,
    addr: ParsecAsyncEnrollmentAddr,
    enrollment_id: AsyncEnrollmentID,
) -> Result<(), SubmitterCancelAsyncEnrollmentError> {
    let cmds = Arc::new(
        AnonymousCmds::new(&config.config_dir, addr.into(), config.proxy.clone())
            .map_err(|e| anyhow::anyhow!("Error while configuring connection to server: {e}"))?,
    );

    // 1. Cancel the enrollment on the server

    {
        use libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_cancel::{Req, Rep};

        let req = Req { enrollment_id };
        let rep = cmds.send(req).await?;
        match rep {
            // We basically ignore the server answer and go idempotent here:
            // - Most likely case is `enrollment_id` has been obtained by listing
            //   the pending enrollment's local files. In this case we have to
            //   remove our enrollment's local file no matter what.
            // - If the enrollment ID is a dummy one, the server returns a not found,
            //   but so will `remove_pending_async_enrollment()`.
            // - Last case is a very weird one: the enrollment ID is a valid one (hence
            //   the server cancels it), but it doesn't exist locally. This leads us
            //   to return a not found error without informing the user the enrollment
            //   has been cancelled, but we don't really care about this given how
            //   unlikely this case is (how does the user know the ID in the first place?).
            Rep::Ok | Rep::EnrollmentNoLongerAvailable | Rep::EnrollmentNotFound => {}
            bad_rep @ Rep::UnknownStatus { .. } => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        }
    };

    // 2. Remove the enrollment info locally

    let path = libparsec_platform_device_loader::get_default_pending_async_enrollment_file(
        &config.config_dir,
        enrollment_id,
    );
    libparsec_platform_device_loader::remove_pending_async_enrollment(&config.config_dir, &path)
        .await
        .map_err(|err| match err {
            libparsec_platform_device_loader::RemoveDeviceError::NotFound => {
                SubmitterCancelAsyncEnrollmentError::NotFound
            }
            libparsec_platform_device_loader::RemoveDeviceError::StorageNotAvailable => {
                SubmitterCancelAsyncEnrollmentError::StorageNotAvailable
            }
            libparsec_platform_device_loader::RemoveDeviceError::Internal(err) => {
                SubmitterCancelAsyncEnrollmentError::Internal(err)
            }
        })?;

    Ok(())
}
