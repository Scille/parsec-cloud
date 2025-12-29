// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{fmt::Display, str::FromStr};

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

#[serde_with::serde_as]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct X509CertificateReference {
    /// List of URI to reference a given certificate.
    ///
    /// Currently the field is private as we only allow for only one of each types of URIs in the list,
    /// we use [`X509CertificateReference::add_or_replace_uri`] to enforce that constrain.
    #[serde_as(as = "serde_with::VecSkipError<_>")]
    uris: Vec<X509URIFlavorValue>,
    pub hash: X509CertificateHash,
}

use private::X509URIFlavor;

impl X509CertificateReference {
    /// Add or replace a certificate URI.
    ///
    /// The list will only contain a single types of URI
    pub fn add_or_replace_uri<Flavor: X509URIFlavor>(self, new_uri: Flavor) -> Self {
        self.add_or_replace_uri_wrapped(new_uri.into())
    }

    pub fn add_or_replace_uri_wrapped(mut self, new_uri: X509URIFlavorValue) -> Self {
        match self
            .uris
            .binary_search_by(|uri| uri.header().cmp(new_uri.header()))
        {
            // A URI already use that flavor, replacing it.
            Ok(pos) => self.uris[pos] = new_uri,
            // No URIs use that flavor, inserting so the list is keep sorted.
            Err(pos) => self.uris.insert(pos, new_uri),
        }
        self
    }

    pub fn uris(&self) -> impl Iterator<Item = &X509URIFlavorValue> {
        self.uris.iter()
    }

    pub fn get_uri<Flavor: X509URIFlavor>(&self) -> Option<&Flavor> {
        self.uris.iter().find_map(|uri| Flavor::may_unwrap(uri))
    }
}

mod private {
    pub trait X509URIFlavor: Sized + Into<super::X509URIFlavorValue> {
        const HEADER: &'static str;

        fn header(&self) -> &'static str {
            Self::HEADER
        }

        fn may_unwrap(wrapper: &super::X509URIFlavorValue) -> Option<&Self>;
    }
}

impl From<X509CertificateHash> for X509CertificateReference {
    fn from(hash: X509CertificateHash) -> Self {
        Self {
            uris: Vec::new(),
            hash,
        }
    }
}

#[serde_with::serde_as]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct X509WindowsCngURI {
    #[serde_as(as = "serde_with::Bytes")]
    pub issuer: Vec<u8>,
    #[serde_as(as = "serde_with::Bytes")]
    pub serial_number: Vec<u8>,
}

impl X509URIFlavor for X509WindowsCngURI {
    const HEADER: &'static str = "windows-cng";
    fn may_unwrap(wrapper: &X509URIFlavorValue) -> Option<&Self> {
        match wrapper {
            X509URIFlavorValue::WindowsCNG(uri) => Some(uri),
            _ => None,
        }
    }
}

impl Display for X509WindowsCngURI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            Self::HEADER,
            data_encoding::BASE64.encode_display(&self.issuer),
            data_encoding::BASE64.encode_display(&self.serial_number)
        )
    }
}

impl From<X509WindowsCngURI> for X509URIFlavorValue {
    fn from(value: X509WindowsCngURI) -> Self {
        Self::WindowsCNG(value)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
// TODO: See if we store raw pkcs11 attribute in a list
// Or a string representing a valid pkcs11-URI (following RFC-7512)
// Or something else
pub struct X509Pkcs11URI;

impl X509URIFlavor for X509Pkcs11URI {
    const HEADER: &'static str = "pkcs11";

    fn may_unwrap(wrapper: &X509URIFlavorValue) -> Option<&Self> {
        match wrapper {
            X509URIFlavorValue::PKCS11(uri) => Some(uri),
            _ => None,
        }
    }
}

impl Display for X509Pkcs11URI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", Self::HEADER)
    }
}

impl From<X509Pkcs11URI> for X509URIFlavorValue {
    fn from(value: X509Pkcs11URI) -> Self {
        Self::PKCS11(value)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum X509URIFlavorValue {
    WindowsCNG(X509WindowsCngURI),
    PKCS11(X509Pkcs11URI),
}

impl X509URIFlavorValue {
    fn header(&self) -> &'static str {
        match self {
            X509URIFlavorValue::WindowsCNG(uri) => uri.header(),
            X509URIFlavorValue::PKCS11(uri) => uri.header(),
        }
    }
}

impl Display for X509URIFlavorValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WindowsCNG(data) => data.fmt(f),
            Self::PKCS11(data) => data.fmt(f),
        }
    }
}
