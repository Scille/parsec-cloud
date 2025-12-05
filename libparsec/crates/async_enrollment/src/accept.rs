// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum AsyncEnrollementAcceptError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),

    #[error("Author not allowed")]
    AuthorNotAllowed,
    #[error("Enrollment not found")]
    EnrollmentNotFound,
    #[error("Enrollment no longer available (either already accepted or rejected)")]
    EnrollmentNoLongerAvailable,
    #[error("Submitter has provided an invalid request: {0}")]
    BadSubmitPayload(#[from] AsyncEnrollmentVerifySubmitPayloadError),
    #[error("Active users limit reached")]
    ActiveUsersLimitReached,
    #[error("Human handle (i.e. email address) already taken")]
    HumanHandleAlreadyTaken,
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub trait AsyncEnrollmentAcceptIdentityStrategy {
    fn verify_submit_payload(
        &self,
        payload: &[u8],
        payload_signature: libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_list::PayloadSignature,
    ) -> Result<AsyncEnrollmentSubmitPayload, AsyncEnrollmentVerifySubmitPayloadError> {
        let submit_payload = AsyncEnrollmentSubmitPayload::load(payload)
            .map_err(|_| AsyncEnrollmentVerifySubmitPayloadError::BadPayload)?;
        self._verify_submit_payload(payload, payload_signature)?;
        Ok(submit_payload)
    }
    fn _verify_submit_payload(
        &self,
        payload: &[u8],
        payload_signature: libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_list::PayloadSignature,
    ) -> Result<AsyncEnrollmentSubmitPayload, AsyncEnrollmentVerifySubmitPayloadError>;
    fn sign_accept_payload(&self, payload: &[u8]) -> libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_accept::PayloadSignature;
}

#[derive(Debug, thiserror::Error)]
pub enum AsyncEnrollmentVerifySubmitPayloadError {
    #[error("Invalid payload serialization")]
    BadPayload,
    /// The signature couldn't be verified
    #[error("Invalid payload signature")]
    BadSignature,
    /// The signature is valid, but it has been done by someone unrelated to
    /// the requested email present in the payload.
    #[error("Requested email in the payload doesn't match the signature author's identity")]
    BadRequestedEmail,
}

pub async fn async_enrollment_accept(
    cmds: &AuthenticatedCmds,
    author: &LocalDevice,
    enrollment_id: AsyncEnrollmentID,
    profile: UserProfile,
    identity_strategy: &dyn AsyncEnrollmentAcceptIdentityStrategy,
) -> Result<(), AsyncEnrollementAcceptError> {
    // 1) Get back the enrollment submit payload from the server

    let enrollment = {
        let enrollments = {
            use libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_list::{Req, Rep};

            let rep = cmds.send(Req).await?;
            match rep {
                Rep::Ok { enrollments } => enrollments,
                Rep::AuthorNotAllowed => return Err(AsyncEnrollementAcceptError::AuthorNotAllowed),
                bad_rep @ Rep::UnknownStatus { .. } => {
                    return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                }
            }
        };
        enrollments
            .into_iter()
            .find(|e| e.enrollment_id == enrollment_id)
            .ok_or(AsyncEnrollementAcceptError::EnrollmentNotFound)?
    };

    // 2) Validate the submit payload & the requested email vs identity consistency

    let submit_payload = identity_strategy
        .verify_submit_payload(&enrollment.payload, enrollment.payload_signature)?;

    // 3) The submit payload is all good, now we can actually enroll the user

    loop {
        let timestamp = author.time_provider.now();

        let (
            submitter_user_certificate,
            submitter_redacted_user_certificate,
            submitter_device_certificate,
            submitter_redacted_device_certificate,
            accept_payload,
        ) = create_certificates_and_accept_payload(
            author,
            submit_payload.requested_device_label,
            submit_payload.requested_human_handle,
            profile,
            submit_payload.public_key,
            submit_payload.verify_key,
            timestamp,
        );
        let accept_payload_signature = identity_strategy.sign_accept_payload(&accept_payload);

        {
            use libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_accept::{Req, Rep};

            let rep = cmds
                .send(Req {
                    enrollment_id,
                    submitter_user_certificate,
                    submitter_device_certificate,
                    submitter_redacted_user_certificate,
                    submitter_redacted_device_certificate,
                    payload: accept_payload,
                    payload_signature: accept_payload_signature,
                })
                .await?;
            return match rep {
                Rep::Ok => Ok(()),
                Rep::RequireGreaterTimestamp {
                    strictly_greater_than,
                } => {
                    timestamp = greater_timestamp(
                        &author.time_provider,
                        GreaterTimestampOffset::User,
                        strictly_greater_than,
                    );
                    continue;
                }
                Rep::AuthorNotAllowed => Err(AsyncEnrollementAcceptError::AuthorNotAllowed),
                Rep::EnrollmentNotFound => Err(AsyncEnrollementAcceptError::EnrollmentNotFound),
                Rep::EnrollmentNoLongerAvailable => Err(AsyncEnrollementAcceptError::EnrollmentNoLongerAvailable),
                Rep::ActiveUsersLimitReached => Err(AsyncEnrollementAcceptError::ActiveUsersLimitReached),
                Rep::HumanHandleAlreadyTaken => Err(AsyncEnrollementAcceptError::HumanHandleAlreadyTaken),
                Rep::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                } => {
                    let event = EventTooMuchDriftWithServerClock {
                        server_timestamp,
                        ballpark_client_early_offset,
                        ballpark_client_late_offset,
                        client_timestamp,
                    };
                    self.event_bus.send(&event);

                    Err(AsyncEnrollementAcceptError::TimestampOutOfBallpark {
                        server_timestamp,
                        client_timestamp,
                        ballpark_client_early_offset,
                        ballpark_client_late_offset,
                    })
                }
                bad_rep @ (
                    Rep::UserAlreadyExists  // User & device IDs have just been randomly generated
                    | Rep::InvalidCertificate
                    | Rep::InvalidPayload
                    | Rep::InvalidPayloadSignature
                    | Rep::UnknownStatus { .. }
                ) => {
                    return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                }
            };
        }
    }
}

fn create_certificates_and_accept_payload(
    author: &LocalDevice,
    device_label: DeviceLabel,
    human_handle: HumanHandle,
    profile: UserProfile,
    public_key: PublicKey,
    verify_key: VerifyKey,
    timestamp: DateTime,
) -> (Bytes, Bytes, Bytes, Bytes, Bytes) {
    let user_id = UserID::default();
    let device_id = DeviceID::default();

    let (user_certificate_bytes, redacted_user_certificate_bytes) = {
        let user_certificate = UserCertificate {
            author: CertificateSigner::User(author.device_id),
            timestamp,
            user_id,
            human_handle: MaybeRedacted::Real(human_handle.clone()),
            public_key: public_key.clone(),
            algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
            profile,
        };

        (
            user_certificate.dump_and_sign(&author.signing_key).into(),
            user_certificate
                .into_redacted()
                .dump_and_sign(&author.signing_key)
                .into(),
        )
    };

    let (device_certificate_bytes, redacted_device_certificate_bytes) = {
        let device_certificate = DeviceCertificate {
            author: CertificateSigner::User(author.device_id),
            timestamp,
            purpose: DevicePurpose::Standard,
            user_id,
            device_id,
            device_label: MaybeRedacted::Real(device_label.clone()),
            verify_key: verify_key.clone(),
            algorithm: SigningKeyAlgorithm::Ed25519,
        };

        (
            device_certificate.dump_and_sign(&author.signing_key).into(),
            device_certificate
                .into_redacted()
                .dump_and_sign(&author.signing_key)
                .into(),
        )
    };

    let accept_payload = AsyncEnrollmentAcceptPayload {
        user_id,
        device_id,
        device_label,
        human_handle,
        profile,
        root_verify_key: author.root_verify_key().clone(),
    };
    let accept_payload_bytes = accept_payload.dump().into();

    (
        user_certificate_bytes,
        redacted_user_certificate_bytes,
        device_certificate_bytes,
        redacted_device_certificate_bytes,
        accept_payload_bytes,
    )
}
