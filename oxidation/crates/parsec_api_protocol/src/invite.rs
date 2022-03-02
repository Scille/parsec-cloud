// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

use crate::impl_api_protocol_dump_load;
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
 * InviteNewReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "UPPERCASE")]
pub enum InviteNewReqSchema {
    User {
        cmd: String,
        claimer_email: String,
        send_email: bool,
    },
    Device {
        cmd: String,
        send_email: bool,
    },
}

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
#[serde(tag = "status", rename_all = "snake_case")]
pub enum InviteNewRepSchema {
    Ok {
        token: InvitationToken,
        email_sent: InvitationEmailSentStatus,
    },
    NotAllowed,
    AlreadyMember,
    NotAvailable,
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
#[serde(tag = "status", rename_all = "snake_case")]
pub enum InviteDeleteRepSchema {
    Ok,
    NotFound,
    AlreadyDeleted,
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
 * InviteUserOrDevice
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "UPPERCASE")]
pub enum InviteListItemSchema {
    User {
        token: InvitationToken,
        created_on: DateTime,
        claimer_email: String,
        status: InvitationStatus,
    },
    Device {
        token: InvitationToken,
        created_on: DateTime,
        status: InvitationStatus,
    },
}

/*
 * InviteListRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum InviteListRepSchema {
    Ok {
        invitations: Vec<InviteListItemSchema>,
    },
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
 * InviteUserOrDeviceRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "UPPERCASE")]
pub enum InviteInfoUserOrDeviceRep {
    User {
        claimer_email: String,
        greeter_user_id: UserID,
        greeter_human_handle: HumanHandle,
    },
    Device {
        greeter_user_id: UserID,
        greeter_human_handle: HumanHandle,
    },
}

/*
 * InviteInfoRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum InviteInfoRepSchema {
    Ok(InviteInfoUserOrDeviceRep),
}

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
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite1ClaimerWaitPeerRepSchema {
    Ok { greeter_public_key: PublicKey },
    NotFound,
    InvalidState,
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
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite1GreeterWaitPeerRepSchema {
    Ok { claimer_public_key: PublicKey },
    NotFound,
    AlreadyDeleted,
    InvalidState,
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
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite2aClaimerSendHashedNonceHashNonceRepSchema {
    Ok {
        #[serde_as(as = "Bytes")]
        greeter_nonce: Vec<u8>,
    },
    NotFound,
    AlreadyDeleted,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite2aClaimerSendHashedNonceHashNonceRepSchema);

/*
 * Invite2aGreeterGetHashedNonceRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite2aGreeterGetHashedNonceRepSchema {
    Ok { claimer_hashed_nonce: HashDigest },
    NotFound,
    AlreadyDeleted,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite2aGreeterGetHashedNonceRepSchema);

/*
 * Invite2bGreeterSendNonceRepSchema
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite2bGreeterSendNonceRepSchema {
    Ok {
        #[serde_as(as = "Bytes")]
        claimer_nonce: Vec<u8>,
    },
    NotFound,
    AlreadyDeleted,
    InvalidState,
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
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite2bClaimerSendNonceRepSchema {
    Ok,
    NotFound,
    InvalidState,
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
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite3aGreeterWaitPeerTrustRepSchema {
    Ok,
    NotFound,
    AlreadyDeleted,
    InvalidState,
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
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite3bClaimerWaitPeerTrustRepSchema {
    Ok,
    NotFound,
    InvalidState,
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
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite3bGreeterSignifyTrustRepSchema {
    Ok,
    NotFound,
    AlreadyDeleted,
    InvalidState,
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
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite3aClaimerSignifyTrustRepSchema {
    Ok,
    NotFound,
    InvalidState,
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
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite4GreeterCommunicateRepSchema {
    Ok {
        #[serde_as(as = "Bytes")]
        payload: Vec<u8>,
    },
    NotFound,
    AlreadyDeleted,
    InvalidState,
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
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite4ClaimerCommunicateRepSchema {
    Ok {
        #[serde_as(as = "Bytes")]
        payload: Vec<u8>,
    },
    NotFound,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite4ClaimerCommunicateRepSchema);
