// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use serde::{Deserialize, Serialize};

use crate::{compute_from_password, CryptoError, KeyDerivation, Password, SecretKey};

// ⚠️ Changing Argon2id default config also requires updating the fake config generator
// on server side (see `server/parsec/components/account.py`), otherwise it would
// become trivial for a attacker to determine which config is fake !
//
// Note the values has been chosen after libsodium's "MODERATE" configuration,
// they are compatible with what recommends the OWASP Cheat Sheet Series.
// See:
// - https://doc.libsodium.org/password_hashing/default_phf#key-derivation
// - https://pynacl.readthedocs.io/en/latest/password_hashing/#module-level-constants-for-operation-and-memory-cost-tweaking
// - https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#argon2id
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
            memlimit_kb: ARGON2ID_DEFAULT_MEMLIMIT_KB,
            opslimit: ARGON2ID_DEFAULT_OPSLIMIT,
            parallelism: ARGON2ID_DEFAULT_PARALLELISM,
            salt: SecretKey::generate_salt(),
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
    pub fn generate_fake_from_seed(
        // Note: The `email` parameter is not a `libparsec_types::EmailAddress`, this is for two reasons:
        // - `libparsec_types` itself depends on `libparsec_crypto` (i.e. this crate !)
        // - `email` here is only used as-is as a discriminant. Providing an invalid
        //   email has not impact whatsoever on the quality of the output.
        email: &str,
        seed: &SecretKey,
    ) -> Self {
        // Currently there is only a single algorithm type in use and
        // no customization over its parameters, so we only need to generated
        // a fake salt.
        // Note that we have a test (`test_libparsec_actual_password_algorithm`)
        // doing some dark magic in order to detect whenever this changes.

        let raw = seed.mac_512(email.as_bytes());

        let salt = raw[..ARGON2ID_SALTBYTES].to_vec();
        Self::Argon2id {
            memlimit_kb: ARGON2ID_DEFAULT_MEMLIMIT_KB,
            opslimit: ARGON2ID_DEFAULT_OPSLIMIT,
            parallelism: ARGON2ID_DEFAULT_PARALLELISM,
            salt,
        }
    }

    pub fn compute_secret_key(&self, password: &Password) -> Result<SecretKey, CryptoError> {
        let raw: [u8; SecretKey::SIZE] = compute_from_password(self, password)?.into();
        Ok(SecretKey::from(raw))
    }

    pub fn compute_key_derivation(
        &self,
        password: &Password,
    ) -> Result<KeyDerivation, CryptoError> {
        let raw: [u8; KeyDerivation::SIZE] = compute_from_password(self, password)?.into();
        Ok(KeyDerivation::from(raw))
    }
}
