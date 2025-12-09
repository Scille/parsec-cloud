// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::protocol;
use libparsec_platform_async::PinBoxFutureResult;
use libparsec_types::prelude::*;

use crate::{
    AcceptAsyncEnrollmentError, AcceptAsyncEnrollmentIdentityStrategy, SubmitAsyncEnrollmentError,
    SubmitAsyncEnrollmentIdentityStrategy, SubmitterFinalizeAsyncEnrollmentError,
    SubmitterFinalizeAsyncEnrollmentIdentityStrategy,
};

pub(super) struct MockedAsyncEnrollmentIdentityStrategy {
    submitter_requested_human_handle: HumanHandle,
    submitter_signing_key: SigningKey,
    accepter_signing_key: SigningKey,
}

impl MockedAsyncEnrollmentIdentityStrategy {
    pub(super) fn new(submitter_requested_human_handle: HumanHandle) -> Self {
        Self {
            submitter_requested_human_handle,
            submitter_signing_key: SigningKey::generate(),
            accepter_signing_key: SigningKey::generate(),
        }
    }
}

impl Default for MockedAsyncEnrollmentIdentityStrategy {
    fn default() -> Self {
        Self::new("Mike <mike@example.invalid>".parse().unwrap())
    }
}

impl SubmitAsyncEnrollmentIdentityStrategy for MockedAsyncEnrollmentIdentityStrategy {
    fn sign_submit_payload(
        &self,
        payload: Bytes,
    ) -> PinBoxFutureResult<
        protocol::anonymous_cmds::latest::async_enrollment_submit::SubmitPayloadSignature,
        SubmitAsyncEnrollmentError,
    > {
        let signature = self.submitter_signing_key.sign_only_signature(&payload);
        let outcome = Ok(protocol::anonymous_cmds::latest::async_enrollment_submit::SubmitPayloadSignature::OpenBao {
            submitter_openbao_entity_id: "e7d07e2a-b05d-ec59-dcf2-4403efdbf1b7".to_string(),
            signature: data_encoding::BASE64.encode(&signature),
        });
        Box::pin(async move { outcome })
    }

    fn human_handle(&self) -> &HumanHandle {
        &self.submitter_requested_human_handle
    }

    fn generate_ciphertext_key(
        &self,
    ) -> PinBoxFutureResult<
        (SecretKey, AsyncEnrollmentLocalPendingIdentitySystem),
        SubmitAsyncEnrollmentError,
    > {
        let key = SecretKey::generate();
        let identity_system = AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
            openbao_ciphertext_key_path: data_encoding::BASE64.encode(key.as_ref()), // Totally legit encryption here ;-)
            openbao_entity_id: "12979034-ef03-4cf5-b05f-1a67353db92f".to_string(),
            openbao_preferred_auth_id: "HEXAGONE".to_string(),
        };
        Box::pin(async { Ok((key, identity_system)) })
    }
}

impl AcceptAsyncEnrollmentIdentityStrategy for MockedAsyncEnrollmentIdentityStrategy {
    fn _verify_submit_payload(
        &self,
        payload: Bytes,
        payload_signature: libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_list::SubmitPayloadSignature,
        expected_author: EmailAddress,
    ) -> PinBoxFutureResult<(), AcceptAsyncEnrollmentError> {
        let submitter_verify_key = self.submitter_signing_key.verify_key();
        let expected_author_is_ok =
            *self.submitter_requested_human_handle.email() == expected_author;
        Box::pin(async move {
            let signature = match payload_signature {
                protocol::authenticated_cmds::latest::async_enrollment_list::SubmitPayloadSignature::OpenBao {
                    signature,
                    ..
                } => {
                    let signature_as_vec = data_encoding::BASE64.decode(signature.as_bytes())
                    .map_err(|_| AcceptAsyncEnrollmentError::BadSubmitPayload(anyhow::anyhow!("Invalid payload signature")))?;
                    let signature: [u8; SigningKey::SIGNATURE_SIZE] = signature_as_vec.try_into()
                        .map_err(|_| AcceptAsyncEnrollmentError::BadSubmitPayload(anyhow::anyhow!("Invalid payload signature")))?;
                    signature
                },
                protocol::authenticated_cmds::latest::async_enrollment_list::SubmitPayloadSignature::PKI { .. } => return Err(AcceptAsyncEnrollmentError::IdentityStrategyMismatch { submitter: "PKI".to_string(), ours: "OpenBao".to_string() }),
            };

            submitter_verify_key
                .verify_with_signature(&signature, &payload)
                .map_err(|err| AcceptAsyncEnrollmentError::BadSubmitPayload(err.into()))?;

            if !expected_author_is_ok {
                return Err(AcceptAsyncEnrollmentError::BadSubmitPayload(
                    anyhow::anyhow!("Requested email in the payload doesn't match the signature author's identity")
                ));
            }

            Ok(())
        })
    }

    fn sign_accept_payload(&self, payload: Bytes) -> PinBoxFutureResult<
        libparsec_client_connection::protocol::authenticated_cmds::latest::async_enrollment_accept::AcceptPayloadSignature,
        AcceptAsyncEnrollmentError
    >{
        let signature = self.accepter_signing_key.sign_only_signature(&payload);
        let outcome = Ok(protocol::authenticated_cmds::latest::async_enrollment_accept::AcceptPayloadSignature::OpenBao {
            accepter_openbao_entity_id: "b09b013e-e1eb-40d2-a39a-571a87df72a6".to_string(),
            signature: data_encoding::BASE64.encode(&signature),
        });
        Box::pin(async move { outcome })
    }
}

impl SubmitterFinalizeAsyncEnrollmentIdentityStrategy for MockedAsyncEnrollmentIdentityStrategy {
    fn _verify_accept_payload(
        &self,
        payload: Bytes,
        payload_signature: libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_info::AcceptPayloadSignature,
    ) -> PinBoxFutureResult<(), SubmitterFinalizeAsyncEnrollmentError> {
        let accepter_verify_key = self.accepter_signing_key.verify_key();
        Box::pin(async move {
            let signature = match payload_signature {
                protocol::anonymous_cmds::latest::async_enrollment_info::AcceptPayloadSignature::OpenBao {
                    signature,
                    ..
                } => {
                    let signature_as_vec = data_encoding::BASE64.decode(signature.as_bytes())
                    .map_err(|_| SubmitterFinalizeAsyncEnrollmentError::BadAcceptPayload(anyhow::anyhow!("Invalid payload signature")))?;
                    let signature: [u8; SigningKey::SIGNATURE_SIZE] = signature_as_vec.try_into()
                        .map_err(|_| SubmitterFinalizeAsyncEnrollmentError::BadAcceptPayload(anyhow::anyhow!("Invalid payload signature")))?;
                    signature
                }
                protocol::anonymous_cmds::latest::async_enrollment_info::AcceptPayloadSignature::PKI { .. } =>
                    return Err(SubmitterFinalizeAsyncEnrollmentError::IdentityStrategyMismatch { accepter: "PKI".to_string(), ours: "OpenBao".to_string() }),
            };
            accepter_verify_key
                .verify_with_signature(&signature, &payload)
                .map_err(|err| SubmitterFinalizeAsyncEnrollmentError::BadAcceptPayload(err.into()))
        })
    }

    fn retrieve_ciphertext_key(
        &self,
        identity_system: AsyncEnrollmentLocalPendingIdentitySystem,
    ) -> PinBoxFutureResult<SecretKey, SubmitterFinalizeAsyncEnrollmentError> {
        Box::pin(async {
            let key = match identity_system {
                AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
                    openbao_ciphertext_key_path,
                    ..
                } => {
                    let key_as_vec = data_encoding::BASE64
                        .decode(openbao_ciphertext_key_path.as_bytes())
                        .map_err(|err| SubmitterFinalizeAsyncEnrollmentError::EnrollmentFileCannotRetrieveCiphertextKey(err.into()))?;
                    let key: SecretKey = key_as_vec.as_slice().try_into().unwrap();
                    key
                }
                _ => unreachable!(),
            };
            Ok(key)
        })
    }
}
