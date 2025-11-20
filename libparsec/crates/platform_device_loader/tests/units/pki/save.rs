// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_testbed::TestbedEnv;
use libparsec_tests_fixtures::prelude::*;
use libparsec_tests_lite::parsec_test;
use libparsec_types::{
    Bytes, LocalDevice, PKIEnrollmentID, PKILocalPendingEnrollment, ParsecPkiEnrollmentAddr,
    PkiEnrollmentSubmitPayload, X509CertificateHash,
};

use crate::save_pki_local_pending;

#[parsec_test(testbed = "minimal")]
async fn ok_simple(tmp_path: TmpPath, env: &TestbedEnv) {
    let path = tmp_path.join("local_pki_enrol.keys");

    let alice_device = env.local_device("alice@dev1");
    let local_pending = create_pki_local_pending_from_device(alice_device);
    save_pki_local_pending(local_pending, path).await.unwrap();
}

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
            human_handle: device.human_handle.clone(),
        },
        encrypted_key: Bytes::from_static(b"encrypted key"),
        encrypted_key_algo: libparsec_types::PKIEncryptionAlgorithm::RsaesOaepSha256,
        ciphertext: Bytes::from_static(b"encrypted secret part"),
    }
}
