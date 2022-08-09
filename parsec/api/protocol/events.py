# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

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
    PKI_ENROLLMENTS_UPDATED = "pki_enrollment.updated"


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
    token = InvitationTokenField(required=True)  # type: ignore[arg-type]
    invitation_status = InvitationStatusField(required=True)  # type: ignore[arg-type]


class EventPkiEnrollmentUpdated(BaseRepSchema):
    event = fields.EnumCheckedConstant(APIEvent.PKI_ENROLLMENTS_UPDATED, required=True)


class EventsListenReqSchema(BaseReqSchema):
    wait = fields.Boolean(required=True)


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
        APIEvent.PKI_ENROLLMENTS_UPDATED: EventPkiEnrollmentUpdated(),
    }

    def get_obj_type(self, obj: Dict[str, object]) -> APIEvent:
        return cast(APIEvent, obj["event"])


events_listen_serializer = CmdSerializer(EventsListenReqSchema, EventsListenRepSchema)


class EventsSubscribeReqSchema(BaseReqSchema):
    pass


class EventsSubscribeRepSchema(BaseRepSchema):
    pass


events_subscribe_serializer = CmdSerializer(EventsSubscribeReqSchema, EventsSubscribeRepSchema)
