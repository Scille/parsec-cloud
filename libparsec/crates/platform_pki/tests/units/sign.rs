// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::utils::{certificates, InstalledCertificates};
use crate::{SignMessageError, VerifyMessageError, X509CertificateHash, X509CertificateReference};
use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

#[rstest]
fn sign_and_verify(certificates: &InstalledCertificates) {
    // Alice key is 2048 bits (i.e. 256 bytes), so we check that the payload can be larger than the key.
    let payload = [b'x'; 257];
    let certificate_ref = certificates.alice_cert_ref();
    let (algo, signature) = crate::sign_message(payload.as_ref(), &certificate_ref).unwrap();

    let now = DateTime::now();
    let certificate = crate::get_der_encoded_certificate(&certificate_ref).unwrap();
    let validation_path = crate::get_validation_path_for_cert(&certificate_ref, now).unwrap();

    crate::verify_message2(
        payload.as_ref(),
        &signature,
        algo,
        &certificate,
        validation_path
            .intermediate_certs
            .iter()
            .map(|c| c.as_ref()),
        &[validation_path.root],
        now,
    )
    .unwrap();
}

#[rstest]
fn verify(certificates: &InstalledCertificates) {
    let payload = b"The cake is a lie!";
    let algo = PkiSignatureAlgorithm::RsassaPssSha256;
    // Pre-generated signature to ensure test stability
    let signature = hex!(
        "8cf5edab0c6839f53cf155e4ad55393c4648e13c1cf696a2192ba0be7bcea2a4434de8"
        "be846670b827b4d709b982a7a645d6a3d537336a8dc2748415ded7e9842dee46c1981a"
        "d6096b610c0453fefe19ffea1141fb8784a800deb58a63b6d8b480f4bd0c23298846db"
        "c9581fd6549bcf03b4d7a24c730cd6d6e5c48a698b1cf5b5e9e7d8d37024a3f0780a98"
        "bd75a4913e3e0313f8a34e5d661b8e2bd58044c656a3f9490a42e726ce9dc0fdd5ef4f"
        "d815ff2fc34a453f0eaee0a404e5253e5996affa94beb6a7282388034288637653fa7b"
        "57e30dacc34482b59ee9fe1c7279c58510ae1d72866dbbe4c4c71541d148a1a1350834"
        "50a46299802826ac298262"
    );

    let certificate_ref = certificates.alice_cert_ref();
    let now: DateTime = "2026-02-01T00:00:00Z".parse().unwrap();
    let certificate = crate::get_der_encoded_certificate(&certificate_ref).unwrap();
    let validation_path = crate::get_validation_path_for_cert(&certificate_ref, now).unwrap();

    crate::verify_message2(
        payload.as_ref(),
        &signature,
        algo,
        &certificate,
        validation_path
            .intermediate_certs
            .iter()
            .map(|c| c.as_ref()),
        &[validation_path.root],
        now,
    )
    .unwrap();
}

#[rstest]
fn sign_ko_certificate_without_signing_key(certificates: &InstalledCertificates) {
    let payload = b"The cake is a lie!";
    let certificate_ref = certificates.mallory_encrypt_cert_ref();
    p_assert_matches!(
        crate::sign_message(payload.as_ref(), &certificate_ref),
        Err(SignMessageError::CannotSign(_))
    );
}

#[rstest]
fn sign_ko_cannot_use_root_certificate(certificates: &InstalledCertificates) {
    let payload = b"The cake is a lie!";
    let certificate_ref = certificates.black_mesa_cert_ref();
    p_assert_matches!(
        crate::sign_message(payload.as_ref(), &certificate_ref),
        Err(SignMessageError::NotFound)
    );
}

#[test]
fn sign_ko_not_found() {
    let payload = b"The cake is a lie!";
    let dummy_certificate_ref: X509CertificateReference = X509CertificateHash::fake_sha256().into();
    p_assert_matches!(
        crate::sign_message(payload.as_ref(), &dummy_certificate_ref),
        Err(SignMessageError::NotFound)
    );
}

#[rstest]
fn verify_message_ko_outdated_certificate(certificates: &InstalledCertificates) {
    let payload = b"The cake is a lie!";
    let (algo, signature, certificate, validation_path) = certificates.alice_sign_message(payload);

    p_assert_matches!(
        crate::verify_message2(
            payload.as_ref(),
            &signature,
            algo,
            &certificate,
            validation_path
                .intermediate_certs
                .iter()
                .map(|c| c.as_ref()),
            &[validation_path.root],
            "9999-01-01T00:00:00Z".parse().unwrap(), // Certificate is outdated at this date
        ),
        Err(VerifyMessageError::X509CertificateUntrusted(
            webpki::Error::CertExpired { .. }
        ))
    );
}

#[rstest]
fn verify_message_ko_different_certificate(certificates: &InstalledCertificates) {
    let payload = b"The cake is a lie!";
    let (algo, signature, _, validation_path) = certificates.alice_sign_message(payload);
    let certificate = crate::get_der_encoded_certificate(&certificates.bob_cert_ref()).unwrap();

    p_assert_matches!(
        crate::verify_message2(
            payload.as_ref(),
            &signature,
            algo,
            &certificate,
            validation_path
                .intermediate_certs
                .iter()
                .map(|c| c.as_ref()),
            &[validation_path.root],
            DateTime::now(),
        ),
        Err(VerifyMessageError::InvalidSignature(
            webpki::Error::InvalidSignatureForPublicKey
        ))
    );
}

#[rstest]
fn verify_message_ko_different_payload(certificates: &InstalledCertificates) {
    let payload = b"The cake is a lie!";
    let (algo, signature, certificate, validation_path) = certificates.alice_sign_message(payload);

    p_assert_matches!(
        crate::verify_message2(
            b"Different payload",
            &signature,
            algo,
            &certificate,
            validation_path
                .intermediate_certs
                .iter()
                .map(|c| c.as_ref()),
            &[validation_path.root],
            DateTime::now(),
        ),
        Err(VerifyMessageError::InvalidSignature(
            webpki::Error::InvalidSignatureForPublicKey
        ))
    );
}
