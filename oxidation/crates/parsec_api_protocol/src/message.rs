// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use parsec_api_types::{DateTimeExtFormat, DeviceID};

/*
 * MessageGetReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageGetReqSchema {
    pub cmd: String,
    pub offset: u64,
}

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
    pub body: Vec<u8>,
}

/*
 * MessageGetRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageGetRepSchema {
    pub messages: Vec<MessageSchema>,
}
