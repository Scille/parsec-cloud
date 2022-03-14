// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::collections::HashMap;

use crate::impl_api_protocol_dump_load;
use parsec_api_types::{maybe_field, DateTime, DeviceID, RealmID, VlobID};

/*
 * VlobCreateReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobCreateReq {
    pub cmd: String,
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

impl_api_protocol_dump_load!(VlobCreateReq);

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

impl_api_protocol_dump_load!(VlobCreateRep);

/*
 * VlobReadReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobReadReq {
    pub cmd: String,
    pub encryption_revision: u64,
    pub vlob_id: VlobID,
    pub version: Option<u64>,
    pub timestamp: Option<DateTime>,
}

impl_api_protocol_dump_load!(VlobReadReq);

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

impl_api_protocol_dump_load!(VlobReadRep);

/*
 * VlobUpdateReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobUpdateReq {
    pub cmd: String,
    pub encryption_revision: u64,
    pub vlob_id: VlobID,
    pub timestamp: DateTime,
    pub version: u64,
    #[serde_as(as = "Bytes")]
    pub blob: Vec<u8>,
}

impl_api_protocol_dump_load!(VlobUpdateReq);

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

impl_api_protocol_dump_load!(VlobUpdateRep);

/*
 * VlobPollChangesReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobPollChangesReq {
    pub cmd: String,
    pub realm_id: RealmID,
    pub last_checkpoint: u64,
}

impl_api_protocol_dump_load!(VlobPollChangesReq);

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

impl_api_protocol_dump_load!(VlobPollChangesRep);

/*
 * VlobPollChangesRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobListVersionsReq {
    pub cmd: String,
    pub vlob_id: VlobID,
}

impl_api_protocol_dump_load!(VlobListVersionsReq);

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

impl_api_protocol_dump_load!(VlobListVersionsRep);

/*
 * VlobMaintenanceGetReencryptionBatchReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobMaintenanceGetReencryptionBatchReq {
    pub cmd: String,
    pub realm_id: RealmID,
    pub encryption_revision: u64,
    pub size: u64,
}

impl_api_protocol_dump_load!(VlobMaintenanceGetReencryptionBatchReq);

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

impl_api_protocol_dump_load!(VlobMaintenanceGetReencryptionBatchRep);

/*
 * VlobMaintenanceSaveReencryptionBatchReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobMaintenanceSaveReencryptionBatchReq {
    pub cmd: String,
    pub realm_id: RealmID,
    pub encryption_revision: u64,
    pub batch: Vec<ReencryptionBatchEntry>,
}

impl_api_protocol_dump_load!(VlobMaintenanceSaveReencryptionBatchReq);

/*
 * VlobMaintenanceSaveReencryptionBatchReq
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

impl_api_protocol_dump_load!(VlobMaintenanceSaveReencryptionBatchRep);
