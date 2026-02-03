// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::Path;
use std::sync::Arc;

pub use libparsec_client::{
    AsyncEnrollmentIdentitySystem, AsyncEnrollmentUntrusted, AvailableDevice,
    AvailablePendingAsyncEnrollment, AvailablePendingAsyncEnrollmentIdentitySystem,
    ClientAcceptAsyncEnrollmentError, ClientListAsyncEnrollmentsError,
    ClientRejectAsyncEnrollmentError, PendingAsyncEnrollmentInfo, SubmitAsyncEnrollmentError,
    SubmitterCancelAsyncEnrollmentError, SubmitterFinalizeAsyncEnrollmentError,
    SubmitterGetAsyncEnrollmentInfoError, SubmitterListLocalAsyncEnrollmentsError,
};
pub use libparsec_protocol::authenticated_cmds::latest::invite_list::InvitationCreatedBy as InviteListInvitationCreatedBy;
pub use libparsec_protocol::invited_cmds::latest::invite_info::{
    InvitationCreatedBy as InviteInfoInvitationCreatedBy, ShamirRecoveryRecipient,
    UserGreetingAdministrator, UserOnlineStatus,
};
use libparsec_types::prelude::*;

use crate::{
    handle::{borrow_from_handle, Handle, HandleItem},
    ClientConfig, DeviceSaveStrategy,
};

mod strategy {
    use std::sync::Arc;

    use libparsec_client::{
        AcceptAsyncEnrollmentError, SubmitAsyncEnrollmentError,
        SubmitterFinalizeAsyncEnrollmentError,
    };
    use libparsec_client_connection::protocol;
    use libparsec_crypto::SecretKey;
    use libparsec_openbao::{
        OpenBaoCmds, OpenBaoFetchOpaqueKeyError, OpenBaoSignError, OpenBaoUploadOpaqueKeyError,
        OpenBaoVerifyError,
    };
    use libparsec_platform_async::{pretend_future_is_send_on_web, PinBoxFutureResult};
    use libparsec_platform_pki::{
        DecryptMessageError as PKIDecryptMessageError,
        EncryptMessageError as PKIEncryptMessageError,
        GetValidationPathForCertError as PKIGetValidationPathForCertError,
        SignMessageError as PKISignMessageError,
    };
    use libparsec_types::prelude::*;

    /*
     * SubmitAsyncEnrollmentIdentityStrategy
     */

    #[derive(Debug, Clone)]
    // This enum is used in the bindings, so boxing its field to reduce its total
    // size is unpractical (and probably not needed since this field is not massively
    // used)
    #[allow(clippy::large_enum_variant)]
    pub enum SubmitAsyncEnrollmentIdentityStrategy {
        OpenBao {
            /// Should be obtained from querying the identity provider service
            requested_human_handle: HumanHandle,
            openbao_server_url: String,
            openbao_transit_mount_path: String,
            openbao_secret_mount_path: String,
            openbao_entity_id: String,
            openbao_auth_token: String,
            openbao_preferred_auth_id: String,
        },
        PKI {
            certificate_reference: X509CertificateReference,
        },
    }

    impl SubmitAsyncEnrollmentIdentityStrategy {
        pub(super) async fn convert(
            self,
        ) -> anyhow::Result<Box<dyn libparsec_client::SubmitAsyncEnrollmentIdentityStrategy>>
        {
            match self {
                SubmitAsyncEnrollmentIdentityStrategy::OpenBao {
                    requested_human_handle,
                    openbao_server_url,
                    openbao_transit_mount_path,
                    openbao_secret_mount_path,
                    openbao_entity_id,
                    openbao_auth_token,
                    openbao_preferred_auth_id,
                } => {
                    let client = libparsec_client_connection::build_client_with_proxy(
                        libparsec_client_connection::ProxyConfig::default(),
                    )?;

                    let cmds = Arc::new(OpenBaoCmds::new(
                        client,
                        openbao_server_url,
                        openbao_secret_mount_path,
                        openbao_transit_mount_path,
                        openbao_entity_id,
                        openbao_auth_token,
                    ));

                    Ok(Box::new(SubmitAsyncEnrollmentOpenBaoIdentityStrategy {
                        cmds,
                        requested_human_handle,
                        openbao_preferred_auth_id,
                    }))
                }
                SubmitAsyncEnrollmentIdentityStrategy::PKI {
                    certificate_reference,
                } => {
                    // Get certificate DER
                    let cert_der =
                        libparsec_platform_pki::get_der_encoded_certificate(&certificate_reference)
                            .await
                            .map_err(|e| anyhow::anyhow!("Cannot get certificate: {}", e))?;

                    // Parse certificate and extract human handle
                    let cert_info =
                        libparsec_platform_pki::x509::X509CertificateInformation::load_der(
                            &cert_der,
                        )
                        .map_err(|e| anyhow::anyhow!("Cannot load certificate info: {}", e))?;

                    let human_handle = cert_info.human_handle().map_err(|e| {
                        anyhow::anyhow!("Cannot extract human handle from certificate: {}", e)
                    })?;

                    Ok(Box::new(SubmitAsyncEnrollmentPKIIdentityStrategy {
                        certificate_reference,
                        human_handle,
                    }))
                }
            }
        }
    }

    /*
     * SubmitAsyncEnrollmentOpenBaoIdentityStrategy
     */

    #[derive(Debug)]
    struct SubmitAsyncEnrollmentOpenBaoIdentityStrategy {
        cmds: Arc<OpenBaoCmds>,
        requested_human_handle: HumanHandle,
        openbao_preferred_auth_id: String,
    }

    impl libparsec_client::SubmitAsyncEnrollmentIdentityStrategy
        for SubmitAsyncEnrollmentOpenBaoIdentityStrategy
    {
        fn sign_submit_payload(
            &self,
            payload: Bytes,
        ) -> PinBoxFutureResult<
            protocol::anonymous_cmds::latest::async_enrollment_submit::SubmitPayloadSignature,
            SubmitAsyncEnrollmentError,
        > {
            let cmds = self.cmds.clone();
            Box::pin(pretend_future_is_send_on_web(async move {
                let signature = cmds.sign(&payload).await.map_err(|err| match err {
                    OpenBaoSignError::BadURL(err) => SubmitAsyncEnrollmentError::OpenBaoBadURL(err),
                    OpenBaoSignError::NoServerResponse(err) => {
                        SubmitAsyncEnrollmentError::OpenBaoNoServerResponse(err.into())
                    }
                    OpenBaoSignError::BadServerResponse(err) => {
                        SubmitAsyncEnrollmentError::OpenBaoBadServerResponse(err)
                    }
                })?;
                Ok(protocol::anonymous_cmds::latest::async_enrollment_submit::SubmitPayloadSignature::OpenBao {
                    signature,
                    submitter_openbao_entity_id: cmds.openbao_entity_id().to_owned(),
                })
            }))
        }

        fn human_handle(&self) -> &HumanHandle {
            &self.requested_human_handle
        }

        fn generate_ciphertext_key(
            &self,
        ) -> PinBoxFutureResult<
            (SecretKey, AsyncEnrollmentLocalPendingIdentitySystem),
            SubmitAsyncEnrollmentError,
        > {
            let cmds = self.cmds.clone();
            let openbao_entity_id = self.cmds.openbao_entity_id().to_owned();
            let openbao_preferred_auth_id = self.openbao_preferred_auth_id.clone();
            Box::pin(pretend_future_is_send_on_web(async move {
                let (openbao_ciphertext_key_path, key) =
                    cmds.upload_opaque_key().await.map_err(|err| match err {
                        OpenBaoUploadOpaqueKeyError::BadURL(err) => {
                            SubmitAsyncEnrollmentError::OpenBaoBadURL(err)
                        }
                        OpenBaoUploadOpaqueKeyError::NoServerResponse(err) => {
                            SubmitAsyncEnrollmentError::OpenBaoNoServerResponse(err.into())
                        }
                        OpenBaoUploadOpaqueKeyError::BadServerResponse(err) => {
                            SubmitAsyncEnrollmentError::OpenBaoBadServerResponse(err)
                        }
                    })?;
                let identity_system = AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
                    openbao_ciphertext_key_path,
                    openbao_entity_id,
                    openbao_preferred_auth_id,
                };
                Ok((key, identity_system))
            }))
        }
    }

    /*
     * SubmitAsyncEnrollmentPKIIdentityStrategy
     */

    #[derive(Debug)]
    struct SubmitAsyncEnrollmentPKIIdentityStrategy {
        certificate_reference: X509CertificateReference,
        human_handle: HumanHandle,
    }

    impl libparsec_client::SubmitAsyncEnrollmentIdentityStrategy
        for SubmitAsyncEnrollmentPKIIdentityStrategy
    {
        fn sign_submit_payload(
            &self,
            payload: Bytes,
        ) -> PinBoxFutureResult<
            protocol::anonymous_cmds::latest::async_enrollment_submit::SubmitPayloadSignature,
            SubmitAsyncEnrollmentError,
        > {
            let cert_ref = self.certificate_reference.clone();
            Box::pin(pretend_future_is_send_on_web(async move {
                // 1. Sign the payload
                let (algorithm, signature) =
                    libparsec_platform_pki::sign_message(&payload, &cert_ref)
                        .await
                        .map_err(|err| match err {
                            err @ (PKISignMessageError::NotFound
                            | PKISignMessageError::CannotAcquireKeypair(_)
                            | PKISignMessageError::CannotSign(_)) => {
                                SubmitAsyncEnrollmentError::PKIUnusableX509CertificateReference(
                                    err.into(),
                                )
                            }
                            PKISignMessageError::CannotOpenStore(e) => {
                                SubmitAsyncEnrollmentError::PKICannotOpenCertificateStore(e.into())
                            }
                        })?;

                // 2. Get leaf & intermediate certificates
                let validation_path: libparsec_platform_pki::ValidationPathOwned =
                    libparsec_platform_pki::get_validation_path_for_cert(&cert_ref, DateTime::now())
                    .await
                        .map_err(|err| match err {
                            err @ (
                                PKIGetValidationPathForCertError::NotFound
                                | PKIGetValidationPathForCertError::InvalidCertificateDer(_)
                                | PKIGetValidationPathForCertError::InvalidCertificateDateTimeOutOfRange(_)
                                | PKIGetValidationPathForCertError::InvalidCertificateUntrusted(_)
                            ) => SubmitAsyncEnrollmentError::PKIUnusableX509CertificateReference(err.into()),
                            PKIGetValidationPathForCertError::CannotOpenStore(e) => {
                                SubmitAsyncEnrollmentError::PKICannotOpenCertificateStore(e.into())
                            }
                        })?;

                Ok(protocol::anonymous_cmds::latest::async_enrollment_submit::SubmitPayloadSignature::PKI {
                    signature,
                    algorithm,
                    submitter_der_x509_certificate: validation_path.leaf,
                    intermediate_der_x509_certificates: validation_path.intermediates,
                })
            }))
        }

        fn human_handle(&self) -> &HumanHandle {
            &self.human_handle
        }

        fn generate_ciphertext_key(
            &self,
        ) -> PinBoxFutureResult<
            (SecretKey, AsyncEnrollmentLocalPendingIdentitySystem),
            SubmitAsyncEnrollmentError,
        > {
            let cert_ref = self.certificate_reference.clone();
            Box::pin(pretend_future_is_send_on_web(async move {
                // 1. Generate a random secret key
                let key = SecretKey::generate();

                // 2. Encrypt it with the PKI certificate's public key
                let (algorithm, encrypted_key) =
                    libparsec_platform_pki::encrypt_message(key.as_ref(), &cert_ref)
                        .await
                        .map_err(|err| match err {
                            err @ (PKIEncryptMessageError::NotFound
                            | PKIEncryptMessageError::CannotAcquireKeypair(_)
                            | PKIEncryptMessageError::CannotEncrypt(_)) => {
                                SubmitAsyncEnrollmentError::PKIUnusableX509CertificateReference(
                                    err.into(),
                                )
                            }
                            PKIEncryptMessageError::CannotOpenStore(e) => {
                                SubmitAsyncEnrollmentError::PKICannotOpenCertificateStore(e.into())
                            }
                        })?;

                // 3. Create the identity system metadata
                let identity_system = AsyncEnrollmentLocalPendingIdentitySystem::PKI {
                    encrypted_key,
                    certificate_ref: cert_ref,
                    algorithm,
                };

                Ok((key, identity_system))
            }))
        }
    }

    /*
     * AcceptFinalizeAsyncEnrollmentIdentityStrategy
     */

    #[derive(Debug, Clone)]
    pub enum AcceptFinalizeAsyncEnrollmentIdentityStrategy {
        OpenBao {
            openbao_server_url: String,
            openbao_transit_mount_path: String,
            openbao_secret_mount_path: String,
            openbao_entity_id: String,
            openbao_auth_token: String,
        },
        PKI {
            certificate_reference: X509CertificateReference,
        },
    }

    pub(super) trait AcceptFinalizeAsyncEnrollmentIdentityStrategyTrait:
        libparsec_client::AcceptAsyncEnrollmentIdentityStrategy
        + libparsec_client::SubmitterFinalizeAsyncEnrollmentIdentityStrategy
    {
    }

    impl AcceptFinalizeAsyncEnrollmentIdentityStrategy {
        pub(super) fn convert(
            self,
        ) -> anyhow::Result<Box<dyn AcceptFinalizeAsyncEnrollmentIdentityStrategyTrait>> {
            match self {
                Self::OpenBao {
                    openbao_server_url,
                    openbao_transit_mount_path,
                    openbao_secret_mount_path,
                    openbao_entity_id,
                    openbao_auth_token,
                } => {
                    let client = libparsec_client_connection::build_client_with_proxy(
                        libparsec_client_connection::ProxyConfig::default(),
                    )?;

                    let cmds = Arc::new(OpenBaoCmds::new(
                        client,
                        openbao_server_url,
                        openbao_secret_mount_path,
                        openbao_transit_mount_path,
                        openbao_entity_id,
                        openbao_auth_token,
                    ));

                    Ok(Box::new(
                        AcceptFinalizeAsyncEnrollmentOpenBaoIdentityStrategy { cmds },
                    ))
                }

                Self::PKI {
                    certificate_reference,
                } => Ok(Box::new(AcceptFinalizeAsyncEnrollmentPKIIdentityStrategy {
                    certificate_reference,
                })),
            }
        }

        pub fn convert_for_finalize(self) {}
    }

    /*
     * AcceptFinalizeAsyncEnrollmentOpenBaoIdentityStrategy
     */

    #[derive(Debug)]
    struct AcceptFinalizeAsyncEnrollmentOpenBaoIdentityStrategy {
        cmds: Arc<OpenBaoCmds>,
    }

    impl AcceptFinalizeAsyncEnrollmentIdentityStrategyTrait
        for AcceptFinalizeAsyncEnrollmentOpenBaoIdentityStrategy
    {
    }

    impl libparsec_client::AcceptAsyncEnrollmentIdentityStrategy
        for AcceptFinalizeAsyncEnrollmentOpenBaoIdentityStrategy
    {
        fn _verify_submit_payload(
            &self,
            payload: Bytes,
            payload_signature: protocol::authenticated_cmds::latest::async_enrollment_list::SubmitPayloadSignature,
            expected_author: EmailAddress,
        ) -> PinBoxFutureResult<(), AcceptAsyncEnrollmentError> {
            let cmds = self.cmds.clone();
            Box::pin(pretend_future_is_send_on_web(async move {
                let (signature, submitter_openbao_entity_id) = match payload_signature {
                    protocol::authenticated_cmds::v5::async_enrollment_list::SubmitPayloadSignature::OpenBao { signature, submitter_openbao_entity_id } => (signature, submitter_openbao_entity_id),
                    protocol::authenticated_cmds::v5::async_enrollment_list::SubmitPayloadSignature::PKI { .. } => {
                        return Err(AcceptAsyncEnrollmentError::IdentityStrategyMismatch {
                            submitter: "PKI".to_string(),
                            ours: "OpenBao".to_string(),
                        });
                    }
                };
                cmds.verify(
                    &submitter_openbao_entity_id,
                    &signature,
                    &payload,
                    Some(&expected_author),
                )
                .await
                .map_err(|err| match err {
                    err @ (OpenBaoVerifyError::BadSignature
                    | OpenBaoVerifyError::UnexpectedAuthor { .. }
                    | OpenBaoVerifyError::AuthorOpenBaoEntityHasNoEmail { .. }) => {
                        AcceptAsyncEnrollmentError::BadSubmitPayload(err.into())
                    }
                    OpenBaoVerifyError::BadURL(err) => {
                        AcceptAsyncEnrollmentError::OpenBaoBadURL(err)
                    }
                    OpenBaoVerifyError::NoServerResponse(err) => {
                        AcceptAsyncEnrollmentError::OpenBaoNoServerResponse(err.into())
                    }
                    OpenBaoVerifyError::BadServerResponse(err) => {
                        AcceptAsyncEnrollmentError::OpenBaoBadServerResponse(err)
                    }
                })
            }))
        }

        fn sign_accept_payload(
            &self,
            payload: Bytes,
        ) -> PinBoxFutureResult<
            protocol::authenticated_cmds::latest::async_enrollment_accept::AcceptPayloadSignature,
            AcceptAsyncEnrollmentError,
        > {
            let cmds = self.cmds.clone();
            Box::pin(pretend_future_is_send_on_web(async move {
                let signature = cmds.sign(&payload).await.map_err(|err| match err {
                    OpenBaoSignError::BadURL(err) => AcceptAsyncEnrollmentError::OpenBaoBadURL(err),
                    OpenBaoSignError::NoServerResponse(err) => {
                        AcceptAsyncEnrollmentError::OpenBaoNoServerResponse(err.into())
                    }
                    OpenBaoSignError::BadServerResponse(err) => {
                        AcceptAsyncEnrollmentError::OpenBaoBadServerResponse(err)
                    }
                })?;

                Ok(protocol::authenticated_cmds::latest::async_enrollment_accept::AcceptPayloadSignature::OpenBao {
                    signature,
                    accepter_openbao_entity_id: cmds.openbao_entity_id().to_owned(),
                })
            }))
        }
    }

    impl libparsec_client::SubmitterFinalizeAsyncEnrollmentIdentityStrategy
        for AcceptFinalizeAsyncEnrollmentOpenBaoIdentityStrategy
    {
        fn _verify_accept_payload(
            &self,
            payload: Bytes,
            payload_signature: protocol::anonymous_cmds::latest::async_enrollment_info::AcceptPayloadSignature,
        ) -> PinBoxFutureResult<(), SubmitterFinalizeAsyncEnrollmentError> {
            let cmds = self.cmds.clone();
            Box::pin(pretend_future_is_send_on_web(async move {
                let (signature, accepter_openbao_entity_id) = match payload_signature {
                    protocol::anonymous_cmds::latest::async_enrollment_info::AcceptPayloadSignature::OpenBao { signature, accepter_openbao_entity_id } => (signature, accepter_openbao_entity_id),
                    protocol::anonymous_cmds::latest::async_enrollment_info::AcceptPayloadSignature::PKI { .. } => {
                        return Err(SubmitterFinalizeAsyncEnrollmentError::IdentityStrategyMismatch {
                            accepter: "PKI".to_string(),
                            ours: "OpenBao".to_string(),
                        });
                    }
                };

                cmds.verify(
                    &accepter_openbao_entity_id,
                    &signature,
                    &payload,
                    None, // Don't check author's email
                )
                .await
                .map_err(|err| match err {
                    err @ (OpenBaoVerifyError::BadSignature
                    | OpenBaoVerifyError::UnexpectedAuthor { .. }
                    | OpenBaoVerifyError::AuthorOpenBaoEntityHasNoEmail { .. }) => {
                        SubmitterFinalizeAsyncEnrollmentError::BadAcceptPayload(err.into())
                    }
                    OpenBaoVerifyError::BadURL(err) => {
                        SubmitterFinalizeAsyncEnrollmentError::OpenBaoBadURL(err)
                    }
                    OpenBaoVerifyError::NoServerResponse(err) => {
                        SubmitterFinalizeAsyncEnrollmentError::OpenBaoNoServerResponse(err.into())
                    }
                    OpenBaoVerifyError::BadServerResponse(err) => {
                        SubmitterFinalizeAsyncEnrollmentError::OpenBaoBadServerResponse(err)
                    }
                })

                // Note, unlike when we verify the submit payload, we don't care about the
                // email in the accepter's X509 certificate.
                // This is because we have nothing the check it against (i.e. we accept
                // that anyone that is part of the PKI can enroll us).
            }))
        }

        fn retrieve_ciphertext_key(
            &self,
            identity_system: AsyncEnrollmentLocalPendingIdentitySystem,
        ) -> PinBoxFutureResult<SecretKey, SubmitterFinalizeAsyncEnrollmentError> {
            let cmds = self.cmds.clone();
            Box::pin(pretend_future_is_send_on_web(async move {
                let openbao_ciphertext_key_path = match identity_system {
                    AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
                        openbao_ciphertext_key_path,
                        ..
                    } => openbao_ciphertext_key_path,

                    AsyncEnrollmentLocalPendingIdentitySystem::PKI { .. } => {
                        return Err(SubmitterFinalizeAsyncEnrollmentError::Internal(
                            anyhow::anyhow!("Identity system mismatch: expected OpenBao, got PKI"),
                        ))
                    }
                };
                cmds.fetch_opaque_key(&openbao_ciphertext_key_path)
                    .await
                    .map_err(|err| match err {
                        OpenBaoFetchOpaqueKeyError::BadURL(err) => {
                            SubmitterFinalizeAsyncEnrollmentError::OpenBaoBadURL(err)
                        }
                        OpenBaoFetchOpaqueKeyError::NoServerResponse(err) => {
                            SubmitterFinalizeAsyncEnrollmentError::OpenBaoNoServerResponse(
                                err.into(),
                            )
                        }
                        OpenBaoFetchOpaqueKeyError::BadServerResponse(err) => {
                            SubmitterFinalizeAsyncEnrollmentError::OpenBaoBadServerResponse(err)
                        }
                    })
            }))
        }
    }

    /*
     * AcceptFinalizeAsyncEnrollmentPKIIdentityStrategy
     */

    #[derive(Debug)]
    struct AcceptFinalizeAsyncEnrollmentPKIIdentityStrategy {
        certificate_reference: X509CertificateReference,
    }

    impl AcceptFinalizeAsyncEnrollmentIdentityStrategyTrait
        for AcceptFinalizeAsyncEnrollmentPKIIdentityStrategy
    {
    }

    impl libparsec_client::AcceptAsyncEnrollmentIdentityStrategy
        for AcceptFinalizeAsyncEnrollmentPKIIdentityStrategy
    {
        fn _verify_submit_payload(
            &self,
            payload: Bytes,
            payload_signature: protocol::authenticated_cmds::latest::async_enrollment_list::SubmitPayloadSignature,
            expected_author: EmailAddress,
        ) -> PinBoxFutureResult<(), AcceptAsyncEnrollmentError> {
            let cert_ref = self.certificate_reference.clone();
            Box::pin(pretend_future_is_send_on_web(async move {
                let (signature, algorithm, submitter_cert, intermediate_certs) = match payload_signature {
                    protocol::authenticated_cmds::v5::async_enrollment_list::SubmitPayloadSignature::PKI {
                        signature,
                        algorithm,
                        submitter_der_x509_certificate,
                        intermediate_der_x509_certificates,
                    } => (signature, algorithm, submitter_der_x509_certificate, intermediate_der_x509_certificates),
                    protocol::authenticated_cmds::v5::async_enrollment_list::SubmitPayloadSignature::OpenBao { .. } => {
                        return Err(AcceptAsyncEnrollmentError::IdentityStrategyMismatch {
                            submitter: "OpenBao".to_string(),
                            ours: "PKI".to_string(),
                        });
                    }
                };

                // 1. Retrieve the root certificate used by our own X059 certificate,
                // as the submitter X509 certificate must also depend on it.

                let validation_path: libparsec_platform_pki::ValidationPathOwned =
                    libparsec_platform_pki::get_validation_path_for_cert(&cert_ref, DateTime::now())
                        .await
                        .map_err(|err| match err {
                            err @ (
                                PKIGetValidationPathForCertError::NotFound
                                | PKIGetValidationPathForCertError::InvalidCertificateDer(_)
                                | PKIGetValidationPathForCertError::InvalidCertificateDateTimeOutOfRange(_)
                                | PKIGetValidationPathForCertError::InvalidCertificateUntrusted(_)
                            ) => AcceptAsyncEnrollmentError::PKIUnusableX509CertificateReference(err.into()),
                            PKIGetValidationPathForCertError::CannotOpenStore(e) => {
                                AcceptAsyncEnrollmentError::PKICannotOpenCertificateStore(e.into())
                            }

                        })?;

                // 2. Validate certificate trustchain and payload signature

                libparsec_platform_pki::verify_message2(
                    &payload,
                    &signature,
                    algorithm,
                    &submitter_cert,
                    intermediate_certs.iter().map(|x| x.as_ref()),
                    &[validation_path.root],
                    DateTime::now(),
                )
                .map_err(|err| AcceptAsyncEnrollmentError::BadSubmitPayload(err.into()))?;

                // 2. Check that the email in the payload matches the submitter X509 certificate

                let cert_info = libparsec_platform_pki::x509::X509CertificateInformation::load_der(
                    &submitter_cert,
                )
                .map_err(|e| {
                    AcceptAsyncEnrollmentError::BadSubmitPayload(anyhow::anyhow!(
                        "Cannot parse submitter X509 certificate: {}",
                        e
                    ))
                })?;

                // An X509 certificate may contain multiple emails (i.e. field
                // `subjectAltName` can be repeated, or a deprecated `emailAddress`
                // field can also be present).
                // However the most common case is to have a single email.
                // So here we test against all the possible email, but display an
                // error mentioning only the first email.

                let cert_first_email = cert_info.emails().next().ok_or_else(|| {
                    AcceptAsyncEnrollmentError::BadSubmitPayload(anyhow::anyhow!(
                        "Submitter X509 Certificate does not contain an email address"
                    ))
                })?;

                let valid_email = cert_info.emails().any({
                    let expected_author = expected_author.to_string();
                    move |email| email == expected_author
                });
                if !valid_email {
                    return Err(AcceptAsyncEnrollmentError::BadSubmitPayload(
                        anyhow::anyhow!(
                            "Signature is valid, but doesn't come from the expected author (expected `{}` according to the X509 certificate, got `{}`)",
                            expected_author,
                            cert_first_email
                        )
                    ));
                }

                Ok(())
            }))
        }

        fn sign_accept_payload(
            &self,
            payload: Bytes,
        ) -> PinBoxFutureResult<
            protocol::authenticated_cmds::latest::async_enrollment_accept::AcceptPayloadSignature,
            AcceptAsyncEnrollmentError,
        > {
            let cert_ref = self.certificate_reference.clone();
            Box::pin(pretend_future_is_send_on_web(async move {
                // 1. Sign the payload
                let (algorithm, signature) =
                    libparsec_platform_pki::sign_message(&payload, &cert_ref)
                        .await
                        .map_err(|err| match err {
                            err @ (PKISignMessageError::NotFound
                            | PKISignMessageError::CannotAcquireKeypair(_)
                            | PKISignMessageError::CannotSign(_)) => {
                                AcceptAsyncEnrollmentError::PKIUnusableX509CertificateReference(
                                    err.into(),
                                )
                            }
                            PKISignMessageError::CannotOpenStore(e) => {
                                AcceptAsyncEnrollmentError::PKICannotOpenCertificateStore(e.into())
                            }
                        })?;

                // 2. Get leaf & intermediate certificates
                let validation_path: libparsec_platform_pki::ValidationPathOwned =
                    libparsec_platform_pki::get_validation_path_for_cert(&cert_ref, DateTime::now())
                        .await
                        .map_err(|err| match err {
                            err @ (
                                PKIGetValidationPathForCertError::NotFound
                                | PKIGetValidationPathForCertError::InvalidCertificateDer(_)
                                | PKIGetValidationPathForCertError::InvalidCertificateDateTimeOutOfRange(_)
                                | PKIGetValidationPathForCertError::InvalidCertificateUntrusted(_)
                            ) => AcceptAsyncEnrollmentError::PKIUnusableX509CertificateReference(err.into()),
                            PKIGetValidationPathForCertError::CannotOpenStore(e) => {
                                AcceptAsyncEnrollmentError::PKICannotOpenCertificateStore(e.into())
                            }

                        })?;

                Ok(protocol::authenticated_cmds::latest::async_enrollment_accept::AcceptPayloadSignature::PKI {
                    signature,
                    algorithm,
                    accepter_der_x509_certificate: validation_path.leaf,
                    intermediate_der_x509_certificates: validation_path.intermediates,
                })
            }))
        }
    }

    impl libparsec_client::SubmitterFinalizeAsyncEnrollmentIdentityStrategy
        for AcceptFinalizeAsyncEnrollmentPKIIdentityStrategy
    {
        fn _verify_accept_payload(
            &self,
            payload: Bytes,
            payload_signature: protocol::anonymous_cmds::latest::async_enrollment_info::AcceptPayloadSignature,
        ) -> PinBoxFutureResult<(), SubmitterFinalizeAsyncEnrollmentError> {
            let cert_ref = self.certificate_reference.clone();
            Box::pin(pretend_future_is_send_on_web(async move {
                let (signature, algorithm, accepter_cert, intermediate_certs) = match payload_signature {
                    protocol::anonymous_cmds::latest::async_enrollment_info::AcceptPayloadSignature::PKI {
                        signature,
                        algorithm,
                        accepter_der_x509_certificate,
                        intermediate_der_x509_certificates,
                    } => (signature, algorithm, accepter_der_x509_certificate, intermediate_der_x509_certificates),
                    protocol::anonymous_cmds::latest::async_enrollment_info::AcceptPayloadSignature::OpenBao { .. } => {
                        return Err(SubmitterFinalizeAsyncEnrollmentError::IdentityStrategyMismatch {
                            accepter: "OpenBao".to_string(),
                            ours: "PKI".to_string(),
                        });
                    }
                };

                // 1. Retrieve the root certificate used by our own X059 certificate,
                // as the accepter X509 certificate must also depend on it.

                let validation_path: libparsec_platform_pki::ValidationPathOwned =
                    libparsec_platform_pki::get_validation_path_for_cert(&cert_ref, DateTime::now())
                        .await
                        .map_err(|err| match err {
                            err @ (
                                PKIGetValidationPathForCertError::NotFound
                                | PKIGetValidationPathForCertError::InvalidCertificateDer(_)
                                | PKIGetValidationPathForCertError::InvalidCertificateDateTimeOutOfRange(_)
                                | PKIGetValidationPathForCertError::InvalidCertificateUntrusted(_)
                            ) => SubmitterFinalizeAsyncEnrollmentError::PKIUnusableX509CertificateReference(err.into()),
                            PKIGetValidationPathForCertError::CannotOpenStore(e) => {
                                SubmitterFinalizeAsyncEnrollmentError::PKICannotOpenCertificateStore(e.into())
                            }

                        })?;

                // 2. Validate certificate trustchain and payload signature

                libparsec_platform_pki::verify_message2(
                    &payload,
                    &signature,
                    algorithm,
                    &accepter_cert,
                    intermediate_certs.iter().map(|x| x.as_ref()),
                    &[validation_path.root],
                    DateTime::now(),
                )
                .map_err(|err| {
                    SubmitterFinalizeAsyncEnrollmentError::BadAcceptPayload(err.into())
                })?;

                // Note, unlike when we verify the submit payload, we don't care about the
                // email in the accepter's X509 certificate.
                // This is because we have nothing the check it against (i.e. we accept
                // that anyone that is part of the PKI can enroll us).

                Ok(())
            }))
        }

        fn retrieve_ciphertext_key(
            &self,
            identity_system: AsyncEnrollmentLocalPendingIdentitySystem,
        ) -> PinBoxFutureResult<SecretKey, SubmitterFinalizeAsyncEnrollmentError> {
            Box::pin(pretend_future_is_send_on_web(async move {
                let (encrypted_key, certificate_ref, algorithm) = match identity_system {
                    AsyncEnrollmentLocalPendingIdentitySystem::PKI {
                        encrypted_key,
                        certificate_ref,
                        algorithm,
                    } => (encrypted_key, certificate_ref, algorithm),
                    AsyncEnrollmentLocalPendingIdentitySystem::OpenBao { .. } => {
                        return Err(SubmitterFinalizeAsyncEnrollmentError::Internal(
                            anyhow::anyhow!("Identity system mismatch: expected PKI, got OpenBao"),
                        ));
                    }
                };

                let decrypted = libparsec_platform_pki::decrypt_message(
                    algorithm,
                    &encrypted_key,
                    &certificate_ref,
                )
                .await
                .map_err(|err| match err {
                    err @ (PKIDecryptMessageError::NotFound
                    | PKIDecryptMessageError::CannotAcquireKeypair(_)
                    | PKIDecryptMessageError::CannotDecrypt(_)) => {
                        SubmitterFinalizeAsyncEnrollmentError::PKIUnusableX509CertificateReference(
                            err.into(),
                        )
                    }
                    PKIDecryptMessageError::CannotOpenStore(e) => {
                        SubmitterFinalizeAsyncEnrollmentError::PKICannotOpenCertificateStore(
                            e.into(),
                        )
                    }
                })?;

                SecretKey::try_from(decrypted.as_ref())
                    .map_err(|_| SubmitterFinalizeAsyncEnrollmentError::EnrollmentFileInvalidData)
            }))
        }
    }
}

pub use strategy::{
    AcceptFinalizeAsyncEnrollmentIdentityStrategy, SubmitAsyncEnrollmentIdentityStrategy,
};

#[derive(Debug, thiserror::Error)]
pub enum ClientGetAsyncEnrollmentAddrError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub fn client_get_async_enrollment_addr(
    client: Handle,
) -> Result<ParsecAsyncEnrollmentAddr, ClientGetAsyncEnrollmentAddrError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    Ok(client.get_async_enrollment_addr())
}

pub async fn client_list_async_enrollments(
    client: Handle,
) -> Result<Vec<AsyncEnrollmentUntrusted>, ClientListAsyncEnrollmentsError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    client.list_async_enrollments().await
}

pub async fn client_reject_async_enrollment(
    client: Handle,
    enrollment_id: AsyncEnrollmentID,
) -> Result<(), ClientRejectAsyncEnrollmentError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    client.reject_async_enrollment(enrollment_id).await
}

pub async fn client_accept_async_enrollment(
    client: Handle,
    profile: UserProfile,
    enrollment_id: AsyncEnrollmentID,
    identity_strategy: AcceptFinalizeAsyncEnrollmentIdentityStrategy,
) -> Result<(), ClientAcceptAsyncEnrollmentError> {
    let client = borrow_from_handle(client, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })
    .map_err(ClientAcceptAsyncEnrollmentError::Internal)?;

    let identity_strategy = identity_strategy
        .convert()
        .map_err(ClientAcceptAsyncEnrollmentError::Internal)?;

    client
        .accept_async_enrollment(profile, enrollment_id, identity_strategy.as_ref())
        .await
}

pub async fn submit_async_enrollment(
    config: ClientConfig,
    addr: ParsecAsyncEnrollmentAddr,
    force: bool,
    requested_device_label: DeviceLabel,
    identity_strategy: SubmitAsyncEnrollmentIdentityStrategy,
) -> Result<AvailablePendingAsyncEnrollment, SubmitAsyncEnrollmentError> {
    let config: Arc<libparsec_client::ClientConfig> = config.into();
    let identity_strategy = identity_strategy.convert().await?;

    libparsec_client::submit_async_enrollment(
        config,
        addr,
        force,
        requested_device_label,
        identity_strategy.as_ref(),
    )
    .await
}

pub async fn submitter_list_async_enrollments(
    config_dir: &Path,
) -> Result<Vec<AvailablePendingAsyncEnrollment>, SubmitterListLocalAsyncEnrollmentsError> {
    libparsec_client::submitter_list_local_async_enrollments(config_dir).await
}

pub async fn submitter_get_async_enrollment_info(
    config: ClientConfig,
    addr: ParsecAsyncEnrollmentAddr,
    enrollment_id: AsyncEnrollmentID,
) -> Result<PendingAsyncEnrollmentInfo, SubmitterGetAsyncEnrollmentInfoError> {
    let config: Arc<libparsec_client::ClientConfig> = config.into();

    libparsec_client::submitter_get_async_enrollment_info(config, addr, enrollment_id).await
}

pub async fn submitter_finalize_async_enrollment(
    config: ClientConfig,
    enrollment_file: &Path,
    new_device_save_strategy: DeviceSaveStrategy,
    identity_strategy: AcceptFinalizeAsyncEnrollmentIdentityStrategy,
) -> Result<AvailableDevice, SubmitterFinalizeAsyncEnrollmentError> {
    let config: Arc<libparsec_client::ClientConfig> = config.into();
    let identity_strategy = identity_strategy.convert()?;
    let new_device_save_strategy = new_device_save_strategy.convert_with_side_effects()?;

    libparsec_client::submitter_finalize_async_enrollment(
        config,
        enrollment_file,
        &new_device_save_strategy,
        identity_strategy.as_ref(),
    )
    .await
}

pub async fn submitter_cancel_async_enrollment(
    config: ClientConfig,
    addr: ParsecAsyncEnrollmentAddr,
    enrollment_id: AsyncEnrollmentID,
) -> Result<(), SubmitterCancelAsyncEnrollmentError> {
    let config: Arc<libparsec_client::ClientConfig> = config.into();
    libparsec_client::submitter_cancel_async_enrollment(config, addr, enrollment_id).await
}
