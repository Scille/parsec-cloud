// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod list;
mod remove;
mod save;

use std::sync::Arc;

use libparsec_types::{
    Bytes, LocalDevice, PKIEnrollmentID, PKILocalPendingEnrollment, ParsecPkiEnrollmentAddr,
    PkiEnrollmentSubmitPayload, X509CertificateHash,
};

pub fn create_pki_local_pending_from_device(device: Arc<LocalDevice>) -> PKILocalPendingEnrollment {
    let pki_addr = ParsecPkiEnrollmentAddr::new(
        device.organization_addr.clone(),
        device.organization_id().clone(),
    );
    PKILocalPendingEnrollment {
        cert_ref: X509CertificateHash::fake_sha256().into(),
        addr: pki_addr,
        submitted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
        enrollment_id: PKIEnrollmentID::default(),
        payload: PkiEnrollmentSubmitPayload {
            // We reuse `device` attribute for simplicity sake.
            // IRL, those values are RNG and provided by the user.
            verify_key: device.signing_key.verify_key(),
            public_key: device.private_key.public_key(),
            device_label: device.device_label.clone(),
        },
        encrypted_key: Bytes::from_static(b"encrypted key"),
        encrypted_key_algo: libparsec_types::PKIEncryptionAlgorithm::RsaesOaepSha256,
        ciphertext: Bytes::from_static(b"encrypted secret part"),
    }
}
