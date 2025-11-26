// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_pki::LoadSubmitPayloadError;
use libparsec_types::anyhow::Context;
use libparsec_types::prelude::*;

use libparsec_client_connection::{
    protocol::authenticated_cmds, AuthenticatedCmds, ConnectionError,
};

/*
 * list_enrollments
 */

#[derive(Debug, thiserror::Error)]
pub enum PkiEnrollmentListError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Not allowed to list enrollments")]
    AuthorNotAllowed,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PkiEnrollmentListItem {
    Valid {
        enrollment_id: PKIEnrollmentID,
        submitted_on: DateTime,
        payload: PkiEnrollmentSubmitPayload,
    },
    Invalid {
        enrollment_id: PKIEnrollmentID,
        submitted_on: DateTime,
        reason: InvalidityReason,
        details: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum InvalidityReason {
    InvalidCertificateDer,
    InvalidRootCertificate,
    CannotOpenStore,
    NotFound,
    CannotGetCertificateInfo,
    DateTimeOutOfRange,
    Untrusted,
    InvalidSignature,
    UnexpectedError,
    DataError,
}

impl From<LoadSubmitPayloadError> for InvalidityReason {
    fn from(value: LoadSubmitPayloadError) -> Self {
        match value {
            LoadSubmitPayloadError::InvalidCertificateDer(..) => {
                InvalidityReason::InvalidCertificateDer
            }
            LoadSubmitPayloadError::InvalidRootCertificate(..) => {
                InvalidityReason::InvalidRootCertificate
            }
            LoadSubmitPayloadError::CannotOpenStore(..) => InvalidityReason::CannotOpenStore,
            LoadSubmitPayloadError::NotFound => InvalidityReason::NotFound,
            LoadSubmitPayloadError::CannotGetCertificateInfo(..) => {
                InvalidityReason::CannotGetCertificateInfo
            }
            LoadSubmitPayloadError::DateTimeOutOfRange(..) => InvalidityReason::DateTimeOutOfRange,
            LoadSubmitPayloadError::Untrusted(..) => InvalidityReason::Untrusted,
            LoadSubmitPayloadError::InvalidSignature => InvalidityReason::InvalidSignature,
            LoadSubmitPayloadError::UnexpectedError(..) => InvalidityReason::UnexpectedError,
            LoadSubmitPayloadError::DataError(..) => InvalidityReason::DataError,
        }
    }
}

pub async fn list_enrollments(
    cmds: &AuthenticatedCmds,
    cert_ref: X509CertificateReference,
) -> Result<Vec<PkiEnrollmentListItem>, PkiEnrollmentListError> {
    use authenticated_cmds::latest::pki_enrollment_list::{Rep, Req};

    let rep = cmds.send(Req).await?;

    let pki_requests = match rep {
        Rep::Ok { enrollments } => enrollments,
        Rep::AuthorNotAllowed => return Err(PkiEnrollmentListError::AuthorNotAllowed),
        rep @ Rep::UnknownStatus { .. } => {
            return Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    };

    // Early stop
    if pki_requests.is_empty() {
        return Ok(Vec::new());
    }

    let now = cmds.time_provider.now();
    // Potentially expensive check, spawning a new task
    libparsec_platform_async::spawn(async move {
        let root_certs = libparsec_platform_pki::list_trusted_root_certificate_anchor()
            .context("Failed to list trusted root certificates")
            .map_err(PkiEnrollmentListError::Internal)?;

        // Obtain the root cert used by the PKI
        let base_raw_cert = libparsec_platform_pki::get_der_encoded_certificate(&cert_ref)
            .map(|v| libparsec_platform_pki::Certificate::from_der_owned(v.der_content.into()))
            .context("Cannot get certificate to use to obtain root cert")
            .map_err(PkiEnrollmentListError::Internal)?;
        let base_cert = base_raw_cert
            .to_end_certificate()
            .map_err(libparsec_platform_pki::LoadAnswerPayloadError::InvalidCertificateDer)
            .context("Invalid certificate from PKI")
            .map_err(PkiEnrollmentListError::Internal)?;

        let verified_path = libparsec_platform_pki::verify_certificate(
            &base_cert,
            &root_certs,
            // TODO: list client intermediate certs
            // https://github.com/Scille/parsec-cloud/issues/11760
            &[],
            now,
            libparsec_platform_pki::KeyUsage::client_auth(),
        )
        .context("Device does not trust user PKI certificate")
        .map_err(PkiEnrollmentListError::Internal)?;

        let pki_root_certs = [verified_path.anchor().clone()];

        let items = pki_requests
            .into_iter()
            .map(|req| {
                let message = libparsec_platform_pki::SignedMessage {
                    algo: req.payload_signature_algorithm,
                    signature: req.payload_signature,
                    message: req.payload,
                };

                let payload = match libparsec_platform_pki::load_submit_payload(
                    &req.der_x509_certificate,
                    &message,
                    &pki_root_certs,
                    &req.intermediate_der_x509_certificates,
                    now,
                ) {
                    Ok(payload) => payload,
                    Err(err) => {
                        log::debug!(err:%; "Cannot validate submit payload");
                        return PkiEnrollmentListItem::Invalid {
                            enrollment_id: req.enrollment_id,
                            submitted_on: req.submitted_on,
                            details: err.to_string(),
                            reason: err.into(),
                        };
                    }
                };

                // We successfully parsed the payload, we can drop the message.
                drop(message);

                PkiEnrollmentListItem::Valid {
                    enrollment_id: req.enrollment_id,
                    submitted_on: req.submitted_on,
                    payload,
                }
            })
            .collect::<Vec<_>>();
        Ok(items)
    })
    .await
    .context("Failed to verify pending pki requests")
    .map_err(PkiEnrollmentListError::Internal)
    .and_then(std::convert::identity)
}
