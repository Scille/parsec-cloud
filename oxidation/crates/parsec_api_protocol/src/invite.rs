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
 * InviteNewReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "UPPERCASE")]
pub enum InviteNewReq {
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

impl_api_protocol_dump_load!(InviteNewReq);

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
 * InviteNewRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum InviteNewRep {
    Ok {
        token: InvitationToken,
        email_sent: InvitationEmailSentStatus,
    },
    NotAllowed,
    AlreadyMember,
    NotAvailable,
}

impl_api_protocol_dump_load!(InviteNewRep);

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
 * InviteDeleteReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteDeleteReq {
    pub cmd: String,
    pub token: InvitationToken,
    pub reason: InvitationDeletedReason,
}

impl_api_protocol_dump_load!(InviteDeleteReq);

/*
 * InviteDeleteRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum InviteDeleteRep {
    Ok,
    NotFound,
    AlreadyDeleted,
}

impl_api_protocol_dump_load!(InviteDeleteRep);

/*
 * InviteListReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteListReq {
    pub cmd: String,
}

impl_api_protocol_dump_load!(InviteListReq);

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
pub enum InviteListItem {
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
 * InviteListRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum InviteListRep {
    Ok { invitations: Vec<InviteListItem> },
}

impl_api_protocol_dump_load!(InviteListRep);

/*
 * InviteInfoReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteInfoReq {
    pub cmd: String,
}

impl_api_protocol_dump_load!(InviteInfoReq);

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
 * InviteInfoRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum InviteInfoRep {
    Ok(InviteInfoUserOrDeviceRep),
}

impl_api_protocol_dump_load!(InviteInfoRep);

/*
 * Invite1ClaimerWaitPeerReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite1ClaimerWaitPeerReq {
    pub cmd: String,
    pub claimer_public_key: PublicKey,
}

impl_api_protocol_dump_load!(Invite1ClaimerWaitPeerReq);

/*
 * Invite1ClaimerWaitPeerRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite1ClaimerWaitPeerRep {
    Ok { greeter_public_key: PublicKey },
    NotFound,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite1ClaimerWaitPeerRep);

/*
 * Invite1GreeterWaitPeerReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite1GreeterWaitPeerReq {
    pub cmd: String,
    pub token: InvitationToken,
    pub greeter_public_key: PublicKey,
}

impl_api_protocol_dump_load!(Invite1GreeterWaitPeerReq);

/*
 * Invite1GreeterWaitPeerRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite1GreeterWaitPeerRep {
    Ok { claimer_public_key: PublicKey },
    NotFound,
    AlreadyDeleted,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite1GreeterWaitPeerRep);

/*
 * Invite2aClaimerSendHashedNonceHashNonceReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2aClaimerSendHashedNonceHashNonceReq {
    pub cmd: String,
    pub claimer_hashed_nonce: HashDigest,
}

impl_api_protocol_dump_load!(Invite2aClaimerSendHashedNonceHashNonceReq);

/*
 * Invite2aClaimerSendHashedNonceHashNonceRep
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite2aClaimerSendHashedNonceHashNonceRep {
    Ok {
        #[serde_as(as = "Bytes")]
        greeter_nonce: Vec<u8>,
    },
    NotFound,
    AlreadyDeleted,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite2aClaimerSendHashedNonceHashNonceRep);

/*
 * Invite2aGreeterGetHashedNonceRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite2aGreeterGetHashedNonceRep {
    Ok { claimer_hashed_nonce: HashDigest },
    NotFound,
    AlreadyDeleted,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite2aGreeterGetHashedNonceRep);

/*
 * Invite2bGreeterSendNonceRep
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite2bGreeterSendNonceRep {
    Ok {
        #[serde_as(as = "Bytes")]
        claimer_nonce: Vec<u8>,
    },
    NotFound,
    AlreadyDeleted,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite2bGreeterSendNonceRep);

/*
 * Invite2bClaimerSendNonceReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2bClaimerSendNonceReq {
    pub cmd: String,
    #[serde_as(as = "Bytes")]
    pub claimer_nonce: Vec<u8>,
}

impl_api_protocol_dump_load!(Invite2bClaimerSendNonceReq);

/*
 * Invite2bClaimerSendNonceRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite2bClaimerSendNonceRep {
    Ok,
    NotFound,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite2bClaimerSendNonceRep);

/*
 * Invite2bClaimerSendNonceReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3aGreeterWaitPeerTrustReq {
    pub cmd: String,
    pub token: InvitationToken,
}

impl_api_protocol_dump_load!(Invite3aGreeterWaitPeerTrustReq);

/*
 * Invite3aGreeterWaitPeerTrustRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite3aGreeterWaitPeerTrustRep {
    Ok,
    NotFound,
    AlreadyDeleted,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite3aGreeterWaitPeerTrustRep);

/*
 * Invite3bClaimerWaitPeerTrustReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3bClaimerWaitPeerTrustReq {
    pub cmd: String,
}

impl_api_protocol_dump_load!(Invite3bClaimerWaitPeerTrustReq);

/*
 * Invite3bClaimerWaitPeerTrustRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite3bClaimerWaitPeerTrustRep {
    Ok,
    NotFound,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite3bClaimerWaitPeerTrustRep);

/*
 * Invite3bGreeterSignifyTrustReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3bGreeterSignifyTrustReq {
    pub cmd: String,
    pub token: InvitationToken,
}

impl_api_protocol_dump_load!(Invite3bGreeterSignifyTrustReq);

/*
 * Invite3bGreeterSignifyTrustRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite3bGreeterSignifyTrustRep {
    Ok,
    NotFound,
    AlreadyDeleted,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite3bGreeterSignifyTrustRep);

/*
 * Invite3aClaimerSignifyTrustReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3aClaimerSignifyTrustReq {
    pub cmd: String,
}

impl_api_protocol_dump_load!(Invite3aClaimerSignifyTrustReq);

/*
 * Invite3aClaimerSignifyTrustRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite3aClaimerSignifyTrustRep {
    Ok,
    NotFound,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite3aClaimerSignifyTrustRep);

/*
 * Invite4GreeterCommunicateReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite4GreeterCommunicateReq {
    pub cmd: String,
    pub token: InvitationToken,
    #[serde_as(as = "Bytes")]
    pub payload: Vec<u8>,
}

impl_api_protocol_dump_load!(Invite4GreeterCommunicateReq);

/*
 * Invite4GreeterCommunicateRep
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite4GreeterCommunicateRep {
    Ok {
        #[serde_as(as = "Bytes")]
        payload: Vec<u8>,
    },
    NotFound,
    AlreadyDeleted,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite4GreeterCommunicateRep);

/*
 * Invite4GreeterCommunicateRep
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite4ClaimerCommunicateReq {
    pub cmd: String,
    #[serde_as(as = "Bytes")]
    pub payload: Vec<u8>,
}

impl_api_protocol_dump_load!(Invite4ClaimerCommunicateReq);

/*
 * Invite4GreeterCommunicateRep
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite4ClaimerCommunicateRep {
    Ok {
        #[serde_as(as = "Bytes")]
        payload: Vec<u8>,
    },
    NotFound,
    InvalidState,
}

impl_api_protocol_dump_load!(Invite4ClaimerCommunicateRep);
