// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AnonymousCmds, ConnectionError};
use libparsec_protocol::anonymous_cmds::latest::pki_enrollment_submit::{Rep, Req};
use libparsec_types::prelude::*;
use std::sync::Arc;

use crate::ClientConfig;

#[derive(Debug, thiserror::Error)]
pub enum PkiEnrollmentSubmitError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("User already enrolled with this certificate")]
    AlreadyEnrolled,
    #[error("Certificate already submitted on {0}")]
    AlreadySubmitted(DateTime),
    #[error("User already enrolled with this email")]
    EmailAlreadyUsed,
    #[error("EnrollmentId already used")]
    IdAlreadyUsed,
    #[error("Unable to read payload")]
    InvalidPayload,
    #[error("PKI operation error: {0}")]
    PkiOperationError(anyhow::Error),
}

pub async fn pki_enrollment_submit(
    config: Arc<ClientConfig>,
    addr: ParsecPkiEnrollmentAddr,
    x509_cert_ref: X509CertificateReference,
    device_label: DeviceLabel,
    force: bool,
) -> Result<DateTime, PkiEnrollmentSubmitError> {
    let cmds = AnonymousCmds::new(
        &config.config_dir,
        addr.clone().into(),
        config.proxy.clone(),
    )?;

    let enrollment_id = PKIEnrollmentID::default();
    let signing_key = SigningKey::generate();
    let private_key = PrivateKey::generate();

    let payload = PkiEnrollmentSubmitPayload {
        verify_key: signing_key.verify_key(),
        public_key: private_key.public_key(),
        device_label,
    };
    let raw_payload = payload.dump();
    let (payload_signature_algorithm, payload_signature) =
        libparsec_platform_pki::sign_message(&raw_payload, &x509_cert_ref)
            .await
            .map_err(anyhow::Error::from)
            .context("Failed to sign payload with PKI")
            .map_err(PkiEnrollmentSubmitError::PkiOperationError)?;

    let intermediate_certificates =
        libparsec_platform_pki::get_validation_path_for_cert(&x509_cert_ref, DateTime::now())
            .await
            .map_err(anyhow::Error::from)
            .context("Failed to get intermediate certificates for itself")
            .map_err(PkiEnrollmentSubmitError::PkiOperationError)?;

    let submitted_on = match cmds
        .send(Req {
            der_x509_certificate: intermediate_certificates.leaf,
            intermediate_der_x509_certificates: intermediate_certificates.intermediates,
            enrollment_id,
            force,
            payload: raw_payload.into(),
            payload_signature,
            payload_signature_algorithm,
        })
        .await?
    {
        Rep::Ok { submitted_on } => Ok(submitted_on),
        Rep::AlreadyEnrolled => Err(PkiEnrollmentSubmitError::AlreadyEnrolled),
        Rep::AlreadySubmitted { submitted_on } => {
            Err(PkiEnrollmentSubmitError::AlreadySubmitted(submitted_on))
        }
        Rep::EmailAlreadyUsed => Err(PkiEnrollmentSubmitError::EmailAlreadyUsed),
        Rep::IdAlreadyUsed => Err(PkiEnrollmentSubmitError::IdAlreadyUsed),
        Rep::InvalidPayload => Err(PkiEnrollmentSubmitError::InvalidPayload),
        Rep::InvalidPayloadSignature
        | Rep::InvalidDerX509Certificate
        | Rep::InvalidX509Trustchain => todo!(),
        bad_rep @ Rep::UnknownStatus { .. } => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }?;

    let private_parts = PrivateParts {
        private_key,
        signing_key,
    };
    let local_pending = libparsec_platform_pki::create_local_pending(
        &x509_cert_ref,
        addr,
        enrollment_id,
        submitted_on,
        payload,
        private_parts,
    )
    .map_err(anyhow::Error::from)
    .context("Failed to encrypt local parts with PKI")
    .map_err(PkiEnrollmentSubmitError::PkiOperationError)?;

    let local_file = libparsec_platform_device_loader::get_default_local_pending_file(
        &config.config_dir,
        enrollment_id,
    );
    libparsec_platform_device_loader::save_pki_local_pending(local_pending, local_file)
        .await
        .map_err(Into::into)
        .map_err(PkiEnrollmentSubmitError::Internal)?;

    Ok(submitted_on)
}
