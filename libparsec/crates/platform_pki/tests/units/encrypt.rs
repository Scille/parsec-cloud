// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::utils::{certificates, InstalledCertificates};
use crate::{
    DecryptMessageError, EncryptMessageError, X509CertificateHash, X509CertificateReference,
};
use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

#[rstest]
fn encrypt_decrypt(certificates: &InstalledCertificates) {
    // Alice key is 2048 bits (i.e. 256 bytes), so we use the maximum allowed payload size here.
    let payload = [b'x'; 256 - 66]; // 66 bytes is the OAEP SHA-256 overhead
    let certificate_ref = certificates.alice_cert_ref();
    let (algo, encrypted_message) =
        crate::encrypt_message(payload.as_ref(), &certificate_ref).unwrap();

    let decrypted_message =
        crate::decrypt_message(algo, &encrypted_message, &certificate_ref).unwrap();
    assert_eq!(*decrypted_message, payload);
}

#[rstest]
fn decrypt(certificates: &InstalledCertificates) {
    let payload = b"The cake is a lie!";
    let algo = PKIEncryptionAlgorithm::RsaesOaepSha256;
    // Pre-generated encrypted message to ensure test stability
    let encrypted_message = hex!(
        "067238d82df4b8b38417927960cad91f4959d42984484c611fb07ff6e16c3f823480d8"
        "5b6f55a93e6bc265d0a0405f4cb5cf6599e385ce676b466921fa513e4cf025b4b3f467"
        "a2d7d8ccf6441c2491b2a57d6147ff7e3c2ead4c96370096d8f2d822031407f38ab354"
        "537a7d21a4e9e99378226b8fcca726ef028d6e396ba30071207b41664c1bdeaf6a97bf"
        "1f491ddbf2a63ede370f7781e702ec1afd1497090216f5fd2e738a28b880c3a7f49279"
        "080ebe6d0f905cfc0c7823b55562b6c92ac0c10314408e1c799b0cd2c936764a9990b5"
        "063df66b4e70db2f2a0d4ba4c4109f131676cf11da7925a50db9d77745e5a4f22e9e5b"
        "6e0254fcb7c210e0a8fb32"
    );
    let certificate_ref = certificates.alice_cert_ref();
    let decrypted_message =
        crate::decrypt_message(algo, &encrypted_message, &certificate_ref).unwrap();
    assert_eq!(*decrypted_message, *payload);
}

#[test]
fn encrypt_ko_not_found() {
    let payload = b"The cake is a lie!";
    let dummy_certificate_ref: X509CertificateReference = X509CertificateHash::fake_sha256().into();
    p_assert_matches!(
        crate::encrypt_message(payload.as_ref(), &dummy_certificate_ref),
        Err(EncryptMessageError::NotFound)
    );
}

#[rstest]
fn encrypt_ko_certificate_without_encrypting_key(certificates: &InstalledCertificates) {
    let payload = b"The cake is a lie!";
    let certificate_ref = certificates.mallory_sign_cert_ref();
    p_assert_matches!(
        crate::encrypt_message(payload.as_ref(), &certificate_ref),
        Err(EncryptMessageError::CannotEncrypt(_))
    );
}

#[rstest]
fn encrypt_ko_cannot_use_root_certificate(certificates: &InstalledCertificates) {
    let payload = b"The cake is a lie!";
    let certificate_ref = certificates.black_mesa_cert_ref();
    p_assert_matches!(
        crate::encrypt_message(payload.as_ref(), &certificate_ref),
        Err(EncryptMessageError::NotFound)
    );
}

#[rstest]
fn encrypt_payload_too_big(certificates: &InstalledCertificates) {
    // Alice key is 2048 bits (i.e. 256 bytes), and 66 bytes is the OAEP SHA-256 overhead
    let payload = [b'x'; 256 - 66 + 1];
    let certificate_ref = certificates.alice_cert_ref();
    p_assert_matches!(
        crate::encrypt_message(payload.as_ref(), &certificate_ref),
        Err(EncryptMessageError::CannotEncrypt(_))
    );
}

#[rstest]
fn decrypt_ko_not_found(certificates: &InstalledCertificates) {
    let payload = b"The cake is a lie!";
    let (algo, encrypted_message, _) = certificates.alice_encrypt_message(payload);

    let dummy_certificate_ref = X509CertificateReference::from(
        "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="
            .parse::<X509CertificateHash>()
            .unwrap(),
    );
    p_assert_matches!(
        crate::decrypt_message(algo, &encrypted_message, &dummy_certificate_ref),
        Err(DecryptMessageError::NotFound)
    );
}

#[rstest]
fn decrypt_ko_cannot_decrypt(certificates: &InstalledCertificates) {
    let payload = b"The cake is a lie!";
    let (algo, _, certificate_ref) = certificates.alice_encrypt_message(payload);

    p_assert_matches!(
        crate::decrypt_message(algo, b"<dummy_message>", &certificate_ref),
        Err(DecryptMessageError::CannotDecrypt(_))
    );
}
