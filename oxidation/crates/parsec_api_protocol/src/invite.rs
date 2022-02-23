// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use parsec_api_crypto::{HashDigest, PublicKey};
use parsec_api_types::{
    new_data_struct_type, DateTimeExtFormat, HumanHandle, InvitationToken, UserID,
};

/*
 * InvitationType
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InvitationType {
    User(InviteNewUserReqSchemaDataType),
    Device(InviteNewDeviceReqSchemaDataType),
}

/*
 * InviteNewUserReqSchema
 */

new_data_struct_type!(
    InviteNewUserReqSchema,
    type: "USER",
    cmd: String,
    claimer_email: String,
    send_email: bool,
);

/*
 * InviteNewDeviceReqSchema
 */

new_data_struct_type!(
    InviteNewDeviceReqSchema,
    type: "DEVICE",
    cmd: String,
    send_email: bool,
);

/*
 * Type
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "type")]
pub struct Type;

/*
 * InviteNewUserOrDeviceReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum InviteNewUserOrDeviceReq {
    User(InviteNewUserReqSchema),
    Device(InviteNewDeviceReqSchema),
}

/*
 * InviteNewReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteNewReqSchema {
    pub cmd: String,
    pub type_field: Type,
    pub type_schema: InviteNewUserOrDeviceReq,
}

/*
 * InvitationEmailSentStatus
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvitationEmailSentStatus {
    Success,
    NotAvailable,
    BadRecipient,
}

/*
 * InviteNewRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteNewRepSchema {
    pub token: InvitationToken,
    pub email_sent: InvitationEmailSentStatus,
}

/*
 * InvitationDeletedReason
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvitationDeletedReason {
    Finished,
    Cancelled,
    Rotten,
}

/*
 * InviteDeleteReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteDeleteReqSchema {
    pub cmd: String,
    pub token: InvitationToken,
    pub reason: InvitationDeletedReason,
}

/*
 * InviteDeleteRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteDeleteRepSchema;

/*
 * InviteListReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteListReqSchema {
    pub cmd: String,
}

/*
 * InvitationStatus
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvitationStatus {
    Idle,
    Ready,
    Deleted,
}

/*
 * InviteListItemUserSchema
 */

new_data_struct_type!(
    InviteListItemUserSchema,
    type: "USER",
    token: InvitationToken,
    #[serde_as(as = "DateTimeExtFormat")]
    created_on: DateTime<Utc>,
    claimer_email: String,
    status: InvitationStatus,
);

/*
 * InviteListItemDeviceSchema
 */

new_data_struct_type!(
    InviteListItemDeviceSchema,
    type: "DEVICE",
    token: InvitationToken,
    #[serde_as(as = "DateTimeExtFormat")]
    created_on: DateTime<Utc>,
    status: InvitationStatus,
);

/*
 * InviteUserOrDevice
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum InviteListItemUserOrDevice {
    User(InviteListItemUserSchema),
    Device(InviteListItemDeviceSchema),
}

/*
 * InviteListItemSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteListItemSchema {
    pub type_field: Type,
    pub type_schema: InviteListItemUserOrDevice,
}

/*
 * InviteListRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteListRepSchema {
    pub invitations: Vec<InviteListItemSchema>,
}

/*
 * InviteInfoReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteInfoReqSchema {
    pub cmd: String,
}

/*
 * InviteInfoUserRepSchema
 */

new_data_struct_type!(
    InviteInfoUserRepSchema,
    type: "USER",
    claimer_email: String,
    greeter_user_id: UserID,
    greeter_human_handle: HumanHandle,
);

/*
 * InviteInfoDeviceRepSchema
 */

new_data_struct_type!(
    InviteInfoDeviceRepSchema,
    type: "DEVICE",
    greeter_user_id: UserID,
    greeter_human_handle: HumanHandle,
);

/*
 * InviteUserOrDeviceRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum InviteInfoUserOrDeviceRep {
    User(InviteInfoUserRepSchema),
    Device(InviteInfoDeviceRepSchema),
}

/*
 * InviteInfoRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteInfoRepSchema {
    pub type_field: Type,
    pub type_schema: InviteInfoUserOrDeviceRep,
}

/*
 * Invite1ClaimerWaitPeerReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite1ClaimerWaitPeerReqSchema {
    pub cmd: String,
    pub claimer_public_key: PublicKey,
}

/*
 * Invite1ClaimerWaitPeerRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite1ClaimerWaitPeerRepSchema {
    pub greeter_public_key: PublicKey,
}

/*
 * Invite1GreeterWaitPeerReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite1GreeterWaitPeerReqSchema {
    pub cmd: String,
    pub token: InvitationToken,
    pub greeter_public_key: PublicKey,
}

/*
 * Invite1GreeterWaitPeerRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite1GreeterWaitPeerRepSchema {
    pub claimer_public_key: PublicKey,
}

/*
 * Invite2aClaimerSendHashedNonceHashNonceReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2aClaimerSendHashedNonceHashNonceReqSchema {
    pub cmd: String,
    pub claimer_hashed_nonce: HashDigest,
}

/*
 * Invite2aClaimerSendHashedNonceHashNonceRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2aClaimerSendHashedNonceHashNonceRepSchema {
    pub greeter_nonce: Vec<u8>,
}

/*
 * Invite2aGreeterGetHashedNonceRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2aGreeterGetHashedNonceRepSchema {
    pub token: InvitationToken,
    pub claimer_hashed_nonce: HashDigest,
}

/*
 * Invite2bGreeterSendNonceRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2bGreeterSendNonceRepSchema {
    pub claimer_nonce: Vec<u8>,
}

/*
 * Invite2bClaimerSendNonceReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2bClaimerSendNonceReqSchema {
    pub cmd: String,
    pub claimer_nonce: Vec<u8>,
}

/*
 * Invite2bClaimerSendNonceRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2bClaimerSendNonceRepSchema;

/*
 * Invite2bClaimerSendNonceReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3aGreeterWaitPeerTrustReqSchema {
    pub cmd: String,
    pub token: InvitationToken,
}

/*
 * Invite3aGreeterWaitPeerTrustRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3aGreeterWaitPeerTrustRepSchema;

/*
 * Invite3bClaimerWaitPeerTrustReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3bClaimerWaitPeerTrustReqSchema {
    pub cmd: String,
}

/*
 * Invite3bClaimerWaitPeerTrustRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3bClaimerWaitPeerTrustRepSchema;

/*
 * Invite3bGreeterSignifyTrustReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3bGreeterSignifyTrustReqSchema {
    pub cmd: String,
    pub token: InvitationToken,
}

/*
 * Invite3bGreeterSignifyTrustRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3bGreeterSignifyTrustRepSchema;

/*
 * Invite3aClaimerSignifyTrustReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3aClaimerSignifyTrustReqSchema {
    pub cmd: String,
}

/*
 * Invite3aClaimerSignifyTrustRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3aClaimerSignifyTrustRepSchema;

/*
 * Invite4GreeterCommunicateReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite4GreeterCommunicateReqSchema {
    pub cmd: String,
    pub token: InvitationToken,
    pub payload: Vec<u8>,
}

/*
 * Invite4GreeterCommunicateRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite4GreeterCommunicateRepSchema {
    pub payload: Vec<u8>,
}

/*
 * Invite4GreeterCommunicateRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite4ClaimerCommunicateReqSchema {
    pub cmd: String,
    pub payload: Vec<u8>,
}

/*
 * Invite4GreeterCommunicateRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite4ClaimerCommunicateRepSchema {
    pub payload: Vec<u8>,
}
