# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    MaintenanceType,
    RealmCreateRep,
    RealmCreateReq,
    RealmFinishReencryptionMaintenanceRep,
    RealmFinishReencryptionMaintenanceReq,
    RealmGetRoleCertificatesRep,
    RealmGetRoleCertificatesReq,
    RealmID,
    RealmRole,
    RealmStartReencryptionMaintenanceRep,
    RealmStartReencryptionMaintenanceReq,
    RealmStatsRep,
    RealmStatsReq,
    RealmStatusRep,
    RealmStatusReq,
    RealmUpdateRolesRep,
    RealmUpdateRolesReq,
)
from parsec.api.protocol.base import ApiCommandSerializer
from parsec.serde import fields

__all__ = (
    "RealmID",
    "RealmIDField",
    "RealmRole",
    "RealmRoleField",
    "MaintenanceType",
    "realm_create_serializer",
    "realm_status_serializer",
    "realm_stats_serializer",
    "realm_get_role_certificates_serializer",
    "realm_update_roles_serializer",
    "realm_start_reencryption_maintenance_serializer",
    "realm_finish_reencryption_maintenance_serializer",
)


RealmRoleField = fields.rust_enum_field_factory(RealmRole)
RealmIDField = fields.uuid_based_field_factory(RealmID)

realm_create_serializer = ApiCommandSerializer(RealmCreateReq, RealmCreateRep)
realm_status_serializer = ApiCommandSerializer(RealmStatusReq, RealmStatusRep)
realm_stats_serializer = ApiCommandSerializer(RealmStatsReq, RealmStatsRep)
realm_get_role_certificates_serializer = ApiCommandSerializer(
    RealmGetRoleCertificatesReq, RealmGetRoleCertificatesRep
)
realm_update_roles_serializer = ApiCommandSerializer(RealmUpdateRolesReq, RealmUpdateRolesRep)
realm_start_reencryption_maintenance_serializer = ApiCommandSerializer(
    RealmStartReencryptionMaintenanceReq, RealmStartReencryptionMaintenanceRep
)
realm_finish_reencryption_maintenance_serializer = ApiCommandSerializer(
    RealmFinishReencryptionMaintenanceReq, RealmFinishReencryptionMaintenanceRep
)
