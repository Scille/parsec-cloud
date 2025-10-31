// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub use libparsec_platform_pki::ShowCertificateSelectionDialogError;
use libparsec_types::X509CertificateReference;

pub async fn show_certificate_selection_dialog_windows_only(
) -> Result<Option<X509CertificateReference>, ShowCertificateSelectionDialogError> {
    libparsec_platform_pki::show_certificate_selection_dialog_windows_only()
}

pub async fn is_pki_available() -> bool {
    libparsec_platform_pki::is_available()
}
