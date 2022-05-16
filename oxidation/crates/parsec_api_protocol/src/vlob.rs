// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::collections::HashMap;

use parsec_api_types::{DateTime, DeviceID, RealmID, VlobID};
use parsec_schema::parsec_schema;

#[parsec_schema]
pub struct VlobCreateReq {
    pub realm_id: RealmID,
    pub encryption_revision: u64,
    pub vlob_id: VlobID,
    // If blob contains a signed message, it timestamp cannot be directly enforced
    // by the backend (given the message is probably also encrypted).
    // Hence the timestamp is passed in clear so backend can reject the message
    // if it considers the timestamp invalid. On top of that each client asking
    // for the message will receive the declared timestamp to check against
    // the actual timestamp within the message.
    pub timestamp: DateTime,
    pub blob: Vec<u8>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VlobCreateRep {
    Ok,
    AlreadyExists { reason: Option<String> },
    NotAllowed,
    BadEncryptionRevision,
    InMaintenance,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct VlobReadReq {
    pub encryption_revision: u64,
    pub vlob_id: VlobID,
    pub version: Option<u64>,
    pub timestamp: Option<DateTime>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VlobReadRep {
    Ok {
        version: u64,
        blob: Vec<u8>,
        author: DeviceID,
        timestamp: DateTime,
        // This field is used by the client to figure out if its role certificate cache is up-to-date enough
        // to be able to perform the proper integrity checks on the manifest timestamp.
        author_last_role_granted_on: Option<DateTime>,
    },
    NotFound {
        reason: Option<String>,
    },
    NotAllowed,
    BadVersion,
    BadEncryptionRevision,
    InMaintenance,
    UnknownError {
        error: String,
    },
}

#[parsec_schema]
pub struct VlobUpdateReq {
    pub encryption_revision: u64,
    pub vlob_id: VlobID,
    pub timestamp: DateTime,
    pub version: u64,
    pub blob: Vec<u8>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VlobUpdateRep {
    Ok,
    NotFound { reason: Option<String> },
    NotAllowed,
    BadVersion,
    BadEncryptionRevision,
    InMaintenance,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct VlobPollChangesReq {
    pub realm_id: RealmID,
    pub last_checkpoint: u64,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VlobPollChangesRep {
    Ok {
        changes: HashMap<VlobID, u64>,
        current_checkpoint: u64,
    },
    NotAllowed,
    NotFound {
        reason: Option<String>,
    },
    InMaintenance,
    UnknownError {
        error: String,
    },
}

#[parsec_schema]
pub struct VlobListVersionsReq {
    pub vlob_id: VlobID,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VlobListVersionsRep {
    Ok {
        versions: HashMap<u64, (DateTime, DeviceID)>,
    },
    NotAllowed,
    NotFound {
        reason: Option<String>,
    },
    InMaintenance,
    UnknownError {
        error: String,
    },
}

#[parsec_schema]
pub struct VlobMaintenanceGetReencryptionBatchReq {
    pub realm_id: RealmID,
    pub encryption_revision: u64,
    pub size: u64,
}

#[parsec_schema]
pub struct ReencryptionBatchEntry {
    pub vlob_id: VlobID,
    pub version: u64,
    pub blob: Vec<u8>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VlobMaintenanceGetReencryptionBatchRep {
    Ok { batch: Vec<ReencryptionBatchEntry> },
    NotAllowed,
    NotFound { reason: Option<String> },
    NotInMaintenance { reason: Option<String> },
    BadEncryptionRevision,
    MaintenanceError { reason: Option<String> },
    UnknownError { error: String },
}

#[parsec_schema]
pub struct VlobMaintenanceSaveReencryptionBatchReq {
    pub realm_id: RealmID,
    pub encryption_revision: u64,
    pub batch: Vec<ReencryptionBatchEntry>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VlobMaintenanceSaveReencryptionBatchRep {
    Ok { total: u64, done: u64 },
    NotAllowed,
    NotFound { reason: Option<String> },
    NotInMaintenance { reason: Option<String> },
    BadEncryptionRevision,
    MaintenanceError { reason: Option<String> },
    UnknownError { error: String },
}
