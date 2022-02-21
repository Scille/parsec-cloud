// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::{DateTime, Utc};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use parsec_api_crypto::{SecretKey, SigningKey, VerifyKey};
use serde::{de::DeserializeOwned, Serialize};
use std::io::{Read, Write};

use crate::DeviceID;

pub trait Verify
where
    Self: Sized,
{
    fn author(&self) -> &DeviceID;
    fn timestamp(&self) -> DateTime<Utc>;
    fn check(
        self,
        expected_author: &DeviceID,
        expected_timestamp: DateTime<Utc>,
    ) -> Result<Self, &'static str> {
        if self.author() != expected_author {
            Err("Unexpected author")
        } else if self.timestamp() != expected_timestamp {
            Err("Unexpected timestamp")
        } else {
            Ok(self)
        }
    }
}

pub trait Encrypt
where
    Self: Sized + Serialize + DeserializeOwned,
{
    fn dump_and_encrypt(&self, key: &SecretKey) -> Vec<u8> {
        let serialized = rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
        key.encrypt(&serialized)
    }
    fn decrypt_and_load(encrypted: &[u8], key: &SecretKey) -> Result<Self, &'static str> {
        let serialized = key.decrypt(encrypted).map_err(|_| "Invalid encryption")?;
        rmp_serde::from_read_ref::<_, Self>(&serialized).map_err(|_| "Invalid serialization")
    }
}

pub trait CompSignEncrypt: Verify
where
    Self: Serialize + DeserializeOwned,
{
    fn dump_sign_and_encrypt(&self, author_signkey: &SigningKey, key: &SecretKey) -> Vec<u8> {
        let serialized = rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(&serialized).unwrap_or_else(|_| unreachable!());
        let compressed = e.finish().unwrap_or_else(|_| unreachable!());
        let signed = author_signkey.sign(&compressed);
        key.encrypt(&signed)
    }

    fn decrypt_verify_and_load(
        encrypted: &[u8],
        key: &SecretKey,
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_timestamp: DateTime<Utc>,
    ) -> Result<Self, &'static str> {
        let signed = key.decrypt(encrypted).map_err(|_| "Invalid encryption")?;
        let compressed = author_verify_key
            .verify(&signed)
            .map_err(|_| "Invalid signature")?;
        let mut serialized = vec![];

        ZlibDecoder::new(&compressed[..])
            .read_to_end(&mut serialized)
            .map_err(|_| "Invalid compression")?;

        let obj = rmp_serde::from_read_ref::<_, Self>(&serialized)
            .map_err(|_| "Invalid serialization")?;

        obj.check(expected_author, expected_timestamp)
    }
}
