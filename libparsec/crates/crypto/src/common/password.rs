// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use serde::{Deserialize, Serialize};

use crate::{
    blake2b_hash, compute_from_password, generate_rand, CryptoError, KeyDerivation, Password,
    SecretKey,
};

// https://github.com/sodiumoxide/sodiumoxide/blob/3057acb1a030ad86ed8892a223d64036ab5e8523/libsodium-sys/src/sodium_bindings.rs#L137
pub const ARGON2ID_SALTBYTES: usize = 16;

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

/// All the configuration needed to derive a key from a password !
///
/// Note this type cannot be (de)serialized, instead `TrustedPasswordAlgorithm` &
/// `UntrustedPasswordAlgorithm` should be used depending of the use-case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordAlgorithm {
    Argon2id {
        memlimit_kb: u32,
        opslimit: u32,
        parallelism: u32,
        salt: [u8; ARGON2ID_SALTBYTES],
    },
}

impl PasswordAlgorithm {
    pub fn generate_argon2id() -> Self {
        let mut salt = [0; ARGON2ID_SALTBYTES];
        generate_rand(&mut salt);

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

/// ⚠️ Only use this type for scenario where the derived key can be validated
/// right away without the involvement of any 3rd part.
///
/// Typical use-case is `DeviceFilePassword` that both contains the password
/// algorithm and the `LocalDevice` encrypted with the derived key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrustedPasswordAlgorithm {
    Argon2id {
        memlimit_kb: u32,
        opslimit: u32,
        parallelism: u32,
        #[serde(with = "serde_bytes")]
        salt: [u8; ARGON2ID_SALTBYTES],
    },
}

impl From<TrustedPasswordAlgorithm> for PasswordAlgorithm {
    fn from(value: TrustedPasswordAlgorithm) -> Self {
        match value {
            TrustedPasswordAlgorithm::Argon2id {
                memlimit_kb,
                opslimit,
                parallelism,
                salt,
            } => PasswordAlgorithm::Argon2id {
                memlimit_kb,
                opslimit,
                parallelism,
                salt,
            },
        }
    }
}

impl From<PasswordAlgorithm> for TrustedPasswordAlgorithm {
    fn from(value: PasswordAlgorithm) -> Self {
        match value {
            PasswordAlgorithm::Argon2id {
                memlimit_kb,
                opslimit,
                parallelism,
                salt,
            } => TrustedPasswordAlgorithm::Argon2id {
                memlimit_kb,
                opslimit,
                parallelism,
                salt,
            },
        }
    }
}

/// Password algorithm configuration provided by a 3rd party, with no way for us to
/// validate if the derived key is what we expected.
///
/// Typical use-case for this is the account auth method which can be of two types:
/// - ClientProvided, for which the client is able to store
///   `auth_method_master_secret` all by itself.
/// - Password, for which the client must obtain some configuration
///   (i.e. this type!) from the server in order to know how
///   to turn the password into `auth_method_master_secret`.
///
/// Before being able to use it to derive a key, this configuration must be
/// validated to compute the salt and ensure the provided parameters are at
/// least as strong as the defaults.
///
/// Considering what happen during account authentication:
/// 1. The password algorithm is provided by the server.
/// 2. The client uses it to generate the `auth_method_master_secret`.
/// 3. The client uses `auth_method_master_secret` to generate `auth_method_id`.
/// 4. The client transmits `auth_method_id` to the server during subsequent
///    authenticated requests.
///
/// So by providing a very weak password algorithm (or a constant salt), the server
/// would be able to trick the client into providing him an oracle (i.e.
/// `auth_method_id`) that is much simpler to crack using brute force...
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UntrustedPasswordAlgorithm {
    Argon2id {
        memlimit_kb: u32,
        opslimit: u32,
        parallelism: u32,
    },
}

const UNTRUSTED_PASSWORD_ARGON2IL_SALT_PREFIX: &[u8] = b"PARSEC_PASSWORD_ARGON2ID.";

impl UntrustedPasswordAlgorithm {
    pub fn validate(self, email: &str) -> Result<PasswordAlgorithm, CryptoError> {
        match self {
            UntrustedPasswordAlgorithm::Argon2id {
                memlimit_kb,
                opslimit,
                parallelism,
            } => {
                if memlimit_kb < ARGON2ID_DEFAULT_MEMLIMIT_KB
                    || opslimit < ARGON2ID_DEFAULT_OPSLIMIT
                    || parallelism < ARGON2ID_DEFAULT_PARALLELISM
                {
                    return Err(CryptoError::Algorithm("Config too weak".to_string()));
                }

                let salt: [u8; ARGON2ID_SALTBYTES] = blake2b_hash(
                    [UNTRUSTED_PASSWORD_ARGON2IL_SALT_PREFIX, email.as_bytes()].into_iter(),
                )
                .into();

                Ok(PasswordAlgorithm::Argon2id {
                    memlimit_kb,
                    opslimit,
                    parallelism,
                    salt,
                })
            }
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
        _email: &str,
        _seed: &SecretKey,
    ) -> Self {
        // Currently there is only a single algorithm type in use and
        // no customization over its parameters.
        // Note that we have a test (`test_libparsec_actual_password_algorithm`)
        // doing some dark magic in order to detect whenever this changes.

        Self::Argon2id {
            memlimit_kb: ARGON2ID_DEFAULT_MEMLIMIT_KB,
            opslimit: ARGON2ID_DEFAULT_OPSLIMIT,
            parallelism: ARGON2ID_DEFAULT_PARALLELISM,
        }
    }
}
