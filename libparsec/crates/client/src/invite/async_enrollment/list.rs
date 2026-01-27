// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum ListAsyncEnrollmentsError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Author not allowed")]
    AuthorNotAllowed,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, PartialEq, Eq)]
pub enum AsyncEnrollmentIdentitySystem {
    /// Knowing the identity system is based on PKI is not enough: we also
    /// need to know the X509 root certificate, since a signature from a different
    /// PKI will be considered invalid.
    PKI {
        /// Human-readable identifier for the X509 root certificate
        /// E.g. `GlobalSign Root CA`
        x509_root_certificate_common_name: String,
        /// Opaque unique identifier for the X509 root certificate
        /// It corresponds to the certificate's `Subject` field, which contains
        /// in DER-encoded format something of the style:
        /// `C=BE, O=GlobalSign nv-sa, OU=Root CA, CN=GlobalSign Root CA`
        x509_root_certificate_subject: Vec<u8>,
    },
    /// We cannot extract root certificate info if the trustchain contains invalid certificates.
    /// The enrollment is not ignored though, since it allow the administrator to get
    /// notified about potential issues in the PKI setup (and he can reject the enrollment).
    PKICorrupted { reason: String },
    /// Unlike PKI, OpenBao acts as a single hub for all available SSO authentications.
    OpenBao,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AsyncEnrollmentUntrusted {
    pub enrollment_id: AsyncEnrollmentID,
    pub submitted_on: DateTime,
    pub untrusted_requested_device_label: DeviceLabel,
    /// ⚠️ The submitter might be lying about his email here !
    /// Identity validation requires user interaction (to authenticate to OpenBao,
    /// or to ask the user to plug his smartcard), so we skip this step here.
    /// It is considered okay since the validation is going to be to first step
    /// no matter what when the user chooses to accept an enrollment request.
    pub untrusted_requested_human_handle: HumanHandle,
    pub identity_system: AsyncEnrollmentIdentitySystem,
}

pub(crate) async fn list_async_enrollments(
    cmds: &AuthenticatedCmds,
) -> Result<Vec<AsyncEnrollmentUntrusted>, ListAsyncEnrollmentsError> {
    let enrollments = {
        use libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_list::{Req, Rep};

        let rep = cmds.send(Req).await?;
        match rep {
            Rep::Ok { enrollments } => enrollments,
            Rep::AuthorNotAllowed => return Err(ListAsyncEnrollmentsError::AuthorNotAllowed),
            bad_rep @ Rep::UnknownStatus { .. } => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        }
    };

    use libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_list::SubmitPayloadSignature;
    Ok(enrollments
        .into_iter()
        .filter_map(|enrollment| {
            // Just skip enrollments with invalid payload: in practice this should never occur
            // since the server has already validated the payload format.
            let payload = AsyncEnrollmentSubmitPayload::load(&enrollment.submit_payload).ok()?;
            Some(AsyncEnrollmentUntrusted {
                enrollment_id: enrollment.enrollment_id,
                submitted_on: enrollment.submitted_on,
                untrusted_requested_device_label: payload.requested_device_label,
                untrusted_requested_human_handle: payload.requested_human_handle,
                identity_system: match enrollment.submit_payload_signature {
                    SubmitPayloadSignature::PKI {
                        submitter_der_x509_certificate,
                        intermediate_der_x509_certificates,
                        ..
                    } => {
                        match libparsec_platform_pki::get_root_certificate_info_from_trustchain(
                            &submitter_der_x509_certificate,
                            intermediate_der_x509_certificates
                                .iter()
                                .map(|cert| cert.as_ref()),
                        ) {
                            Ok(root_info) => AsyncEnrollmentIdentitySystem::PKI {
                                x509_root_certificate_common_name: root_info.common_name,
                                x509_root_certificate_subject: root_info.subject,
                            },
                            Err(err) => AsyncEnrollmentIdentitySystem::PKICorrupted {
                                reason: format!(
                                    "Cannot extract X509 root certificate info: {}",
                                    err
                                ),
                            },
                        }
                    }
                    SubmitPayloadSignature::OpenBao { .. } => {
                        AsyncEnrollmentIdentitySystem::OpenBao
                    }
                },
            })
        })
        .collect())
}
