// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

use parsec_api_types::{BlockID, RealmID};

/*
 * BlockCreateReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockCreateReq {
    pub block_id: BlockID,
    pub realm_id: RealmID,
    #[serde_as(as = "Bytes")]
    pub block: Vec<u8>,
}

/*
 * BlockCreateRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum BlockCreateRep {
    Ok,
    AlreadyExists,
    NotFound,
    Timeout,
    NotAllowed,
    InMaintenance,
}

/*
 * BlockReadReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockReadReq {
    pub block_id: BlockID,
}

/*
 * BlockReadRep
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum BlockReadRep {
    Ok {
        #[serde_as(as = "Bytes")]
        block: Vec<u8>,
    },
    NotFound,
    Timeout,
    NotAllowed,
    InMaintenance,
}
