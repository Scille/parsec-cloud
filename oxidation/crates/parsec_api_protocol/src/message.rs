// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

use crate::{impl_api_protocol_dump_load, Status};
use parsec_api_types::{DateTimeExtFormat, DeviceID};

/*
 * MessageGetReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageGetReqSchema {
    pub cmd: String,
    pub offset: u64,
}

impl_api_protocol_dump_load!(MessageGetReqSchema);

/*
 * MessageSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageSchema {
    pub count: u64,
    pub sender: DeviceID,
    #[serde_as(as = "DateTimeExtFormat")]
    pub timestamp: DateTime<Utc>,
    #[serde_as(as = "Bytes")]
    pub body: Vec<u8>,
}

/*
 * MessageGetRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageGetRepSchema {
    pub status: Status,
    pub messages: Vec<MessageSchema>,
}

impl_api_protocol_dump_load!(MessageGetRepSchema);
