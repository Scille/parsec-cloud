// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod async_enrollment;
mod certif;
mod manifest;
mod pki;

pub(crate) use async_enrollment::*;
pub(crate) use certif::*;
pub(crate) use manifest::*;
pub(crate) use pki::*;

use pyo3::{
    prelude::Python,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction, Bound, PyResult,
};

pub(crate) fn add_mod(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Certif
    m.add_class::<PrivateKeyAlgorithm>()?;
    m.add_class::<UserCertificate>()?;
    m.add_class::<RevokedUserCertificate>()?;
    m.add_class::<UserUpdateCertificate>()?;
    m.add_class::<SigningKeyAlgorithm>()?;
    m.add_class::<DeviceCertificate>()?;
    m.add_class::<RealmRoleCertificate>()?;
    m.add_class::<RealmNameCertificate>()?;
    m.add_class::<RealmArchivingCertificate>()?;
    m.add_class::<SecretKeyAlgorithm>()?;
    m.add_class::<HashAlgorithm>()?;
    m.add_class::<RealmKeyRotationCertificate>()?;
    m.add_class::<SequesterAuthorityCertificate>()?;
    m.add_class::<SequesterServiceCertificate>()?;
    m.add_class::<SequesterRevokedServiceCertificate>()?;
    m.add_class::<ShamirRecoveryBriefCertificate>()?;
    m.add_class::<ShamirRecoveryShareCertificate>()?;
    m.add_class::<ShamirRecoveryDeletionCertificate>()?;

    // Manifest
    m.add_class::<EntryName>()?;
    m.add_class::<BlockAccess>()?;
    m.add_class::<FolderManifest>()?;
    m.add_class::<FileManifest>()?;
    m.add_class::<UserManifest>()?;
    m.add_function(wrap_pyfunction!(child_manifest_decrypt_verify_and_load, m)?)?;
    m.add_function(wrap_pyfunction!(child_manifest_verify_and_load, m)?)?;

    // Pki
    m.add_class::<PkiEnrollmentAnswerPayload>()?;
    m.add_class::<PkiEnrollmentSubmitPayload>()?;
    m.add_class::<PkiSignatureAlgorithm>()?;
    m.add_class::<X509Certificate>()?;
    m.add_class::<X509CertificateInformation>()?;
    m.add_class::<TrustAnchor>()?;
    m.add_class::<SignedMessage>()?;
    m.add_function(wrap_pyfunction!(load_submit_payload, m)?)?;
    crate::binding_utils::export_exception!(m, py, PkiInvalidSignature);
    crate::binding_utils::export_exception!(m, py, PkiInvalidCertificateDER);
    crate::binding_utils::export_exception!(m, py, PkiUntrusted);

    // Async enrollment
    m.add_class::<AsyncEnrollmentSubmitPayload>()?;
    m.add_class::<AsyncEnrollmentAcceptPayload>()?;

    Ok(())
}
