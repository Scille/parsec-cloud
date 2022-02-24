// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};

use crate::InvitationStatus;
use parsec_api_types::{InvitationToken, RealmID, RealmRole, VlobID};

/*
 * APIEvent
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum APIEvent {
    Pinged(Pinged),
    MessageReceived(MessageReceived),
    InviteStatusChanged(InviteStatusChanged),
    RealmMaintenanceFinished(RealmMaintenanceFinished),
    RealmMaintenanceStarted(RealmMaintenanceStarted),
    RealmVlobsUpdated(RealmVlobsUpdated),
    RealmRolesUdpated(RealmRolesUdpated),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "pinged")]
pub struct Pinged;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "message.received")]
pub struct MessageReceived;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "invite.status_changed")]
pub struct InviteStatusChanged;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "realm.maintenance_finished")]
pub struct RealmMaintenanceFinished;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "realm.maintenance_started")]
pub struct RealmMaintenanceStarted;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "realm.vlobs_updated")]
pub struct RealmVlobsUpdated;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "realm.roles_updated")]
pub struct RealmRolesUdpated;

/*
 * EventsPingedRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsPingedRepSchema {
    pub event: Pinged,
    pub ping: String,
}

/*
 * EventsRealmRolesUpdatedRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsRealmRolesUpdatedRepSchema {
    pub event: RealmRolesUdpated,
    pub realm_id: RealmID,
    pub role: RealmRole,
}

/*
 * EventsRealmVlobsUpdatedRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsRealmVlobsUpdatedRepSchema {
    pub event: RealmVlobsUpdated,
    pub realm_id: RealmID,
    pub checkpoint: u64,
    pub src_id: VlobID,
    pub src_version: u64,
}

/*
 * EventsRealmMaintenanceStartedRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsRealmMaintenanceStartedRepSchema {
    pub event: RealmMaintenanceStarted,
    pub realm_id: RealmID,
    pub encryption_revision: u64,
}

/*
 * EventsRealmMaintenanceFinishedRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsRealmMaintenanceFinishedRepSchema {
    pub event: RealmMaintenanceFinished,
    pub realm_id: RealmID,
    pub encryption_revision: u64,
}

/*
 * EventsMessageReceivedRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsMessageReceivedRepSchema {
    pub event: MessageReceived,
    pub index: u64,
}

/*
 * EventsInviteStatusChangedRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsInviteStatusChangedRepSchema {
    pub event: InviteStatusChanged,
    pub token: InvitationToken,
    pub invitation_status: InvitationStatus,
}

/*
 * EventsListenReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsListenReqSchema {
    pub wait: bool,
}

/*
 * Event
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "event")]
pub struct Event;

/*
 * EventsListenRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsListenRepSchema {
    pub type_field: Event,
    pub type_schemas: APIEvent,
}

/*
 * EventsSubscribeReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsSubscribeReqSchema {
    pub cmd: String,
}

/*
 * EventsSubscribeRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsSubscribeRepSchema;
