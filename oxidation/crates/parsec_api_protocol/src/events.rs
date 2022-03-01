// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use serde::{Deserialize, Serialize};

use crate::{impl_api_protocol_dump_load, InvitationStatus};
use parsec_api_types::{InvitationToken, RealmID, RealmRole, VlobID};

/*
 * APIEvent
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "event")]
pub enum APIEvent {
    #[serde(rename = "pinged")]
    Pinged(EventsPingedRepSchema),
    #[serde(rename = "message.received")]
    MessageReceived(EventsMessageReceivedRepSchema),
    #[serde(rename = "invite.status_changed")]
    InviteStatusChanged(EventsInviteStatusChangedRepSchema),
    #[serde(rename = "realm.maintenance_finished")]
    RealmMaintenanceFinished(EventsRealmMaintenanceFinishedRepSchema),
    #[serde(rename = "realm.maintenance_started")]
    RealmMaintenanceStarted(EventsRealmMaintenanceStartedRepSchema),
    #[serde(rename = "realm.vlobs_updated")]
    RealmVlobsUpdated(EventsRealmVlobsUpdatedRepSchema),
    #[serde(rename = "realm.roles_updated")]
    RealmRolesUdpated(EventsRealmRolesUpdatedRepSchema),
}

/*
 * EventsPingedRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsPingedRepSchema {
    pub ping: String,
}

/*
 * EventsRealmRolesUpdatedRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsRealmRolesUpdatedRepSchema {
    pub realm_id: RealmID,
    pub role: RealmRole,
}

/*
 * EventsRealmVlobsUpdatedRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsRealmVlobsUpdatedRepSchema {
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
    pub realm_id: RealmID,
    pub encryption_revision: u64,
}

/*
 * EventsRealmMaintenanceFinishedRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsRealmMaintenanceFinishedRepSchema {
    pub realm_id: RealmID,
    pub encryption_revision: u64,
}

/*
 * EventsMessageReceivedRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsMessageReceivedRepSchema {
    pub index: u64,
}

/*
 * EventsInviteStatusChangedRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsInviteStatusChangedRepSchema {
    pub token: InvitationToken,
    pub invitation_status: InvitationStatus,
}

/*
 * EventsListenReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsListenReqSchema {
    pub cmd: String,
    pub wait: bool,
}

impl_api_protocol_dump_load!(EventsListenReqSchema);

/*
 * EventsListenRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum EventsListenRepSchema {
    Ok(APIEvent),
}

impl_api_protocol_dump_load!(EventsListenRepSchema);

/*
 * EventsSubscribeReqSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventsSubscribeReqSchema {
    pub cmd: String,
}

impl_api_protocol_dump_load!(EventsSubscribeReqSchema);

/*
 * EventsSubscribeRepSchema
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum EventsSubscribeRepSchema {
    Ok,
}

impl_api_protocol_dump_load!(EventsSubscribeRepSchema);
