// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use anyhow::Context;
use clap::{
    builder::{NonEmptyStringValueParser, TypedValueParser},
    error::{Error, ErrorKind},
};
use libparsec_platform_pki::{x509::DistinguishedNameValue, Certificate};
use libparsec_types::X509CertificateHash;
use x509_cert::der::{DecodeValue, Header, SliceReader, Tag};

#[derive(Debug, Clone)]
pub struct CertificateSRIHashParser;

impl TypedValueParser for CertificateSRIHashParser {
    type Value = X509CertificateHash;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = NonEmptyStringValueParser::new();
        let val = inner.parse_ref(cmd, arg, value)?;
        val.parse()
            .map_err(|e| Error::raw(ErrorKind::InvalidValue, e))
    }
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
pub struct ContentOpts {
    /// The content to use.
    #[arg(long)]
    pub content: Option<String>,
    /// Read content from a file
    #[arg(long)]
    pub content_file: Option<PathBuf>,
}

impl ContentOpts {
    // Not all examples uses `ContentOpts` so `into_bytes` is not always used.
    #[allow(dead_code)]
    pub fn into_bytes(self) -> anyhow::Result<Vec<u8>> {
        match (self.content, self.content_file) {
            (Some(content), None) => Ok(content.into()),
            (None, Some(filepath)) => std::fs::read(filepath).context("Failed to read file"),
            (Some(_), Some(_)) | (None, None) => unreachable!("Handled by clap"),
        }
    }
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
pub struct CertificateOrRef {
    /// Hash of the certificate from the store to use to verify the signature.
    #[arg(long, value_parser = CertificateSRIHashParser)]
    certificate_hash: Option<X509CertificateHash>,
    /// Path to a file containing the certificate in DER format.
    #[arg(long)]
    der_file: Option<PathBuf>,
    /// Path to a file containing the certificate in PEM format.
    #[arg(long)]
    pem_file: Option<PathBuf>,
    /// Certificate in PEM format but without headers.
    #[arg(long)]
    pem: Option<String>,
}

impl CertificateOrRef {
    // Not all examples uses `CertificateOrRef` so `get_certificate` is not always used.
    #[allow(dead_code)]
    pub fn get_certificate(&self) -> anyhow::Result<Certificate<'static>> {
        let cert = if let Some(hash) = self.certificate_hash.clone() {
            let res = libparsec_platform_pki::get_der_encoded_certificate(&hash.into())?;
            println!("Will verify signature using cert with id {{{}}}", {
                #[expect(clippy::unwrap_used)]
                res.cert_ref.uris().next().unwrap()
            });
            Certificate::from_der_owned(res.der_content.into())
        } else if let Some(der_file) = &self.der_file {
            let raw = std::fs::read(der_file).context("Failed to read file")?;
            Certificate::from_der_owned(raw)
        } else if let Some(pem_file) = &self.pem_file {
            let raw = std::fs::read(pem_file).context("Failed to read file")?;
            Certificate::try_from_pem(&r#raw)?.into_owned()
        } else if let Some(pem) = self.pem.as_deref() {
            let raw = data_encoding::BASE64
                .decode(pem.as_bytes())
                .context("Invalid pem base64")?;
            Certificate::from_der_owned(raw)
        } else {
            unreachable!("Should be handle by clap")
        };

        Ok(cert)
    }
}

/// Used to display x509 certificate subject and issuer field
// Not all examples need to display x509 name
#[allow(dead_code)]
pub fn display_x509_raw_name(raw_name: &[u8]) -> impl std::fmt::Display {
    struct DisplayName(Vec<DistinguishedNameValue>);

    impl std::fmt::Display for DisplayName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    let components = x509_cert::name::Name::decode_value(
        &mut SliceReader::new(raw_name).unwrap(),
        Header::new(Tag::Sequence, raw_name.len()).unwrap(),
    )
    .map(libparsec_platform_pki::x509::extract_dn_list_from_rnd_seq)
    .unwrap_or_default();

    DisplayName(components)
}

// Not all example need to display a x509 cert
#[allow(dead_code)]
pub fn display_cert<'der>(cert: &'der webpki::Cert<'der>) -> impl std::fmt::Display + use<'der> {
    struct DisplayCert<'der>(&'der webpki::Cert<'der>);

    impl<'der> std::fmt::Display for DisplayCert<'der> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "subject={subject}, issuer={issuer}",
                subject = display_x509_raw_name(self.0.subject()),
                issuer = display_x509_raw_name(self.0.issuer())
            )
        }
    }

    DisplayCert(cert)
}
