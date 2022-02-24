// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};

use crate::{impl_api_protocol_dump_load, Status};

/*
 * PingReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PingReqSchema {
    pub cmd: String,
    pub ping: String,
}

impl_api_protocol_dump_load!(PingReqSchema);

/*
 * PingRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PingRepSchema {
    pub status: Status,
    pub pong: String,
}

impl_api_protocol_dump_load!(PingRepSchema);
