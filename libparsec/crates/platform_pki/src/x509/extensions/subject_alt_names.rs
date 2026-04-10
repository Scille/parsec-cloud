// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use x509_cert::{
    der::{Decode, Error as DERError, SliceReader},
    ext::pkix::{self, name::GeneralName},
};

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum SubjectAltName {
    EmailAddress(String),
}

impl TryFrom<GeneralName> for SubjectAltName {
    type Error = &'static str;

    fn try_from(value: GeneralName) -> Result<Self, Self::Error> {
        match value {
            GeneralName::Rfc822Name(email) => Ok(Self::EmailAddress(email.to_string())),
            GeneralName::OtherName(_) => Err("otherName"),
            GeneralName::DnsName(_) => Err("dnsName"),
            GeneralName::DirectoryName(_) => Err("directoryName"),
            GeneralName::EdiPartyName(_) => Err("ediPartyName"),
            GeneralName::UniformResourceIdentifier(_) => Err("uniformResourceIdentifier"),
            GeneralName::IpAddress(_) => Err("ipAddress"),
            GeneralName::RegisteredId(_) => Err("registeredId"),
        }
    }
}
pub(super) fn parse_san_octet_string(raw: &[u8]) -> Result<Vec<SubjectAltName>, DERError> {
    let mut reader = SliceReader::new(raw)?;
    let raw_san = pkix::SubjectAltName::decode(&mut reader)?;
    Ok(raw_san
        .0
        .into_iter()
        .map(SubjectAltName::try_from)
        .filter_map(|res| {
            res.inspect_err(|e| log::debug!("Failure while parsing SAN: {e}"))
                .ok()
        })
        .collect())
}
