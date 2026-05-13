// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::{X509CertificateHash, X509CertificateReference, X509Pkcs11URI};

use sha2::Digest;
use x509_cert::der::{Decode, Encode, Error as DERError, SliceReader};

#[expect(clippy::result_large_err)] // Error variant is still smaller than the Ok variant
pub(crate) fn get_certificate_ref(
    id: Option<Vec<u8>>,
    label: Option<Vec<u8>>,
    cert_der: &[u8],
) -> Result<(X509CertificateReference, x509_cert::Certificate), (X509CertificateReference, DERError)>
{
    let hash = sha2::Sha256::digest(cert_der);
    let hash = X509CertificateHash::SHA256(hash.into());
    let cert_detail = SliceReader::new(cert_der)
        .and_then(|mut reader| x509_cert::Certificate::decode(&mut reader))
        .map_err(|e| (hash.into(), e))?;
    let cert = &cert_detail.tbs_certificate;
    let der_issuer = cert.issuer.to_der().map_err(|e| (hash.into(), e))?;
    let serial = cert.serial_number.as_bytes().into();
    let der_subject = cert.subject.to_der().map_err(|e| (hash.into(), e))?;
    let uri = X509Pkcs11URI {
        id,
        label,
        der_issuer,
        serial,
        der_subject,
    };
    Ok((
        X509CertificateReference {
            uri: uri.into(),
            hash,
        },
        cert_detail,
    ))
}
