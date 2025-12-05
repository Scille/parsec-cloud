// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AnonymousCmds, ConnectionError};
use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum AsyncEnrollementSubmitError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("An enrollment request already exists for the requested email (submitted on: {submitted_on})")]
    EmailAlreadySubmitted { submitted_on: DateTime },
    #[error("A user already exists for the requested email")]
    EmailAlreadyEnrolled,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub trait AsyncEnrollmentSubmitIdentityStrategy {
    fn sign(&self, payload: &[u8]) -> libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_submit::PayloadSignature;
    fn human_handle(&self) -> &HumanHandle;
}

pub async fn async_enrollment_submit(
    cmds: &AnonymousCmds,
    force: bool,
    requested_device_label: DeviceLabel,
    identity_strategy: &dyn AsyncEnrollmentSubmitIdentityStrategy,
) -> Result<(AsyncEnrollmentID, DateTime), AsyncEnrollementSubmitError> {
    let enrollment_id = AsyncEnrollmentID::default();

    let user_private_key = PrivateKey::generate();
    let device_signing_key = SigningKey::generate();

    let payload: Bytes = AsyncEnrollmentSubmitPayload {
        verify_key: device_signing_key.verify_key().to_owned(),
        public_key: user_private_key.public_key().to_owned(),
        requested_device_label,
        requested_human_handle: identity_strategy.human_handle().to_owned(),
    }
    .dump()
    .into();
    let payload_signature = identity_strategy.sign(&payload);

    {
        use libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_submit::{Req, Rep};

        let req = Req {
            enrollment_id,
            force,
            payload,
            payload_signature,
        };

        let rep = cmds.send(req).await?;
        match rep {
            Rep::Ok { submitted_on } => Ok((enrollment_id, submitted_on)),
            Rep::EmailAlreadySubmitted { submitted_on } => {
                Err(AsyncEnrollementSubmitError::EmailAlreadySubmitted { submitted_on })
            }
            Rep::EmailAlreadyEnrolled => Err(AsyncEnrollementSubmitError::EmailAlreadyEnrolled),
            bad_rep @ (Rep::IdAlreadyUsed
            | Rep::InvalidPayload
            | Rep::InvalidPayloadSignature
            | Rep::UnknownStatus { .. }) => {
                Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        }
    }
}
