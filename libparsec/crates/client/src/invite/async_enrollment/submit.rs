// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{AnonymousCmds, ConnectionError};
use libparsec_platform_async::PinBoxFutureResult;
use libparsec_platform_device_loader::{
    AvailablePendingAsyncEnrollment, SaveAsyncEnrollmentLocalPendingError,
};
use libparsec_types::prelude::*;

use crate::ClientConfig;

#[derive(Debug, thiserror::Error)]
pub enum SubmitAsyncEnrollmentError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("An enrollment request already exists for the requested email (submitted on: {submitted_on})")]
    EmailAlreadySubmitted { submitted_on: DateTime },
    #[error("A user already exists for the requested email")]
    EmailAlreadyEnrolled,
    #[error("Device storage is not available")]
    StorageNotAvailable,
    #[error(transparent)]
    InvalidPath(anyhow::Error),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),

    // OpenBao-related errors
    #[error("Invalid OpenBao server URL: {0}")]
    OpenBaoBadURL(anyhow::Error),
    #[error("No response from the OpenBao server: {0}")]
    OpenBaoNoServerResponse(anyhow::Error),
    #[error("The OpenBao server returned an unexpected response: {0}")]
    OpenBaoBadServerResponse(anyhow::Error),

    // PKI-related errors
    #[error("Invalid X509 trustchain (server doesn't recognize the root certificate)")]
    PKIServerInvalidX509Trustchain,
    #[error("Cannot open PKI X509 certificate store: {0}")]
    PKICannotOpenCertificateStore(anyhow::Error),
    // Certificate not found, invalid certificate, cannot use to sign etc.
    #[error("Cannot use the referenced X509 certificate for PKI operation: {0}")]
    PKIUnusableX509CertificateReference(anyhow::Error),
}

pub trait SubmitAsyncEnrollmentIdentityStrategy: Send + Sync {
    fn sign_submit_payload(&self, payload: Bytes) -> PinBoxFutureResult<
        libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_submit::SubmitPayloadSignature,
        SubmitAsyncEnrollmentError,
    >;
    fn human_handle(&self) -> &HumanHandle;
    fn generate_ciphertext_key(
        &self,
    ) -> PinBoxFutureResult<
        (SecretKey, AsyncEnrollmentLocalPendingIdentitySystem),
        SubmitAsyncEnrollmentError,
    >;
}

pub async fn submit_async_enrollment(
    config: Arc<ClientConfig>,
    addr: ParsecAsyncEnrollmentAddr,
    force: bool,
    requested_device_label: DeviceLabel,
    identity_strategy: &dyn SubmitAsyncEnrollmentIdentityStrategy,
) -> Result<AvailablePendingAsyncEnrollment, SubmitAsyncEnrollmentError> {
    let cmds = Arc::new(
        AnonymousCmds::new(&config.config_dir, addr.into(), config.proxy.clone())
            .map_err(|e| anyhow::anyhow!("Error while configuring connection to server: {e}"))?,
    );

    let enrollment_id = AsyncEnrollmentID::default();
    let requested_human_handle = identity_strategy.human_handle().to_owned();
    let to_become_user_private_key = PrivateKey::generate();
    let to_become_device_signing_key = SigningKey::generate();

    // 1) Send the submit request to the server

    let submit_payload: Bytes = AsyncEnrollmentSubmitPayload {
        verify_key: to_become_device_signing_key.verify_key().to_owned(),
        public_key: to_become_user_private_key.public_key().to_owned(),
        requested_device_label: requested_device_label.clone(),
        requested_human_handle: requested_human_handle.clone(),
    }
    .dump()
    .into();
    let submit_payload_signature = identity_strategy
        .sign_submit_payload(submit_payload.clone())
        .await?;

    let submitted_on = {
        use libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_submit::{Req, Rep};

        let req = Req {
            enrollment_id,
            force,
            submit_payload,
            submit_payload_signature,
        };

        let rep = cmds.send(req).await?;
        match rep {
            Rep::Ok { submitted_on } => Ok(submitted_on),
            Rep::EmailAlreadySubmitted { submitted_on } => {
                Err(SubmitAsyncEnrollmentError::EmailAlreadySubmitted { submitted_on })
            }
            Rep::EmailAlreadyEnrolled => Err(SubmitAsyncEnrollmentError::EmailAlreadyEnrolled),
            Rep::InvalidX509Trustchain => {
                Err(SubmitAsyncEnrollmentError::PKIServerInvalidX509Trustchain)
            }
            bad_rep @ (Rep::IdAlreadyUsed
            | Rep::InvalidSubmitPayload
            | Rep::InvalidDerX509Certificate
            | Rep::InvalidSubmitPayloadSignature
            | Rep::UnknownStatus { .. }) => {
                Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        }?
    };

    // 2) Generate & store the ciphertext key

    let (ciphertext_key, identity_system) = identity_strategy.generate_ciphertext_key().await?;

    // 3) Store the pending enrollment file

    let file_path = libparsec_platform_device_loader::get_default_pending_async_enrollment_file(
        &config.config_dir,
        enrollment_id,
    );

    let available = libparsec_platform_device_loader::save_pending_async_enrollment(
        &config.config_dir,
        AsyncEnrollmentLocalPendingCleartextContent {
            server_url: cmds.addr().to_owned().into(),
            organization_id: cmds.addr().organization_id().to_owned(),
            submitted_on,
            enrollment_id,
            requested_device_label,
            requested_human_handle,
            identity_system,
        },
        &to_become_device_signing_key,
        &to_become_user_private_key,
        &ciphertext_key,
        file_path,
    )
    .await
    .map_err(|err| match err {
        SaveAsyncEnrollmentLocalPendingError::NoSpaceLeft => {
            SubmitAsyncEnrollmentError::StorageNotAvailable
        }
        SaveAsyncEnrollmentLocalPendingError::InvalidPath => {
            SubmitAsyncEnrollmentError::InvalidPath(anyhow::anyhow!(
                "invalid path while saving async enrollment"
            ))
        }
        SaveAsyncEnrollmentLocalPendingError::Internal(err) => {
            SubmitAsyncEnrollmentError::Internal(err)
        }
    })?;

    Ok(available)
}
