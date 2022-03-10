# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from enum import Enum
from typing import Dict, cast

from parsec.serde import OneOfSchema, fields, validate
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.realm import RealmIDField, RealmRoleField
from parsec.api.protocol.vlob import VlobIDField
from parsec.api.protocol.invite import InvitationTokenField, InvitationStatusField


__all__ = ("APIEvent", "events_listen_serializer", "events_subscribe_serializer")


class APIEvent(Enum):
    PINGED = "pinged"
    MESSAGE_RECEIVED = "message.received"
    INVITE_STATUS_CHANGED = "invite.status_changed"
    REALM_MAINTENANCE_FINISHED = "realm.maintenance_finished"
    REALM_MAINTENANCE_STARTED = "realm.maintenance_started"
    REALM_VLOBS_UPDATED = "realm.vlobs_updated"
    REALM_ROLES_UPDATED = "realm.roles_updated"


class EventsPingedRepSchema(BaseRepSchema):
    event = fields.EnumCheckedConstant(APIEvent.PINGED, required=True)
    ping = fields.String(validate=validate.Length(max=64), required=True)


class EventsRealmRolesUpdatedRepSchema(BaseRepSchema):
    event = fields.EnumCheckedConstant(APIEvent.REALM_ROLES_UPDATED, required=True)
    realm_id = RealmIDField(required=True)
    role = RealmRoleField(required=True, allow_none=True)


class EventsRealmVlobsUpdatedRepSchema(BaseRepSchema):
    event = fields.EnumCheckedConstant(APIEvent.REALM_VLOBS_UPDATED, required=True)
    realm_id = RealmIDField(required=True)
    checkpoint = fields.Integer(required=True)
    src_id = VlobIDField(required=True)
    src_version = fields.Integer(required=True)


class EventsRealmMaintenanceStartedRepSchema(BaseRepSchema):
    event = fields.EnumCheckedConstant(APIEvent.REALM_MAINTENANCE_STARTED, required=True)
    realm_id = RealmIDField(required=True)
    encryption_revision = fields.Integer(required=True)


class EventsRealmMaintenanceFinishedRepSchema(BaseRepSchema):
    event = fields.EnumCheckedConstant(APIEvent.REALM_MAINTENANCE_FINISHED, required=True)
    realm_id = RealmIDField(required=True)
    encryption_revision = fields.Integer(required=True)


class EventsMessageReceivedRepSchema(BaseRepSchema):
    event = fields.EnumCheckedConstant(APIEvent.MESSAGE_RECEIVED, required=True)
    index = fields.Integer(required=True)


class EventsInviteStatusChangedRepSchema(BaseRepSchema):
    event = fields.EnumCheckedConstant(APIEvent.INVITE_STATUS_CHANGED, required=True)
    token = InvitationTokenField(required=True)
    invitation_status = InvitationStatusField(required=True)


class EventsListenReqSchema(BaseReqSchema):
    wait = fields.Boolean(missing=True)


class EventsListenRepSchema(OneOfSchema):
    type_field = "event"
    type_schemas = {
        APIEvent.PINGED: EventsPingedRepSchema(),
        APIEvent.MESSAGE_RECEIVED: EventsMessageReceivedRepSchema(),
        APIEvent.INVITE_STATUS_CHANGED: EventsInviteStatusChangedRepSchema(),
        APIEvent.REALM_ROLES_UPDATED: EventsRealmRolesUpdatedRepSchema(),
        APIEvent.REALM_VLOBS_UPDATED: EventsRealmVlobsUpdatedRepSchema(),
        APIEvent.REALM_MAINTENANCE_STARTED: EventsRealmMaintenanceStartedRepSchema(),
        APIEvent.REALM_MAINTENANCE_FINISHED: EventsRealmMaintenanceFinishedRepSchema(),
    }

    def get_obj_type(self, obj: Dict[str, object]) -> APIEvent:
        return cast(APIEvent, obj["event"])


events_listen_serializer = CmdSerializer(EventsListenReqSchema, EventsListenRepSchema)


class EventsSubscribeReqSchema(BaseReqSchema):
    pass


class EventsSubscribeRepSchema(BaseRepSchema):
    pass


events_subscribe_serializer = CmdSerializer(EventsSubscribeReqSchema, EventsSubscribeRepSchema)
