// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use parsec_api_types::{BlockID, RealmID};
use parsec_schema::parsec_schema;

#[parsec_schema]
pub struct BlockCreateReq {
    pub block_id: BlockID,
    pub realm_id: RealmID,
    pub block: Vec<u8>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum BlockCreateRep {
    Ok,
    AlreadyExists,
    NotFound,
    Timeout,
    NotAllowed,
    InMaintenance,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct BlockReadReq {
    pub block_id: BlockID,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum BlockReadRep {
    Ok { block: Vec<u8> },
    NotFound,
    Timeout,
    NotAllowed,
    InMaintenance,
    UnknownError { error: String },
}
