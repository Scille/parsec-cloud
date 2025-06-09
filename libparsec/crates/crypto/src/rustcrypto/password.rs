// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use argon2::{Algorithm, Argon2, Params, Version};
use generic_array::{
    typenum::{consts::U64, IsLessOrEqual, LeEq, NonZero},
    ArrayLength, GenericArray,
};
use serde::{Deserialize, Serialize};

use crate::{CryptoError, KeyDerivation, Password, SecretKey};

// ⚠️ Changing Argon2id default config also requires updating the fake config generator
// on server side (see `server/parsec/components/account.py`), otherwise it would
// become trivial for a attacker to determine which config is fake !
pub const ARGON2ID_DEFAULT_MEMLIMIT_KB: u32 = 128 * 1024; // 128 Mo
pub const ARGON2ID_DEFAULT_OPSLIMIT: u32 = 3;
// Be careful when changing parallelism: libsodium only supports 1 thread !
pub const ARGON2ID_DEFAULT_PARALLELISM: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Auth method can be of two types:
/// - ClientProvided, for which the client is able to store
///   `auth_method_master_secret` all by itself.
/// - Password, for which the client must obtain some configuration
///   (i.e. this type !) from the server in order to know how
///   to turn the password into `auth_method_master_secret`.
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PasswordAlgorithm {
    Argon2id {
        memlimit_kb: u32,
        opslimit: u32,
        parallelism: u32,
        #[serde(with = "serde_bytes")]
        salt: Vec<u8>,
    },
}

impl PasswordAlgorithm {
    pub fn generate_argon2id() -> Self {
        Self::Argon2id {
            memlimit_kb: ARGON2ID_DEFAULT_MEMLIMIT_KB.into(),
            opslimit: ARGON2ID_DEFAULT_OPSLIMIT.into(),
            parallelism: ARGON2ID_DEFAULT_PARALLELISM.into(),
            salt: SecretKey::generate_salt().into(),
        }
    }

    /// Return a fake password algorithm configuration for the given email.
    ///
    /// The key point here is to prevent an attacker from using the password
    /// algorithm configuration as an oracle to know if a given email address
    /// already has an account on the Parsec server.
    ///
    /// This requires the generated configuration to be:
    /// 1 - Realistic (the result could have been created by a real client)
    /// 2 - Unpredictable (the result cannot be guessed by only knowing the email)
    /// 3 - Stable (multiple calls return the same result)
    pub fn generate_fake_from_seed(email: &str, seed: &SecretKey) -> Self {
        // Currently there is only a single alogrithm type in use and
        // no customization over its parameters, so we only need to generated
        // a fake salt.
        // Note that we have a test (`test_libparsec_actual_password_algorithm`)
        // doing some dark magic in order to detect whenever this changes.

        let raw = seed.mac_512(&email.as_bytes());

        let salt = raw[..16].to_vec();
        Self::Argon2id {
            memlimit_kb: ARGON2ID_DEFAULT_MEMLIMIT_KB.into(),
            opslimit: ARGON2ID_DEFAULT_OPSLIMIT.into(),
            parallelism: ARGON2ID_DEFAULT_PARALLELISM.into(),
            salt,
        }
    }
}

impl PasswordAlgorithm {
    fn compute<Size>(&self, password: &Password) -> Result<GenericArray<u8, Size>, CryptoError>
    where
        Size: ArrayLength<u8> + IsLessOrEqual<U64>,
        LeEq<Size, U64>: NonZero,
    {
        match self {
            PasswordAlgorithm::Argon2id {
                salt,
                opslimit,
                memlimit_kb,
                parallelism,
            } => {
                let mut key = GenericArray::<u8, Size>::default();

                let argon = Argon2::new(
                    Algorithm::Argon2id,
                    Version::V0x13,
                    Params::new(
                        *memlimit_kb,
                        *opslimit,
                        *parallelism,
                        Some(Size::to_usize()),
                    )
                    .map_err(|_| CryptoError::DataSize)?,
                );

                argon
                    .hash_password_into(password.as_bytes(), salt, &mut key)
                    .map_err(|_| CryptoError::DataSize)?;

                Ok(key)
            }
        }
    }

    pub fn compute_secret_key(&self, password: &Password) -> Result<SecretKey, CryptoError> {
        let raw: [u8; SecretKey::SIZE] = self.compute(password)?.into();
        Ok(SecretKey::from(raw))
    }

    pub fn compute_key_derivation(
        &self,
        password: &Password,
    ) -> Result<KeyDerivation, CryptoError> {
        let raw: [u8; KeyDerivation::SIZE] = self.compute(password)?.into();
        Ok(KeyDerivation::from(raw))
    }
}
