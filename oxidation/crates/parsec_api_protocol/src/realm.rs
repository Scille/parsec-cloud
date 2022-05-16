// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use parsec_api_types::{DateTime, DeviceID, RealmID, UserID};
use parsec_schema::parsec_schema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MaintenanceType {
    GarbageCollection,
    Reencryption,
}

#[parsec_schema]
pub struct RealmCreateReq {
    pub role_certificate: Vec<u8>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmCreateRep {
    Ok,
    InvalidCertification { reason: Option<String> },
    InvalidData { reason: Option<String> },
    NotFound { reason: Option<String> },
    AlreadyExists,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct RealmStatusReq {
    pub realm_id: RealmID,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmStatusRep {
    Ok {
        in_maintenance: bool,
        maintenance_type: Option<MaintenanceType>,
        maintenance_started_on: Option<DateTime>,
        maintenance_started_by: Option<DeviceID>,
        encryption_revision: u64,
    },
    NotAllowed,
    NotFound {
        reason: Option<String>,
    },
    UnknownError {
        error: String,
    },
}

#[parsec_schema]
pub struct RealmStatsReq {
    pub realm_id: RealmID,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmStatsRep {
    Ok { blocks_size: u64, vlobs_size: u64 },
    NotAllowed,
    NotFound { reason: Option<String> },
    UnknownError { error: String },
}

#[parsec_schema]
pub struct RealmGetRoleCertificatesReq {
    pub realm_id: RealmID,
    pub since: Option<DateTime>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmGetRoleCertificatesRep {
    Ok { certificates: Vec<Vec<u8>> },
    NotAllowed,
    NotFound { reason: Option<String> },
    UnknownError { error: String },
}

#[parsec_schema]
pub struct RealmUpdateRolesReq {
    pub role_certificate: Vec<u8>,
    pub recipient_message: Option<Vec<u8>>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmUpdateRolesRep {
    Ok,
    NotAllowed { reason: Option<String> },
    InvalidCertification { reason: Option<String> },
    InvalidData { reason: Option<String> },
    AlreadyGranted,
    IncompatibleProfile { reason: Option<String> },
    NotFound { reason: Option<String> },
    InMaintenance,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct RealmStartReencryptionMaintenanceReq {
    pub realm_id: RealmID,
    pub encryption_revision: u64,
    pub timestamp: DateTime,
    // #[serde_as(as = "HashMap<_, Bytes>")]
    pub per_participant_message: HashMap<UserID, Vec<u8>>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmStartReencryptionMaintenanceRep {
    Ok,
    NotAllowed,
    NotFound { reason: Option<String> },
    BadEncryptionRevision,
    ParticipantMismatch { reason: Option<String> },
    MaintenanceError { reason: Option<String> },
    InMaintenance,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct RealmFinishReencryptionMaintenanceReq {
    pub realm_id: RealmID,
    pub encryption_revision: u64,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmFinishReencryptionMaintenanceRep {
    Ok,
    NotAllowed,
    NotFound { reason: Option<String> },
    BadEncryptionRevision,
    NotInMaintenance { reason: Option<String> },
    MaintenanceError { reason: Option<String> },
    UnknownError { error: String },
}
