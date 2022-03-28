// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};

use parsec_api_types::{maybe_field, InvitationToken, RealmID, RealmRole, VlobID};

use crate::InvitationStatus;

/*
 * APIEvent
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "event")]
pub enum APIEvent {
    #[serde(rename = "pinged")]
    Pinged { ping: String },
    #[serde(rename = "message.received")]
    MessageReceived { index: u64 },
    #[serde(rename = "invite.status_changed")]
    InviteStatusChanged {
        token: InvitationToken,
        invitation_status: InvitationStatus,
    },
    #[serde(rename = "realm.maintenance_finished")]
    RealmMaintenanceFinished {
        realm_id: RealmID,
        encryption_revision: u64,
    },
    #[serde(rename = "realm.maintenance_started")]
    RealmMaintenanceStarted {
        realm_id: RealmID,
        encryption_revision: u64,
    },
    #[serde(rename = "realm.vlobs_updated")]
    RealmVlobsUpdated {
        realm_id: RealmID,
        checkpoint: u64,
        src_id: VlobID,
        src_version: u64,
    },
    #[serde(rename = "realm.roles_updated")]
    RealmRolesUdpated { realm_id: RealmID, role: RealmRole },
}
/*
 * EventsListenReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsListenReq {
    pub wait: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum EventsListenRep {
    Ok(APIEvent),
    Cancelled {
        #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
        reason: Option<String>,
    },
    NoEvents,
    UnknownError {
        error: String,
    },
}

/*
 * EventsSubscribeReq
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsSubscribeReq;

/*
 * EventsSubscribeRep
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum EventsSubscribeRep {
    Ok,
    UnknownError { error: String },
}
