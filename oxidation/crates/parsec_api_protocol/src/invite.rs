// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

use parsec_api_crypto::{HashDigest, PublicKey};
use parsec_api_types::{DateTime, HumanHandle, InvitationToken, UserID};

/*
 * InviteNewReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "UPPERCASE")]
pub enum InviteNewReq {
    User {
        claimer_email: String,
        send_email: bool,
    },
    Device {
        send_email: bool,
    },
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
    UnknownError {
        error: String,
    },
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
 * InviteDeleteReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteDeleteReq {
    pub token: InvitationToken,
    pub reason: InvitationDeletedReason,
}

/*
 * InviteDeleteRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum InviteDeleteRep {
    Ok,
    NotFound,
    AlreadyDeleted,
    UnknownError { error: String },
}

/*
 * InviteListReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteListReq;

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
    UnknownError { error: String },
}

/*
 * InviteInfoReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InviteInfoReq;

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
    UnknownError { error: String },
}

/*
 * Invite1ClaimerWaitPeerReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite1ClaimerWaitPeerReq {
    pub claimer_public_key: PublicKey,
}

/*
 * Invite1ClaimerWaitPeerRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite1ClaimerWaitPeerRep {
    Ok { greeter_public_key: PublicKey },
    NotFound,
    InvalidState,
    UnknownError { error: String },
}

/*
 * Invite1GreeterWaitPeerReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite1GreeterWaitPeerReq {
    pub token: InvitationToken,
    pub greeter_public_key: PublicKey,
}

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
    UnknownError { error: String },
}

/*
 * Invite2aClaimerSendHashedNonceHashNonceReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2aClaimerSendHashedNonceHashNonceReq {
    pub claimer_hashed_nonce: HashDigest,
}

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
    UnknownError {
        error: String,
    },
}

/*
 * Invite2aGreeterGetHashedNonceReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2aGreeterGetHashedNonceReq {
    pub token: InvitationToken,
}

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
    UnknownError { error: String },
}

/*
 * Invite2bClaimerSendNonceReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2bClaimerSendNonceReq {
    #[serde_as(as = "Bytes")]
    pub claimer_nonce: Vec<u8>,
}

/*
 * Invite2bClaimerSendNonceRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite2bClaimerSendNonceRep {
    Ok,
    NotFound,
    InvalidState,
    UnknownError { error: String },
}

/*
 * Invite2bGreeterSendNonceReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite2bGreeterSendNonceReq {
    pub token: InvitationToken,
    #[serde_as(as = "Bytes")]
    pub greeter_nonce: Vec<u8>,
}

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
    UnknownError {
        error: String,
    },
}

/*
 * Invite3aClaimerSignifyTrustReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3aClaimerSignifyTrustReq;

/*
 * Invite3aClaimerSignifyTrustRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite3aClaimerSignifyTrustRep {
    Ok,
    NotFound,
    InvalidState,
    UnknownError { error: String },
}

/*
 * Invite3aGreeterWaitPeerTrustReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3aGreeterWaitPeerTrustReq {
    pub token: InvitationToken,
}

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
    UnknownError { error: String },
}

/*
 * Invite3bClaimerWaitPeerTrustReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3bClaimerWaitPeerTrustReq;

/*
 * Invite3bClaimerWaitPeerTrustRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite3bClaimerWaitPeerTrustRep {
    Ok,
    NotFound,
    InvalidState,
    UnknownError { error: String },
}

/*
 * Invite3bGreeterSignifyTrustReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite3bGreeterSignifyTrustReq {
    pub token: InvitationToken,
}

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
    UnknownError { error: String },
}

/*
 * Invite4ClaimerCommunicateRep
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite4ClaimerCommunicateReq {
    #[serde_as(as = "Bytes")]
    pub payload: Vec<u8>,
}

/*
 * Invite4ClaimerCommunicateRep
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
    UnknownError {
        error: String,
    },
}

/*
 * Invite4GreeterCommunicateReq
 */

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Invite4GreeterCommunicateReq {
    pub token: InvitationToken,
    #[serde_as(as = "Bytes")]
    pub payload: Vec<u8>,
}

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
    UnknownError {
        error: String,
    },
}
