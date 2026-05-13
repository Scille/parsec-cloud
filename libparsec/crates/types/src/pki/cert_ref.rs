// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use serde::{Deserialize, Serialize};

use crate::DataError;

/*
 * X509CertificateHash
 */

#[derive(
    Clone, Copy, Eq, PartialEq, serde_with::SerializeDisplay, serde_with::DeserializeFromStr, Debug,
)]
pub enum X509CertificateHash {
    SHA256([u8; 32]),
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

impl std::str::FromStr for X509CertificateHash {
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

/*
 * X509CertificateReference
 */

/// Reference to an X509 certificate held by a smartcard.
#[serde_with::serde_as]
#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct X509CertificateReference {
    /// Hint used to locate the X509 certificate in the OS store in a more optimized way.
    ///
    /// Field introduced in Parsec 3.9.0
    ///
    /// In Parsec < 3.9.0 there was a (now deprecated) `uris: Vec<X509URIFlavorValue>`
    /// field that could store either `X509WindowsCngURI` or `X509Pkcs11URI` kind of URIs.
    /// In practice this field has only ever been used to store `X509WindowsCngURI` (since
    /// PKI was only supported for Windows). Now we can just ignore this deprecated field
    /// entirely and let legacy documents with it rely on the hash only.
    #[serde(default, skip_serializing_if = "crate::Maybe::is_absent")]
    #[serde_as(r#as = "crate::Maybe<_>")]
    pub uri: crate::Maybe<X509Pkcs11URI>,
    /// Hash is the authoritative identifier
    pub hash: X509CertificateHash,
}

impl std::cmp::PartialEq for X509CertificateReference {
    fn eq(&self, other: &Self) -> bool {
        // Ignore the URI since it's just a hint
        self.hash == other.hash
    }
}

impl From<X509CertificateHash> for X509CertificateReference {
    fn from(hash: X509CertificateHash) -> Self {
        Self {
            hash,
            uri: crate::Maybe::Absent,
        }
    }
}

/*
 * X509Pkcs11URI
 */

#[serde_with::serde_as]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct X509Pkcs11URI {
    /// Key identifier for key.
    ///
    /// Used to link together a certificate & private key.
    /// That information is provided/configured by the smartcard so it may not be present.
    #[serde_as(as = "Option<serde_with::Bytes>")]
    pub id: Option<Vec<u8>>,
    /// Label of the object.
    ///
    /// That information is provided/configured by the smartcard so it may not be present.
    #[serde_as(as = "Option<serde_with::Bytes>")]
    pub label: Option<Vec<u8>>,
    /// Issuer of the certificate.
    #[serde_as(as = "serde_with::Bytes")]
    pub der_issuer: Vec<u8>,
    /// Subject of the certificate.
    #[serde_as(as = "serde_with::Bytes")]
    pub der_subject: Vec<u8>,
    /// Serial number of the certificate.
    ///
    /// It is a positive integer assigned by the CA (cf. https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.2).
    /// It is stored here as an unsigned integer in big-endian, as it is the format used in X509 certificate format
    /// (i.e. ASN.1 DER's INTEGER type, cf. https://datatracker.ietf.org/doc/html/rfc5280#section-4.1 and https://en.wikipedia.org/wiki/X.690)
    ///
    /// Be aware that Windows provide you the serial number in little-endian if the value is coming
    /// from [`CERT_INFO`](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/ns-wincrypt-cert_info)
    #[serde_as(as = "serde_with::Bytes")]
    pub serial: Vec<u8>,
}
