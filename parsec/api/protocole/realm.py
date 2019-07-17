# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from enum import Enum

from parsec.api.protocole.base import BaseRepSchema, BaseReqSchema, CmdSerializer
from parsec.serde import fields

__all__ = (
    "RealmRole",
    "RealmRoleField",
    "MaintenanceType",
    "MaintenanceTypeField",
    "realm_create_serializer",
    "realm_status_serializer",
    "realm_get_roles_serializer",
    "realm_get_role_certificates_serializer",
    "realm_update_roles_serializer",
    "realm_start_reencryption_maintenance_serializer",
    "realm_finish_reencryption_maintenance_serializer",
)


class MaintenanceType(Enum):
    GARBAGE_COLLECTION = "GARBAGE_COLLECTION"
    REENCRYPTION = "REENCRYPTION"


class RealmRole(Enum):
    OWNER = "OWNER"
    MANAGER = "MANAGER"
    CONTRIBUTOR = "CONTRIBUTOR"
    READER = "READER"


RealmRoleField = fields.enum_field_factory(RealmRole)
MaintenanceTypeField = fields.enum_field_factory(MaintenanceType)


class RealmCreateReqSchema(BaseReqSchema):
    role_certificate = fields.Bytes(required=True)


class RealmCreateRepSchema(BaseRepSchema):
    pass


realm_create_serializer = CmdSerializer(RealmCreateReqSchema, RealmCreateRepSchema)


class RealmStatusReqSchema(BaseReqSchema):
    realm_id = fields.UUID(required=True)


class RealmStatusRepSchema(BaseRepSchema):
    in_maintenance = fields.Boolean(required=True)
    maintenance_type = MaintenanceTypeField(allow_none=True)
    maintenance_started_on = fields.DateTime(allow_none=True, missing=None)
    maintenance_started_by = fields.DeviceID(allow_none=True, missing=None)
    encryption_revision = fields.Integer(required=True)


realm_status_serializer = CmdSerializer(RealmStatusReqSchema, RealmStatusRepSchema)


class RealmGetRoleCertificatesReqSchema(BaseReqSchema):
    realm_id = fields.UUID(required=True)
    since = fields.DateTime(allow_none=True, missing=None)


class RealmGetRoleCertificatesRepSchema(BaseRepSchema):
    certificates = fields.List(fields.Bytes(required=True), required=True)


realm_get_role_certificates_serializer = CmdSerializer(
    RealmGetRoleCertificatesReqSchema, RealmGetRoleCertificatesRepSchema
)


# TODO: get_roles api is deprecated
class RealmGetRolesReqSchema(BaseReqSchema):
    realm_id = fields.UUID(required=True)


class RealmGetRolesRepSchema(BaseRepSchema):
    # TODO: add granted_on, granted_by, terminated_on, terminated_by fields ?
    users = fields.Map(fields.UserID(), RealmRoleField(required=True), required=True)


realm_get_roles_serializer = CmdSerializer(RealmGetRolesReqSchema, RealmGetRolesRepSchema)


class RealmUpdateRolesReqSchema(BaseReqSchema):
    role_certificate = fields.Bytes(required=True)


class RealmUpdateRolesRepSchema(BaseRepSchema):
    pass


realm_update_roles_serializer = CmdSerializer(RealmUpdateRolesReqSchema, RealmUpdateRolesRepSchema)


class RealmStartReencryptionMaintenanceReqSchema(BaseReqSchema):
    realm_id = fields.UUID(required=True)
    encryption_revision = fields.Integer(required=True)
    timestamp = fields.DateTime(required=True)
    per_participant_message = fields.Map(
        fields.UserID(), fields.Bytes(required=True), required=True
    )


class RealmStartReencryptionMaintenanceRepSchema(BaseRepSchema):
    pass


realm_start_reencryption_maintenance_serializer = CmdSerializer(
    RealmStartReencryptionMaintenanceReqSchema, RealmStartReencryptionMaintenanceRepSchema
)


class RealmFinishReencryptionMaintenanceReqSchema(BaseReqSchema):
    realm_id = fields.UUID(required=True)
    encryption_revision = fields.Integer(required=True)


class RealmFinishReencryptionMaintenanceRepSchema(BaseRepSchema):
    pass


realm_finish_reencryption_maintenance_serializer = CmdSerializer(
    RealmFinishReencryptionMaintenanceReqSchema, RealmFinishReencryptionMaintenanceRepSchema
)
