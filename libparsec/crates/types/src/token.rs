// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/// The size of the generated tokens.
const TOKEN_SIZE: usize = 16;

#[derive(thiserror::Error, Debug)]
pub enum TokenDecodeError {
    #[error("The token is not a valid hex string")]
    InvalidHex,
    #[error("Invalid size, got {got} bytes, expected {TOKEN_SIZE} bytes")]
    InvalidSize { got: usize },
}

macro_rules! new_token_type {
    ($name:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name([u8; TOKEN_SIZE]);

        impl Default for $name {
            fn default() -> Self {
                use ::rand::{thread_rng, Rng};

                let bytes = thread_rng().gen::<[u8; TOKEN_SIZE]>();
                Self::from(bytes)
            }
        }

        impl $name {
            #[inline]
            pub fn from_hex(hex: &str) -> Result<Self, TokenDecodeError> {
                let bytes = ::hex::decode(hex).map_err(|_| TokenDecodeError::InvalidHex)?;
                Self::try_from(bytes)
            }

            /// Returns the token as a hex string.
            pub fn hex(&self) -> String {
                ::hex::encode(&self.0)
            }

            pub fn as_bytes(&self) -> &[u8] {
                &self.0
            }
        }

        impl ::std::convert::AsRef<[u8]> for $name {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, concat!(stringify!($name), "(\"{}\")"), self)
            }
        }

        // Note: Display is used for Serialization !
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(&::hex::encode(&self.0))
            }
        }

        impl From<[u8; TOKEN_SIZE]> for $name {
            fn from(bytes: [u8; TOKEN_SIZE]) -> Self {
                Self(bytes)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = TokenDecodeError;

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                Self::from_hex(s)
            }
        }

        impl TryFrom<&[u8]> for $name {
            type Error = TokenDecodeError;

            fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
                if b.len() == TOKEN_SIZE {
                    let mut arr = [0; TOKEN_SIZE];
                    arr.copy_from_slice(b);
                    Ok(Self(arr))
                } else {
                    Err(TokenDecodeError::InvalidSize { got: b.len() })
                }
            }
        }

        impl TryFrom<Vec<u8>> for $name {
            type Error = TokenDecodeError;

            fn try_from(b: Vec<u8>) -> Result<Self, Self::Error> {
                Self::try_from(b.as_slice())
            }
        }

        // Note: FromStr is used for Deserialization !
        impl ::std::str::FromStr for $name {
            type Err = TokenDecodeError;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $name::try_from(s)
            }
        }

        impl ::serde::Serialize for $name {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                ::serde_bytes::serialize(self.as_ref() as &[u8], serializer)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let raw_bytes: Vec<u8> = ::serde_bytes::deserialize(deserializer)?;
                Self::try_from(raw_bytes).map_err(|e| ::serde::de::Error::custom(e.to_string()))
            }
        }
    };
}

new_token_type!(BootstrapToken);
new_token_type!(InvitationToken);
new_token_type!(EmailValidationToken);
new_token_type!(AccountDeletionToken);

#[cfg(test)]
#[path = "../tests/unit/token.rs"]
mod tests;
