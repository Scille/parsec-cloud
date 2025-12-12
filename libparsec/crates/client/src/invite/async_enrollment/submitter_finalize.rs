// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use libparsec_client_connection::{AnonymousCmds, ConnectionError};
use libparsec_platform_device_loader::{AvailableDevice, DeviceSaveStrategy};
use libparsec_types::prelude::*;

use crate::submitter_destroy_async_enrollment;

#[derive(Debug, thiserror::Error)]
pub enum PollAsyncEnrollmentError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Enrollment not found")]
    EnrollmentNotFound,
    #[error("Enrollment is not in the accepted state")]
    NotAccepted,
    #[error("Accepter has provided an invalid request: {0}")]
    BadAcceptPayload(#[from] AsyncEnrollmentVerifyAcceptPayloadError),
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

pub trait PollAsyncEnrollmentIdentityStrategy {
    fn verify_accept_payload(
        &self,
        payload: &[u8],
        payload_signature: libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_info::AcceptPayloadSignature,
    ) -> Result<AsyncEnrollmentAcceptPayload, AsyncEnrollmentVerifyAcceptPayloadError> {
        let accept_payload = AsyncEnrollmentAcceptPayload::load(payload)
            .map_err(|_| AsyncEnrollmentVerifyAcceptPayloadError::BadPayload)?;
        self._verify_accept_payload(payload, payload_signature)?;
        Ok(accept_payload)
    }
    fn _verify_accept_payload(
        &self,
        payload: &[u8],
        payload_signature: libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_info::AcceptPayloadSignature,
    ) -> Result<AsyncEnrollmentAcceptPayload, AsyncEnrollmentVerifyAcceptPayloadError>;
}

pub enum PollAsyncEnrollment {
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
        accept_payload: AsyncEnrollmentAcceptPayload,
    },
}

pub async fn submitter_finalize_async_enrollment(
    config_dir: &Path,
    cmds: &AnonymousCmds,
    enrollment_id: AsyncEnrollmentID,
    strategy: &DeviceSaveStrategy,
    key_file: &Path,
    identity_strategy: &dyn PollAsyncEnrollmentIdentityStrategy,
) -> Result<AvailableDevice, PollAsyncEnrollmentError> {
    todo!()
    // // 1) Load the local pending async enrollement file

    // // 2) Get back the enrollment from the server

    // let status = {
    //     use libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_info::{Req, Rep};

    //     let req = Req { enrollment_id };
    //     let rep = cmds.send(req).await?;
    //     match rep {
    //         Rep::Ok(status) => status,
    //         Rep::EnrollmentNotFound => return Err(PollAsyncEnrollmentError::EnrollmentNotFound),
    //         bad_rep @ Rep::UnknownStatus { .. } => {
    //             return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
    //         }
    //     }
    // };

    // // 2) Ensure the enrollment has been accepted

    // let (accept_payload, accept_payload_signature) = {
    //     use libparsec_protocol::anonymous_cmds::v5::async_enrollment_info::InfoStatus;
    //     match status {
    //         InfoStatus::Accepted {
    //             accept_payload,
    //             accept_payload_signature,
    //             ..
    //         } => (accept_payload, accept_payload_signature),
    //         _ => return Err(PollAsyncEnrollmentError::NotAccepted),
    //     }
    // };

    // // 3) Validate the accept payload

    // let accept_payload =
    //     identity_strategy.verify_accept_payload(&accept_payload, accept_payload_signature)?;

    // // All good, now we are officially part of the organization !

    // let organization_addr = ParsecOrganizationAddr::new(
    //     addr,
    //     addr.organization_id().clone(),
    //     accept_payload.root_verify_key,
    // );

    // let new_local_device = Arc::new(LocalDevice::generate_new_device(
    //     organization_addr,
    //     accept_payload.profile,
    //     accept_payload.human_handle,
    //     accept_payload.device_label,
    //     Some(accept_payload.user_id),
    //     Some(accept_payload.device_id),
    //     Some(signing_key),
    //     Some(private_key),
    //     None,
    //     None,
    //     None,
    // ));

    // // 4) Remove the pending async enrollment file and save the final local device file \o/

    // // Note we destroy the pending file, then save the final local device file.
    // //
    // // This means there is a small chance that we lost the the new device if an
    // // unexpected crash occurs between the two operations.
    // // This is considered okay since it is very unlikely, and if it occurs the admin
    // // can revoke the newly enrolled user and start again.
    // //
    // // Note that saving the device file first is more hazardous since, if an unexpected
    // // crash leads us to retry the operation, we could end up overwritting the device
    // // file. Which means generating new `local_symkey` and `user_realm_id/user_realm_key`
    // // that may clash with any existing data in the local database...
    // submitter_destroy_async_enrollment(config, enrollment_id).await?;

    // let available_device = libparsec_platform_device_loader::save_device(
    //     &config_dir,
    //     strategy,
    //     &new_local_device,
    //     key_file.to_path_buf(),
    // )
    // .await?;

    // Ok(available_device)
}
