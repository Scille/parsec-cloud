// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod distinguished_name;

use distinguished_name::extract_dn_list_from_rnd_seq;
use x509_cert::der::{Decode, Error as DERError, SliceReader};

pub use distinguished_name::DistinguishedNameValue;

error_set::error_set! {
    X509LoadError = {
        #[display("Invalid DER format: {0}")]
        DERError(DERError)
    };
}

#[derive(Debug)]
pub struct Certificate {
    pub subject: Vec<DistinguishedNameValue>,
    pub issuer: Vec<DistinguishedNameValue>,
}

impl Certificate {
    pub fn load_der(der: &[u8]) -> Result<Self, X509LoadError> {
        x509_cert::Certificate::decode(&mut SliceReader::new(der)?)
            .map_err(Into::into)
            .and_then(TryInto::try_into)
    }
}

impl TryFrom<x509_cert::Certificate> for Certificate {
    type Error = X509LoadError;

    fn try_from(value: x509_cert::Certificate) -> Result<Self, Self::Error> {
        let subject = extract_dn_list_from_rnd_seq(value.tbs_certificate.subject);
        log::trace!(subject:?; "Collected subject");

        let issuer = extract_dn_list_from_rnd_seq(value.tbs_certificate.issuer);
        log::trace!(issuer:?; "Collected issuer");

        Ok(Certificate { subject, issuer })
    }
}
