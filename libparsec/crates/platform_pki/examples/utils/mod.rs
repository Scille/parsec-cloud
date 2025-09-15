// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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
