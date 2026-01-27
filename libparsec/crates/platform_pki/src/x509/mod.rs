// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod distinguished_name;
mod extensions;

use x509_cert::{
    der::{Decode, Error as DERError, SliceReader},
    Version,
};

use libparsec_types::prelude::*;

pub(crate) use distinguished_name::extract_common_name_from_subject;
pub use distinguished_name::{extract_dn_list_from_rnd_seq, DistinguishedNameValue};
pub use extensions::{Extensions, SubjectAltName};

error_set::error_set! {
    X509LoadError := {
        #[display("Invalid DER format: {0}")]
        DERError(DERError)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HandleFromCertInfoError {
    #[error("missing name")]
    MissingName,
    #[error("missing email")]
    MissingEmail,
    #[error("invalid human handle: {0}")]
    InvalidHumanHandle(#[from] libparsec_types::HumanHandleParseError),
}

// TODO: Remove `pub` qualifier once pki_enrollment is removed.
//       see https://github.com/Scille/parsec-cloud/issues/12054
#[derive(Debug, Clone)]
pub struct X509CertificateInformation {
    pub subject: Vec<DistinguishedNameValue>,
    pub issuer: Vec<DistinguishedNameValue>,
    pub extensions: Extensions,
}

impl X509CertificateInformation {
    pub fn load_der(der: &[u8]) -> Result<Self, X509LoadError> {
        x509_cert::Certificate::decode(&mut SliceReader::new(der)?)
            .map_err(Into::into)
            .and_then(TryInto::try_into)
    }

    pub fn common_name(&self) -> Option<&str> {
        self.subject.iter().find_map(|i| {
            if let DistinguishedNameValue::CommonName(cn) = i {
                Some(cn.as_str())
            } else {
                None
            }
        })
    }

    /// List emails contained in the certificate
    ///
    /// Will favor `subjectAltName` over subject `emailAddress` as the later one is
    /// deprecated.
    pub fn emails(&self) -> impl Iterator<Item = &str> {
        // First look for email in SAN
        self.extensions
            .subject_alt_names
            .iter()
            .map(|entry| {
                let SubjectAltName::EmailAddress(email) = entry;
                email.as_str()
            })
            // Then chain `emailAddress` from subject
            .chain(self.subject.iter().find_map(|entry| {
                if let DistinguishedNameValue::EmailAddress(email) = entry {
                    Some(email.as_str())
                } else {
                    None
                }
            }))
    }

    pub fn email(&self) -> Option<&str> {
        self.emails().next()
    }

    pub fn human_handle(&self) -> Result<HumanHandle, HandleFromCertInfoError> {
        let name = self
            .common_name()
            .ok_or(HandleFromCertInfoError::MissingName)?;
        let email = self.email().ok_or(HandleFromCertInfoError::MissingEmail)?;
        HumanHandle::from_raw(email, name).map_err(Into::into)
    }
}

impl TryFrom<x509_cert::Certificate> for X509CertificateInformation {
    type Error = X509LoadError;

    fn try_from(value: x509_cert::Certificate) -> Result<Self, Self::Error> {
        let subject = extract_dn_list_from_rnd_seq(value.tbs_certificate.subject);
        log::trace!("Collected subject: {subject:?}");

        let issuer = extract_dn_list_from_rnd_seq(value.tbs_certificate.issuer);
        log::trace!("Collected issuer: {issuer:?}");

        // Extensions are only available on V3 certificate
        let extensions: Extensions = if value.tbs_certificate.version == Version::V3 {
            value
                .tbs_certificate
                .extensions
                .unwrap_or_default()
                .try_into()?
        } else {
            Default::default()
        };

        Ok(X509CertificateInformation {
            subject,
            issuer,
            extensions,
        })
    }
}
