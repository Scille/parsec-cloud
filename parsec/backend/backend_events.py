# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from enum import Enum
from parsec.api.protocol import (
    OrganizationIDField,
    DeviceIDField,
    UserIDField,
    InvitationTokenField,
    InvitationStatusField,
    VlobIDField,
    RealmIDField,
    RealmRoleField,
)
from parsec.serde import fields, OneOfSchema, BaseSchema, MsgpackSerializer


__all__ = ("BackendEvent",)


class BackendEvent(Enum):
    """ Backend internal events"""

    # DEVICE_CLAIMED = "device.claimed"  # TODO: not used anymore
    DEVICE_CREATED = "device.created"  # TODO: not needed anymore ?
    # DEVICE_INVITATION_CANCELLED = "device.invitation.cancelled"
    INVITE_CONDUIT_UPDATED = "invite.conduit_updated"
    # USER_CLAIMED = "user.claimed"  # TODO: not used anymore
    USER_CREATED = "user.created"  # TODO: not needed anymore ?
    USER_REVOKED = "user.revoked"
    # USER_INVITATION_CANCELLED = "user.invitation.cancelled"  # TODO: not used anymore
    ORGANIZATION_EXPIRED = "organization.expired"
    # api Event mirror
    PINGED = "pinged"
    MESSAGE_RECEIVED = "message.received"
    INVITE_STATUS_CHANGED = "invite.status_changed"
    REALM_MAINTENANCE_FINISHED = "realm.maintenance_finished"
    REALM_MAINTENANCE_STARTED = "realm.maintenance_started"
    REALM_VLOBS_UPDATED = "realm.vlobs_updated"
    REALM_ROLES_UPDATED = "realm.roles_updated"


class DeviceCreatedSchema(BaseSchema):
    __id__ = fields.String(required=True)
    __signal__ = fields.EnumCheckedConstant(BackendEvent.DEVICE_CREATED, required=True)
    organization_id = OrganizationIDField(requise=True)
    device_id = DeviceIDField(requise=True)
    device_certificate = fields.Bytes(requise=True)
    encrypted_answer = fields.Bytes(requise=True)


class InviteConduitUpdatedSchema(BaseSchema):
    __id__ = fields.String(required=True)
    __signal__ = fields.EnumCheckedConstant(BackendEvent.INVITE_CONDUIT_UPDATED, required=True)
    organization_id = OrganizationIDField(requise=True)
    token = InvitationTokenField(required=True)


class UserCreatedSchema(BaseSchema):
    __id__ = fields.String(required=True)
    __signal__ = fields.EnumCheckedConstant(BackendEvent.USER_CREATED, required=True)
    organization_id = OrganizationIDField(requise=True)
    user_id = UserIDField(required=True)
    user_certificate = fields.Bytes(required=True)
    first_device_id = DeviceIDField(required=True)
    first_device_certificate = fields.Bytes(required=True)


class UserRevokedSchema(BaseSchema):
    __id__ = fields.String(required=True)
    __signal__ = fields.EnumCheckedConstant(BackendEvent.USER_REVOKED, required=True)
    organization_id = OrganizationIDField(requise=True)
    user_id = UserIDField(required=True)


class OrganizationExpiredSchema(BaseSchema):
    __id__ = fields.String(required=True)
    __signal__ = fields.EnumCheckedConstant(BackendEvent.ORGANIZATION_EXPIRED, required=True)
    organization_id = OrganizationIDField(requise=True)


class PingedSchema(BaseSchema):
    __id__ = fields.String(required=True)
    __signal__ = fields.EnumCheckedConstant(BackendEvent.PINGED, required=True)
    organization_id = OrganizationIDField(requise=True)
    author = DeviceIDField(required=True)
    ping = fields.String(required=True)


class MessageReceivedSchema(BaseSchema):
    __id__ = fields.String(required=True)
    __signal__ = fields.EnumCheckedConstant(BackendEvent.MESSAGE_RECEIVED, required=True)
    organization_id = OrganizationIDField(requise=True)
    author = DeviceIDField(required=True)
    recipient = UserIDField(required=True)
    index = fields.Integer(required=True)


class InviteStatusChangedSchema(BaseSchema):
    __id__ = fields.String(required=True)
    __signal__ = fields.EnumCheckedConstant(BackendEvent.INVITE_STATUS_CHANGED, required=True)
    organization_id = OrganizationIDField(requise=True)
    greeter = UserIDField(required=True)
    token = InvitationTokenField(required=True)
    status = InvitationStatusField(required=True)


class RealmMaintenanceFinishedSchema(BaseSchema):
    __id__ = fields.String(required=True)
    __signal__ = fields.EnumCheckedConstant(BackendEvent.REALM_MAINTENANCE_FINISHED, required=True)
    organization_id = OrganizationIDField(requise=True)
    author = DeviceIDField(required=True)
    realm_id = RealmIDField(required=True)
    encryption_revision = fields.Integer(required=True)


class RealmMaintenanceStartedSchema(BaseSchema):
    __id__ = fields.String(required=True)
    __signal__ = fields.EnumCheckedConstant(BackendEvent.REALM_MAINTENANCE_STARTED, required=True)
    organization_id = OrganizationIDField(requise=True)
    author = DeviceIDField(required=True)
    realm_id = RealmIDField(required=True)
    encryption_revision = fields.Integer(required=True)


class RealmVlobsUpdatedSchema(BaseSchema):
    __id__ = fields.String(required=True)
    __signal__ = fields.EnumCheckedConstant(BackendEvent.REALM_VLOBS_UPDATED, required=True)
    organization_id = OrganizationIDField(requise=True)
    author = DeviceIDField(required=True)
    realm_id = RealmIDField(required=True)
    checkpoint = fields.Integer(required=True)
    src_id = VlobIDField(required=True)
    src_version = fields.Integer(required=True)


class RealmRolesUpdatedSchema(BaseSchema):
    __id__ = fields.String(required=True)
    __signal__ = fields.EnumCheckedConstant(BackendEvent.REALM_ROLES_UPDATED, required=True)
    organization_id = OrganizationIDField(requise=True)
    author = DeviceIDField(required=True)
    realm_id = RealmIDField(required=True)
    user = UserIDField(required=True)
    role = RealmRoleField(required=True, allow_none=True)


class BackendEventSchema(OneOfSchema):
    type_field = "__signal__"
    type_schemas = {
        BackendEvent.DEVICE_CREATED: DeviceCreatedSchema,
        BackendEvent.INVITE_CONDUIT_UPDATED: InviteConduitUpdatedSchema,
        BackendEvent.USER_CREATED: UserCreatedSchema,
        BackendEvent.USER_REVOKED: UserRevokedSchema,
        BackendEvent.ORGANIZATION_EXPIRED: OrganizationExpiredSchema,
        BackendEvent.PINGED: PingedSchema,
        BackendEvent.MESSAGE_RECEIVED: MessageReceivedSchema,
        BackendEvent.INVITE_STATUS_CHANGED: InviteStatusChangedSchema,
        BackendEvent.REALM_MAINTENANCE_FINISHED: RealmMaintenanceFinishedSchema,
        BackendEvent.REALM_MAINTENANCE_STARTED: RealmMaintenanceStartedSchema,
        BackendEvent.REALM_VLOBS_UPDATED: RealmVlobsUpdatedSchema,
        BackendEvent.REALM_ROLES_UPDATED: RealmRolesUpdatedSchema,
    }

    def get_obj_type(self, obj):
        return obj["__signal__"]


backend_event_serializer = MsgpackSerializer(BackendEventSchema)
