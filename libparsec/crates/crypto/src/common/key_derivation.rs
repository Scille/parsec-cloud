// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

macro_rules! impl_key_derivation {
    ($key: ident) => {
        crate::impl_key_debug!($key);

        impl AsRef<[u8]> for $key {
            fn as_ref(&self) -> &[u8] {
                &self.0.as_ref()
            }
        }

        impl TryFrom<&[u8]> for $key {
            type Error = $crate::CryptoError;
            fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
                <[u8; Self::SIZE]>::try_from(data)
                    .map(Self::from)
                    .map_err(|_| $crate::CryptoError::KeySize {
                        expected: Self::SIZE,
                        got: data.len(),
                    })
            }
        }

        impl TryFrom<&::serde_bytes::Bytes> for $key {
            type Error = $crate::CryptoError;
            fn try_from(data: &Bytes) -> Result<Self, Self::Error> {
                Self::try_from(data.as_ref())
            }
        }

        impl ::serde::Serialize for $key {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_bytes(self.as_ref())
            }
        }
    };
}

pub(crate) use impl_key_derivation;
