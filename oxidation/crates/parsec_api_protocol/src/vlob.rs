// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::collections::HashMap;

use parsec_api_types::{maybe_field, DateTime, DeviceID, RealmID, VlobID};

/*
 * VlobCreateReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    #[serde_as(as = "Bytes")]
    pub blob: Vec<u8>,
}

/*
 * VlobCreateRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VlobCreateRep {
    Ok,
    AlreadyExists {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    NotAllowed,
    BadEncryptionRevision,
    InMaintenance,
}

/*
 * VlobReadReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobReadReq {
    pub encryption_revision: u64,
    pub vlob_id: VlobID,
    pub version: Option<u64>,
    pub timestamp: Option<DateTime>,
}

/*
 * VlobReadRep
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VlobReadRep {
    Ok {
        version: u64,
        #[serde_as(as = "Bytes")]
        blob: Vec<u8>,
        author: DeviceID,
        timestamp: DateTime,
        // This field is used by the client to figure out if its role certificate cache is up-to-date enough
        // to be able to perform the proper integrity checks on the manifest timestamp.
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        author_last_role_granted_on: Option<DateTime>,
    },
    NotFound {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    NotAllowed,
    BadVersion,
    BadEncryptionRevision,
    InMaintenance,
}

/*
 * VlobUpdateReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobUpdateReq {
    pub encryption_revision: u64,
    pub vlob_id: VlobID,
    pub timestamp: DateTime,
    pub version: u64,
    #[serde_as(as = "Bytes")]
    pub blob: Vec<u8>,
}

/*
 * VlobUpdateRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VlobUpdateRep {
    Ok,
    NotFound {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    NotAllowed,
    BadVersion,
    BadEncryptionRevision,
    InMaintenance,
}

/*
 * VlobPollChangesReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobPollChangesReq {
    pub realm_id: RealmID,
    pub last_checkpoint: u64,
}

/*
 * VlobPollChangesRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VlobPollChangesRep {
    Ok {
        changes: HashMap<VlobID, u64>,
        current_checkpoint: u64,
    },
    NotAllowed,
    NotFound {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    InMaintenance,
}

/*
 * VlobPollChangesRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobListVersionsReq {
    pub vlob_id: VlobID,
}

/*
 * VlobListVersionsRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VlobListVersionsRep {
    Ok {
        versions: HashMap<u64, (DateTime, DeviceID)>,
    },
    NotAllowed,
    NotFound {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    InMaintenance,
}

/*
 * VlobMaintenanceGetReencryptionBatchReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobMaintenanceGetReencryptionBatchReq {
    pub realm_id: RealmID,
    pub encryption_revision: u64,
    pub size: u64,
}

/*
 * ReencryptionBatchEntry
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReencryptionBatchEntry {
    pub vlob_id: VlobID,
    pub version: u64,
    #[serde_as(as = "Bytes")]
    pub blob: Vec<u8>,
}

/*
 * VlobMaintenanceGetReencryptionBatchRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VlobMaintenanceGetReencryptionBatchRep {
    Ok {
        batch: Vec<ReencryptionBatchEntry>,
    },
    NotAllowed,
    NotFound {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    NotInMaintenance {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    BadEncryptionRevision,
    MaintenanceError {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
}

/*
 * VlobMaintenanceSaveReencryptionBatchReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobMaintenanceSaveReencryptionBatchReq {
    pub realm_id: RealmID,
    pub encryption_revision: u64,
    pub batch: Vec<ReencryptionBatchEntry>,
}

/*
 * VlobMaintenanceSaveReencryptionBatchRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VlobMaintenanceSaveReencryptionBatchRep {
    Ok {
        total: u64,
        done: u64,
    },
    NotAllowed,
    NotFound {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    NotInMaintenance {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    BadEncryptionRevision,
    MaintenanceError {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
}
