// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use miniserde::Deserialize;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

#[derive(Debug, Deserialize)]
pub(crate) enum Serializing {
    #[serde(rename = "msgpack")]
    Msgpack,
    #[serde(rename = "zip+msgpack")]
    ZipMsgpack,
    #[serde(rename = "sign+zip+msgpack")]
    SignZipMsgpack,
}

impl Serializing {
    pub(crate) fn quote(&self, name: &Ident) -> TokenStream {
        match self {
            Serializing::Msgpack => {
                quote! {
                    impl #name {
                        pub fn dump_and_encrypt(
                            &self,
                            key: &::parsec_api_crypto::SecretKey
                        ) -> Vec<u8> {
                            let serialized =
                                ::rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
                            key.encrypt(&serialized)
                        }

                        pub fn decrypt_and_load(
                            encrypted: &[u8],
                            key: &::parsec_api_crypto::SecretKey,
                        ) -> Result<Self, &'static str> {
                            let serialized = key.decrypt(encrypted).map_err(|_| "Invalid encryption")?;
                            ::rmp_serde::from_read_ref(&serialized).map_err(|_| "Invalid serialization")
                        }
                    }
                }
            }
            Serializing::ZipMsgpack => {
                quote! {
                    impl #name {
                        pub fn dump_and_encrypt(&self, key: &::parsec_api_crypto::SecretKey) -> Vec<u8> {
                            use std::io::Write;
                            let serialized =
                                ::rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
                            let mut e =
                                ::flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                            e.write_all(&serialized).unwrap_or_else(|_| unreachable!());
                            let compressed = e.finish().unwrap_or_else(|_| unreachable!());
                            key.encrypt(&compressed)
                        }

                        pub fn decrypt_and_load(
                            encrypted: &[u8],
                            key: &::parsec_api_crypto::SecretKey,
                        ) -> Result<Self, &'static str> {
                            use std::io::Read;
                            let compressed = key.decrypt(encrypted).map_err(|_| "Invalid encryption")?;
                            let mut serialized = vec![];
                            ::flate2::read::ZlibDecoder::new(&compressed[..])
                                .read_to_end(&mut serialized)
                                .map_err(|_| "Invalid compression")?;
                            Ok(::rmp_serde::from_read_ref(&serialized).map_err(|_| "Invalid serialization")?)
                        }
                    }
                }
            }
            Serializing::SignZipMsgpack => {
                quote! {
                    impl #name {
                        pub fn dump_sign_and_encrypt(
                            &self,
                            author_signkey: &::parsec_api_crypto::SigningKey,
                            key: &::parsec_api_crypto::SecretKey,
                        ) -> Vec<u8> {
                            use ::flate2::Compression;
                            use ::std::io::{Read, Write};

                            let serialized =
                                ::rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
                            let mut e = ::flate2::write::ZlibEncoder::new(Vec::new(), Compression::default());
                            e.write_all(&serialized).unwrap_or_else(|_| unreachable!());
                            let compressed = e.finish().unwrap_or_else(|_| unreachable!());
                            let signed = author_signkey.sign(&compressed);
                            key.encrypt(&signed)
                        }

                        pub fn decrypt_verify_and_load(
                            encrypted: &[u8],
                            key: &::parsec_api_crypto::SecretKey,
                            author_verify_key: &::parsec_api_crypto::VerifyKey,
                            expected_author: &::parsec_api_types::DeviceID,
                            expected_timestamp: ::parsec_api_types::DateTime,
                        ) -> Result<Self, ::parsec_api_types::DataError> {
                            use ::parsec_api_types::DataError;
                            use ::std::io::{Read, Write};

                            let signed = key
                            .serializing let mut serialized = vec![];

                            ::flate2::read::ZlibDecoder::new(&compressed[..])
                                .read_to_end(&mut serialized)
                                .map_err(|_| DataError::InvalidCompression)?;

                            let obj = rmp_serde::from_read_ref::<_, Self>(&serialized)
                                .map_err(|_| DataError::InvalidSerialization)?;

                            if obj.author != *expected_author {
                                Err(DataError::UnexpectedAuthor {
                                    expected: expected_author.clone(),
                                    got: obj.author,
                                })
                            } else if obj.timestamp != expected_timestamp {
                                Err(DataError::UnexpectedTimestamp {
                                    expected: expected_timestamp,
                                    got: obj.timestamp,
                                })
                            } else {
                                Ok(obj)
                            }
                        }
                    }
                }
            }
        }
    }
}
