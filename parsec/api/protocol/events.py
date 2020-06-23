# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from enum import Enum

from parsec.serde import OneOfSchema, fields, validate
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.realm import RealmRoleField
from parsec.api.protocol.invite import InvitationStatusField


class Event(Enum):
    pinged = "pinged"
    message_received = "message.received"
    invite_status_changed = "invite.status_changed"
    realm_maintenance_finished = "realm.maintenance_finished"
    realm_maintenance_started = "realm.maintenance_started"
    realm_vlobs_updated = "realm.vlobs_updated"
    realm_roles_updated = "realm.roles_updated"


class EventsPingedRepSchema(BaseRepSchema):
    event = fields.EnumCheckedConstant(Event.pinged, required=True)
    ping = fields.String(validate=validate.Length(max=64), required=True)


class EventsRealmRolesUpdatedRepSchema(BaseRepSchema):
    event = fields.EnumCheckedConstant(Event.realm_roles_updated, required=True)
    realm_id = fields.UUID(required=True)
    role = RealmRoleField(required=True, allow_none=True)


class EventsRealmVlobsUpdatedRepSchema(BaseRepSchema):
    event = fields.EnumCheckedConstant(Event.realm_vlobs_updated, required=True)
    realm_id = fields.UUID(required=True)
    checkpoint = fields.Integer(required=True)
    src_id = fields.UUID(required=True)
    src_version = fields.Integer(required=True)


class EventsRealmMaintenanceStartedRepSchema(BaseRepSchema):
    event = fields.EnumCheckedConstant(Event.realm_maintenance_started, required=True)
    realm_id = fields.UUID(required=True)
    encryption_revision = fields.Integer(required=True)


class EventsRealmMaintenanceFinishedRepSchema(BaseRepSchema):
    event = fields.EnumCheckedConstant(Event.realm_maintenance_finished, required=True)
    realm_id = fields.UUID(required=True)
    encryption_revision = fields.Integer(required=True)


class EventsMessageReceivedRepSchema(BaseRepSchema):
    event = fields.EnumCheckedConstant(Event.message_received, required=True)
    index = fields.Integer(required=True)


class EventsInviteStatusChangedRepSchema(BaseRepSchema):
    event = fields.EnumCheckedConstant(Event.invite_status_changed, required=True)
    token = fields.UUID(required=True)
    invitation_status = InvitationStatusField(required=True)


class EventsListenReqSchema(BaseReqSchema):
    wait = fields.Boolean(missing=True)


class EventsListenRepSchema(OneOfSchema):
    type_field = "event"
    type_field_remove = False
    type_schemas = {
        Event.pinged: EventsPingedRepSchema(),
        Event.message_received: EventsMessageReceivedRepSchema(),
        Event.invite_status_changed: EventsInviteStatusChangedRepSchema(),
        Event.realm_roles_updated: EventsRealmRolesUpdatedRepSchema(),
        Event.realm_vlobs_updated: EventsRealmVlobsUpdatedRepSchema(),
        Event.realm_maintenance_started: EventsRealmMaintenanceStartedRepSchema(),
        Event.realm_maintenance_finished: EventsRealmMaintenanceFinishedRepSchema(),
    }

    def get_obj_type(self, obj):
        return obj["event"]


events_listen_serializer = CmdSerializer(EventsListenReqSchema, EventsListenRepSchema)


class EventsSubscribeReqSchema(BaseReqSchema):
    pass


class EventsSubscribeRepSchema(BaseRepSchema):
    pass


events_subscribe_serializer = CmdSerializer(EventsSubscribeReqSchema, EventsSubscribeRepSchema)
