// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::num::NonZeroU64;

use crate::impl_api_protocol_dump_load;
use parsec_api_types::{maybe_field, HumanHandle, UserID};

/*** Access user API ***/

/*
 * Trustchain
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Trustchain {
    #[serde_as(as = "Vec<Bytes>")]
    pub devices: Vec<Vec<u8>>,
    #[serde_as(as = "Vec<Bytes>")]
    pub users: Vec<Vec<u8>>,
    #[serde_as(as = "Vec<Bytes>")]
    pub revoked_users: Vec<Vec<u8>>,
}

/*
 * UserGetReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserGetReq {
    pub cmd: String,
    pub user_id: UserID,
}

impl_api_protocol_dump_load!(UserGetReq);

/*
 * UserGetRep
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum UserGetRep {
    Ok {
        #[serde_as(as = "Bytes")]
        user_certificate: Vec<u8>,
        #[serde_as(as = "Bytes")]
        revoked_user_certificate: Vec<u8>,
        #[serde_as(as = "Vec<Bytes>")]
        device_certificates: Vec<Vec<u8>>,
        trustchain: Trustchain,
    },
    NotFound,
}

impl_api_protocol_dump_load!(UserGetRep);

/*** User creation API ***/

/*
 * UserCreateReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserCreateReq {
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

impl_api_protocol_dump_load!(UserCreateReq);

/*
 * UserCreateRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum UserCreateRep {
    Ok,
    NotAllowed {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
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
    AlreadyExists {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    ActiveUsersLimitReached {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
}

impl_api_protocol_dump_load!(UserCreateRep);

/*
 * UserRevokeReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserRevokeReq {
    pub cmd: String,
    #[serde_as(as = "Bytes")]
    pub revoked_user_certificate: Vec<u8>,
}

impl_api_protocol_dump_load!(UserRevokeReq);

/*
 * UserRevokeRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum UserRevokeRep {
    Ok,
    NotAllowed {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    InvalidCertification {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    NotFound,
    AlreadyRevoked {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
}

impl_api_protocol_dump_load!(UserRevokeRep);

/*** Device creation API ***/

/*
 * DeviceCreateReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceCreateReq {
    pub cmd: String,
    #[serde_as(as = "Bytes")]
    pub device_certificate: Vec<u8>,
    // Same certificate than above, but expurged of device_label
    #[serde_as(as = "Bytes")]
    pub redacted_device_certificate: Vec<u8>,
}

impl_api_protocol_dump_load!(DeviceCreateReq);

/*
 * DeviceCreateRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum DeviceCreateRep {
    Ok,
    InvalidCertification {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    BadUserId {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    InvalidData {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    AlreadyExists {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
}

impl_api_protocol_dump_load!(DeviceCreateRep);

/*** Hman search API ***/

/*
 * HumanFindReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HumanFindReq {
    pub cmd: String,
    pub query: Option<String>,
    pub omit_revoked: bool,
    pub omit_non_human: bool,
    pub page: NonZeroU64,
    pub per_page: NonZeroU64,
}

impl_api_protocol_dump_load!(HumanFindReq);

/*
 * HumanFindResultItem
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HumanFindResultItem {
    pub user_id: UserID,
    pub human_handle: Option<HumanHandle>,
    pub revoked: bool,
}

/*
 * HumanFindResultItem
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum HumanFindRep {
    Ok {
        results: Vec<HumanFindResultItem>,
        page: NonZeroU64,
        per_page: NonZeroU64,
        total: u64,
    },
    NotAllowed {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
}

impl_api_protocol_dump_load!(HumanFindRep);
