// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod certif;
mod manifest;

pub(crate) use certif::*;
pub(crate) use manifest::*;

use pyo3::{types::PyModule, wrap_pyfunction, PyResult};

pub(crate) fn add_mod(m: &PyModule) -> PyResult<()> {
    // Certif
    m.add_class::<UserCertificate>()?;
    m.add_class::<RevokedUserCertificate>()?;
    m.add_class::<UserUpdateCertificate>()?;
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

    // Manifest
    m.add_class::<EntryName>()?;
    m.add_class::<BlockAccess>()?;
    m.add_class::<FolderManifest>()?;
    m.add_class::<FileManifest>()?;
    m.add_class::<WorkspaceManifest>()?;
    m.add_class::<UserManifest>()?;
    m.add_function(wrap_pyfunction!(child_manifest_decrypt_verify_and_load, m)?)?;
    m.add_function(wrap_pyfunction!(child_manifest_verify_and_load, m)?)?;

    Ok(())
}
