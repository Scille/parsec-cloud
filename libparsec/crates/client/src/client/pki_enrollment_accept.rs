// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_platform_pki::{sign_message, x509::X509CertificateInformation};
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
    enrollment_id: PKIEnrollmentID,
    accepter_cert_ref: &X509CertificateReference,
    submitter_der_cert: &[u8],
    submit_payload: PkiEnrollmentSubmitPayload,
) -> Result<(), PkiEnrollmentAcceptError> {
    // Keep looping while `RequireGreaterTimestamp` is returned
    let mut to_use_timestamp = client.device.now();
    let submitter_human_handle = X509CertificateInformation::load_der(submitter_der_cert)
        .context("Failed to load submitter certificate information")
        .and_then(|info| {
            info.human_handle()
                .context("Missing human handle from submitter certificate")
        })?;
    let accepter_intermediate_certs =
        libparsec_platform_pki::get_validation_path_for_cert(accepter_cert_ref, DateTime::now())
            .await
            .map_err(anyhow::Error::from)
            .context("Failed to get intermediate certificates for itself")
            .map_err(PkiEnrollmentAcceptError::PkiOperationError)?;

    loop {
        let outcome = accept_internal(
            client,
            profile,
            enrollment_id,
            Accepter {
                cert_ref: accepter_cert_ref,
                der_cert: &accepter_intermediate_certs.leaf,
                intermediate_der_certs: &accepter_intermediate_certs.intermediates,
            },
            Submitter {
                payload: submit_payload.clone(),
                human_handle: submitter_human_handle.clone(),
            },
            to_use_timestamp,
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

struct Accepter<'a> {
    cert_ref: &'a X509CertificateReference,
    der_cert: &'a [u8],
    intermediate_der_certs: &'a [Bytes],
}

struct Submitter {
    payload: PkiEnrollmentSubmitPayload,
    human_handle: HumanHandle,
}

async fn accept_internal(
    client: &Client,
    profile: UserProfile,
    enrollment_id: PKIEnrollmentID,
    accepter: Accepter<'_>,
    submitter: Submitter,
    now: DateTime,
) -> Result<PkiAcceptOutcome, PkiEnrollmentAcceptError> {
    // Generate payload and user/device certificates
    let (
        submitter_user_certificate,
        submitter_redacted_user_certificate,
        submitter_device_certificate,
        submitter_redacted_device_certificate,
        payload,
    ) = create_new_signed_user_certificates(
        &client.device,
        submitter.payload.device_label,
        submitter.human_handle,
        profile,
        submitter.payload.public_key,
        submitter.payload.verify_key,
        now,
    );

    // sign payload with accepter x509certificate
    let (payload_signature_algorithm, payload_signature) =
        sign_message(&payload, accepter.cert_ref)
            .map_err(|e| PkiEnrollmentAcceptError::PkiOperationError(e.into()))?;

    // send request
    match client
        .cmds
        .send(Req {
            enrollment_id,
            accepter_der_x509_certificate: Bytes::copy_from_slice(accepter.der_cert),
            accepter_intermediate_der_x509_certificates: accepter.intermediate_der_certs.to_vec(),
            payload,
            payload_signature,
            payload_signature_algorithm,
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
        Rep::UserAlreadyExists
        /* TODO: https://github.com/Scille/parsec-cloud/issues/11648 */
        | Rep::InvalidPayloadSignature
        | Rep::InvalidDerX509Certificate
        | Rep::InvalidX509Trustchain
        /* END-TODO: https://github.com/Scille/parsec-cloud/issues/11648 */
        => todo!(),
        bad_rep @ (Rep::InvalidPayload | Rep::UnknownStatus { .. }) => {
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

    let answer_payload = PkiEnrollmentAnswerPayload {
        user_id,
        device_id,
        device_label,
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
