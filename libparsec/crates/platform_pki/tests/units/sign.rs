// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::utils::{certificates, initialize_pki_system, InstalledCertificates};
use crate::VerifyMessageError;

#[cfg(target_os = "windows")]
#[parsec_test]
async fn sign_and_verify(certificates: &InstalledCertificates) {
    // Alice key is 2048 bits (i.e. 256 bytes), so we check that the payload can be larger than the key.
    let payload = [b'x'; 257];

    let pki = initialize_pki_system().await;
    let cert_ref = certificates.alice_cert_ref();
    let cert = pki.find_certificate(&cert_ref).await.unwrap().unwrap();
    let key = cert.request_private_key().await.unwrap();
    let (algo, signature) = key.sign(payload.as_ref()).await.unwrap();

    let now = DateTime::now();
    crate::verify_message(
        payload.as_ref(),
        &signature,
        algo,
        &certificates.alice_der_cert(),
        [].into_iter(),
        &[certificates.black_mesa_trust_anchor()],
        now,
    )
    .unwrap();
}

#[parsec_test]
async fn verify(certificates: &InstalledCertificates) {
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

    let now: DateTime = "2026-02-01T00:00:00Z".parse().unwrap();

    crate::verify_message(
        payload.as_ref(),
        &signature,
        algo,
        &certificates.alice_der_cert(),
        [].into_iter(),
        &[certificates.black_mesa_trust_anchor()],
        now,
    )
    .unwrap();
}

// TODO: Support `KeyUsage` field in X509 certificate
//       see https://github.com/Scille/parsec-cloud/issues/12087
// #[parsec_test]
// async fn sign_ko_certificate_without_signing_key(certificates: &InstalledCertificates) {
//     let payload = b"The cake is a lie!";
//     let certificate_ref = certificates.mallory_encrypt_cert_ref();
//     p_assert_matches!(
//         crate::sign_message(payload.as_ref(), &certificate_ref).await,
//         Err(SignMessageError::CannotSign(_))
//     );
// }

#[cfg(target_os = "windows")]
#[parsec_test]
async fn verify_message_ko_outdated_certificate(certificates: &InstalledCertificates) {
    let payload = b"The cake is a lie!";

    let pki = initialize_pki_system().await;

    let cert = pki
        .find_certificate(&certificates.alice_cert_ref())
        .await
        .unwrap()
        .unwrap();
    let validation_path = cert.get_validation_path().await.unwrap();
    let (algo, signature) = cert
        .request_private_key()
        .await
        .unwrap()
        .sign(payload)
        .await
        .unwrap();

    // Try to validate with a far-future date (certificate is expired)
    p_assert_matches!(
        crate::verify_message(
            payload.as_ref(),
            &signature,
            algo,
            &certificates.alice_der_cert(),
            validation_path.intermediates.iter().map(|c| c.as_ref()),
            &[validation_path.root],
            "9999-01-01T00:00:00Z".parse().unwrap(),
        ),
        Err(VerifyMessageError::X509CertificateUntrusted(
            webpki::Error::CertExpired { .. }
        ))
    );
}

#[cfg(target_os = "windows")] // TODO: verify only supported by Windows so far
#[parsec_test]
async fn verify_message_ko_different_certificate(certificates: &InstalledCertificates) {
    let payload = b"The cake is a lie!";

    let pki = initialize_pki_system().await;

    let cert = pki
        .find_certificate(&certificates.alice_cert_ref())
        .await
        .unwrap()
        .unwrap();
    let validation_path = cert.get_validation_path().await.unwrap();
    let (algo, signature) = cert
        .request_private_key()
        .await
        .unwrap()
        .sign(payload)
        .await
        .unwrap();

    // Verify with Bob's certificate instead of Alice's
    p_assert_matches!(
        crate::verify_message(
            payload.as_ref(),
            &signature,
            algo,
            &certificates.bob_der_cert(),
            validation_path.intermediates.iter().map(|c| c.as_ref()),
            &[validation_path.root],
            DateTime::now(),
        ),
        Err(VerifyMessageError::InvalidSignature(
            webpki::Error::InvalidSignatureForPublicKey
        ))
    );
}

#[cfg(target_os = "windows")] // TODO: verify only supported by Windows so far
#[parsec_test]
async fn verify_message_ko_different_payload(certificates: &InstalledCertificates) {
    let payload = b"The cake is a lie!";

    let pki = initialize_pki_system().await;

    let cert = pki
        .find_certificate(&certificates.alice_cert_ref())
        .await
        .unwrap()
        .unwrap();
    let validation_path = cert.get_validation_path().await.unwrap();
    let (algo, signature) = cert
        .request_private_key()
        .await
        .unwrap()
        .sign(payload)
        .await
        .unwrap();

    p_assert_matches!(
        crate::verify_message(
            b"Different payload",
            &signature,
            algo,
            &validation_path.leaf,
            validation_path.intermediates.iter().map(|c| c.as_ref()),
            &[validation_path.root],
            DateTime::now(),
        ),
        Err(VerifyMessageError::InvalidSignature(
            webpki::Error::InvalidSignatureForPublicKey
        ))
    );
}
