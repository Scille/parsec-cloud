# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.serde import OneOfSchema, fields, validate
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.realm import RealmRoleField
from parsec.api.protocol.invite import InvitationStatusField


EVENTS = (
    "pinged",
    "realm.roles_updated",
    "realm.vlobs_updated",
    "realm.maintenance_started",
    "realm.maintenance_finished",
    "message.received",
    "invite.status_changed",
)


class EventsPingedRepSchema(BaseRepSchema):
    event = fields.CheckedConstant("pinged", required=True)
    ping = fields.String(validate=validate.Length(max=64), required=True)


class EventsRealmRolesUpdatedRepSchema(BaseRepSchema):
    event = fields.CheckedConstant("realm.roles_updated", required=True)
    realm_id = fields.UUID(required=True)
    role = RealmRoleField(required=True, allow_none=True)


class EventsRealmVlobsUpdatedRepSchema(BaseRepSchema):
    event = fields.CheckedConstant("realm.vlobs_updated", required=True)
    realm_id = fields.UUID(required=True)
    checkpoint = fields.Integer(required=True)
    src_id = fields.UUID(required=True)
    src_version = fields.Integer(required=True)


class EventsRealmMaintenanceStartedRepSchema(BaseRepSchema):
    event = fields.CheckedConstant("realm.maintenance_started", required=True)
    realm_id = fields.UUID(required=True)
    encryption_revision = fields.Integer(required=True)


class EventsRealmMaintenanceFinishedRepSchema(BaseRepSchema):
    event = fields.CheckedConstant("realm.maintenance_finished", required=True)
    realm_id = fields.UUID(required=True)
    encryption_revision = fields.Integer(required=True)


class EventsMessageReceivedRepSchema(BaseRepSchema):
    event = fields.CheckedConstant("message.received", required=True)
    index = fields.Integer(required=True)


class EventsInviteStatusChangedRepSchema(BaseRepSchema):
    event = fields.CheckedConstant("invite.status_changed", required=True)
    token = fields.UUID(required=True)
    invitation_status = InvitationStatusField(required=True)


class EventsListenReqSchema(BaseReqSchema):
    wait = fields.Boolean(missing=True)


class EventsListenRepSchema(OneOfSchema):
    type_field = "event"
    type_field_remove = False
    type_schemas = {
        "pinged": EventsPingedRepSchema(),
        "realm.roles_updated": EventsRealmRolesUpdatedRepSchema(),
        "realm.vlobs_updated": EventsRealmVlobsUpdatedRepSchema(),
        "realm.maintenance_started": EventsRealmMaintenanceStartedRepSchema(),
        "realm.maintenance_finished": EventsRealmMaintenanceFinishedRepSchema(),
        "message.received": EventsMessageReceivedRepSchema(),
        "invite.status_changed": EventsInviteStatusChangedRepSchema(),
    }

    def get_obj_type(self, obj):
        return obj["event"]


events_listen_serializer = CmdSerializer(EventsListenReqSchema, EventsListenRepSchema)


class EventsSubscribeReqSchema(BaseReqSchema):
    pass


class EventsSubscribeRepSchema(BaseRepSchema):
    pass


events_subscribe_serializer = CmdSerializer(EventsSubscribeReqSchema, EventsSubscribeRepSchema)
