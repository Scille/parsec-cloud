# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.serde import OneOfSchema, fields, validate
from parsec.api.protocole.base import BaseReqSchema, BaseRepSchema, CmdSerializer


EVENTS = ("pinged", "vlob_group.updated", "message.received")


class EventsPingedRepSchema(BaseRepSchema):
    status = fields.CheckedConstant("ok", required=True)
    event = fields.CheckedConstant("pinged", required=True)
    ping = fields.String(validate=validate.Length(max=64), required=True)


class EventsVlobGroupUpdatedRepSchema(BaseRepSchema):
    status = fields.CheckedConstant("ok", required=True)
    event = fields.CheckedConstant("vlob_group.updated", required=True)
    id = fields.UUID(required=True)
    checkpoint = fields.Integer(required=True)
    src_id = fields.UUID(required=True)
    src_version = fields.Integer(required=True)


class EventsMessageReceivedRepSchema(BaseRepSchema):
    status = fields.CheckedConstant("ok", required=True)
    event = fields.CheckedConstant("message.received", required=True)
    index = fields.Integer(required=True)


class EventsListenReqSchema(BaseReqSchema):
    wait = fields.Boolean(missing=True)


class EventsListenRepSchema(OneOfSchema):
    type_field = "event"
    type_field_remove = False
    type_schemas = {
        "pinged": EventsPingedRepSchema(),
        "vlob_group.updated": EventsVlobGroupUpdatedRepSchema(),
        "message.received": EventsMessageReceivedRepSchema(),
    }

    def get_obj_type(self, obj):
        return obj["event"]


events_listen_serializer = CmdSerializer(EventsListenReqSchema, EventsListenRepSchema)


class EventsSubscribeReqSchema(BaseReqSchema):
    pinged = fields.List(fields.String(validate=validate.Length(max=64)), missing=None)
    vlob_group_updated = fields.List(fields.UUID(), missing=None)
    message_received = fields.Boolean(missing=None)


class EventsSubscribeRepSchema(BaseRepSchema):
    pass


events_subscribe_serializer = CmdSerializer(EventsSubscribeReqSchema, EventsSubscribeRepSchema)
