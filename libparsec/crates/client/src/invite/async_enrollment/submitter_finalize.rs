// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use libparsec_client_connection::{AnonymousCmds, ConnectionError};
use libparsec_platform_async::PinBoxFutureResult;
use libparsec_platform_device_loader::{
    AvailableDevice, DeviceSaveStrategy, LoadPendingAsyncEnrollmentError, RemoteOperationServer,
    RemoveDeviceError, SaveDeviceError,
};
use libparsec_types::prelude::*;

use crate::ClientConfig;

#[derive(Debug, thiserror::Error)]
pub enum SubmitterFinalizeAsyncEnrollmentError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("No space available")]
    NoSpaceAvailable,
    #[error("Enrollment is not in the accepted state")]
    NotAccepted,
    #[error("Accepter has provided an invalid request payload: {0}")]
    BadAcceptPayload(anyhow::Error),
    #[error(
        "Accept payload is signed with a different identity system ({accepter}) than ours ({ours})"
    )]
    IdentityStrategyMismatch { accepter: String, ours: String },
    #[error("Cannot load enrollment file: Invalid path: {0}")]
    EnrollmentFileInvalidPath(anyhow::Error),
    #[error("Cannot  load enrollment file: Cannot retrieve its decryption key: {0}")]
    EnrollmentFileCannotRetrieveCiphertextKey(anyhow::Error),
    #[error("Cannot load enrollment file: Invalid data")]
    EnrollmentFileInvalidData,
    #[error("Server doesn't know this enrollment")]
    EnrollmentNotFoundOnServer,
    #[error("Cannot save device: Invalid path: {0}")]
    SaveDeviceInvalidPath(anyhow::Error),
    /// Note only a subset of save strategies requires server access to
    /// upload an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("Cannot save device: No response from {server} server: {error}")]
    // We don't use `ConnectionError` here since this type only corresponds to
    // an answer from the Parsec server and here any arbitrary server may have
    // been (unsuccessfully) requested (e.g. OpenBao server).
    SaveDeviceRemoteOpaqueKeyUploadOffline {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    /// Note only a subset of save strategies requires server access to
    /// upload an opaque key that itself protects the ciphertext key
    /// (e.g. account vault).
    #[error("Cannot save device: {server} server opaque key upload failed: {error}")]
    SaveDeviceRemoteOpaqueKeyUploadFailed {
        server: RemoteOperationServer,
        error: anyhow::Error,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),

    // OpenBao-related errors
    #[error("Invalid OpenBao server URL: {0}")]
    OpenBaoBadURL(anyhow::Error),
    #[error("No response from the OpenBao server: {0}")]
    OpenBaoNoServerResponse(anyhow::Error),
    #[error("The OpenBao server returned an unexpected response: {0}")]
    OpenBaoBadServerResponse(anyhow::Error),

    // PKI-related errors
    #[error("Cannot open PKI X509 certificate store: {0}")]
    PKICannotOpenCertificateStore(anyhow::Error),
    // Certificate not found, invalid certificate, cannot use to sign etc.
    #[error("Cannot use the referenced X509 certificate for PKI operation: {0}")]
    PKIUnusableX509CertificateReference(anyhow::Error),
}

pub trait SubmitterFinalizeAsyncEnrollmentIdentityStrategy: Send + Sync {
    fn verify_accept_payload(
        &self,
        payload: Bytes,
        payload_signature: libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_info::AcceptPayloadSignature,
    ) -> PinBoxFutureResult<AsyncEnrollmentAcceptPayload, SubmitterFinalizeAsyncEnrollmentError>
    {
        let maybe_accept_payload = AsyncEnrollmentAcceptPayload::load(&payload)
            .map_err(|err| SubmitterFinalizeAsyncEnrollmentError::BadAcceptPayload(err.into()));
        let verify_accept_payload_future = self._verify_accept_payload(payload, payload_signature);
        Box::pin(async {
            let accept_payload = maybe_accept_payload?;
            verify_accept_payload_future.await?;
            Ok(accept_payload)
        })
    }
    fn _verify_accept_payload(
        &self,
        payload: Bytes,
        payload_signature: libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_info::AcceptPayloadSignature,
    ) -> PinBoxFutureResult<(), SubmitterFinalizeAsyncEnrollmentError>;
    fn retrieve_ciphertext_key(
        &self,
        identity_system: AsyncEnrollmentLocalPendingIdentitySystem,
    ) -> PinBoxFutureResult<SecretKey, SubmitterFinalizeAsyncEnrollmentError>;
}

pub async fn submitter_finalize_async_enrollment(
    config: Arc<ClientConfig>,
    enrollment_file: &Path,
    new_device_save_strategy: &DeviceSaveStrategy,
    identity_strategy: &dyn SubmitterFinalizeAsyncEnrollmentIdentityStrategy,
) -> Result<AvailableDevice, SubmitterFinalizeAsyncEnrollmentError> {
    // 1) Load the local pending async enrollment file

    let (full_content, cleartext_content) =
        libparsec_platform_device_loader::load_pending_async_enrollment(
            &config.config_dir,
            enrollment_file,
        )
        .await
        .map_err(|err| match err {
            LoadPendingAsyncEnrollmentError::StorageNotAvailable => {
                SubmitterFinalizeAsyncEnrollmentError::NoSpaceAvailable
            }
            LoadPendingAsyncEnrollmentError::InvalidPath(err) => {
                SubmitterFinalizeAsyncEnrollmentError::EnrollmentFileInvalidPath(err)
            }
            LoadPendingAsyncEnrollmentError::InvalidData => {
                SubmitterFinalizeAsyncEnrollmentError::EnrollmentFileInvalidData
            }
            LoadPendingAsyncEnrollmentError::Internal(err) => {
                SubmitterFinalizeAsyncEnrollmentError::Internal(err)
            }
        })?;

    // 2) Fetch back the ciphertext key protecting the async enrollment file

    let ciphertext_key = identity_strategy
        .retrieve_ciphertext_key(cleartext_content.identity_system)
        .await?;

    // 3) Decrypt the async enrollment file and use the encrypted hash digest to
    //    make sure the non-encrypted parts haven't been tempered with.

    {
        let cleartext_content_digest = ciphertext_key
            .decrypt(&full_content.ciphertext_cleartext_content_digest)
            .map_err(|_| SubmitterFinalizeAsyncEnrollmentError::EnrollmentFileInvalidData)?;

        if HashDigest::from_data(&full_content.cleartext_content).as_ref()
            != cleartext_content_digest
        {
            return Err(SubmitterFinalizeAsyncEnrollmentError::EnrollmentFileInvalidData);
        }
    }

    let to_become_user_private_key: PrivateKey = {
        // Note `raw` is a vector, so in theory it could have been extended
        // multiple time during the decryption process, leaving piece of data
        // behind that this zeroize-on-drop cannot reach...
        // However this is not the case in practice since both RustCrypto and
        // libsodium_rs start the decryption process by allocating the output
        // vector at correct final size.
        let raw: zeroize::Zeroizing<_> = ciphertext_key
            .decrypt(&full_content.ciphertext_private_key)
            .map_err(|_| SubmitterFinalizeAsyncEnrollmentError::EnrollmentFileInvalidData)?
            .into();
        raw.as_slice()
            .try_into()
            .map_err(|_| SubmitterFinalizeAsyncEnrollmentError::EnrollmentFileInvalidData)?
    };

    let to_become_device_signing_key: SigningKey = {
        let raw: zeroize::Zeroizing<_> = ciphertext_key
            .decrypt(&full_content.ciphertext_signing_key)
            .map_err(|_| SubmitterFinalizeAsyncEnrollmentError::EnrollmentFileInvalidData)?
            .into();
        raw.as_slice()
            .try_into()
            .map_err(|_| SubmitterFinalizeAsyncEnrollmentError::EnrollmentFileInvalidData)?
    };

    // 4) Get back the enrollment status from the server

    let addr = ParsecAsyncEnrollmentAddr::new(
        cleartext_content.server_url.clone(),
        cleartext_content.organization_id.clone(),
    );
    let cmds = Arc::new(
        AnonymousCmds::new(&config.config_dir, addr.into(), config.proxy.clone())
            .map_err(|e| anyhow::anyhow!("Error while configuring connection to server: {e}"))?,
    );

    let status = {
        use libparsec_client_connection::protocol::anonymous_cmds::latest::async_enrollment_info::{Req, Rep};

        let req = Req {
            enrollment_id: cleartext_content.enrollment_id,
        };
        let rep = cmds.send(req).await?;
        match rep {
            Rep::Ok(status) => status,
            Rep::EnrollmentNotFound => {
                // Note the server might not know about our valid enrollment if
                // it is a very old one (think of a periodic database cleanup
                // run by the database admin).
                // However this is extremely unlikely to occur here since the
                // `submitter_get_async_enrollment_info()` should have been called
                // prior to us.
                return Err(SubmitterFinalizeAsyncEnrollmentError::EnrollmentNotFoundOnServer);
            }
            bad_rep @ Rep::UnknownStatus { .. } => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
            }
        }
    };

    // 2) Ensure the enrollment has been accepted

    let (accept_payload, accept_payload_signature) = {
        use libparsec_protocol::anonymous_cmds::v5::async_enrollment_info::InfoStatus;
        match status {
            InfoStatus::Accepted {
                accept_payload,
                accept_payload_signature,
                ..
            } => (accept_payload, accept_payload_signature),
            _ => return Err(SubmitterFinalizeAsyncEnrollmentError::NotAccepted),
        }
    };

    // 3) Validate the accept payload

    let accept_payload = identity_strategy
        .verify_accept_payload(accept_payload, accept_payload_signature)
        .await?;

    // All good, now we know we are officially part of the organization !

    let organization_addr = ParsecOrganizationAddr::new(
        cleartext_content.server_url,
        cleartext_content.organization_id,
        accept_payload.root_verify_key,
    );

    let new_local_device = Arc::new(LocalDevice::generate_new_device(
        organization_addr,
        accept_payload.profile,
        accept_payload.human_handle,
        accept_payload.device_label,
        Some(accept_payload.user_id),
        Some(accept_payload.device_id),
        Some(to_become_device_signing_key),
        Some(to_become_user_private_key),
        None,
        None,
        None,
    ));

    // 4) Remove the pending async enrollment file and save the final local device file \o/

    let new_device_key_file = libparsec_platform_device_loader::get_default_key_file(
        &config.config_dir,
        new_local_device.device_id,
    );

    // Note we destroy the pending file, then save the final local device file.
    //
    // This means there is a small chance that we lost the the new device if an
    // unexpected crash occurs between the two operations.
    // This is considered okay since it is very unlikely, and if it occurs the admin
    // can revoke the newly enrolled user and start again.
    //
    // Note that saving the device file first is more hazardous since, if an unexpected
    // crash leads us to retry the operation, we could end up overwriting the device
    // file. Which means generating new `local_symkey` and `user_realm_id/user_realm_key`
    // that may clash with any existing data in the local database...
    libparsec_platform_device_loader::remove_pending_async_enrollment(
        &config.config_dir,
        enrollment_file,
    )
    .await
    .or_else(|err| match err {
        RemoveDeviceError::StorageNotAvailable => {
            Err(SubmitterFinalizeAsyncEnrollmentError::NoSpaceAvailable)
        }
        // Not found is unexpected... but we can continue nevertheless
        RemoveDeviceError::NotFound => Ok(()),
        RemoveDeviceError::Internal(err) => {
            Err(SubmitterFinalizeAsyncEnrollmentError::Internal(err))
        }
    })?;

    let available_device = libparsec_platform_device_loader::save_device(
        &config.config_dir,
        new_device_save_strategy,
        &new_local_device,
        new_device_key_file,
    )
    .await
    .map_err(|err| match err {
        SaveDeviceError::NoSpaceAvailable => {
            SubmitterFinalizeAsyncEnrollmentError::NoSpaceAvailable
        }
        SaveDeviceError::InvalidPath => {
            SubmitterFinalizeAsyncEnrollmentError::SaveDeviceInvalidPath(anyhow::anyhow!(
                "invalid path while saving device"
            ))
        }
        SaveDeviceError::RemoteOpaqueKeyUploadOffline { server, error } => {
            SubmitterFinalizeAsyncEnrollmentError::SaveDeviceRemoteOpaqueKeyUploadOffline {
                server,
                error,
            }
        }
        SaveDeviceError::RemoteOpaqueKeyUploadFailed { server, error } => {
            SubmitterFinalizeAsyncEnrollmentError::SaveDeviceRemoteOpaqueKeyUploadFailed {
                server,
                error,
            }
        }
        SaveDeviceError::Internal(err) => SubmitterFinalizeAsyncEnrollmentError::Internal(err),
    })?;

    Ok(available_device)
}
