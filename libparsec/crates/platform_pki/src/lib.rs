// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub mod errors;
#[cfg(not(target_os = "windows"))]
mod pkcs11;
mod shared;
#[cfg(any(test, feature = "test-fixture"))]
pub mod test_fixture;
#[cfg(target_os = "windows")]
mod windows;
pub mod x509;
#[cfg(any(test, feature = "test-fixture"))]
pub use test_fixture::*;

#[cfg(test)]
#[path = "../tests/units/mod.rs"]
mod test;

use libparsec_types::prelude::*;

#[cfg(target_os = "windows")]
pub(crate) use windows as platform;

#[cfg(not(target_os = "windows"))]
pub(crate) use pkcs11 as platform;

// TODO: https://github.com/Scille/parsec-cloud/issues/11215
// This is specific to windows, it cannot be replicated on other platform.
// Instead, we likely need to go the manual way and show a custom dialog on the client side with a
// list of certificate that we retrieve from the platform certstore.
pub use errors::ShowCertificateSelectionDialogError;
pub use platform::show_certificate_selection_dialog_windows_only;

pub use errors::GetDerEncodedCertificateError;
pub use platform::get_der_encoded_certificate;

pub use errors::ListTrustedRootCertificatesError;
pub use platform::list_trusted_root_certificate_anchors;

pub use errors::ListIntermediateCertificatesError;
pub use platform::list_intermediate_certificates;

pub use errors::SignMessageError;
pub use platform::sign_message;

pub use shared::{verify_message, Certificate, SignedMessage, X509EndCertificate};

pub use errors::EncryptMessageError;
pub use platform::encrypt_message;

pub use errors::DecryptMessageError;
pub use platform::decrypt_message;

pub use platform::is_available;

pub use shared::verify_certificate;
pub use webpki::KeyUsage;

pub use errors::VerifyMessageError;
pub use shared::verify_message2;

pub use errors::GetValidationPathForCertError;
pub use shared::{get_validation_path_for_cert, ValidationPathOwned};

pub use shared::get_root_certificate_info_from_trustchain;
