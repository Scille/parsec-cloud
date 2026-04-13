// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{PkiPrivateKeyDecryptError, PkiPrivateKeySignError};
use libparsec_types::prelude::*;

#[derive(Debug)]
pub struct PlatformPkiPrivateKey {}

impl PlatformPkiPrivateKey {
    pub async fn sign(
        &self,
        _message: &[u8],
    ) -> Result<(PkiSignatureAlgorithm, Bytes), PkiPrivateKeySignError> {
        unimplemented!("platform not supported")
    }

    pub async fn decrypt(
        &self,
        _algorithm: PKIEncryptionAlgorithm,
        _ciphertext: &[u8],
    ) -> Result<Bytes, PkiPrivateKeyDecryptError> {
        unimplemented!("platform not supported")
    }
}
