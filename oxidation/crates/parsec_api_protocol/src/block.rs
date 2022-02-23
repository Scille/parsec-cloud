// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};

use parsec_api_types::{BlockID, RealmID};

/*
 * BlockCreateReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockCreateReqSchema {
    pub cmd: String,
    pub block_id: BlockID,
    pub realm_id: RealmID,
    pub block: Vec<u8>,
}

/*
 * BlockCreateRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockCreateRepSchema;

/*
 * BlockReadReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockReadReqSchema {
    pub cmd: String,
    pub block: Vec<u8>,
}

/*
 * BlockReadRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockReadRepSchema {
    pub block: Vec<u8>,
}
