// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

pub use libparsec_client::PkiEnrollmentSubmitError;
pub use libparsec_platform_pki::ShowCertificateSelectionDialogError;
use libparsec_types::{
    DateTime, DeviceLabel, HumanHandle, ParsecPkiEnrollmentAddr, X509CertificateReference,
};

use crate::ClientConfig;

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
