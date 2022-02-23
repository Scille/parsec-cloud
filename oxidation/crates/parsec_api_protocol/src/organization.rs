// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};

use parsec_api_crypto::VerifyKey;
use parsec_api_types::{DeviceID, DeviceLabel, OrganizationID, UserProfile};

/*
 * APIV1_OrganizationBootstrapReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "APIV1_OrganizationBootstrapReqSchema")]
pub struct APIV1OrganizationBootstrapReqSchema {
    pub cmd: String,
    pub bootstrap_token: String,
    pub root_verify_key: VerifyKey,
    pub user_certificate: Vec<u8>,
    pub device_certificate: Vec<u8>,
    // Same certificates than above, but expurged of human_handle/device_label
    // Backward compatibility prevent those field to be required, however
    // they should be considered so by recent version of Parsec (hence the
    // `allow_none=False`).
    // Hence only old version of Parsec will provide a payload with missing
    // redacted fields. In such case we consider the non-redacted can also
    // be used as redacted given the to-be-redacted fields have been introduce
    // in later version of Parsec.
    pub redacted_user_certificates: Vec<u8>,
    pub redacted_device_certificate: Vec<u8>,
}

/*
 * APIV1_OrganizationBootstrapRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "APIV1_OrganizationBootstrapRepSchema")]
pub struct APIV1OrganizationBootstrapRepSchema;

/*
 * OrganizationBootstrapWebhookSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrganizationBootstrapWebhookSchema {
    pub organization_id: OrganizationID,
    pub device_id: DeviceID,
    pub device_label: DeviceLabel,
    pub human_email: Option<String>,
    pub human_label: Option<String>,
}

/*
 * UsersPerProfileDetailItemSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsersPerProfileDetailItemSchema {
    pub profile: UserProfile,
    pub active: u64,
    pub revoked: u64,
}

/*
 * OrganizationStatsReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrganizationStatsReqSchema {
    pub cmd: String,
}

/*
 * OrganizationStatsRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrganizationStatsRepSchema {
    pub data_size: u64,
    pub metadata_size: u64,
    pub realms: u64,
    pub users: u64,
    pub active_users: u64,
    pub users_per_profile_detail: Vec<UsersPerProfileDetailItemSchema>,
}

/*
 * OrganizationConfigReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrganizationConfigReqSchema {
    pub cmd: String,
}

/*
 * OrganizationConfigRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrganizationConfigRepSchema {
    pub user_profile_outsider_allowed: bool,
    pub active_users_limit: Option<u64>,
}
