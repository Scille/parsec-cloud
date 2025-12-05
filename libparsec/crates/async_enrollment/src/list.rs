// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum AsyncEnrollementListError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Author not allowed")]
    AuthorNotAllowed,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub enum AsyncEnrollementIdentitySystem {
    PKI,
    OpenBao,
}

pub struct AsyncEnrollmentPending {
    pub enrollment_id: AsyncEnrollmentID,
    pub submitted_on: DateTime,
    pub requested_device_label: DeviceLabel,
    /// ⚠️ The submitter might be lying about his email here !
    /// Identity validation requires user interaction (to authenticate to OpenBao,
    /// or to ask the user to plug his smartcard), so we skip this step here.
    /// It is considered okay since the validation is going to be to first step
    /// no matter what when the user chooses to accept an enrollment request.
    pub requested_human_handle: HumanHandle,
    pub identity_validation_system: AsyncEnrollementIdentitySystem,
}

pub async fn async_enrollment_list(
    cmds: &AuthenticatedCmds,
) -> Result<Vec<AsyncEnrollmentPending>, AsyncEnrollementListError> {
    let enrollments = {
        use libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_list::{Req, Rep};

        let rep = cmds.send(Req).await?;
        match rep {
            Rep::Ok { enrollments } => enrollments,
            Rep::AuthorNotAllowed => return Err(AsyncEnrollementListError::AuthorNotAllowed),
            bad_rep @ Rep::UnknownStatus { .. } => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        }
    };

    use libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_list::PayloadSignature;
    Ok(enrollments
        .into_iter()
        .filter_map(|enrollment| {
            // Just skip enrollments with invalid payload: in practice this should never occur
            // since the server has already validated the payload format.
            let payload = AsyncEnrollmentSubmitPayload::load(&enrollment.payload).ok()?;
            Some(AsyncEnrollmentPending {
                enrollment_id: enrollment.enrollment_id,
                submitted_on: enrollment.submitted_on,
                requested_device_label: payload.requested_device_label,
                requested_human_handle: payload.requested_human_handle,
                identity_validation_system: match enrollment.payload_signature {
                    PayloadSignature::PKI { .. } => AsyncEnrollementIdentitySystem::PKI,
                    PayloadSignature::OpenBao { .. } => AsyncEnrollementIdentitySystem::OpenBao,
                },
            })
        })
        .collect())
}
