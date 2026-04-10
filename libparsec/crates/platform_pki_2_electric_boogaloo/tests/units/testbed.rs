// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::certificates;
use crate::{encrypt_message, verify_message, PkiSystem, PkiSystemOpenCertificateError};

async fn init_pki(env: &TestbedEnv) -> PkiSystem {
    // We initialize the testbed mocked version of the PKI system, hence we never
    // need to provide a SCWS configuration even if the test is running on web.
    PkiSystem::init(&env.discriminant_dir, None).await.unwrap()
}

#[parsec_test(testbed = "minimal")]
async fn testbed_open_certificate(env: &TestbedEnv) {
    let pki = init_pki(env).await;

    let alice_ref = certificates().alice_cert_ref();
    p_assert_matches!(pki.open_certificate(&alice_ref).await, Ok(_));

    // Non-existent certificate
    let dummy_ref = crate::X509CertificateReference::from(
        "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="
            .parse::<crate::X509CertificateHash>()
            .unwrap(),
    );
    p_assert_matches!(
        pki.open_certificate(&dummy_ref).await,
        Err(PkiSystemOpenCertificateError::NotFound)
    );
}

#[parsec_test(testbed = "minimal")]
async fn testbed_list_user_certificates(env: &TestbedEnv) {
    let pki = init_pki(env).await;
    let certs = pki.list_user_certificates().await.unwrap();
    // We have alice, bob, mallory-sign, mallory-encrypt
    assert_eq!(certs.len(), 4);
}

#[parsec_test(testbed = "minimal")]
async fn testbed_encrypt_decrypt(env: &TestbedEnv) {
    let pki = init_pki(env).await;

    let alice_ref = certificates().alice_cert_ref();
    let cert = pki.open_certificate(&alice_ref).await.unwrap();

    let payload = b"The cake is a lie!";
    let (algo, encrypted) = encrypt_message(cert.get_der().await.unwrap(), payload.as_ref())
        .await
        .unwrap();
    assert_eq!(algo, PKIEncryptionAlgorithm::RsaesOaepSha256);

    let key = cert.request_private_key().await.unwrap();
    let decrypted = key.decrypt(algo, &encrypted).await.unwrap();
    assert_eq!(*decrypted, *payload);
}

#[parsec_test(testbed = "minimal")]
async fn testbed_sign_verify(env: &TestbedEnv) {
    let pki = init_pki(env).await;

    let alice_ref = certificates().alice_cert_ref();
    let cert = pki.open_certificate(&alice_ref).await.unwrap();

    // Alice key is 2048 bits, check payload can be larger than the key
    let payload = [b'x'; 257];

    let key = cert.request_private_key().await.unwrap();
    let (algo, signature) = key.sign(payload.as_ref()).await.unwrap();
    assert_eq!(algo, PkiSignatureAlgorithm::RsassaPssSha256);

    // Verify the signature using the certificate

    let validation_path = cert.get_validation_path().await.unwrap();
    verify_message(
        payload.as_ref(),
        &signature,
        algo,
        validation_path.leaf.as_ref(),
        validation_path.intermediates.iter().map(|x| x.as_ref()),
        &[validation_path.root],
        "2026-06-01T00:00:00Z".parse().unwrap(),
    )
    .unwrap();
}

#[parsec_test(testbed = "minimal")]
async fn testbed_get_validation_path(env: &TestbedEnv) {
    let pki = init_pki(env).await;

    let alice_ref = certificates().alice_cert_ref();
    let cert = pki.open_certificate(&alice_ref).await.unwrap();

    let path = cert.get_validation_path().await.unwrap();
    // Alice's cert is directly signed by black_mesa root, so no intermediates
    assert!(path.intermediates.is_empty());
    assert_eq!(path.leaf, cert.get_der().await.unwrap());
}

#[parsec_test(testbed = "minimal")]
async fn testbed_to_reference(env: &TestbedEnv) {
    let pki = init_pki(env).await;

    let alice_ref = certificates().alice_cert_ref();
    let cert = pki.open_certificate(&alice_ref).await.unwrap();
    let computed_ref = cert.to_reference().await.unwrap();
    assert_eq!(computed_ref.hash, alice_ref.hash);
}
