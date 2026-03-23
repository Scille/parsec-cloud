// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub struct Certificate(schannel::cert_context::CertContext);

impl From<schannel::cert_context::CertContext> for Certificate {
    fn from(value: schannel::cert_context::CertContext) -> Self {
        Self(value)
    }
}

impl Certificate {
    pub async fn get_der(
        &self,
    ) -> Result<crate::X509CertificateDer<'static>, crate::GetCertificateDerError> {
        Ok(crate::X509CertificateDer::from_slice(self.0.to_der()).into_owned())
    }

    pub async fn request_private_key(
        &self,
    ) -> Result<super::PrivateKey, crate::RequestPrivateKeyError> {
        self.0
            .private_key()
            .compare_key(true)
            .acquire()
            .map(Into::into)
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => crate::RequestPrivateKeyError::NotFound,
                _ => crate::RequestPrivateKeyError::Internal(e.into()),
            })
    }
}
