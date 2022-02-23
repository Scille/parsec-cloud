// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;

use parsec_api_types::{HumanHandle, UserID};

/*** Access user API ***/

/*
 * TrustchainSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrustchainSchema {
    pub devices: Vec<Vec<u8>>,
    pub users: Vec<Vec<u8>>,
    pub revoked_users: Vec<Vec<u8>>,
}

/*
 * UserGetReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserGetReqSchema {
    pub cmd: String,
    pub user_id: UserID,
}

/*
 * UserGetRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserGetRepSchema {
    pub user_certificate: Vec<u8>,
    pub revoked_user_certificate: Vec<u8>,
    pub device_certificates: Vec<Vec<u8>>,
    pub trustchain: TrustchainSchema,
}

/*** User creation API ***/

/*
 * UserCreateReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserCreateReqSchema {
    pub cmd: String,
    pub user_certificate: Vec<u8>,
    pub device_certifacte: Vec<u8>,
    // Same certificates than above, but expurged of human_handle/device_label
    pub redacted_user_certificate: Vec<u8>,
    pub redacted_device_certifacte: Vec<u8>,
}

/*
 * UserCreateRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserCreateRepSchema;

/*
 * UserRevokeReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserRevokeReqSchema {
    pub cmd: String,
    pub revoked_user_certificate: Vec<u8>,
}

/*
 * UserRevokeRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserRevokeRepSchema;

/*** Device creation API ***/

/*
 * DeviceCreateReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceCreateReqSchema {
    pub cmd: String,
    pub device_certificate: Vec<u8>,
    // Same certificate than above, but expurged of device_label
    pub redacted_device_certifacte: Vec<u8>,
}

/*
 * DeviceCreateRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceCreateRepSchema;

/*** Hman search API ***/

/*
 * HumanFindReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HumanFindReqSchema {
    pub cmd: String,
    pub query: Option<String>,
    pub omit_revoked: bool,
    pub omit_non_human: bool,
    pub page: NonZeroU64,
    pub per_page: NonZeroU64,
}

/*
 * HumanFindResultItemSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HumanFindResultItemSchema {
    pub user_id: UserID,
    pub human_handle: Option<HumanHandle>,
    pub revoked: bool,
}

/*
 * HumanFindResultItemSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HumanFindRepSchema {
    pub results: Vec<HumanFindResultItemSchema>,
    pub page: NonZeroU64,
    pub per_page: NonZeroU64,
    pub total: u64,
}
