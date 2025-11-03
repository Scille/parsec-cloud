// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_testbed::TestbedEnv;
use libparsec_tests_fixtures::prelude::*;
use libparsec_tests_lite::parsec_test;
use libparsec_types::{
    Bytes, EnrollmentID, LocalPendingEnrollment, ParsecPkiEnrollmentAddr,
    PkiEnrollmentSubmitPayload, X509CertificateHash,
};

use crate::save_pki_local_pending;

#[parsec_test(testbed = "minimal")]
async fn ok_simple(tmp_path: TmpPath, env: &TestbedEnv) {
    let path = tmp_path.join("local_pki_enrol.keys");

    let alice_device = env.local_device("alice@dev1");
    let pki_addr = ParsecPkiEnrollmentAddr::new(
        alice_device.organization_addr.clone(),
        alice_device.organization_id().clone(),
    );
    let local_pending = LocalPendingEnrollment {
        cert_ref: X509CertificateHash::fake_sha256().into(),
        addr: pki_addr,
        submitted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
        enrollment_id: EnrollmentID::default(),
        payload: PkiEnrollmentSubmitPayload {
            // We reuse `alice_device` attribute for simplicity sake.
            // IRL, those values are RNG and provided by the user.
            verify_key: alice_device.signing_key.verify_key(),
            public_key: alice_device.private_key.public_key(),
            device_label: alice_device.device_label.clone(),
            human_handle: alice_device.human_handle.clone(),
        },
        encrypted_key: Bytes::from_static(b"encrypted key"),
        encrypted_key_algo: libparsec_types::EncryptionAlgorithm::RsaesOaepSha256,
        ciphertext: Bytes::from_static(b"encrypted secret part"),
    };
    save_pki_local_pending(local_pending, path).await.unwrap();
}
