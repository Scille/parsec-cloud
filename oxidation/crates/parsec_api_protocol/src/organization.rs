// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};

use parsec_api_types::{DeviceID, DeviceLabel, OrganizationID, UserProfile};
use parsec_schema::parsec_schema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrganizationBootstrapWebhook {
    pub organization_id: OrganizationID,
    pub device_id: DeviceID,
    pub device_label: DeviceLabel,
    pub human_email: Option<String>,
    pub human_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsersPerProfileDetailItem {
    pub profile: UserProfile,
    pub active: u64,
    pub revoked: u64,
}

#[parsec_schema]
pub struct OrganizationStatsReq;

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum OrganizationStatsRep {
    Ok {
        data_size: u64,
        metadata_size: u64,
        realms: u64,
        users: u64,
        active_users: u64,
        users_per_profile_detail: Vec<UsersPerProfileDetailItem>,
    },
    NotAllowed {
        reason: Option<String>,
    },
    NotFound,
    UnknownError {
        error: String,
    },
}

#[parsec_schema]
pub struct OrganizationConfigReq;

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum OrganizationConfigRep {
    Ok {
        user_profile_outsider_allowed: bool,
        active_users_limit: Option<u64>,
    },
    NotFound,
    UnknownError {
        error: String,
    },
}
