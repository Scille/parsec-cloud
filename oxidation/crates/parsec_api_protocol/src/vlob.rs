// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::collections::HashMap;

use crate::{impl_api_protocol_dump_load, Status};
use parsec_api_types::{DateTimeExtFormat, DeviceID, RealmID, VlobID};

/*
 * VlobCreateReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobCreateReqSchema {
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
    #[serde_as(as = "DateTimeExtFormat")]
    pub timestamp: DateTime<Utc>,
    #[serde_as(as = "Bytes")]
    pub blob: Vec<u8>,
}

impl_api_protocol_dump_load!(VlobCreateReqSchema);

/*
 * VlobCreateRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobCreateRepSchema {
    pub status: Status,
}

impl_api_protocol_dump_load!(VlobCreateRepSchema);

/*
 * VlobReadReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobReadReqSchema {
    pub cmd: String,
    pub encryption_revision: u64,
    pub vlob_id: VlobID,
    pub version: Option<u64>,
    #[serde_as(as = "Option<DateTimeExtFormat>")]
    pub timestamp: Option<DateTime<Utc>>,
}

impl_api_protocol_dump_load!(VlobReadReqSchema);

/*
 * VlobReadRepSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobReadRepSchema {
    pub status: Status,
    pub version: u64,
    #[serde_as(as = "Bytes")]
    pub blob: Vec<u8>,
    pub author: DeviceID,
    #[serde_as(as = "DateTimeExtFormat")]
    pub timestamp: DateTime<Utc>,
    // This field is used by the client to figure out if its role certificate cache is up-to-date enough
    // to be able to perform the proper integrity checks on the manifest timestamp.
    // The `missing=None` argument is used to provide compatibilty of new clients with old backends.
    // New in API version 2.3
    #[serde_as(as = "Option<DateTimeExtFormat>")]
    pub author_last_role_granted_on: Option<DateTime<Utc>>,
}

impl_api_protocol_dump_load!(VlobReadRepSchema);

/*
 * VlobUpdateReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobUpdateReqSchema {
    pub cmd: String,
    pub encryption_revision: u64,
    pub vlob_id: VlobID,
    #[serde_as(as = "DateTimeExtFormat")]
    pub timestamp: DateTime<Utc>,
    pub version: u64,
    #[serde_as(as = "Bytes")]
    pub blob: Vec<u8>,
}

impl_api_protocol_dump_load!(VlobUpdateReqSchema);

/*
 * VlobUpdateRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobUpdateRepSchema {
    pub status: Status,
}

impl_api_protocol_dump_load!(VlobUpdateRepSchema);

/*
 * VlobPollChangesReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobPollChangesReqSchema {
    pub cmd: String,
    pub realm_id: RealmID,
    pub last_checkpoint: u64,
}

impl_api_protocol_dump_load!(VlobPollChangesReqSchema);

/*
 * VlobPollChangesRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobPollChangesRepSchema {
    pub status: Status,
    pub changes: HashMap<VlobID, u64>,
    pub current_checkpoint: u64,
}

impl_api_protocol_dump_load!(VlobPollChangesRepSchema);

/*
 * VlobPollChangesRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobListVersionsReqSchema {
    pub cmd: String,
    pub vlob_id: VlobID,
}

impl_api_protocol_dump_load!(VlobListVersionsReqSchema);

/*
 * VlobListVersionsRepSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobListVersionsRepSchema {
    pub status: Status,
    #[serde_as(as = "HashMap<_, (DateTimeExtFormat, _)>")]
    pub versions: HashMap<u64, (DateTime<Utc>, DeviceID)>,
}

impl_api_protocol_dump_load!(VlobListVersionsRepSchema);

/*
 * VlobMaintenanceGetReencryptionBatchReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobMaintenanceGetReencryptionBatchReqSchema {
    pub cmd: String,
    pub realm_id: RealmID,
    pub encryption_revision: u64,
    pub size: u64,
}

impl_api_protocol_dump_load!(VlobMaintenanceGetReencryptionBatchReqSchema);

/*
 * ReencryptionBatchEntrySchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReencryptionBatchEntrySchema {
    pub vlob_id: VlobID,
    pub version: u64,
    #[serde_as(as = "Bytes")]
    pub blob: Vec<u8>,
}

/*
 * VlobMaintenanceGetReencryptionBatchRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobMaintenanceGetReencryptionBatchRepSchema {
    pub status: Status,
    pub batch: Vec<ReencryptionBatchEntrySchema>,
}

impl_api_protocol_dump_load!(VlobMaintenanceGetReencryptionBatchRepSchema);

/*
 * VlobMaintenanceSaveReencryptionBatchReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobMaintenanceSaveReencryptionBatchReqSchema {
    pub cmd: String,
    pub realm_id: RealmID,
    pub encryption_revision: u64,
    pub batch: Vec<ReencryptionBatchEntrySchema>,
}

impl_api_protocol_dump_load!(VlobMaintenanceSaveReencryptionBatchReqSchema);

/*
 * VlobMaintenanceSaveReencryptionBatchReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VlobMaintenanceSaveReencryptionBatchRepSchema {
    pub status: Status,
    pub total: u64,
    pub done: u64,
}

impl_api_protocol_dump_load!(VlobMaintenanceSaveReencryptionBatchRepSchema);
