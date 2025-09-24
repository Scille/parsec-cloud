// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use anyhow::Context;
use clap::{
    builder::{NonEmptyStringValueParser, TypedValueParser},
    error::{Error, ErrorKind},
};
use libparsec_platform_pki::CertificateHash;

#[derive(Debug, Clone)]
pub struct CertificateSRIHashParser;

impl TypedValueParser for CertificateSRIHashParser {
    type Value = CertificateHash;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = NonEmptyStringValueParser::new();
        let val = inner.parse_ref(cmd, arg, value)?;
        let (hash_ty, b64_hash) = val
            .split_once('-')
            .ok_or_else(|| Error::raw(ErrorKind::InvalidValue, "Missing `-` delimiter"))?;
        let raw_data = data_encoding::BASE64
            .decode(b64_hash.as_bytes())
            .map_err(|e| Error::raw(ErrorKind::InvalidValue, e))?;
        if hash_ty.eq_ignore_ascii_case("sha256") {
            Ok(CertificateHash::SHA256(raw_data.try_into().map_err(
                |_| Error::raw(ErrorKind::InvalidValue, "Invalid data size"),
            )?))
        } else {
            Err(Error::raw(
                ErrorKind::InvalidValue,
                format!("Unsupported hash type `{hash_ty}`"),
            ))
        }
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
