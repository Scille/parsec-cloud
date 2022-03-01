// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

use crate::impl_api_protocol_dump_load;
use parsec_api_types::{BlockID, RealmID};

/*
 * BlockCreateReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockCreateReqSchema {
    pub cmd: String,
    pub block_id: BlockID,
    pub realm_id: RealmID,
    #[serde_as(as = "Bytes")]
    pub block: Vec<u8>,
}

impl_api_protocol_dump_load!(BlockCreateReqSchema);

/*
 * BlockCreateRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum BlockCreateRepSchema {
    Ok,
}

impl_api_protocol_dump_load!(BlockCreateRepSchema);

/*
 * BlockReadReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockReadReqSchema {
    pub cmd: String,
    pub block_id: BlockID,
}

impl_api_protocol_dump_load!(BlockReadReqSchema);

/*
 * BlockReadRepSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum BlockReadRepSchema {
    Ok {
        #[serde_as(as = "Bytes")]
        block: Vec<u8>,
    },
}

impl_api_protocol_dump_load!(BlockReadRepSchema);
