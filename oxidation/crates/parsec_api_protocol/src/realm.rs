// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::collections::HashMap;

use crate::impl_api_protocol_dump_load;
use parsec_api_types::{maybe_field, DateTime, DeviceID, RealmID, UserID};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MaintenanceType {
    GarbageCollection,
    Reencryption,
}

/*
 * RealmCreateReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmCreateReq {
    pub cmd: String,
    #[serde_as(as = "Bytes")]
    pub role_certificate: Vec<u8>,
}

impl_api_protocol_dump_load!(RealmCreateReq);

/*
 * RealmCreateRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmCreateRep {
    Ok,
    InvalidCertification {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    InvalidData {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    NotFound {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    AlreadyExists,
}

impl_api_protocol_dump_load!(RealmCreateRep);

/*
 * RealmStatusReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmStatusReq {
    pub cmd: String,
    pub realm_id: RealmID,
}

impl_api_protocol_dump_load!(RealmStatusReq);

/*
 * RealmStatusRep
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
}

impl_api_protocol_dump_load!(RealmStatusRep);

/*
 * RealmStatsReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmStatsReq {
    pub cmd: String,
    pub realm_id: RealmID,
}

impl_api_protocol_dump_load!(RealmStatsReq);

/*
 * RealmStatsRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmStatsRep {
    Ok {
        blocks_size: u64,
        vlobs_size: u64,
    },
    NotAllowed,
    NotFound {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
}

impl_api_protocol_dump_load!(RealmStatsRep);

/*
 * RealmGetRoleCertificatesReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmGetRoleCertificatesReq {
    pub cmd: String,
    pub realm_id: RealmID,
    pub since: Option<DateTime>,
}

impl_api_protocol_dump_load!(RealmGetRoleCertificatesReq);

/*
 * RealmGetRoleCertificatesRep
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmGetRoleCertificatesRep {
    Ok {
        #[serde_as(as = "Vec<Bytes>")]
        certificates: Vec<Vec<u8>>,
    },
    NotAllowed,
    NotFound {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
}

impl_api_protocol_dump_load!(RealmGetRoleCertificatesRep);

/*
 * RealmUpdateRolesReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmUpdateRolesReq {
    pub cmd: String,
    #[serde_as(as = "Bytes")]
    pub role_certificate: Vec<u8>,
    #[serde_as(as = "Option<Bytes>")]
    pub recipient_message: Option<Vec<u8>>,
}

impl_api_protocol_dump_load!(RealmUpdateRolesReq);

/*
 * RealmUpdateRolesRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmUpdateRolesRep {
    Ok,
    NotAllowed {
        reason: Option<String>,
    },
    InvalidCertification {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    InvalidData {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    AlreadyGranted,
    IncompatibleProfile {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    NotFound {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    InMaintenance,
}

impl_api_protocol_dump_load!(RealmUpdateRolesRep);

/*
 * RealmStartReencryptionMaintenanceReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmStartReencryptionMaintenanceReq {
    pub cmd: String,
    pub realm_id: RealmID,
    pub encryption_revision: u64,
    pub timestamp: DateTime,
    #[serde_as(as = "HashMap<_, Bytes>")]
    pub per_participant_message: HashMap<UserID, Vec<u8>>,
}

impl_api_protocol_dump_load!(RealmStartReencryptionMaintenanceReq);

/*
 * RealmStartReencryptionMaintenanceRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmStartReencryptionMaintenanceRep {
    Ok,
    NotAllowed,
    NotFound {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    BadEncryptionRevision,
    ParticipantMismatch {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    MaintenanceError {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    InMaintenance,
}

impl_api_protocol_dump_load!(RealmStartReencryptionMaintenanceRep);

/*
 * RealmFinishReencryptionMaintenanceReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmFinishReencryptionMaintenanceReq {
    pub cmd: String,
    pub realm_id: RealmID,
    pub encryption_revision: u64,
}

impl_api_protocol_dump_load!(RealmFinishReencryptionMaintenanceReq);

/*
 * RealmFinishReencryptionMaintenanceRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RealmFinishReencryptionMaintenanceRep {
    Ok,
    NotAllowed,
    NotFound {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    BadEncryptionRevision,
    NotInMaintenance {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    MaintenanceError {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
}

impl_api_protocol_dump_load!(RealmFinishReencryptionMaintenanceRep);
