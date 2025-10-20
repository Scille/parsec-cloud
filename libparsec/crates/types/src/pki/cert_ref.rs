// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::str::FromStr;

use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::DataError;

#[derive(
    Clone, Eq, PartialEq, serde_with::SerializeDisplay, serde_with::DeserializeFromStr, Debug,
)]
pub enum X509CertificateHash {
    SHA256(Box<[u8; 32]>),
}

impl X509CertificateHash {
    pub fn fake_sha256() -> Self {
        Self::SHA256(Default::default())
    }
}

impl std::fmt::Display for X509CertificateHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (hash_str, data) = match self {
            X509CertificateHash::SHA256(data) => ("sha256", data.as_ref()),
        };
        write!(
            f,
            "{hash_str}-{}",
            ::data_encoding::BASE64.encode_display(data)
        )
    }
}

impl FromStr for X509CertificateHash {
    type Err = DataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hash_ty, b64_hash) = s.split_once('-').ok_or(DataError::BadSerialization {
            format: None,
            step: "Missing `-` delimiter",
        })?;
        let raw_data = data_encoding::BASE64
            .decode(b64_hash.as_bytes())
            .map_err(|_| DataError::BadSerialization {
                format: None,
                step: "error decoding hash",
            })?;
        if hash_ty.eq_ignore_ascii_case("sha256") {
            Ok(X509CertificateHash::SHA256(raw_data.try_into().map_err(
                |_| DataError::BadSerialization {
                    format: None,
                    step: "Invalid data size",
                },
            )?))
        } else {
            Err(DataError::BadSerialization {
                format: None,
                step: "Unsupported hash type ",
            })
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct X509CertificateReference {
    pub uri: Option<Bytes>,
    pub hash: X509CertificateHash,
}

impl From<X509CertificateHash> for X509CertificateReference {
    fn from(hash: X509CertificateHash) -> Self {
        Self { uri: None, hash }
    }
}

impl X509CertificateReference {
    /// Add or replace a certificate URI.
    ///
    /// The list will only contain a single types of URI
    pub fn add_or_replace_uri(mut self, uri: Bytes) -> Self {
        let _ = self.uri.replace(uri);
        self
    }

    pub fn get_uri(&self) -> Option<&Bytes> {
        self.uri.as_ref()
    }

    pub fn uris(&self) -> impl Iterator<Item = &Bytes> {
        self.uri.iter()
    }
}
