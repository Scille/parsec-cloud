// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::str::FromStr;

use crate::X509CertificateDer;

pub use webpki::EndEntityCert as X509EndCertificate;

use libparsec_types::prelude::*;

#[derive(Debug)]
pub struct RootX509CertificateInfo {
    pub common_name: String,
    pub subject: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum GetRootX509CertificateInfoFromTrustchainError {
    #[error("Invalid certificate: invalid DER format: {0}")]
    InvalidCertificateDer(anyhow::Error),
    #[error("Invalid certificate: missing common name")]
    InvalidCertificateNoCommonName,
}

pub fn get_root_x509_certificate_info_from_trustchain<'cert>(
    submitter_der_x509_certificate: &[u8],
    intermediate_der_x509_certificates: impl Iterator<Item = &'cert [u8]>,
) -> Result<RootX509CertificateInfo, GetRootX509CertificateInfoFromTrustchainError> {
    // 1. Walk up the chain until we reach the root

    let submitter_der_x509_certificate_der =
        X509CertificateDer::from_slice(submitter_der_x509_certificate);
    let submitter_der_x509_certificate_end =
        X509EndCertificate::try_from(&submitter_der_x509_certificate_der).map_err(|err| {
            GetRootX509CertificateInfoFromTrustchainError::InvalidCertificateDer(err.into())
        })?;

    let intermediate_der_x509_certificates_der = intermediate_der_x509_certificates
        .map(X509CertificateDer::from_slice)
        .collect::<Vec<_>>();
    let mut intermediate_der_x509_certificates_end = intermediate_der_x509_certificates_der
        .iter()
        .map(X509EndCertificate::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| {
            GetRootX509CertificateInfoFromTrustchainError::InvalidCertificateDer(err.into())
        })?;

    let mut current = submitter_der_x509_certificate_end;
    while let Some(i) = intermediate_der_x509_certificates_end
        .iter()
        .position(|cert| cert.subject() == current.issuer())
    {
        current = intermediate_der_x509_certificates_end.swap_remove(i);
    }

    // 2. Extract root subject & common name from the certificate issuer

    let subject = current.issuer().to_vec();

    let common_name = match crate::x509::extract_common_name_from_subject(&subject) {
        Ok(Some(common_name)) => common_name,
        Ok(None) => {
            return Err(
                GetRootX509CertificateInfoFromTrustchainError::InvalidCertificateNoCommonName,
            )
        }
        Err(err) => {
            return Err(
                GetRootX509CertificateInfoFromTrustchainError::InvalidCertificateDer(err.into()),
            )
        }
    };

    Ok(RootX509CertificateInfo {
        subject,
        common_name,
    })
}

#[derive(thiserror::Error, Debug)]
pub enum UserX509CertificateLoadError {
    #[error("Invalid certificate: invalid DER format")]
    InvalidCertificateDer,
    #[error("Invalid certificate: missing common name")]
    NoCommonName,
    #[error("Invalid certificate: invalid time field")]
    InvalidTime,
    #[error("Invalid certificate: invalid email field")]
    InvalidEmail,
}

pub struct UserX509CertificateDetails {
    pub common_name: String,
    pub subject: Vec<crate::x509::DistinguishedNameValue>,
    pub issuer: Vec<crate::x509::DistinguishedNameValue>,
    pub not_before: DateTime,
    pub not_after: DateTime,
    pub serial: Vec<u8>,
    pub emails: Vec<EmailAddress>,
    pub can_sign: bool,
    pub can_encrypt: bool,
}

impl UserX509CertificateDetails {
    pub fn load_der(der: &[u8]) -> Result<Self, UserX509CertificateLoadError> {
        let info = crate::x509::X509CertificateInformation::load_der(der)
            .map_err(|_| UserX509CertificateLoadError::InvalidCertificateDer)?;
        let emails = info
            .emails()
            .map(EmailAddress::from_str)
            .collect::<Result<_, _>>()
            .map_err(|_| UserX509CertificateLoadError::InvalidEmail)?;

        let common_name = match info.common_name() {
            Some(common_name) => common_name.to_string(),
            None => return Err(UserX509CertificateLoadError::NoCommonName),
        };
        let subject = info.subject;
        let issuer = info.issuer;

        let can_encrypt = info.extensions.can_encrypt();
        let can_sign = info.extensions.can_sign();
        let serial = info.serial;

        let not_before = x509_cert_time_to_datetime(info.validity.not_before)?;
        let not_after = x509_cert_time_to_datetime(info.validity.not_after)?;

        Ok(Self {
            common_name,
            subject,
            issuer,
            not_before,
            not_after,
            serial,
            emails,
            can_sign,
            can_encrypt,
        })
    }
}

fn x509_cert_time_to_datetime(
    time: x509_cert::time::Time,
) -> Result<DateTime, UserX509CertificateLoadError> {
    let time = time.to_date_time();
    DateTime::from_ymd_hms_us(
        time.year().into(),
        time.month().into(),
        time.day().into(),
        time.hour().into(),
        time.minutes().into(),
        time.seconds().into(),
        0,
    )
    .map_err(|_| UserX509CertificateLoadError::InvalidTime)
}
