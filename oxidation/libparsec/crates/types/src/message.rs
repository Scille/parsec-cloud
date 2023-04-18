// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::io::{Read, Write};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use serde::{Deserialize, Serialize};
use serde_with::*;

use libparsec_crypto::{PrivateKey, PublicKey, SecretKey, SigningKey, VerifyKey};

use crate::{DateTime, DeviceID, EntryID, EntryName};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum MessageContent {
    #[serde(rename = "sharing.granted")]
    SharingGranted {
        author: DeviceID,
        timestamp: DateTime,

        name: EntryName,
        id: EntryID,
        encryption_revision: u32,
        encrypted_on: DateTime,
        key: SecretKey,
        // Don't include role given the only reliable way to get this information
        // is to fetch the realm role certificate from the backend.
        // Besides, we will also need the message sender's realm role certificate
        // to make sure he is an owner.
    },

    #[serde(rename = "sharing.reencrypted")]
    SharingReencrypted {
        author: DeviceID,
        timestamp: DateTime,

        // This message is similar to `sharing.granted`. Hence both can be processed
        // interchangeably, which avoid possible concurrency issues when a sharing
        // occurs right before a reencryption.
        name: EntryName,
        id: EntryID,
        encryption_revision: u32,
        encrypted_on: DateTime,
        key: SecretKey,
    },

    #[serde(rename = "sharing.revoked")]
    SharingRevoked {
        author: DeviceID,
        timestamp: DateTime,

        id: EntryID,
    },

    #[serde(rename = "ping")]
    Ping {
        author: DeviceID,
        timestamp: DateTime,

        ping: String,
    },
}

impl MessageContent {
    pub fn decrypt_verify_and_load_for(
        ciphered: &[u8],
        recipient_privkey: &PrivateKey,
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_timestamp: DateTime,
    ) -> Result<MessageContent, &'static str> {
        let signed = recipient_privkey
            .decrypt_from_self(ciphered)
            .map_err(|_| "Invalid encryption")?;
        let compressed = author_verify_key
            .verify(&signed)
            .map_err(|_| "Invalid signature")?;
        let mut serialized = vec![];
        ZlibDecoder::new(&compressed[..])
            .read_to_end(&mut serialized)
            .map_err(|_| "Invalid compression")?;
        let data: MessageContent =
            rmp_serde::from_slice(&serialized).map_err(|_| "Invalid serialization")?;
        let (author, &timestamp) = match &data {
            MessageContent::SharingGranted {
                author, timestamp, ..
            } => (author, timestamp),
            MessageContent::SharingReencrypted {
                author, timestamp, ..
            } => (author, timestamp),
            MessageContent::SharingRevoked {
                author, timestamp, ..
            } => (author, timestamp),
            MessageContent::Ping {
                author, timestamp, ..
            } => (author, timestamp),
        };
        if author != expected_author {
            Err("Invalid author")
        } else if timestamp != expected_timestamp {
            Err("Invalid timestamp")
        } else {
            Ok(data)
        }
    }

    pub fn dump_sign_and_encrypt_for(
        &self,
        author_signkey: &SigningKey,
        recipient_pubkey: &PublicKey,
    ) -> Vec<u8> {
        let serialized = rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
        let mut e = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        e.write_all(&serialized).unwrap_or_else(|_| unreachable!());
        let compressed = e.finish().unwrap_or_else(|_| unreachable!());
        let signed = author_signkey.sign(&compressed);
        recipient_pubkey.encrypt_for_self(&signed)
    }
}
