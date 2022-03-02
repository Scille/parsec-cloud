// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::collections::HashMap;

use crate::impl_api_protocol_dump_load;
use parsec_api_types::{DateTime, DeviceID, RealmID, UserID};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MaintenanceType {
    GarbageCollection,
    Reencryption,
}

/*
 * RealmCreateReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmCreateReqSchema {
    pub cmd: String,
    #[serde_as(as = "Bytes")]
    pub role_certificate: Vec<u8>,
}

impl_api_protocol_dump_load!(RealmCreateReqSchema);

/*
 * RealmCreateRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmCreateRepSchema {
    Ok,
    InvalidCertification { reason: String },
    InvalidData { reason: String },
    NotFound { reason: String },
    AlreadyExists,
}

impl_api_protocol_dump_load!(RealmCreateRepSchema);

/*
 * RealmStatusReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmStatusReqSchema {
    pub cmd: String,
    pub realm_id: RealmID,
}

impl_api_protocol_dump_load!(RealmStatusReqSchema);

/*
 * RealmStatusRepSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmStatusRepSchema {
    Ok {
        in_maintenance: bool,
        maintenance_type: Option<MaintenanceType>,
        maintenance_started_on: Option<DateTime>,
        maintenance_started_by: Option<DeviceID>,
        encryption_revision: u64,
    },
    NotAllowed,
    NotFound {
        reason: String,
    },
}

impl_api_protocol_dump_load!(RealmStatusRepSchema);

/*
 * RealmStatsReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmStatsReqSchema {
    pub cmd: String,
    pub realm_id: RealmID,
}

impl_api_protocol_dump_load!(RealmStatsReqSchema);

/*
 * RealmStatsRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmStatsRepSchema {
    Ok { blocks_size: u64, vlobs_size: u64 },
    NotAllowed,
    NotFound { reason: String },
}

impl_api_protocol_dump_load!(RealmStatsRepSchema);

/*
 * RealmGetRoleCertificatesReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmGetRoleCertificatesReqSchema {
    pub cmd: String,
    pub realm_id: RealmID,
    pub since: Option<DateTime>,
}

impl_api_protocol_dump_load!(RealmGetRoleCertificatesReqSchema);

/*
 * RealmGetRoleCertificatesRepSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmGetRoleCertificatesRepSchema {
    Ok {
        #[serde_as(as = "Vec<Bytes>")]
        certificates: Vec<Vec<u8>>,
    },
    NotAllowed,
    NotFound {
        reason: String,
    },
}

impl_api_protocol_dump_load!(RealmGetRoleCertificatesRepSchema);

/*
 * RealmUpdateRolesReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmUpdateRolesReqSchema {
    pub cmd: String,
    #[serde_as(as = "Bytes")]
    pub role_certificate: Vec<u8>,
    #[serde_as(as = "Option<Bytes>")]
    pub recipient_message: Option<Vec<u8>>,
}

impl_api_protocol_dump_load!(RealmUpdateRolesReqSchema);

/*
 * RealmUpdateRolesRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmUpdateRolesRepSchema {
    Ok,
    NotAllowed { reason: Option<String> },
    InvalidCertification { reason: String },
    InvalidData { reason: String },
    AlreadyGranted,
    IncompatibleProfile { reason: String },
    NotFound { reason: String },
    InMaintenance,
}

impl_api_protocol_dump_load!(RealmUpdateRolesRepSchema);

/*
 * RealmStartReencryptionMaintenanceReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmStartReencryptionMaintenanceReqSchema {
    pub cmd: String,
    pub realm_id: RealmID,
    pub encryption_revision: u64,
    pub timestamp: DateTime,
    #[serde_as(as = "HashMap<_, Bytes>")]
    pub per_participant_message: HashMap<UserID, Vec<u8>>,
}

impl_api_protocol_dump_load!(RealmStartReencryptionMaintenanceReqSchema);

/*
 * RealmStartReencryptionMaintenanceRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmStartReencryptionMaintenanceRepSchema {
    Ok,
    NotAllowed,
    NotFound { reason: String },
    BadEncryptionRevision,
    ParticipantMismatch { reason: String },
    MaintenanceError { reason: String },
    InMaintenance,
}

impl_api_protocol_dump_load!(RealmStartReencryptionMaintenanceRepSchema);

/*
 * RealmFinishReencryptionMaintenanceReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmFinishReencryptionMaintenanceReqSchema {
    pub cmd: String,
    pub realm_id: RealmID,
    pub encryption_revision: u64,
}

impl_api_protocol_dump_load!(RealmFinishReencryptionMaintenanceReqSchema);

/*
 * RealmFinishReencryptionMaintenanceRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmFinishReencryptionMaintenanceRepSchema {
    Ok,
    NotAllowed,
    NotFound { reason: String },
    BadEncryptionRevision,
    NotInMaintenance { reason: String },
    MaintenanceError { reason: String },
}

impl_api_protocol_dump_load!(RealmFinishReencryptionMaintenanceRepSchema);
