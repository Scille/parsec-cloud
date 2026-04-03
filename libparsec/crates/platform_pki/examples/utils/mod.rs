// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use anyhow::Context;
use bytes::Bytes;
use clap::{
    builder::{NonEmptyStringValueParser, TypedValueParser},
    error::{Error, ErrorKind},
};
use libparsec_platform_pki::{x509::DistinguishedNameValue, DerCertificate, PkiSystem};
use libparsec_types::X509CertificateHash;
use x509_cert::der::{DecodeValue, Header, SliceReader, Tag};

// Not all examples uses `Base64Parser` so it is not always used.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Base64Parser;

impl TypedValueParser for Base64Parser {
    type Value = Bytes;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::builder::OsStringValueParser::new();
        let val = inner.parse_ref(cmd, arg, value)?;
        data_encoding::BASE64
            .decode(val.as_encoded_bytes())
            .map_err(|e| Error::raw(ErrorKind::InvalidValue, e))
            .map(Bytes::from)
    }
}

// Not all examples uses `Base64Parser` so it is not always used.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Base64SerdeParser<T>(std::marker::PhantomData<T>);

impl<T> Default for Base64SerdeParser<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> TypedValueParser for Base64SerdeParser<T>
where
    T: Send + Sync + Clone + 'static,
    T: for<'a> serde::Deserialize<'a>,
{
    type Value = T;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = Base64Parser.parse_ref(cmd, arg, value)?;
        rmp_serde::decode::from_slice(inner.as_ref())
            .map_err(|e| Error::raw(ErrorKind::InvalidValue, e))
    }
}

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
    pub content_b64: Option<String>,
    /// Read content from a file
    #[arg(long)]
    pub content_file: Option<PathBuf>,
}

impl ContentOpts {
    // Not all examples uses `ContentOpts` so `into_bytes` is not always used.
    #[allow(dead_code)]
    pub fn into_bytes(self) -> anyhow::Result<Bytes> {
        match (self.content_b64, self.content_file) {
            (Some(content), None) => Ok(data_encoding::BASE64
                .decode(content.as_ref())
                .context("Invalid base64 content")?
                .into()),
            (None, Some(filepath)) => std::fs::read(filepath)
                .context("Failed to read file")
                .map(Into::into),
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
    pub async fn get_certificate(&self) -> anyhow::Result<DerCertificate<'static>> {
        let cert = if let Some(hash) = self.certificate_hash.clone() {
            let cert_ref: libparsec_types::X509CertificateReference = hash.into();
            // Config dir is only used for testbed, we can safely provide an empty path here
            let pki = PkiSystem::init(std::path::Path::new(""), None)
                .await
                .context("Failed to initialize PKI system")?;
            let certificate = pki
                .find_certificate(&cert_ref)
                .await
                .context("Failed to find certificate")?
                .context("Certificate not found")?;
            let der = certificate.get_der().await.context("Failed to get DER")?;
            DerCertificate::from(der)
        } else if let Some(der_file) = &self.der_file {
            let raw = std::fs::read(der_file).context("Failed to read file")?;
            DerCertificate::from_der_owned(raw)
        } else if let Some(pem_file) = &self.pem_file {
            let raw = std::fs::read(pem_file).context("Failed to read file")?;
            DerCertificate::try_from_pem(&r#raw)?.into_owned()
        } else if let Some(pem) = self.pem.as_deref() {
            let raw = data_encoding::BASE64
                .decode(pem.as_bytes())
                .context("Invalid pem base64")?;
            DerCertificate::from_der_owned(raw)
        } else {
            unreachable!("Should be handle by clap")
        };

        Ok(cert)
    }
}
