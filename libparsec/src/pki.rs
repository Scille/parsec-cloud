// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client::{AvailableDevice, PKIInfoItem, PkiEnrollmentInfoError};
pub use libparsec_client::{PkiEnrollmentFinalizeError, PkiEnrollmentSubmitError};
pub use libparsec_platform_pki::ShowCertificateSelectionDialogError;
use libparsec_types::{
    DateTime, DeviceLabel, HumanHandle, PKIEnrollmentID, PKILocalPendingEnrollment,
    ParsecPkiEnrollmentAddr, PkiEnrollmentAnswerPayload, X509CertificateReference,
};

use crate::{ClientConfig, DeviceSaveStrategy};

pub async fn show_certificate_selection_dialog_windows_only(
) -> Result<Option<X509CertificateReference>, ShowCertificateSelectionDialogError> {
    libparsec_platform_pki::show_certificate_selection_dialog_windows_only()
}

pub async fn is_pki_available() -> bool {
    libparsec_platform_pki::is_available()
}

pub async fn pki_enrollment_submit(
    config: ClientConfig,
    addr: ParsecPkiEnrollmentAddr,
    cert_ref: X509CertificateReference,
    human_handle: HumanHandle,
    device_label: DeviceLabel,
    force: bool,
) -> Result<DateTime, PkiEnrollmentSubmitError> {
    let config: Arc<libparsec_client::ClientConfig> = config.into();

    libparsec_client::pki_enrollment_submit(
        config,
        addr,
        cert_ref,
        human_handle,
        device_label,
        force,
    )
    .await
}

pub async fn pki_enrollment_finalize(
    config: ClientConfig,
    save_strategy: DeviceSaveStrategy,
    accepted: PkiEnrollmentAnswerPayload,
    local_pending: PKILocalPendingEnrollment,
) -> Result<AvailableDevice, PkiEnrollmentFinalizeError> {
    let strategy = save_strategy
        .convert_with_side_effects()
        .map_err(PkiEnrollmentFinalizeError::Internal)?;
    let config: Arc<libparsec_client::ClientConfig> = config.into();

    let config_dir = &config.config_dir;
    let key_file =
        libparsec_platform_device_loader::get_default_key_file(config_dir, accepted.device_id);
    libparsec_client::pki_enrollment_finalize(
        config_dir,
        key_file,
        &strategy,
        accepted,
        local_pending,
    )
    .await
}

pub async fn pki_enrollment_info(
    config: ClientConfig,
    addr: ParsecPkiEnrollmentAddr,
    enrollment_id: PKIEnrollmentID,
) -> Result<PKIInfoItem, PkiEnrollmentInfoError> {
    libparsec_client::pki_enrollment_info(config.into(), addr, enrollment_id).await
}
