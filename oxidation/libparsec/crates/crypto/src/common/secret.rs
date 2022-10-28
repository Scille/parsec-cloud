// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

/// 34 symbols to 32 values due to 0/O and 1/I
pub(crate) const RECOVERY_PASSPHRASE_SYMBOLS: [char; 34] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7',
];

macro_rules! impl_secret_key {
    ($key: ident) => {
        crate::impl_key_debug!($key);

        impl $key {
            pub fn generate_recovery_passphrase() -> (String, Self) {
                let key = Self::generate();
                let b32 =
                    ::base32::encode(::base32::Alphabet::RFC4648 { padding: false }, key.as_ref());
                let passphrase = b32
                    .as_bytes()
                    .chunks(4)
                    .map(std::str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .expect("Unreachable because variable b32 is a utf8 string")
                    .join("-");
                (passphrase, key)
            }

            pub fn from_recovery_passphrase(passphrase: &str) -> crate::CryptoResult<Self> {
                // Lowercase is not allowed in theory, but it's too tempting to fix this here ;-)
                let passphrase = passphrase.to_uppercase();
                // Filter out any unknown characters, this is typically useful to remove
                // the `-` and whitespaces.
                // Note we also discard possible typos from the user (for instance if he types
                // a `8` or a `9`), but this is no big deal given 1) it should not happen
                // because GUI should use `RECOVERY_PASSPHRASE_SYMBOLS` to prevent user
                // from being able to provide invalid characters and 2) it will most likely
                // lead to a bad password anyway
                let mut b32 = passphrase
                    .chars()
                    .filter(|c| crate::common::RECOVERY_PASSPHRASE_SYMBOLS.contains(c))
                    .map(|c| match c {
                        '0' => 'O',
                        '1' => 'I',
                        _ => c,
                    })
                    .collect::<String>();
                // Add padding
                for _ in -(passphrase.len() as i32) % 8..0 {
                    b32.push('=');
                }

                let rawkey = ::base32::decode(::base32::Alphabet::RFC4648 { padding: true }, &b32)
                    .expect("Unreachable due to construction of variable b32");

                Self::try_from(&rawkey[..])
            }
        }

        impl AsRef<[u8]> for $key {
            fn as_ref(&self) -> &[u8] {
                &self.0.as_ref()
            }
        }

        impl TryFrom<&[u8]> for $key {
            type Error = CryptoError;
            fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
                <[u8; Self::SIZE]>::try_from(data)
                    .map(Self::from)
                    .map_err(|_| CryptoError::DataSize)
            }
        }

        impl TryFrom<&::serde_bytes::Bytes> for SecretKey {
            type Error = CryptoError;
            fn try_from(data: &Bytes) -> Result<Self, Self::Error> {
                Self::try_from(data.as_ref())
            }
        }

        impl ::serde::Serialize for SecretKey {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_bytes(self.as_ref())
            }
        }
    };
}

pub(crate) use impl_secret_key;
