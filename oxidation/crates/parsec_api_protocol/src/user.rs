// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::num::NonZeroU64;

use crate::impl_api_protocol_dump_load;
use parsec_api_types::{HumanHandle, UserID};

/*** Access user API ***/

/*
 * TrustchainSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrustchainSchema {
    #[serde_as(as = "Vec<Bytes>")]
    pub devices: Vec<Vec<u8>>,
    #[serde_as(as = "Vec<Bytes>")]
    pub users: Vec<Vec<u8>>,
    #[serde_as(as = "Vec<Bytes>")]
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

impl_api_protocol_dump_load!(UserGetReqSchema);

/*
 * UserGetRepSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum UserGetRepSchema {
    Ok {
        #[serde_as(as = "Bytes")]
        user_certificate: Vec<u8>,
        #[serde_as(as = "Bytes")]
        revoked_user_certificate: Vec<u8>,
        #[serde_as(as = "Vec<Bytes>")]
        device_certificates: Vec<Vec<u8>>,
        trustchain: TrustchainSchema,
    },
    NotFound,
}

impl_api_protocol_dump_load!(UserGetRepSchema);

/*** User creation API ***/

/*
 * UserCreateReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserCreateReqSchema {
    pub cmd: String,
    #[serde_as(as = "Bytes")]
    pub user_certificate: Vec<u8>,
    #[serde_as(as = "Bytes")]
    pub device_certificate: Vec<u8>,
    // Same certificates than above, but expurged of human_handle/device_label
    #[serde_as(as = "Bytes")]
    pub redacted_user_certificate: Vec<u8>,
    #[serde_as(as = "Bytes")]
    pub redacted_device_certificate: Vec<u8>,
}

impl_api_protocol_dump_load!(UserCreateReqSchema);

/*
 * UserCreateRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum UserCreateRepSchema {
    Ok,
    NotAllowed { reason: String },
    InvalidCertification { reason: String },
    InvalidData { reason: String },
    AlreadyExists { reason: String },
    ActiveUsersLimitReached { reason: String },
}

impl_api_protocol_dump_load!(UserCreateRepSchema);

/*
 * UserRevokeReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserRevokeReqSchema {
    pub cmd: String,
    #[serde_as(as = "Bytes")]
    pub revoked_user_certificate: Vec<u8>,
}

impl_api_protocol_dump_load!(UserRevokeReqSchema);

/*
 * UserRevokeRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum UserRevokeRepSchema {
    Ok,
    NotAllowed { reason: String },
    InvalidCertification { reason: String },
    NotFound,
    AlreadyRevoked { reason: String },
}

impl_api_protocol_dump_load!(UserRevokeRepSchema);

/*** Device creation API ***/

/*
 * DeviceCreateReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceCreateReqSchema {
    pub cmd: String,
    #[serde_as(as = "Bytes")]
    pub device_certificate: Vec<u8>,
    // Same certificate than above, but expurged of device_label
    #[serde_as(as = "Bytes")]
    pub redacted_device_certificate: Vec<u8>,
}

impl_api_protocol_dump_load!(DeviceCreateReqSchema);

/*
 * DeviceCreateRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum DeviceCreateRepSchema {
    Ok,
    InvalidCertification { reason: String },
    BadUserId { reason: String },
    InvalidData { reason: String },
    AlreadyExists { reason: String },
}

impl_api_protocol_dump_load!(DeviceCreateRepSchema);

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

impl_api_protocol_dump_load!(HumanFindReqSchema);

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
#[serde(tag = "status", rename_all = "snake_case")]
pub enum HumanFindRepSchema {
    Ok {
        results: Vec<HumanFindResultItemSchema>,
        page: NonZeroU64,
        per_page: NonZeroU64,
        total: u64,
    },
    NotAllowed {
        reason: String,
    },
}

impl_api_protocol_dump_load!(HumanFindRepSchema);
