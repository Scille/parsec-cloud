// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use rustls_pki_types::CertificateDer;

pub struct Certificate(schannel::cert_context::CertContext);

impl Certificate {
    pub async fn get_der(&self) -> Result<CertificateDer<'static>, crate::GetCertificateDerError> {
        Ok(CertificateDer::from_slice(self.0.to_der()).into_owned())
    }
}
