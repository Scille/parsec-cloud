// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};

use parsec_api_crypto::{HashDigest, PublicKey};
use parsec_api_types::{DateTime, HumanHandle, InvitationToken, UserID};
use parsec_schema::parsec_schema;

#[parsec_schema]
#[serde(tag = "type", rename_all = "UPPERCASE")]
pub enum InviteNewReq {
    User {
        claimer_email: String,
        send_email: bool,
    },
    Device {
        send_email: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvitationEmailSentStatus {
    Success,
    NotAvailable,
    BadRecipient,
}

#[parsec_schema]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvitationDeletedReason {
    Finished,
    Cancelled,
    Rotten,
}

#[parsec_schema]
pub struct InviteDeleteReq {
    pub token: InvitationToken,
    pub reason: InvitationDeletedReason,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum InviteDeleteRep {
    Ok,
    NotFound,
    AlreadyDeleted,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct InviteListReq;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvitationStatus {
    Idle,
    Ready,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
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

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum InviteListRep {
    Ok { invitations: Vec<InviteListItem> },
    UnknownError { error: String },
}

#[parsec_schema]
pub struct InviteInfoReq;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
pub enum InviteInfoUserOrDevice {
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

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum InviteInfoRep {
    Ok(InviteInfoUserOrDevice),
    UnknownError { error: String },
}

#[parsec_schema]
pub struct Invite1ClaimerWaitPeerReq {
    pub claimer_public_key: PublicKey,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite1ClaimerWaitPeerRep {
    Ok { greeter_public_key: PublicKey },
    NotFound,
    InvalidState,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct Invite1GreeterWaitPeerReq {
    pub token: InvitationToken,
    pub greeter_public_key: PublicKey,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite1GreeterWaitPeerRep {
    Ok { claimer_public_key: PublicKey },
    NotFound,
    AlreadyDeleted,
    InvalidState,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct Invite2aClaimerSendHashedNonceHashNonceReq {
    pub claimer_hashed_nonce: HashDigest,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite2aClaimerSendHashedNonceHashNonceRep {
    Ok { greeter_nonce: Vec<u8> },
    NotFound,
    AlreadyDeleted,
    InvalidState,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct Invite2aGreeterGetHashedNonceReq {
    pub token: InvitationToken,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite2aGreeterGetHashedNonceRep {
    Ok { claimer_hashed_nonce: HashDigest },
    NotFound,
    AlreadyDeleted,
    InvalidState,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct Invite2bClaimerSendNonceReq {
    pub claimer_nonce: Vec<u8>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite2bClaimerSendNonceRep {
    Ok,
    NotFound,
    InvalidState,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct Invite2bGreeterSendNonceReq {
    pub token: InvitationToken,
    pub greeter_nonce: Vec<u8>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite2bGreeterSendNonceRep {
    Ok { claimer_nonce: Vec<u8> },
    NotFound,
    AlreadyDeleted,
    InvalidState,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct Invite3aClaimerSignifyTrustReq;

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite3aClaimerSignifyTrustRep {
    Ok,
    NotFound,
    InvalidState,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct Invite3aGreeterWaitPeerTrustReq {
    pub token: InvitationToken,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite3aGreeterWaitPeerTrustRep {
    Ok,
    NotFound,
    AlreadyDeleted,
    InvalidState,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct Invite3bClaimerWaitPeerTrustReq;

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite3bClaimerWaitPeerTrustRep {
    Ok,
    NotFound,
    InvalidState,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct Invite3bGreeterSignifyTrustReq {
    pub token: InvitationToken,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite3bGreeterSignifyTrustRep {
    Ok,
    NotFound,
    AlreadyDeleted,
    InvalidState,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct Invite4ClaimerCommunicateReq {
    pub payload: Vec<u8>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite4ClaimerCommunicateRep {
    Ok { payload: Vec<u8> },
    NotFound,
    InvalidState,
    UnknownError { error: String },
}

#[parsec_schema]
pub struct Invite4GreeterCommunicateReq {
    pub token: InvitationToken,
    pub payload: Vec<u8>,
}

#[parsec_schema]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Invite4GreeterCommunicateRep {
    Ok { payload: Vec<u8> },
    NotFound,
    AlreadyDeleted,
    InvalidState,
    UnknownError { error: String },
}
