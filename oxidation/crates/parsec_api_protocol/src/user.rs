// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::num::NonZeroU64;

use parsec_api_types::{HumanHandle, UserID};
use parsec_schema::parsec_schema;

/*** Access user API ***/

#[parsec_schema]
pub struct Trustchain {
    pub devices: Vec<Vec<u8>>,
    pub users: Vec<Vec<u8>>,
    pub revoked_users: Vec<Vec<u8>>,
}

#[parsec_schema]
pub struct UserGetReq {
    pub user_id: UserID,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum UserGetRep {
    Ok {
        user_certificate: Vec<u8>,
        revoked_user_certificate: Vec<u8>,
        device_certificates: Vec<Vec<u8>>,
        trustchain: Trustchain,
    },
    NotFound,
    UnknownError {
        error: String,
    },
}

/*** User creation API ***/

#[parsec_schema]
pub struct UserCreateReq {
    pub user_certificate: Vec<u8>,
    pub device_certificate: Vec<u8>,
    // Same certificates than above, but expunged of human_handle/device_label
    pub redacted_user_certificate: Vec<u8>,
    pub redacted_device_certificate: Vec<u8>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum UserCreateRep {
    Ok,
    NotAllowed { reason: Option<String> },
    InvalidCertification { reason: Option<String> },
    InvalidData { reason: Option<String> },
    AlreadyExists { reason: Option<String> },
    ActiveUsersLimitReached { reason: Option<String> },
    UnknownError { error: String },
}

#[parsec_schema]
pub struct UserRevokeReq {
    pub revoked_user_certificate: Vec<u8>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum UserRevokeRep {
    Ok,
    NotAllowed { reason: Option<String> },
    InvalidCertification { reason: Option<String> },
    NotFound,
    AlreadyRevoked { reason: Option<String> },
    UnknownError { error: String },
}

/*** Device creation API ***/

#[parsec_schema]
pub struct DeviceCreateReq {
    pub device_certificate: Vec<u8>,
    // Same certificate than above, but expunged of device_label
    pub redacted_device_certificate: Vec<u8>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum DeviceCreateRep {
    Ok,
    InvalidCertification { reason: Option<String> },
    BadUserId { reason: Option<String> },
    InvalidData { reason: Option<String> },
    AlreadyExists { reason: Option<String> },
    UnknownError { error: String },
}

/*** Human search API ***/

#[parsec_schema]
pub struct HumanFindReq {
    pub query: Option<String>,
    pub omit_revoked: bool,
    pub omit_non_human: bool,
    pub page: NonZeroU64,
    pub per_page: NonZeroU64,
}

#[parsec_schema]
pub struct HumanFindResultItem {
    pub user_id: UserID,
    pub human_handle: Option<HumanHandle>,
    pub revoked: bool,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum HumanFindRep {
    Ok {
        results: Vec<HumanFindResultItem>,
        page: NonZeroU64,
        per_page: NonZeroU64,
        total: u64,
    },
    NotAllowed {
        reason: Option<String>,
    },
    UnknownError {
        error: String,
    },
}
