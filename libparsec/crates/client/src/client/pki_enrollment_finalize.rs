// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::{Path, PathBuf};

use libparsec_platform_device_loader::{AvailableDevice, DeviceSaveStrategy};
use libparsec_platform_pki::x509::X509CertificateInformation;
use libparsec_types::{
    anyhow::{self, Context},
    LocalDevice, PKILocalPendingEnrollment, PkiEnrollmentAnswerPayload, PrivateParts, SecretKey,
};

#[derive(Debug, thiserror::Error)]
pub enum PkiEnrollmentFinalizeError {
    #[error("Failed to save device: {0}")]
    SaveError(#[from] libparsec_platform_device_loader::SaveDeviceError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

/// Generates and saves the new device
pub async fn finalize(
    config_dir: &Path,
    key_file: PathBuf,
    strategy: &DeviceSaveStrategy,
    accepted: PkiEnrollmentAnswerPayload,
    local_pending: PKILocalPendingEnrollment,
) -> Result<AvailableDevice, PkiEnrollmentFinalizeError> {
    let PKILocalPendingEnrollment {
        cert_ref,
        addr,
        encrypted_key,
        encrypted_key_algo,
        ciphertext,
        ..
    } = local_pending;

    let key =
        libparsec_platform_pki::decrypt_message(encrypted_key_algo, &encrypted_key, &cert_ref)
            .await
            .context("Cannot decrypt key")
            .and_then(|raw| SecretKey::try_from(raw.as_ref()).context("Invalid key"))?;
    let human_handle = libparsec_platform_pki::get_der_encoded_certificate(&cert_ref)
        .await
        .context("Cannot get certificate content")
        .and_then(|der| {
            X509CertificateInformation::load_der(&der)
                .context("Cannot load user information from certificate")
        })
        .and_then(|info| {
            info.human_handle()
                .context("Missing human handle from certificate information")
        })?;

    let raw_private_parts = key
        .decrypt(&ciphertext)
        .context("Cannot decrypt local pending")?;
    let PrivateParts {
        private_key,
        signing_key,
    } = PrivateParts::load(&raw_private_parts).context("Cannot load local pending")?;

    let PkiEnrollmentAnswerPayload {
        user_id,
        device_id,
        device_label,
        profile,
        root_verify_key,
    } = accepted;
    let organization_addr = addr.generate_organization_addr(root_verify_key);
    let time_provider = None;
    let user_realm_id = None;
    let user_realm_key = None;

    let device = LocalDevice::generate_new_device(
        organization_addr,
        profile,
        human_handle,
        device_label,
        Some(user_id),
        Some(device_id),
        Some(signing_key),
        Some(private_key),
        time_provider,
        user_realm_id,
        user_realm_key,
    );

    let available_device =
        libparsec_platform_device_loader::save_device(config_dir, strategy, &device, key_file)
            .await?;

    let local_pending_path = libparsec_platform_device_loader::get_default_local_pending_file(
        config_dir,
        local_pending.enrollment_id,
    );
    // Remove local pending part as best effort.
    drop(
        libparsec_platform_device_loader::remove_device(config_dir, &local_pending_path)
            .await
            .inspect_err(|err| log::warn!("Failed to remove pki local pending file: {err}")),
    );

    Ok(available_device)
}
