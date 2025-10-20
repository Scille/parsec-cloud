// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use anyhow::Context;
use clap::{
    builder::{NonEmptyStringValueParser, TypedValueParser},
    error::{Error, ErrorKind},
};
use libparsec_platform_pki::Certificate;
use libparsec_types::X509CertificateHash;

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
            println!(
                "Will verify signature using cert with id {{{}}}",
                data_encoding::BASE64.encode_display(
                    #[expect(clippy::unwrap_used)]
                    res.cert_ref.uris().next().unwrap()
                )
            );
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
