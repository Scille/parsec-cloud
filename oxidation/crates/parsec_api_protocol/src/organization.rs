// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

use crate::impl_api_protocol_dump_load;
use parsec_api_crypto::VerifyKey;
use parsec_api_types::{maybe_field, DeviceID, DeviceLabel, OrganizationID, UserProfile};

/*
 * APIV1_OrganizationBootstrapReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "APIV1_OrganizationBootstrapReq")]
pub struct APIV1OrganizationBootstrapReq {
    pub cmd: String,
    pub bootstrap_token: String,
    pub root_verify_key: VerifyKey,
    #[serde_as(as = "Bytes")]
    pub user_certificate: Vec<u8>,
    #[serde_as(as = "Bytes")]
    pub device_certificate: Vec<u8>,
    // Same certificates than above, but expurged of human_handle/device_label
    // Backward compatibility prevent those field to be required, however
    // they should be considered so by recent version of Parsec (hence the
    // `allow_none=False`).
    // Hence only old version of Parsec will provide a payload with missing
    // redacted fields. In such case we consider the non-redacted can also
    // be used as redacted given the to-be-redacted fields have been introduce
    // in later version of Parsec.
    #[serde_as(as = "Option<Bytes>")]
    pub redacted_user_certificate: Option<Vec<u8>>,
    #[serde_as(as = "Option<Bytes>")]
    pub redacted_device_certificate: Option<Vec<u8>>,
}

impl_api_protocol_dump_load!(APIV1OrganizationBootstrapReq);

/*
 * APIV1_OrganizationBootstrapRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "APIV1_OrganizationBootstrapRep")]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum APIV1OrganizationBootstrapRep {
    Ok,
    InvalidCertification {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    InvalidData {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    AlreadyBootstrapped,
    NotFound,
}

impl_api_protocol_dump_load!(APIV1OrganizationBootstrapRep);

/*
 * OrganizationBootstrapWebhook
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrganizationBootstrapWebhook {
    pub organization_id: OrganizationID,
    pub device_id: DeviceID,
    pub device_label: DeviceLabel,
    pub human_email: Option<String>,
    pub human_label: Option<String>,
}

/*
 * UsersPerProfileDetailItem
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsersPerProfileDetailItem {
    pub profile: UserProfile,
    pub active: u64,
    pub revoked: u64,
}

/*
 * OrganizationStatsReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrganizationStatsReq {
    pub cmd: String,
}

impl_api_protocol_dump_load!(OrganizationStatsReq);

/*
 * OrganizationStatsRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    NotFound,
}

impl_api_protocol_dump_load!(OrganizationStatsRep);

/*
 * OrganizationConfigReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrganizationConfigReq {
    pub cmd: String,
}

impl_api_protocol_dump_load!(OrganizationConfigReq);

/*
 * OrganizationConfigRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum OrganizationConfigRep {
    Ok {
        user_profile_outsider_allowed: bool,
        active_users_limit: Option<u64>,
    },
    NotFound,
}

impl_api_protocol_dump_load!(OrganizationConfigRep);
