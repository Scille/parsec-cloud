// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

use parsec_api_types::{DateTimeExtFormat, DeviceID, RealmID, UserID};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MaintenanceType {
    GarbageCollection,
    Reencryption,
}

/*
 * RealmCreateReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmCreateReqSchema {
    pub cmd: String,
    pub role_certificate: Vec<u8>,
}

/*
 * RealmCreateRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmCreateRepSchema;

/*
 * RealmStatusReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmStatusReqSchema {
    pub cmd: String,
    pub realm_id: RealmID,
}

/*
 * RealmStatusRepSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmStatusRepSchema {
    pub in_maintenance: bool,
    pub maintenance_type: Option<MaintenanceType>,
    #[serde_as(as = "Option<DateTimeExtFormat>")]
    pub maintenance_started_on: Option<DateTime<Utc>>,
    pub maintenance_started_by: Option<DeviceID>,
    pub encryption_revision: u64,
}

/*
 * RealmStatsReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmStatsReqSchema {
    pub cmd: String,
    pub realm_id: RealmID,
}

/*
 * RealmStatsRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmStatsRepSchema {
    pub blocks_size: u64,
    pub vlobs_size: u64,
}

/*
 * RealmGetRoleCertificatesReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmGetRoleCertificatesReqSchema {
    pub cmd: String,
    pub realm_id: RealmID,
    #[serde_as(as = "Option<DateTimeExtFormat>")]
    pub since: Option<DateTime<Utc>>,
}

/*
 * RealmGetRoleCertificatesRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmGetRoleCertificatesRepSchema {
    pub certificates: Vec<Vec<u8>>,
}

/*
 * RealmUpdateRolesReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmUpdateRolesReqSchema {
    pub cmd: String,
    pub role_certificate: Vec<u8>,
    pub recipient_message: Option<Vec<u8>>,
}

/*
 * RealmUpdateRolesRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmUpdateRolesRepSchema;

/*
 * RealmStartReencryptionMaintenanceReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmStartReencryptionMaintenanceReqSchema {
    pub cmd: String,
    pub realm_id: RealmID,
    pub encryption_revision: u64,
    #[serde_as(as = "DateTimeExtFormat")]
    pub timestamp: DateTime<Utc>,
    pub per_participant_message: HashMap<UserID, Vec<u8>>,
}

/*
 * RealmStartReencryptionMaintenanceRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmStartReencryptionMaintenanceRepSchema;

/*
 * RealmFinishReencryptionMaintenanceReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmFinishReencryptionMaintenanceReqSchema {
    pub cmd: String,
    pub realm_id: RealmID,
    pub encryption_revision: u64,
}

/*
 * RealmFinishReencryptionMaintenanceRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RealmFinishReencryptionMaintenanceRepSchema;
