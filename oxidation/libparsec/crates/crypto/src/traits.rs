// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// Traits to make sure both implementation share the same api

use serde_bytes::ByteBuf;

pub(crate) trait BaseHashDigest<'de, 'f>:
    serde::Serialize
    + serde::Deserialize<'de>
    + AsRef<[u8]>
    + TryFrom<&'f [u8]>
    + From<[u8; HashDigest::SIZE]>
    + TryFrom<ByteBuf>
    + Into<ByteBuf>
{
    const ALGORITHM: &'static str;
    const SIZE: usize;

    fn from_data(data: &[u8]) -> Self;
    fn hexdigest(&self) -> String;
}

pub(crate) trait BasePrivateKey {
    const SIZE: usize;
    type PublicKey;
    type SecretKey;

    fn as_bytes(&self) -> &[u8];
    fn public_key(&self) -> Self::PublicKey;
    fn generate() -> Self;
    // TODO: implement API !
    fn decrypt_from_self(&self, ciphered: &[u8]) -> Vec<u8>;
    fn generate_shared_secret_key(&self, peer_public_key: &Self::PublicKey) -> Self::SecretKey;
}

pub(crate) trait BasePublicKey {
    const SIZE: usize;

    fn as_bytes(&self) -> &[u8];
    // TODO: implement API !
    fn encrypt_from_self(&self, data: &[u8]) -> Vec<u8>;
}

pub(crate) trait BaseSecretKey {
    const SIZE: usize;

    fn generate() -> Self;
    fn encrypt(&self, data: &[u8]) -> Vec<u8>;
    fn decrypt(&self, ciphered: &[u8]) -> Vec<u8>;
    fn hmac(&self, data: &[u8], digest_size: usize) -> Vec<u8>;
}

pub(crate) trait BaseSigningKey {
    const SIZE: usize;
    type VerifyKey;

    fn as_bytes(&self) -> &[u8];
    fn verify_key(&self) -> Self::VerifyKey;
    fn generate() -> Self;
    fn verify(self, data: &[u8]) -> Result<Vec<u8>, &'static str>;
}

pub(crate) trait BaseVerifyKey {
    const SIZE: usize;

    fn as_bytes(&self) -> &[u8];
    fn unsecure_unwrap(&self, signed: &[u8]) -> Vec<u8>;
    fn verify(self, signed: &[u8]) -> Result<Vec<u8>, &'static str>;
}

// trait BytesSerialized : std::marker::Sized {
//     const SIZE: usize;

//     fn as_bytes(&self) -> &[u8];
//     fn from_bytes(data: &[u8]) -> Result<Self, &'static str>;
// }

// use serde::ser::{Serialize, Serializer};

// impl<T: BytesSerialized> Serialize for T {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_bytes(self.as_bytes())
//     }
// }

// use serde::de::{self, Deserialize, Deserializer, Visitor};

// impl<'de, T: BytesSerialized> Deserialize<'de> for T {
//     fn deserialize<D, U>(deserializer: D) -> Result<T, D::Error>
//     where
//         D: Deserializer<'de>,
//         U: BytesSerializedVisitor<T>,
//     {
//         // TODO: use deserialize_byte_buf instead ?
//         deserializer.deserialize_bytes(U)
//     }
// }

// struct BytesSerializedVisitor<T: BytesSerialized> (std::marker::PhantomData<T>);

// impl<'de, T: BytesSerialized> Visitor<'de> for BytesSerializedVisitor<T> {
//     type Value = T;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str(concat!("a ", stringify!(T::size()), " long bytes array"))
//     }

//     fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
//     where
//         E: de::Error,
//     {
//         T::from_bytes(value).map_err(|e| E::custom(e))
//     }
// }

// macro_rules! impl_serde {
//     ($name:ident) => {
//         use serde::de::{self, Deserialize, Deserializer, Visitor};
//         use serde::ser::{Serialize, Serializer};

//         impl<T: $name> Serialize for T {
//             fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//             where
//                 S: Serializer,
//             {
//                 serializer.serialize_bytes(&self.0.0)
//             }
//         }

//         paste::paste! {
//             impl<T: $name, 'de> Deserialize<'de> for T {
//                 fn deserialize<D>(deserializer: D) -> Result<T, D::Error>
//                 where
//                     D: Deserializer<'de>,
//                 {
//                     deserializer.deserialize_bytes([<T Visitor>])
//                 }
//             }

//             struct [<T Visitor>];

//             impl<T: $name, 'de> Visitor<'de> for [<T Visitor>] {
//                 type Value = T;

//                 fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                     formatter.write_str(concat!("a ", stringify!(T::size()), " long bytes array"))
//                 }

//                 fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
//                 where
//                     E: de::Error,
//                 {
//                     T::try_from(value).map_err(|e| E::custom(e))
//                 }
//             }
//         }

//     };
// }
