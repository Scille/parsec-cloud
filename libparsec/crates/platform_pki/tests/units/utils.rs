// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{ValidationPathOwned, X509CertificateHash, X509CertificateReference};
use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

// See `libparsec/crates/platform_pki/test-pki/README.md`
pub(super) const ALICE_SHA256_CERT_HASH: &str =
    "sha256-FH45Rn8sFI5XTBxE1inkRUxeVmBA9jtsdSkY+6w3+gQ=";
pub(super) const BOB_SHA256_CERT_HASH: &str = "sha256-639SRoRFC0jog3h76fY63ccZmlSORK2mR+IcBf2apqg=";
pub(super) const BLACK_MESA_SHA256_CERT_HASH: &str =
    "sha256-DrumDH+peXOqrXywKiTQTwfBE6gBiq4jawDfJlBlVRg=";
pub(super) const MALLORY_SIGN_SHA256_CERT_HASH: &str =
    "sha256-00e71WWTQGUjt0GQWPqhYzL8a8NPY3qNS/WhVfeXj9Q=";
pub(super) const MALLORY_ENCRYPT_SHA256_CERT_HASH: &str =
    "sha256-lPHjEH4TmPMO377j8OoL18hnUBEqNy0CfhNoP60+seY=";

const ALICE_PEM: &[u8] = include_bytes!("../../test-pki/Cert/alice.crt");
const BOB_PEM: &[u8] = include_bytes!("../../test-pki/Cert/bob.crt");
const MALLORY_ENCRYPT_PEM: &[u8] = include_bytes!("../../test-pki/Cert/mallory-encrypt.crt");
const MALLORY_SIGN_PEM: &[u8] = include_bytes!("../../test-pki/Cert/mallory-sign.crt");
const OLD_BOBY_PEM: &[u8] = include_bytes!("../../test-pki/Cert/old-boby.crt");
const GLADOS_DEV_TEAM_PEM: &[u8] =
    include_bytes!("../../test-pki/Intermediate/glados_dev_team.crt");
const APERTURE_SCIENCE_PEM: &[u8] = include_bytes!("../../test-pki/Root/aperture_science.crt");
const BLACK_MESA_PEM: &[u8] = include_bytes!("../../test-pki/Root/black_mesa.crt");

fn load_pem_and_return_der(pem: &[u8]) -> Bytes {
    crate::Certificate::try_from_pem(pem)
        .map(|cert| Bytes::copy_from_slice(cert.as_ref()))
        .unwrap_or_else(|e| panic!("Failed to read PEM certificate: {e}"))
}

pub(super) struct InstalledCertificates {}

impl InstalledCertificates {
    pub async fn alice_cert_ref(&self) -> X509CertificateReference {
        self.cert_ref(ALICE_SHA256_CERT_HASH).await
    }

    #[cfg_attr(not(windows), expect(dead_code))]
    pub async fn bob_cert_ref(&self) -> X509CertificateReference {
        self.cert_ref(BOB_SHA256_CERT_HASH).await
    }

    #[cfg_attr(not(windows), expect(dead_code))]
    pub fn black_mesa_cert_ref(&self) -> X509CertificateReference {
        // Black Mesa cert is a root certificate, so the sanity check in `cert_ref` will fail
        X509CertificateReference::from(
            BLACK_MESA_SHA256_CERT_HASH
                .parse::<X509CertificateHash>()
                .unwrap(),
        )
    }

    #[expect(dead_code)]
    pub async fn mallory_sign_cert_ref(&self) -> X509CertificateReference {
        self.cert_ref(MALLORY_SIGN_SHA256_CERT_HASH).await
    }

    #[expect(dead_code)]
    pub async fn mallory_encrypt_cert_ref(&self) -> X509CertificateReference {
        self.cert_ref(MALLORY_ENCRYPT_SHA256_CERT_HASH).await
    }

    #[expect(dead_code)]
    pub fn glados_dev_team_cert_ref(&self) -> X509CertificateReference {
        // GLaDOS Dev Team cert is an intermediate certificate, so the sanity check in `cert_ref` will fail
        X509CertificateReference::from(
            "sha256-4kC1hYV+2l6l5c6qtfLAqJNBPB8ETqAGKriNrAN8mUGVU="
                .parse::<X509CertificateHash>()
                .unwrap(),
        )
    }

    #[expect(dead_code)]
    pub fn aperture_science_cert_ref(&self) -> X509CertificateReference {
        // Aperture Science cert is a root certificate, so the sanity check in `cert_ref` will fail
        X509CertificateReference::from(
            "sha256-+rumDH+peXOqrXywKiTQTwfBE6gBiq4jawDfJlBlVRg="
                .parse::<X509CertificateHash>()
                .unwrap(),
        )
    }

    async fn cert_ref(&self, sha256_hash: &str) -> X509CertificateReference {
        let certificate_reference =
            X509CertificateReference::from(sha256_hash.parse::<X509CertificateHash>().unwrap());
        // Sanity check to ensure the certificate is installed
        if crate::get_der_encoded_certificate(&certificate_reference)
            .await
            .is_err()
        {
            #[cfg(windows)]
            panic!(
                "Certificate not found: \x1b[1;31m{}\x1b[0m\n\
                This probably means the test PKI certificates are not installed correctly\n\
                tl;dr: run in a PowerShell:\n\
                \t\x1b[1;35m& libparsec\\crates\\platform_pki\\examples\\import_testpki_windows.ps1\x1b[0m\n\
                ",
                sha256_hash
            );
            // TODO: Explain how to install the test PKI certificates once Linux support is available
            //       see https://github.com/Scille/parsec-cloud/issues/11848
            #[cfg(not(windows))]
            panic!(
                "Certificate not found: \x1b[1;31m{}\x1b[0m\n\
                This probably means the test PKI certificates are not installed correctly\n\
                ",
                sha256_hash
            );
        }
        certificate_reference
    }

    pub fn alice_der_cert(&self) -> Bytes {
        load_pem_and_return_der(ALICE_PEM)
    }

    pub fn bob_der_cert(&self) -> Bytes {
        load_pem_and_return_der(BOB_PEM)
    }

    #[expect(dead_code)]
    pub fn old_boby_der_cert(&self) -> Bytes {
        load_pem_and_return_der(OLD_BOBY_PEM)
    }

    pub fn black_mesa_der_cert(&self) -> Bytes {
        load_pem_and_return_der(BLACK_MESA_PEM)
    }

    pub fn mallory_sign_der_cert(&self) -> Bytes {
        load_pem_and_return_der(MALLORY_SIGN_PEM)
    }

    pub fn mallory_encrypt_der_cert(&self) -> Bytes {
        load_pem_and_return_der(MALLORY_ENCRYPT_PEM)
    }

    pub fn aperture_science_der_cert(&self) -> Bytes {
        load_pem_and_return_der(APERTURE_SCIENCE_PEM)
    }

    pub fn glados_dev_team_der_cert(&self) -> Bytes {
        load_pem_and_return_der(GLADOS_DEV_TEAM_PEM)
    }

    #[cfg_attr(not(windows), expect(dead_code))]
    pub async fn alice_encrypt_message(
        &self,
        payload: &[u8],
    ) -> (PKIEncryptionAlgorithm, Bytes, X509CertificateReference) {
        let certificate_ref = self.alice_cert_ref().await;
        let der = crate::get_der_encoded_certificate(&certificate_ref)
            .await
            .unwrap();
        let (algo, encrypted_message) =
            crate::encrypt_message(der.as_ref(), payload).await.unwrap();
        (algo, encrypted_message, certificate_ref)
    }

    #[cfg_attr(not(windows), expect(dead_code))]
    pub async fn alice_sign_message(
        &self,
        payload: &[u8],
    ) -> (PkiSignatureAlgorithm, Bytes, ValidationPathOwned) {
        let certificate_ref = self.alice_cert_ref().await;
        let (algo, signature) = crate::sign_message(payload, &certificate_ref)
            .await
            .unwrap();
        let now = DateTime::now();
        let validation_path = crate::get_validation_path_for_cert(&certificate_ref, now)
            .await
            .unwrap();
        (algo, signature, validation_path)
    }
}

#[fixture]
#[once]
pub(super) fn certificates() -> InstalledCertificates {
    InstalledCertificates {}
}
