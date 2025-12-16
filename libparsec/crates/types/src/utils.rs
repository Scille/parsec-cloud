// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/*
 * Helpers
 */

macro_rules! impl_dump {
    ($name:ident) => {
        impl $name {
            pub fn dump(&self) -> Vec<u8> {
                format_v0_dump(&self)
            }
        }
    };
}

macro_rules! impl_dump_and_encrypt {
    ($name:ident) => {
        impl $name {
            pub fn dump_and_encrypt(&self, key: &::libparsec_crypto::SecretKey) -> Vec<u8> {
                let serialized = format_v0_dump(&self);
                key.encrypt(&serialized)
            }
        }
    };
}

macro_rules! impl_load {
    ($name:ident) => {
        impl $name {
            pub fn load(serialized: &[u8]) -> Result<$name, DataError> {
                format_vx_load(&serialized)
            }
        }
    };
}

macro_rules! impl_decrypt_and_load {
    ($name:ident) => {
        impl $name {
            pub fn decrypt_and_load(
                encrypted: &[u8],
                key: &::libparsec_crypto::SecretKey,
            ) -> Result<$name, DataError> {
                let serialized = key.decrypt(encrypted).map_err(|_| DataError::Decryption)?;
                format_vx_load(&serialized)
            }
        }
    };
}

pub(super) use impl_decrypt_and_load;
pub(super) use impl_dump;
pub(super) use impl_dump_and_encrypt;
pub(super) use impl_load;
