// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use generic_array::{
    typenum::{consts::U64, IsLessOrEqual, LeEq, NonZero},
    ArrayLength, GenericArray,
};
use sodiumoxide::randombytes::randombytes_into;

macro_rules! impl_try_from {
    ($name: ident, $sub_type: expr) => {
        impl TryFrom<&[u8]> for $name {
            type Error = CryptoError;

            fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
                let arr: [u8; Self::SIZE] = v.try_into().map_err(|_| CryptoError::DataSize)?;
                Ok(Self($sub_type(arr)))
            }
        }

        impl From<[u8; Self::SIZE]> for $name {
            fn from(key: [u8; Self::SIZE]) -> Self {
                Self($sub_type(key))
            }
        }
    };
}

pub(super) use impl_try_from;

pub(crate) fn generate_rand(out: &mut [u8]) {
    randombytes_into(out)
}

pub(crate) fn blake2b_hash<'a, Size>(data: impl Iterator<Item = &'a [u8]>) -> GenericArray<u8, Size>
where
    Size: ArrayLength<u8> + IsLessOrEqual<U64>,
    LeEq<Size, U64>: NonZero,
{
    let mut state = libsodium_sys::crypto_generichash_blake2b_state {
        opaque: [0u8; 384usize],
    };
    // TODO: replace `from_exact_iter` by `from_array` once generic-array is updated to v1.0+
    let mut out =
        GenericArray::<u8, Size>::from_exact_iter(std::iter::repeat(0u8).take(Size::USIZE))
            .expect("correct iterator size");

    // SAFETY: Sodiumoxide doesn't expose those methods, so we have to access
    // the libsodium C API directly.
    // this remains safe because we provide bounds defined in Rust land when passing vectors.
    // The only data structure provided by remote code is dropped
    // at the end of the function.
    unsafe {
        libsodium_sys::crypto_generichash_blake2b_init(
            &mut state,
            std::ptr::null(),
            0,
            Size::USIZE,
        );
        for part in data {
            libsodium_sys::crypto_generichash_blake2b_update(
                &mut state,
                part.as_ptr(),
                part.len() as u64,
            );
        }
        libsodium_sys::crypto_generichash_blake2b_final(&mut state, out.as_mut_ptr(), Size::USIZE);
        out
    }
}
