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
        human_handle: HumanHandle,
        enrollment_id: PKIEnrollmentID,
        submitted_on: DateTime,
        submitter_der_cert: Bytes,
        payload: PkiEnrollmentSubmitPayload,
    },
    Invalid {
        /// We try to provide the human handle from the cert, but we may fail to do so causing the
        /// item to be invalid
        human_handle: Option<HumanHandle>,
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
    InvalidUserInformation,
}

impl From<LoadSubmitPayloadError> for InvalidityReason {
    fn from(value: LoadSubmitPayloadError) -> Self {
        match value {
            LoadSubmitPayloadError::InvalidCertificateDer(..) => {
                InvalidityReason::InvalidCertificateDer
            }
            LoadSubmitPayloadError::DateTimeOutOfRange(..) => InvalidityReason::DateTimeOutOfRange,
            LoadSubmitPayloadError::Untrusted(..) => InvalidityReason::Untrusted,
            LoadSubmitPayloadError::InvalidSignature => InvalidityReason::InvalidSignature,
            LoadSubmitPayloadError::UnexpectedError(..) => InvalidityReason::UnexpectedError,
            LoadSubmitPayloadError::DataError(..) => InvalidityReason::DataError,
        }
    }
}

pub async fn verify_untrusted_items(
    cmds: &AuthenticatedCmds,
    cert_ref: X509CertificateReference,
    untrusted_items: Vec<
        libparsec_protocol::authenticated_cmds::latest::pki_enrollment_list::PkiEnrollmentListItem,
    >,
) -> Result<Vec<PkiEnrollmentListItem>, PkiEnrollmentListError> {
    // Early stop
    if untrusted_items.is_empty() {
        return Ok(Vec::new());
    }

    let now = cmds.time_provider.now();
    // Potentially expensive check, spawning a new task
    libparsec_platform_async::spawn(async move {
        // Obtain the root cert used by the PKI
        let path = libparsec_platform_pki::get_validation_path_for_cert(&cert_ref, now)
            .context("Failed to validate own certificate")
            .map_err(PkiEnrollmentListError::Internal)?;

        let pki_root_certs = [path.root];

        let items = untrusted_items
            .into_iter()
            .map(|req| {
                let message = libparsec_platform_pki::SignedMessage {
                    algo: req.payload_signature_algorithm,
                    signature: req.payload_signature,
                    message: req.payload,
                };

                let cert_info =
                    match libparsec_platform_pki::x509::X509CertificateInformation::load_der(
                        &req.der_x509_certificate,
                    ) {
                        Ok(info) => info,
                        Err(err) => {
                            return PkiEnrollmentListItem::Invalid {
                                human_handle: None,
                                enrollment_id: req.enrollment_id,
                                submitted_on: req.submitted_on,
                                reason: InvalidityReason::InvalidUserInformation,
                                details: err.to_string(),
                            }
                        }
                    };

                let human_handle = cert_info.human_handle();

                let payload = match libparsec_platform_pki::load_submit_payload(
                    &message,
                    &req.der_x509_certificate,
                    req.intermediate_der_x509_certificates
                        .iter()
                        .map(Bytes::as_ref),
                    &pki_root_certs,
                    now,
                ) {
                    Ok(payload) => payload,
                    Err(err) => {
                        log::debug!("Cannot validate submit payload: {err}");
                        return PkiEnrollmentListItem::Invalid {
                            human_handle: human_handle.ok(),
                            enrollment_id: req.enrollment_id,
                            submitted_on: req.submitted_on,
                            details: err.to_string(),
                            reason: err.into(),
                        };
                    }
                };

                // We successfully parsed the payload, we can drop the message.
                drop(message);
                match human_handle {
                    Ok(human_handle) => PkiEnrollmentListItem::Valid {
                        human_handle,
                        enrollment_id: req.enrollment_id,
                        submitted_on: req.submitted_on,
                        submitter_der_cert: req.der_x509_certificate,
                        payload,
                    },
                    Err(e) => PkiEnrollmentListItem::Invalid {
                        human_handle: None,
                        enrollment_id: req.enrollment_id,
                        submitted_on: req.submitted_on,
                        reason: InvalidityReason::InvalidUserInformation,
                        details: e.to_string(),
                    },
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

pub async fn list_enrollments_untrusted(
    cmds: &AuthenticatedCmds,
) -> Result<
    Vec<libparsec_protocol::authenticated_cmds::latest::pki_enrollment_list::PkiEnrollmentListItem>,
    PkiEnrollmentListError,
> {
    use authenticated_cmds::latest::pki_enrollment_list::{Rep, Req};

    let rep = cmds.send(Req).await?;

    let pki_requests = match rep {
        Rep::Ok { enrollments } => enrollments,
        Rep::AuthorNotAllowed => return Err(PkiEnrollmentListError::AuthorNotAllowed),
        rep @ Rep::UnknownStatus { .. } => {
            return Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into())
        }
    };

    Ok(pki_requests)
}
