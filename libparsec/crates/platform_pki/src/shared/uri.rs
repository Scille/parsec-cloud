// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::fmt::Debug;

use libparsec_types::prelude::*;
use rustls_pki_types::CertificateDer;

use x509_cert::der::{Decode, Encode, SliceReader};

#[derive(Debug, thiserror::Error)]
pub enum GetCertificateUriError {
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[allow(unused)]
pub(crate) async fn get_certificate_pkcs11_uri(
    der: &CertificateDer<'_>,
    id: Option<Bytes>,
    label: Option<Bytes>,
) -> Result<X509Pkcs11URI, GetCertificateUriError> {
    let cert = x509_cert::Certificate::decode(
        &mut SliceReader::new(der).map_err(|e| GetCertificateUriError::Internal(e.into()))?,
    )
    .map_err(|e| GetCertificateUriError::Internal(e.into()))?
    .tbs_certificate;

    let issuer = Bytes::from(
        cert.issuer
            .to_der()
            .map_err(|e| GetCertificateUriError::Internal(e.into()))?,
    );
    let serial = Bytes::copy_from_slice(cert.serial_number.as_bytes());
    let subject = Bytes::from(
        cert.subject
            .to_der()
            .map_err(|e| GetCertificateUriError::Internal(e.into()))?,
    );
    Ok(X509Pkcs11URI {
        id,
        label,
        issuer,
        serial,
        subject,
    })
}
