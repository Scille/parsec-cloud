// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};

use crate::impl_api_protocol_dump_load;

/*
 * PingReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PingReq {
    pub cmd: String,
    pub ping: String,
}

impl_api_protocol_dump_load!(PingReq);

/*
 * PingRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum PingRep {
    Ok { pong: String },
}

impl_api_protocol_dump_load!(PingRep);
