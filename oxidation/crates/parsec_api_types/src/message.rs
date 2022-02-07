// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::ext_types::DateTimeExtFormat;
use crate::{EntryID, EntryName};
use parsec_api_crypto::SecretKey;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum MessageContent {
    #[serde(rename = "sharing.granted")]
    SharingGranted {
        name: EntryName,
        id: EntryID,
        encryption_revision: u32,
        #[serde(with = "DateTimeExtFormat")]
        encrypted_on: DateTime<Utc>,
        key: SecretKey,
        // Don't include role given the only reliable way to get this information
        // is to fetch the realm role certificate from the backend.
        // Besides, we will also need the message sender's realm role certificate
        // to make sure he is an owner.
    },

    #[serde(rename = "sharing.reencrypted")]
    SharingReencrypted {
        // This message is similar to `sharing.granted`. Hence both can be processed
        // interchangeably, which avoid possible concurrency issues when a sharing
        // occurs right before a reencryption.
        name: EntryName,
        id: EntryID,
        encryption_revision: u32,
        #[serde(with = "DateTimeExtFormat")]
        encrypted_on: DateTime<Utc>,
        key: SecretKey,
    },

    #[serde(rename = "sharing.revoked")]
    SharingRevoked { id: EntryID },

    #[serde(rename = "ping")]
    Ping { ping: String },
}
