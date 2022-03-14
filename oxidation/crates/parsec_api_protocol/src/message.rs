// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

use crate::impl_api_protocol_dump_load;
use parsec_api_types::{DateTime, DeviceID};

/*
 * MessageGetReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageGetReq {
    pub cmd: String,
    pub offset: u64,
}

impl_api_protocol_dump_load!(MessageGetReq);

/*
 * Message
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub count: u64,
    pub sender: DeviceID,
    pub timestamp: DateTime,
    #[serde_as(as = "Bytes")]
    pub body: Vec<u8>,
}

/*
 * MessageGetRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum MessageGetRep {
    Ok { messages: Vec<Message> },
}

impl_api_protocol_dump_load!(MessageGetRep);
