// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::anyhow;
use rustls_pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer};
use std::collections::HashMap;

use bytes::Bytes;

use crate::x509::X509CertificateInformation;

#[derive(Debug, Clone, Default)]
pub struct TestPKI {
    pub cert: HashMap<String, Certificate>,
    pub intermediate: HashMap<String, Certificate>,
    pub root: HashMap<String, Certificate>,
}

#[derive(Debug, Clone)]
pub struct Certificate {
    pub name: String,
    pub cert_info: X509CertificateInformation,
    pub der_certificate: Bytes,
    pub der_key: Bytes,
}

impl TryFrom<PartialCertificate> for Certificate {
    type Error = anyhow::Error;

    fn try_from(value: PartialCertificate) -> Result<Self, Self::Error> {
        let der_certificate = value
            .der_certificate
            .ok_or_else(|| anyhow::anyhow!("Missing certificate for entry {}", value.name))?;
        let cert_info = X509CertificateInformation::load_der(&der_certificate)?;
        let der_key = value
            .der_key
            .ok_or_else(|| anyhow::anyhow!("Missing key for entry {}", value.name))?;
        Ok(Self {
            der_certificate,
            name: value.name,
            cert_info,
            der_key,
        })
    }
}

struct PartialCertificate {
    name: String,
    der_certificate: Option<Bytes>,
    der_key: Option<Bytes>,
}

impl PartialCertificate {
    fn new(name: String) -> Self {
        Self {
            name,
            der_certificate: None,
            der_key: None,
        }
    }

    fn with_pem_certificate(&mut self, pem_cert: Vec<u8>) -> &mut Self {
        let der_cert = CertificateDer::from_pem_slice(&pem_cert).expect("Invalid pem key");
        self.der_certificate = Some(Bytes::copy_from_slice(&der_cert));
        self
    }

    fn with_pem_key(&mut self, pem_key: Vec<u8>) -> &mut Self {
        let der_key = PrivateKeyDer::from_pem_slice(&pem_key).expect("Invalid pem key");
        self.der_key = Some(Bytes::copy_from_slice(der_key.secret_der()));
        self
    }
}

// wasm32 does not provide a filesystem API so we cannot load the test pki.
#[cfg(target_arch = "wasm32")]
mod platform_implementation {
    #[libparsec_tests_lite::rstest::fixture]
    #[once] // Run fixture once
    pub fn test_pki() -> super::TestPKI {
        super::TestPKI::default()
    }
}

// Everything else does provide a filesystem, so using it.
#[cfg(not(target_arch = "wasm32"))]
mod platform_implementation {
    use std::path::{Path, PathBuf};

    use super::*;
    use libparsec_tests_lite::rstest;

    #[rstest::fixture]
    #[once] // Run fixture once
    pub fn test_pki() -> TestPKI {
        let test_pki_dir = std::env::var_os("TEST_PKI_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                // TODO: Use `git-root` instead so it can also be used by `parsec-cli`
                let fixture_manifest_path =
                    <str as std::convert::AsRef<Path>>::as_ref(std::env!("CARGO_MANIFEST_DIR"));
                fixture_manifest_path.join("../../../libparsec/crates/platform_pki/test-pki")
            });

        println!("Using test pki form: {}", test_pki_dir.display());

        assert!(
            test_pki_dir.is_dir(),
            "{} is not a dir",
            std::path::absolute(&test_pki_dir)
                .as_ref()
                .unwrap_or(&test_pki_dir)
                .display()
        );

        let cert_dir = test_pki_dir.join("Cert");
        let intermediate_dir = test_pki_dir.join("Intermediate");
        let root_dir = test_pki_dir.join("Root");

        let cert = list_files(&cert_dir);
        let intermediate = list_files(&intermediate_dir);
        let root = list_files(&root_dir);

        TestPKI {
            cert,
            intermediate,
            root,
        }
    }

    fn list_files(dir: &Path) -> HashMap<String, Certificate> {
        let mut store = HashMap::<String, PartialCertificate>::with_capacity(10);
        for entry in dir.read_dir().expect("Failed to read folder") {
            let entry = entry.expect("Invalid dir entry");

            match entry.metadata() {
                Ok(metadata) if !metadata.is_file() => continue,
                Err(_) => continue,
                _ => (),
            }

            let path = entry.path();
            let name =
                String::from_utf8_lossy(path.file_stem().expect("No file name").as_encoded_bytes())
                    .to_string();
            let suffix = path.extension();
            let content = std::fs::read(&path).expect("Cannot read entry");

            match suffix.and_then(|s| s.to_str()) {
                Some("key") => {
                    store
                        .entry(name)
                        .or_insert_with_key(|k| PartialCertificate::new(k.to_string()))
                        .with_pem_key(content);
                }
                Some("crt") => {
                    store
                        .entry(name)
                        .or_insert_with_key(|k| PartialCertificate::new(k.to_string()))
                        .with_pem_certificate(content);
                }
                _ => {
                    eprintln!(
                        "Ignoring {name}: unknown suffix {suffix:?} (path={})",
                        path.display()
                    )
                }
            }
        }

        store
            .into_iter()
            .map(|(k, v)| Certificate::try_from(v).map(|v| (k, v)))
            .collect::<Result<_, _>>()
            .expect("Invalid entry")
    }
}

pub use platform_implementation::test_pki;
