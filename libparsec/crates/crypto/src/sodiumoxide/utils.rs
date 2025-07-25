// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use generic_array::{
    typenum::{consts::U64, IsLessOrEqual, LeEq, NonZero},
    ArrayLength, GenericArray,
};

macro_rules! impl_bytes_traits {
    ($name: ident, $sub_type: ty) => {
        impl TryFrom<&[u8]> for $name {
            type Error = CryptoError;

            fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
                <[u8; Self::SIZE]>::try_from(v)
                    .map(Self::from)
                    .map_err(|_| CryptoError::DataSize)
            }
        }

        impl From<[u8; Self::SIZE]> for $name {
            fn from(key: [u8; Self::SIZE]) -> Self {
                Self(<$sub_type>::from(key))
            }
        }

        impl AsRef<[u8]> for $name {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                self.0.as_bytes()
            }
        }
    };
}

pub(crate) use impl_bytes_traits;

#[macro_export]
macro_rules! impl_serde_traits {
    ($name:ident, $as_bytes: expr) => {
        impl TryFrom<&::serde_bytes::Bytes> for $name {
            type Error = CryptoError;

            fn try_from(v: &::serde_bytes::Bytes) -> Result<Self, Self::Error> {
                Self::try_from(v.as_ref())
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_bytes($as_bytes(self))
            }
        }
    };

    ($name:ident) => {
        $crate::impl_serde_traits!($name, AsRef::as_ref);
    };
}

pub(crate) use impl_serde_traits;

pub(crate) fn generate_rand(out: &mut [u8]) {
    libsodium_rs::random::fill_bytes(out)
}

pub(crate) fn blake2b_hash<'a, Size>(data: impl Iterator<Item = &'a [u8]>) -> GenericArray<u8, Size>
where
    Size: ArrayLength<u8> + IsLessOrEqual<U64>,
    LeEq<Size, U64>: NonZero,
{
    let mut state = libsodium_rs::crypto_generichash::blake2b::State::new(None, Size::USIZE)
        .expect("Cannot initialize blake2b state");
    data.for_each(|blob| state.update(blob));
    let raw_hash = state.finalize();
    // TODO: replace `from_exact_iter` by `from_array` once generic-array is updated to v1.0+
    GenericArray::<u8, Size>::from_exact_iter(raw_hash).expect("correct iterator size")
}
