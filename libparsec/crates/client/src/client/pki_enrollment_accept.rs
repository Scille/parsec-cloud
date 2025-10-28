// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_platform_pki::{get_der_encoded_certificate, sign_message};
use libparsec_protocol::authenticated_cmds::latest::pki_enrollment_accept::{Rep, Req};
use libparsec_types::prelude::*;

use crate::{
    greater_timestamp, utils::create_user_and_device_certificates, Client, GreaterTimestampOffset,
};

#[derive(Debug, thiserror::Error)]
pub enum PkiEnrollmentAcceptError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("author not allowed: must be admin to accept pki enrollment")]
    AuthorNotAllowed,
    #[error("enrollment is no longer pending")]
    EnrollmentNoLongerAvailable,
    #[error("enrollment not found")]
    EnrollmentNotFound,
    #[error("active users limit reached")]
    ActiveUsersLimitReached,
    #[error("human handle already taken")]
    HumanHandleAlreadyTaken,
    #[error("PKI operation error: {0}")]
    PkiOperationError(anyhow::Error),
}

enum PkiAcceptOutcome {
    Done,
    RequireGreaterTimestamp(DateTime),
}

pub async fn accept(
    client: &Client,
    profile: UserProfile,
    enrollment_id: EnrollmentID,
    human_handle: &HumanHandle,
    cert_ref: &X509CertificateReference,
    submit_payload: &PkiEnrollmentSubmitPayload,
) -> Result<(), PkiEnrollmentAcceptError> {
    // Keep looping while `RequireGreaterTimestamp` is returned
    let mut to_use_timestamp = client.device.now();
    loop {
        let outcome = accept_internal(
            client,
            profile,
            enrollment_id,
            human_handle,
            cert_ref,
            to_use_timestamp,
            submit_payload,
        )
        .await?;

        match outcome {
            PkiAcceptOutcome::Done => break Ok(()),
            PkiAcceptOutcome::RequireGreaterTimestamp(strictly_greater_than) => {
                to_use_timestamp = greater_timestamp(
                    &client.device.time_provider,
                    GreaterTimestampOffset::User,
                    strictly_greater_than,
                );
            }
        }
    }
}

async fn accept_internal(
    client: &Client,
    profile: UserProfile,
    enrollment_id: EnrollmentID,
    human_handle: &HumanHandle,
    cert_ref: &X509CertificateReference,
    now: DateTime,
    submit_payload: &PkiEnrollmentSubmitPayload,
) -> Result<PkiAcceptOutcome, PkiEnrollmentAcceptError> {
    let accepter_der_x509_certificate = get_der_encoded_certificate(cert_ref)
        .map_err(|e| PkiEnrollmentAcceptError::PkiOperationError(e.into()))?
        .der_content;

    // Generate payload and user/device certificates
    let (
        submitter_user_certificate,
        submitter_redacted_user_certificate,
        submitter_device_certificate,
        submitter_redacted_device_certificate,
        payload,
    ) = create_new_signed_user_certificates(
        &client.device,
        &submit_payload.device_label,
        human_handle,
        profile,
        &submit_payload.public_key,
        &submit_payload.verify_key,
        now,
    );

    // sign payload with accepter x509certificate
    let payload_signature = sign_message(&payload, cert_ref)
        .map_err(|e| PkiEnrollmentAcceptError::PkiOperationError(e.into()))?
        .signature;

    // send request
    match client
        .cmds
        .send(Req {
            enrollment_id,
            accepter_der_x509_certificate,
            payload,
            payload_signature,
            submitter_device_certificate,
            submitter_redacted_device_certificate,
            submitter_redacted_user_certificate,
            submitter_user_certificate,
        })
        .await?
    {
        Rep::Ok => Ok(PkiAcceptOutcome::Done),
        Rep::AuthorNotAllowed => Err(PkiEnrollmentAcceptError::AuthorNotAllowed),
        Rep::EnrollmentNoLongerAvailable => {
            Err(PkiEnrollmentAcceptError::EnrollmentNoLongerAvailable)
        }
        Rep::EnrollmentNotFound => Err(PkiEnrollmentAcceptError::EnrollmentNotFound),

        Rep::ActiveUsersLimitReached => Err(PkiEnrollmentAcceptError::ActiveUsersLimitReached),
        Rep::HumanHandleAlreadyTaken => Err(PkiEnrollmentAcceptError::HumanHandleAlreadyTaken),

        Rep::RequireGreaterTimestamp {
            strictly_greater_than,
        } => Ok(PkiAcceptOutcome::RequireGreaterTimestamp(
            strictly_greater_than,
        )),
        Rep::TimestampOutOfBallpark { .. } => {
            todo!("needs certificate ops, maybe move this file to certif ?")
            // let event = EventTooMuchDriftWithServerClock {
            //     server_timestamp,
            //     ballpark_client_early_offset,
            //     ballpark_client_late_offset,
            //     client_timestamp,
            // };
            // client.certificates_ops.event_bus.send(&event);

            // Err(PkiEnrollmentAcceptError::TimestampOutOfBallpark {
            //     server_timestamp,
            //     client_timestamp,
            //     ballpark_client_early_offset,
            //     ballpark_client_late_offset,
            // })
        }
        Rep::UserAlreadyExists => todo!(),
        bad_rep @ (Rep::InvalidCertificate
        | Rep::InvalidPayloadData
        | Rep::UnknownStatus { .. }) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

/// Helper to prepare the creation of a new user.
/// Returns:
/// - new user certificate
/// - redacted ser certificate
/// - device certificate
/// - redacted certificate
/// - accept answer payload
fn create_new_signed_user_certificates(
    author: &LocalDevice,
    device_label: &DeviceLabel,
    human_handle: &HumanHandle,
    profile: UserProfile,
    public_key: &PublicKey,
    verify_key: &VerifyKey,
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
        device_label,
        human_handle,
        profile,
        public_key,
        verify_key,
        timestamp,
    );

    let answer_payload = PkiEnrollmentAnswerPayload {
        user_id,
        device_id,
        device_label: device_label.clone(),
        human_handle: human_handle.clone(),
        profile,
        root_verify_key: author.root_verify_key().clone(),
    };

    (
        user_certificate_bytes,
        redacted_user_certificate_bytes,
        device_certificate_bytes,
        redacted_device_certificate_bytes,
        answer_payload.dump().into(),
    )
}
