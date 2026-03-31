// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_pki::PkiConfig;
pub use libparsec_platform_pki::{
    CertificateDetails, CertificateWithDetails, DistinguishedNameValue, InvalidCertificateReason,
    ListUserCertificatesError, ShowCertificateSelectionDialogError,
};
use libparsec_types::X509CertificateReference;

pub async fn show_certificate_selection_dialog_windows_only(
) -> Result<Option<X509CertificateReference>, ShowCertificateSelectionDialogError> {
    libparsec_platform_pki::show_certificate_selection_dialog_windows_only()
}

pub async fn is_pki_available() -> bool {
    libparsec_platform_pki::PkiSystem::init(PkiConfig::new(&0))
        .await
        .is_ok()
}

pub async fn list_user_certificates_with_details(
) -> Result<Vec<CertificateWithDetails>, ListUserCertificatesError> {
    libparsec_platform_pki::list_user_certificates_with_details().await
}
