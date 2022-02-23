// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

use crate::{impl_api_protocol_dump_load, Status};
use parsec_api_crypto::{HashDigest, PublicKey};
use parsec_api_types::{DateTime, HumanHandle, InvitationToken, UserID};

/*
 * InvitationType
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum InvitationType {
    User,
    Device,
}

/*
 * InviteNewUserReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteNewUserReqSchema {
    pub cmd: String,
    pub claimer_email: String,
    pub send_email: bool,
}

/*
 * InviteNewDeviceReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteNewDeviceReqSchema {
    pub cmd: String,
    pub send_email: bool,
}

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
#[serde(tag = "type")]
#[serde(rename_all = "UPPERCASE")]
pub enum InviteNewUserOrDeviceReq {
    User(InviteNewUserReqSchema),
    Device(InviteNewDeviceReqSchema),
}

/*
 * InviteNewReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteNewReqSchema(pub InviteNewUserOrDeviceReq);

impl_api_protocol_dump_load!(InviteNewReqSchema);

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
    pub status: Status,
    pub token: InvitationToken,
    pub email_sent: InvitationEmailSentStatus,
}

impl_api_protocol_dump_load!(InviteNewRepSchema);

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

impl_api_protocol_dump_load!(InviteDeleteReqSchema);

/*
 * InviteDeleteRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteDeleteRepSchema {
    pub status: Status,
}

impl_api_protocol_dump_load!(InviteDeleteRepSchema);

/*
 * InviteListReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteListReqSchema {
    pub cmd: String,
}

impl_api_protocol_dump_load!(InviteListReqSchema);

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

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

pub struct InviteListItemUserSchema {
    pub token: InvitationToken,
    pub created_on: DateTime,
    pub claimer_email: String,
    pub status: InvitationStatus,
}

/*
 * InviteListItemDeviceSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteListItemDeviceSchema {
    pub token: InvitationToken,
    pub created_on: DateTime,
    pub status: InvitationStatus,
}

/*
 * InviteUserOrDevice
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "UPPERCASE")]
pub enum InviteListItemUserOrDevice {
    User(InviteListItemUserSchema),
    Device(InviteListItemDeviceSchema),
}

/*
 * InviteListItemSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteListItemSchema(pub InviteListItemUserOrDevice);

/*
 * InviteListRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteListRepSchema {
    pub status: Status,
    pub invitations: Vec<InviteListItemSchema>,
}

impl_api_protocol_dump_load!(InviteListRepSchema);

/*
 * InviteInfoReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteInfoReqSchema {
    pub cmd: String,
}

impl_api_protocol_dump_load!(InviteInfoReqSchema);

/*
 * InviteInfoUserRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteInfoUserRepSchema {
    pub claimer_email: String,
    pub greeter_user_id: UserID,
    pub greeter_human_handle: HumanHandle,
}

/*
 * InviteInfoDeviceRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteInfoDeviceRepSchema {
    pub greeter_user_id: UserID,
    pub greeter_human_handle: HumanHandle,
}

/*
 * InviteUserOrDeviceRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "UPPERCASE")]
pub enum InviteInfoUserOrDeviceRep {
    User(InviteInfoUserRepSchema),
    Device(InviteInfoDeviceRepSchema),
}

/*
 * InviteInfoRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteInfoRepSchema(pub InviteInfoUserOrDeviceRep);

impl_api_protocol_dump_load!(InviteInfoRepSchema);

/*
 * Invite1ClaimerWaitPeerReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite1ClaimerWaitPeerReqSchema {
    pub cmd: String,
    pub claimer_public_key: PublicKey,
}

impl_api_protocol_dump_load!(Invite1ClaimerWaitPeerReqSchema);

/*
 * Invite1ClaimerWaitPeerRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite1ClaimerWaitPeerRepSchema {
    pub status: Status,
    pub greeter_public_key: PublicKey,
}

impl_api_protocol_dump_load!(Invite1ClaimerWaitPeerRepSchema);

/*
 * Invite1GreeterWaitPeerReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite1GreeterWaitPeerReqSchema {
    pub cmd: String,
    pub token: InvitationToken,
    pub greeter_public_key: PublicKey,
}

impl_api_protocol_dump_load!(Invite1GreeterWaitPeerReqSchema);

/*
 * Invite1GreeterWaitPeerRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite1GreeterWaitPeerRepSchema {
    pub status: Status,
    pub claimer_public_key: PublicKey,
}

impl_api_protocol_dump_load!(Invite1GreeterWaitPeerRepSchema);

/*
 * Invite2aClaimerSendHashedNonceHashNonceReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2aClaimerSendHashedNonceHashNonceReqSchema {
    pub cmd: String,
    pub claimer_hashed_nonce: HashDigest,
}

impl_api_protocol_dump_load!(Invite2aClaimerSendHashedNonceHashNonceReqSchema);

/*
 * Invite2aClaimerSendHashedNonceHashNonceRepSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2aClaimerSendHashedNonceHashNonceRepSchema {
    pub status: Status,
    #[serde_as(as = "Bytes")]
    pub greeter_nonce: Vec<u8>,
}

impl_api_protocol_dump_load!(Invite2aClaimerSendHashedNonceHashNonceRepSchema);

/*
 * Invite2aGreeterGetHashedNonceRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2aGreeterGetHashedNonceRepSchema {
    pub status: Status,
    pub claimer_hashed_nonce: HashDigest,
}

impl_api_protocol_dump_load!(Invite2aGreeterGetHashedNonceRepSchema);

/*
 * Invite2bGreeterSendNonceRepSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2bGreeterSendNonceRepSchema {
    pub status: Status,
    #[serde_as(as = "Bytes")]
    pub claimer_nonce: Vec<u8>,
}

impl_api_protocol_dump_load!(Invite2bGreeterSendNonceRepSchema);

/*
 * Invite2bClaimerSendNonceReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2bClaimerSendNonceReqSchema {
    pub cmd: String,
    #[serde_as(as = "Bytes")]
    pub claimer_nonce: Vec<u8>,
}

impl_api_protocol_dump_load!(Invite2bClaimerSendNonceReqSchema);

/*
 * Invite2bClaimerSendNonceRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2bClaimerSendNonceRepSchema {
    pub status: Status,
}

impl_api_protocol_dump_load!(Invite2bClaimerSendNonceRepSchema);

/*
 * Invite2bClaimerSendNonceReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3aGreeterWaitPeerTrustReqSchema {
    pub cmd: String,
    pub token: InvitationToken,
}

impl_api_protocol_dump_load!(Invite3aGreeterWaitPeerTrustReqSchema);

/*
 * Invite3aGreeterWaitPeerTrustRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3aGreeterWaitPeerTrustRepSchema {
    pub status: Status,
}

impl_api_protocol_dump_load!(Invite3aGreeterWaitPeerTrustRepSchema);

/*
 * Invite3bClaimerWaitPeerTrustReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3bClaimerWaitPeerTrustReqSchema {
    pub cmd: String,
}

impl_api_protocol_dump_load!(Invite3bClaimerWaitPeerTrustReqSchema);

/*
 * Invite3bClaimerWaitPeerTrustRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3bClaimerWaitPeerTrustRepSchema {
    pub status: Status,
}

impl_api_protocol_dump_load!(Invite3bClaimerWaitPeerTrustRepSchema);

/*
 * Invite3bGreeterSignifyTrustReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3bGreeterSignifyTrustReqSchema {
    pub cmd: String,
    pub token: InvitationToken,
}

impl_api_protocol_dump_load!(Invite3bGreeterSignifyTrustReqSchema);

/*
 * Invite3bGreeterSignifyTrustRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3bGreeterSignifyTrustRepSchema {
    pub status: Status,
}

impl_api_protocol_dump_load!(Invite3bGreeterSignifyTrustRepSchema);

/*
 * Invite3aClaimerSignifyTrustReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3aClaimerSignifyTrustReqSchema {
    pub cmd: String,
}

impl_api_protocol_dump_load!(Invite3aClaimerSignifyTrustReqSchema);

/*
 * Invite3aClaimerSignifyTrustRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3aClaimerSignifyTrustRepSchema {
    pub status: Status,
}

impl_api_protocol_dump_load!(Invite3aClaimerSignifyTrustRepSchema);

/*
 * Invite4GreeterCommunicateReqSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite4GreeterCommunicateReqSchema {
    pub cmd: String,
    pub token: InvitationToken,
    #[serde_as(as = "Bytes")]
    pub payload: Vec<u8>,
}

impl_api_protocol_dump_load!(Invite4GreeterCommunicateReqSchema);

/*
 * Invite4GreeterCommunicateRepSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite4GreeterCommunicateRepSchema {
    pub status: Status,
    #[serde_as(as = "Bytes")]
    pub payload: Vec<u8>,
}

impl_api_protocol_dump_load!(Invite4GreeterCommunicateRepSchema);

/*
 * Invite4GreeterCommunicateRepSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite4ClaimerCommunicateReqSchema {
    pub cmd: String,
    #[serde_as(as = "Bytes")]
    pub payload: Vec<u8>,
}

impl_api_protocol_dump_load!(Invite4ClaimerCommunicateReqSchema);

/*
 * Invite4GreeterCommunicateRepSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite4ClaimerCommunicateRepSchema {
    pub status: Status,
    #[serde_as(as = "Bytes")]
    pub payload: Vec<u8>,
}

impl_api_protocol_dump_load!(Invite4ClaimerCommunicateRepSchema);
