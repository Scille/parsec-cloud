// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_platform_async::PinBoxFutureResult;
use libparsec_types::prelude::*;

use crate::{
    greater_timestamp, utils::create_user_and_device_certificates, EventBus,
    EventTooMuchDriftWithServerClock, GreaterTimestampOffset,
};

#[derive(Debug, thiserror::Error)]
pub enum AcceptAsyncEnrollmentError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Author not allowed")]
    AuthorNotAllowed,
    #[error("Enrollment not found")]
    EnrollmentNotFound,
    #[error("Enrollment no longer available (either already accepted or rejected)")]
    EnrollmentNoLongerAvailable,
    #[error("Submitter has provided an invalid request payload: {0}")]
    BadSubmitPayload(anyhow::Error),
    #[error(
        "Submit payload is signed with a different identity system ({submitter}) than ours ({ours})"
    )]
    IdentityStrategyMismatch { submitter: String, ours: String },
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
    Internal(anyhow::Error),

    // OpenBao-related errors
    #[error("Invalid OpenBao server URL: {0}")]
    OpenBaoBadURL(anyhow::Error),
    #[error("No response from the OpenBao server: {0}")]
    OpenBaoNoServerResponse(anyhow::Error),
    #[error("The OpenBao server returned an unexpected response: {0}")]
    OpenBaoBadServerResponse(anyhow::Error),

    // PKI-related errors
    #[error("Invalid X509 trustchain (server doesn't recognize the root certificate)")]
    InvalidX509Trustchain,
    // TODO: add other PKI-related errors
    // (see https://github.com/Scille/parsec-cloud/issues/12028)
}

pub trait AcceptAsyncEnrollmentIdentityStrategy: Send + Sync {
    fn verify_submit_payload(
        &self,
        payload: Bytes,
        payload_signature: libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_list::SubmitPayloadSignature,
    ) -> PinBoxFutureResult<AsyncEnrollmentSubmitPayload, AcceptAsyncEnrollmentError> {
        let maybe_submit_payload = AsyncEnrollmentSubmitPayload::load(&payload)
            .map_err(|err| AcceptAsyncEnrollmentError::BadSubmitPayload(err.into()))
            .map(|submit_payload| {
                let verify_submit_payload_future = self._verify_submit_payload(
                    payload,
                    payload_signature,
                    submit_payload.requested_human_handle.email().to_owned(),
                );
                (submit_payload, verify_submit_payload_future)
            });
        Box::pin(async {
            let (submit_payload, verify_submit_payload_future) = maybe_submit_payload?;
            verify_submit_payload_future.await?;
            Ok(submit_payload)
        })
    }
    fn _verify_submit_payload(
        &self,
        payload: Bytes,
        payload_signature: libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_list::SubmitPayloadSignature,
        expected_author: EmailAddress,
    ) -> PinBoxFutureResult<(), AcceptAsyncEnrollmentError>;
    fn sign_accept_payload(&self, payload: Bytes) -> PinBoxFutureResult<
        libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_accept::AcceptPayloadSignature,
        AcceptAsyncEnrollmentError
    >;
}

pub(crate) async fn accept_async_enrollment(
    cmds: &AuthenticatedCmds,
    event_bus: &EventBus,
    author: &LocalDevice,
    enrollment_id: AsyncEnrollmentID,
    profile: UserProfile,
    identity_strategy: &dyn AcceptAsyncEnrollmentIdentityStrategy,
) -> Result<(), AcceptAsyncEnrollmentError> {
    // 1) Get back the enrollment submit payload from the server

    let enrollment = {
        let enrollments = {
            use libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_list::{Req, Rep};

            let rep = cmds.send(Req).await?;
            match rep {
                Rep::Ok { enrollments } => enrollments,
                Rep::AuthorNotAllowed => return Err(AcceptAsyncEnrollmentError::AuthorNotAllowed),
                bad_rep @ Rep::UnknownStatus { .. } => {
                    return Err(AcceptAsyncEnrollmentError::Internal(anyhow::anyhow!(
                        "Unexpected server response: {:?}",
                        bad_rep
                    )))
                }
            }
        };
        enrollments
            .into_iter()
            .find(|e| e.enrollment_id == enrollment_id)
            .ok_or(AcceptAsyncEnrollmentError::EnrollmentNotFound)?
    };

    // 2) Validate the submit payload & the requested email vs identity consistency

    let submit_payload = identity_strategy
        .verify_submit_payload(
            enrollment.submit_payload,
            enrollment.submit_payload_signature,
        )
        .await?;

    // 3) The submit payload is all good, now we can actually enroll the user

    let mut timestamp = author.time_provider.now();
    loop {
        let (
            submitter_user_certificate,
            submitter_redacted_user_certificate,
            submitter_device_certificate,
            submitter_redacted_device_certificate,
            accept_payload,
        ) = create_certificates_and_accept_payload(
            author,
            submit_payload.requested_device_label.clone(),
            submit_payload.requested_human_handle.clone(),
            profile,
            submit_payload.public_key.clone(),
            submit_payload.verify_key.clone(),
            timestamp,
        );
        let accept_payload_signature = identity_strategy
            .sign_accept_payload(accept_payload.clone())
            .await?;

        {
            use libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_accept::{Req, Rep};

            let rep = cmds
                .send(Req {
                    enrollment_id,
                    submitter_user_certificate,
                    submitter_device_certificate,
                    submitter_redacted_user_certificate,
                    submitter_redacted_device_certificate,
                    accept_payload,
                    accept_payload_signature,
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
                Rep::AuthorNotAllowed => Err(AcceptAsyncEnrollmentError::AuthorNotAllowed),
                Rep::EnrollmentNotFound => Err(AcceptAsyncEnrollmentError::EnrollmentNotFound),
                Rep::EnrollmentNoLongerAvailable => Err(AcceptAsyncEnrollmentError::EnrollmentNoLongerAvailable),
                Rep::ActiveUsersLimitReached => Err(AcceptAsyncEnrollmentError::ActiveUsersLimitReached),
                Rep::HumanHandleAlreadyTaken => Err(AcceptAsyncEnrollmentError::HumanHandleAlreadyTaken),
                Rep::InvalidX509Trustchain => Err(AcceptAsyncEnrollmentError::InvalidX509Trustchain),
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
                    event_bus.send(&event);

                    Err(AcceptAsyncEnrollmentError::TimestampOutOfBallpark {
                        server_timestamp,
                        client_timestamp,
                        ballpark_client_early_offset,
                        ballpark_client_late_offset,
                    })
                }
                bad_rep @ (
                    Rep::UserAlreadyExists  // User & device IDs have just been randomly generated
                    | Rep::SubmitAndAcceptIdentitySystemsMismatch
                    | Rep::InvalidDerX509Certificate
                    | Rep::InvalidCertificate
                    | Rep::InvalidAcceptPayload
                    | Rep::InvalidAcceptPayloadSignature
                    | Rep::UnknownStatus { .. }
                ) => {
                    return Err(AcceptAsyncEnrollmentError::Internal(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep)))
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
    let (
        user_id,
        device_id,
        user_certificate_bytes,
        redacted_user_certificate_bytes,
        device_certificate_bytes,
        redacted_device_certificate_bytes,
    ) = create_user_and_device_certificates(
        author,
        device_label.clone(),
        human_handle.clone(),
        profile,
        public_key,
        verify_key,
        timestamp,
    );

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
