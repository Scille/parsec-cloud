// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#[cfg(all(target_os = "windows", not(feature = "force-scws-on-native")))]
#[path = "windows/mod.rs"]
pub(crate) mod platform;
#[cfg(all(target_family = "unix", not(feature = "force-scws-on-native")))]
#[path = "unix/mod.rs"]
pub(crate) mod platform;
#[cfg(any(target_arch = "wasm32", feature = "force-scws-on-native"))]
#[path = "scws/mod.rs"]
pub(crate) mod platform;

mod shared;
#[cfg(feature = "test-with-testbed")]
pub(crate) mod testbed;
pub mod x509;

mod pki_certificate;
mod pki_private_key;
mod pki_system;

pub use pki_certificate::*;
pub use pki_private_key::*;
pub use pki_system::*;

pub use shared::*;

pub type X509CertificateDer<'a> = rustls_pki_types::CertificateDer<'a>;
pub type X509TrustAnchor<'a> = rustls_pki_types::TrustAnchor<'a>;
pub use libparsec_types::{
    PKIEncryptionAlgorithm, PkiSignatureAlgorithm, X509CertificateHash, X509CertificateReference,
};

#[cfg(test)]
#[path = "../tests/units/mod.rs"]
mod test;

#[derive(Debug, thiserror::Error)]
pub enum ShowCertificateSelectionDialogError {
    #[error("Cannot open certificate store: {0}")]
    CannotOpenStore(std::io::Error),
    #[error("Cannot get certificate info: {0}")]
    CannotGetCertificateInfo(std::io::Error),
}

// TODO: https://github.com/Scille/parsec-cloud/issues/11215
// This is specific to windows, it cannot be replicated on other platform.
// Instead, we likely need to go the manual way and show a custom dialog on the client side with a
// list of certificate that we retrieve from the platform certstore.
pub fn show_certificate_selection_dialog_windows_only(
) -> Result<Option<X509CertificateReference>, ShowCertificateSelectionDialogError> {
    #[cfg(all(target_os = "windows", not(feature = "force-scws-on-native")))]
    {
        platform::show_certificate_selection_dialog_windows_only()
    }
    #[cfg(any(not(target_os = "windows"), feature = "force-scws-on-native"))]
    {
        unimplemented!("platform not supported")
    }
}

pub use webpki::KeyUsage;
pub use x509::DistinguishedNameValue;
