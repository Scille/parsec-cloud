// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::DERError;
use std::fmt::Display;
use x509_cert::{
    attr::AttributeTypeAndValue,
    der::{
        self, asn1,
        oid::db::{rfc3280::EMAIL_ADDRESS, rfc4519::COMMON_NAME},
        Choice, Decode, DecodeValue, ErrorKind as DERErrorKind, Reader, Result as DERResult, Tag,
    },
    name::RdnSequence,
};

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum DistinguishedNameValue {
    CommonName(String),
    EmailAddress(String),
}

pub fn extract_dn_list_from_rnd_seq(value: RdnSequence) -> Vec<DistinguishedNameValue> {
    value
        .0
        .into_iter()
        .flat_map(|dn| {
            dn.0
                // Internally, it just return the wrapped Vec
                .into_vec()
                .into_iter()
                .map(DistinguishedNameValue::try_from)
        })
        .filter_map(|res| {
            res.inspect_err(|e| log::debug!("Failure while parsing distinguished name: {e}"))
                .ok()
        })
        .collect()
}

impl TryFrom<AttributeTypeAndValue> for DistinguishedNameValue {
    type Error = DERError;

    fn try_from(value: AttributeTypeAndValue) -> Result<Self, Self::Error> {
        let AttributeTypeAndValue { oid, value } = value;
        match oid {
            COMMON_NAME => value
                .decode_as::<CommonNameValue>()
                .map(|v| v.to_string())
                .map(Self::CommonName),
            EMAIL_ADDRESS => value
                .decode_as::<asn1::Ia5StringRef>()
                .map(|v| v.to_string())
                .map(Self::EmailAddress),
            _ => Err(DERErrorKind::OidUnknown { oid }.into()),
        }
    }
}

/// Defined in [RFC-5280](https://www.rfc-editor.org/rfc/rfc5280#appendix-A.1)
///
/// ```asn
/// X520CommonName  ::=      CHOICE {
///   teletexString         TeletexString (SIZE (1..ub-common-name)),
///   printableString       PrintableString (SIZE (1..ub-common-name)),
///   universalString       UniversalString (SIZE (1..ub-common-name)),
///   utf8String            UTF8String (SIZE (1..ub-common-name)),
///   bmpString             BMPString (SIZE(1..ub-common-name))   }
/// ```
///
/// > [!NOTE]
/// > We ignore `universalString` as it's in fact never used in favor of `utf8String`
// TODO: Once der_derive@0.8 is released, use derive macro `Choice`.
// Currently, we are not using it because it's missing some der Tag.
// #[derive(Choice)]
enum CommonNameValue<'der> {
    Teletex(asn1::TeletexStringRef<'der>),
    Printable(asn1::PrintableStringRef<'der>),
    Bmp(asn1::BmpString),
    UTF8(asn1::Utf8StringRef<'der>),
}

impl<'der> Choice<'der> for CommonNameValue<'der> {
    fn can_decode(tag: Tag) -> bool {
        matches!(
            tag,
            Tag::TeletexString | Tag::PrintableString | Tag::Utf8String | Tag::BmpString
        )
    }
}

impl<'der> Decode<'der> for CommonNameValue<'der> {
    fn decode<R: Reader<'der>>(reader: &mut R) -> DERResult<Self> {
        match reader.peek_tag()? {
            Tag::TeletexString => Ok(Self::Teletex(asn1::TeletexStringRef::decode(reader)?)),
            Tag::PrintableString => Ok(Self::Printable(asn1::PrintableStringRef::decode(reader)?)),
            Tag::Utf8String => Ok(Self::UTF8(asn1::Utf8StringRef::decode(reader)?)),
            Tag::BmpString => Ok(Self::Bmp(asn1::BmpString::decode(reader)?)),
            actual => Err(der::ErrorKind::TagUnexpected {
                expected: Some(Tag::Utf8String),
                actual,
            }
            .into()),
        }
    }
}

impl<'der> der::Tagged for CommonNameValue<'der> {
    fn tag(&self) -> Tag {
        match self {
            Self::Teletex(_) => Tag::TeletexString,
            Self::Printable(_) => Tag::PrintableString,
            Self::UTF8(_) => Tag::Utf8String,
            Self::Bmp(_) => Tag::BmpString,
        }
    }
}

impl<'der> DecodeValue<'der> for CommonNameValue<'der> {
    fn decode_value<R: der::Reader<'der>>(
        reader: &mut R,
        header: der::Header,
    ) -> der::Result<Self> {
        match header.tag {
            Tag::Utf8String => asn1::Utf8StringRef::decode_value(reader, header).map(Self::UTF8),
            Tag::PrintableString => {
                asn1::PrintableStringRef::decode_value(reader, header).map(Self::Printable)
            }
            Tag::TeletexString => {
                asn1::TeletexStringRef::decode_value(reader, header).map(Self::Teletex)
            }
            Tag::BmpString => asn1::BmpString::decode_value(reader, header).map(Self::Bmp),
            actual => Err(DERErrorKind::TagUnexpected {
                expected: Some(Tag::Utf8String),
                actual,
            }
            .into()),
        }
    }
}

impl<'der> Display for CommonNameValue<'der> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Teletex(teletex_string) => f.write_str(teletex_string.as_str()),
            Self::Printable(printable_string) => f.write_str(printable_string.as_str()),
            Self::UTF8(s) => f.write_str(s.as_str()),
            Self::Bmp(bmp_string) => bmp_string.fmt(f),
        }
    }
}
