// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/// The size of the generated tokens.
const TOKEN_SIZE: usize = 16;

#[derive(thiserror::Error, Debug)]
pub enum AccessTokenDecodeError {
    #[error("The token is not a valid hex string")]
    InvalidHex,
    #[error("Invalid size, got {got} bytes, expected {TOKEN_SIZE} bytes")]
    InvalidSize { got: usize },
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccessToken([u8; TOKEN_SIZE]);

impl Default for AccessToken {
    fn default() -> Self {
        use ::rand::{thread_rng, Rng};
        let bytes = thread_rng().gen::<[u8; TOKEN_SIZE]>();
        Self::from(bytes)
    }
}

impl AccessToken {
    #[inline]
    pub fn from_hex(hex: &str) -> Result<Self, AccessTokenDecodeError> {
        let bytes = hex::decode(hex).map_err(|_| AccessTokenDecodeError::InvalidHex)?;
        Self::try_from(bytes)
    }
    #[doc = r" Returns the token as a hex string."]
    pub fn hex(&self) -> String {
        hex::encode(self.0)
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl std::convert::AsRef<[u8]> for AccessToken {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Debug for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, concat!(stringify!(AccessToken), "(\"{}\")"), self)
    }
}

impl std::fmt::Display for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&::hex::encode(self.0))
    }
}

impl From<[u8; TOKEN_SIZE]> for AccessToken {
    fn from(bytes: [u8; TOKEN_SIZE]) -> Self {
        Self(bytes)
    }
}

impl TryFrom<&str> for AccessToken {
    type Error = AccessTokenDecodeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::from_hex(s)
    }
}

impl TryFrom<&[u8]> for AccessToken {
    type Error = AccessTokenDecodeError;

    fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
        if b.len() == TOKEN_SIZE {
            let mut arr = [0; TOKEN_SIZE];
            arr.copy_from_slice(b);
            Ok(Self(arr))
        } else {
            Err(AccessTokenDecodeError::InvalidSize { got: b.len() })
        }
    }
}

impl TryFrom<Vec<u8>> for AccessToken {
    type Error = AccessTokenDecodeError;

    fn try_from(b: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(b.as_slice())
    }
}

impl std::str::FromStr for AccessToken {
    type Err = AccessTokenDecodeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AccessToken::try_from(s)
    }
}

impl serde::Serialize for AccessToken {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_bytes::serialize(self.as_ref() as &[u8], serializer)
    }
}

impl<'de> serde::Deserialize<'de> for AccessToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_bytes: Vec<u8> = serde_bytes::deserialize(deserializer)?;
        Self::try_from(raw_bytes).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

#[cfg(test)]
#[path = "../tests/unit/token.rs"]
mod tests;
