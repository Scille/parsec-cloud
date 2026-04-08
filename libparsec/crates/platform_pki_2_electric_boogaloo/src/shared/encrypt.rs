// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub use webpki::EndEntityCert as X509EndCertificate;

use libparsec_types::prelude::*;

use crate::X509CertificateDer;

#[derive(Debug, thiserror::Error)]
pub enum EncryptMessageError {
    #[error("Cannot acquire public key: {0}")]
    CannotAcquirePubkey(#[from] crate::x509::GetCertificatePublicKeyError),
    #[error("Cannot encrypt message: {0}")]
    CannotEncrypt(#[from] rsa::errors::Error),
}

pub async fn encrypt_message(
    certificate: X509CertificateDer<'_>,
    message: &[u8],
) -> Result<(PKIEncryptionAlgorithm, Bytes), EncryptMessageError> {
    let pubkey = crate::x509::get_certificate_public_key(certificate.as_ref())?;
    match pubkey {
        crate::x509::PublicKey::Rsa(rsa_public_key) => {
            use rsa::traits::RandomizedEncryptor;

            let enc_key = rsa::oaep::EncryptingKey::<rsa::sha2::Sha256>::new(rsa_public_key);
            enc_key
                .encrypt_with_rng(&mut rsa::rand_core::OsRng, message)
                .map_err(Into::into)
                .map(|v| (PKIEncryptionAlgorithm::RsaesOaepSha256, Bytes::from(v)))
        }
    }
}
