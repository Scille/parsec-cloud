// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use anyhow::Context;
use bytes::Bytes;
use clap::{
    builder::{NonEmptyStringValueParser, TypedValueParser},
    error::{Error, ErrorKind},
};
use libparsec_types::X509CertificateHash;

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
