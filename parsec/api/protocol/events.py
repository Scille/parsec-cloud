# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from enum import Enum

from parsec._parsec import EventsListenRep, EventsListenReq, EventsSubscribeRep, EventsSubscribeReq
from parsec.api.protocol.base import ApiCommandSerializer


__all__ = ("APIEvent", "events_listen_serializer", "events_subscribe_serializer")


class APIEvent(Enum):
    PINGED = "pinged"
    MESSAGE_RECEIVED = "message.received"
    INVITE_STATUS_CHANGED = "invite.status_changed"
    REALM_MAINTENANCE_FINISHED = "realm.maintenance_finished"
    REALM_MAINTENANCE_STARTED = "realm.maintenance_started"
    REALM_VLOBS_UPDATED = "realm.vlobs_updated"
    REALM_ROLES_UPDATED = "realm.roles_updated"
    PKI_ENROLLMENTS_UPDATED = "pki_enrollment.updated"


events_listen_serializer = ApiCommandSerializer(EventsListenReq, EventsListenRep)
events_subscribe_serializer = ApiCommandSerializer(EventsSubscribeReq, EventsSubscribeRep)
