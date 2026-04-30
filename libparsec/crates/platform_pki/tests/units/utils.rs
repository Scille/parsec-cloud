// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{
    PkiSystem, X509CertificateDer, X509CertificateHash, X509CertificateReference,
    X509CertificateRevocationList,
};
use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;
use rustls_pki_types::pem::PemObject;

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
pub(super) const REVOKED_BREEN_SHA256_CERT_HASH: &str =
    "sha256-aEeYHBTCXkj5dCR231HQXH+mzaGV+0MdcBY+c6I+46c=";
pub(super) const GORDON_SHA256_CERT_HASH: &str =
    "sha256-aLOxDhF22Xdl2NRpyf29ncSh3REqgmAmK6ce59YnRO4=";
pub(super) const REVOKED_ANOMALOUS_MATERIALS_LABORATORIES_SHA256_CERT_HASH: &str =
    "sha256-vdvxLsg5jA0fqiLxpwrkBPVU6ad/ZK4/YrImQ4Wt2hw=";
pub(super) const GLADOS_DEV_TEAM_SHA256_CERT_HASH: &str =
    "sha256-SutQOKE6uFNzrG6BI0VM5hYz9BbPquqb/BFvUhJ+LQY=";
pub(super) const APERTURE_SCIENCE_SHA256_CERT_HASH: &str =
    "sha256-YtqVkLM7ehZT8D1PHD6K383EPtFkxuGfQmLLMYXnWp4=";

const ALICE_PEM: &[u8] = include_bytes!("../../test-pki/Cert/alice.crt");
const BOB_PEM: &[u8] = include_bytes!("../../test-pki/Cert/bob.crt");
const MALLORY_ENCRYPT_PEM: &[u8] = include_bytes!("../../test-pki/Cert/mallory-encrypt.crt");
const MALLORY_SIGN_PEM: &[u8] = include_bytes!("../../test-pki/Cert/mallory-sign.crt");
const OLD_BOBY_PEM: &[u8] = include_bytes!("../../test-pki/Cert/old-boby.crt");
const REVOKED_BREEN_PEM: &[u8] = include_bytes!("../../test-pki/Cert/revoked_breen.crt");
const GORDON_PEM: &[u8] = include_bytes!("../../test-pki/Cert/gordon.crt");
const GLADOS_DEV_TEAM_PEM: &[u8] =
    include_bytes!("../../test-pki/Intermediate/glados_dev_team.crt");
const REVOKED_ANOMALOUS_MATERIALS_LABORATORIES_PEM: &[u8] =
    include_bytes!("../../test-pki/Intermediate/revoked_anomalous_materials_laboratories.crt");
const APERTURE_SCIENCE_PEM: &[u8] = include_bytes!("../../test-pki/Root/aperture_science.crt");
const BLACK_MESA_PEM: &[u8] = include_bytes!("../../test-pki/Root/black_mesa.crt");
const BLACK_MESA_CERTIFICATE_REVOCATION_LIST_PEM: &[u8] =
    include_bytes!("../../test-pki/CRL/black_mesa.crl");

fn load_cert_der_from_pem(pem: &[u8]) -> X509CertificateDer<'static> {
    X509CertificateDer::from_pem_slice(pem)
        .unwrap_or_else(|e| panic!("Failed to read PEM certificate: {e}"))
}

fn load_crl_from_pem(pem: &[u8]) -> X509CertificateRevocationList<'static> {
    let der = rustls_pki_types::CertificateRevocationListDer::from_pem_slice(pem)
        .expect("Failed to read PEM CRL certificate");
    webpki::OwnedCertRevocationList::from_der(der.as_ref())
        .expect("Failed to read PEM CRL certificate")
        .into()
}

pub(super) struct InstalledCertificates {}

impl InstalledCertificates {
    pub fn alice_cert_ref(&self) -> X509CertificateReference {
        Self::make_cert_ref(ALICE_SHA256_CERT_HASH)
    }

    #[expect(dead_code)]
    pub fn bob_cert_ref(&self) -> X509CertificateReference {
        Self::make_cert_ref(BOB_SHA256_CERT_HASH)
    }

    #[expect(dead_code)]
    pub fn black_mesa_cert_ref(&self) -> X509CertificateReference {
        Self::make_cert_ref(BLACK_MESA_SHA256_CERT_HASH)
    }

    #[expect(dead_code)]
    pub fn mallory_sign_cert_ref(&self) -> X509CertificateReference {
        Self::make_cert_ref(MALLORY_SIGN_SHA256_CERT_HASH)
    }

    #[expect(dead_code)]
    pub fn mallory_encrypt_cert_ref(&self) -> X509CertificateReference {
        Self::make_cert_ref(MALLORY_ENCRYPT_SHA256_CERT_HASH)
    }

    #[expect(dead_code)]
    pub fn revoked_breen_cert_ref(&self) -> X509CertificateReference {
        Self::make_cert_ref(REVOKED_BREEN_SHA256_CERT_HASH)
    }

    #[expect(dead_code)]
    pub fn gordon_cert_ref(&self) -> X509CertificateReference {
        Self::make_cert_ref(GORDON_SHA256_CERT_HASH)
    }

    #[expect(dead_code)]
    pub fn glados_dev_team_cert_ref(&self) -> X509CertificateReference {
        Self::make_cert_ref(GLADOS_DEV_TEAM_SHA256_CERT_HASH)
    }

    #[expect(dead_code)]
    pub fn revoked_anomalous_materials_laboratories_cert_ref(&self) -> X509CertificateReference {
        Self::make_cert_ref(REVOKED_ANOMALOUS_MATERIALS_LABORATORIES_SHA256_CERT_HASH)
    }

    #[expect(dead_code)]
    pub fn aperture_science_cert_ref(&self) -> X509CertificateReference {
        Self::make_cert_ref(APERTURE_SCIENCE_SHA256_CERT_HASH)
    }

    fn make_cert_ref(sha256_hash: &str) -> X509CertificateReference {
        X509CertificateReference::from(sha256_hash.parse::<X509CertificateHash>().unwrap())
    }

    pub fn alice_der_cert(&self) -> X509CertificateDer<'static> {
        load_cert_der_from_pem(ALICE_PEM)
    }

    pub fn bob_der_cert(&self) -> X509CertificateDer<'static> {
        load_cert_der_from_pem(BOB_PEM)
    }

    #[expect(dead_code)]
    pub fn old_boby_der_cert(&self) -> X509CertificateDer<'static> {
        load_cert_der_from_pem(OLD_BOBY_PEM)
    }

    pub fn black_mesa_der_cert(&self) -> X509CertificateDer<'static> {
        load_cert_der_from_pem(BLACK_MESA_PEM)
    }

    pub fn mallory_sign_der_cert(&self) -> X509CertificateDer<'static> {
        load_cert_der_from_pem(MALLORY_SIGN_PEM)
    }

    pub fn mallory_encrypt_der_cert(&self) -> X509CertificateDer<'static> {
        load_cert_der_from_pem(MALLORY_ENCRYPT_PEM)
    }

    pub fn aperture_science_der_cert(&self) -> X509CertificateDer<'static> {
        load_cert_der_from_pem(APERTURE_SCIENCE_PEM)
    }

    pub fn revoked_breen_der_cert(&self) -> X509CertificateDer<'static> {
        load_cert_der_from_pem(REVOKED_BREEN_PEM)
    }

    pub fn gordon_der_cert(&self) -> X509CertificateDer<'static> {
        load_cert_der_from_pem(GORDON_PEM)
    }

    pub fn glados_dev_team_der_cert(&self) -> X509CertificateDer<'static> {
        load_cert_der_from_pem(GLADOS_DEV_TEAM_PEM)
    }

    pub fn revoked_anomalous_materials_laboratories_der_cert(&self) -> X509CertificateDer<'static> {
        load_cert_der_from_pem(REVOKED_ANOMALOUS_MATERIALS_LABORATORIES_PEM)
    }

    pub fn black_mesa_trust_anchor(&self) -> crate::X509TrustAnchor<'static> {
        webpki::anchor_from_trusted_cert(&self.black_mesa_der_cert())
            .unwrap()
            .to_owned()
    }

    pub fn black_mesa_certificate_revocation_list(
        &self,
    ) -> crate::X509CertificateRevocationList<'static> {
        load_crl_from_pem(BLACK_MESA_CERTIFICATE_REVOCATION_LIST_PEM)
    }

    pub fn aperture_science_trust_anchor(&self) -> crate::X509TrustAnchor<'static> {
        webpki::anchor_from_trusted_cert(&self.aperture_science_der_cert())
            .unwrap()
            .to_owned()
    }

    #[cfg_attr(not(target_os = "windows"), expect(dead_code))]
    pub async fn alice_encrypt_message(
        &self,
        payload: &[u8],
    ) -> (PKIEncryptionAlgorithm, Bytes, X509CertificateReference) {
        let (algo, encrypted_message) = crate::encrypt_message(self.alice_der_cert(), payload)
            .await
            .unwrap();
        (algo, encrypted_message, self.alice_cert_ref())
    }
}

#[fixture]
#[once]
pub(super) fn certificates() -> InstalledCertificates {
    InstalledCertificates {}
}

/// Initialize the *real* PKI system (not the testbed)
/// This requires to have the test certificates properly installed in the system
/// (tl;dr: run in a PowerShell: `& libparsec\crates\platform_pki\examples\import_testpki_windows.ps1`).
#[cfg_attr(not(target_os = "windows"), expect(unused))]
pub(super) async fn initialize_pki_system() -> PkiSystem {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let pki = PkiSystem::init(std::path::Path::new(""), None)
            .await
            .unwrap();

        // Sanity check to ensure the test certificates has been properly configured in the system
        let alice_ref = X509CertificateReference::from(
            ALICE_SHA256_CERT_HASH
                .parse::<X509CertificateHash>()
                .unwrap(),
        );
        if let Err(error) = pki.open_certificate(&alice_ref).await {
            #[cfg(windows)]
            panic!(
                "Certificate \x1b[1;31m{}\x1b[0m not found (error: {:?})\n\
                This probably means the test PKI certificates are not installed correctly\n\
                tl;dr: run in a PowerShell:\n\
                \t\x1b[1;35m& libparsec\\crates\\platform_pki\\examples\\import_testpki_windows.ps1\x1b[0m\n\
                ",
                ALICE_SHA256_CERT_HASH, error
            );
            // TODO: Explain how to install the test PKI certificates once Linux support is available
            //       see https://github.com/Scille/parsec-cloud/issues/11848
            #[cfg(not(windows))]
            panic!(
                "Certificate \x1b[1;31m{}\x1b[0m not found (error: {:?})\n\
                This probably means the test PKI certificates are not installed correctly\n\
                ",
                ALICE_SHA256_CERT_HASH, error
            );
        }

        pki
    }
    #[cfg(target_arch = "wasm32")]
    {
        // TODO:
        // - Get the Parsec server URL from an environment variable
        // - Do a sanity check to ensure the test certificates are properly configured
        //   (or consider having a dedicated test for this if it's too slow to do it for
        //   every test)
        todo!()
    }
}
