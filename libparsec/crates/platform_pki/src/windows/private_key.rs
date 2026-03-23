// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

pub struct PrivateKey(#[expect(dead_code)] schannel::cert_context::PrivateKey);

impl From<schannel::cert_context::PrivateKey> for PrivateKey {
    fn from(value: schannel::cert_context::PrivateKey) -> Self {
        Self(value)
    }
}
