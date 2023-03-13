# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from parsec._parsec import DateTime
from utils import *
from parsec.api.protocol import *
from parsec.api.data import *
from parsec._parsec import (
    RealmCreateRepOk,
    RealmCreateRepAlreadyExists,
    RealmCreateRepBadTimestamp,
    RealmCreateRepInvalidCertification,
    RealmCreateRepInvalidData,
    RealmCreateRepNotFound,
    RealmStatusRepOk,
    RealmStatusRepNotAllowed,
    RealmStatusRepNotFound,
    RealmStatsRepOk,
    RealmStatsRepNotAllowed,
    RealmStatsRepNotFound,
    RealmGetRoleCertificatesRepOk,
    RealmGetRoleCertificatesRepNotFound,
    RealmGetRoleCertificatesRepNotAllowed,
    RealmUpdateRolesRepOk,
    RealmUpdateRolesRepUserRevoked,
    RealmUpdateRolesRepAlreadyGranted,
    RealmUpdateRolesRepBadTimestamp,
    RealmUpdateRolesRepIncompatibleProfile,
    RealmUpdateRolesRepInMaintenance,
    RealmUpdateRolesRepInvalidCertification,
    RealmUpdateRolesRepInvalidData,
    RealmUpdateRolesRepNotAllowed,
    RealmUpdateRolesRepNotFound,
    RealmUpdateRolesRepRequireGreaterTimestamp,
    RealmStartReencryptionMaintenanceRepOk,
    RealmStartReencryptionMaintenanceRepNotFound,
    RealmStartReencryptionMaintenanceRepBadEncryptionRevision,
    RealmStartReencryptionMaintenanceRepBadTimestamp,
    RealmStartReencryptionMaintenanceRepInMaintenance,
    RealmStartReencryptionMaintenanceRepMaintenanceError,
    RealmStartReencryptionMaintenanceRepNotAllowed,
    RealmStartReencryptionMaintenanceRepParticipantMismatch,
    RealmFinishReencryptionMaintenanceRepOk,
    RealmFinishReencryptionMaintenanceRepNotInMaintenance,
    RealmFinishReencryptionMaintenanceRepBadEncryptionRevision,
    RealmFinishReencryptionMaintenanceRepMaintenanceError,
    RealmFinishReencryptionMaintenanceRepNotAllowed,
    RealmFinishReencryptionMaintenanceRepNotFound,
)

################### RealmCreate ##################

serializer = realm_create_serializer

serialized = serializer.req_dumps({"cmd": "realm_create", "role_certificate": b"foobar"})
serializer.req_loads(serialized)
display("realm_create_req", serialized, [])

serialized = serializer.rep_dumps(RealmCreateRepOk())
serializer.rep_loads(serialized)
display("realm_create_rep", serialized, [])

serialized = serializer.rep_dumps(RealmCreateRepInvalidCertification(reason="foobar"))
serializer.rep_loads(serialized)
display("realm_create_rep_invalid_certification", serialized, [])

serialized = serializer.rep_dumps(RealmCreateRepInvalidData(reason="foobar"))
serializer.rep_loads(serialized)
display("realm_create_rep_invalid_data", serialized, [])

serialized = serializer.rep_dumps(RealmCreateRepNotFound(reason="foobar"))
serializer.rep_loads(serialized)
display("realm_create_rep_not_found", serialized, [])

serialized = serializer.rep_dumps(RealmCreateRepAlreadyExists())
serializer.rep_loads(serialized)
display("realm_create_rep_already_exists", serialized, [])

serialized = serializer.rep_dumps(
    RealmCreateRepBadTimestamp(
        reason=None,
        ballpark_client_early_offset=300.0,
        ballpark_client_late_offset=320.0,
        backend_timestamp=DateTime(2000, 1, 2, 1),
        client_timestamp=DateTime(2000, 1, 2, 1),
    )
)
serializer.rep_loads(serialized)
display("realm_create_rep_bad_timestamp", serialized, [])

################### RealmStatus ##################

serializer = realm_status_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "realm_status",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
    }
)
serializer.req_loads(serialized)
display("realm_status_req", serialized, [])

serialized = serializer.rep_dumps(
    RealmStatusRepOk(
        in_maintenance=True,
        maintenance_type=MaintenanceType.REENCRYPTION(),
        maintenance_started_on=None,
        maintenance_started_by=None,
        encryption_revision=8,
    )
)
serializer.rep_loads(serialized)
display("realm_status_rep_without", serialized, [])

serialized = serializer.rep_dumps(
    RealmStatusRepOk(
        in_maintenance=True,
        maintenance_type=MaintenanceType.GARBAGE_COLLECTION(),
        maintenance_started_on=DateTime(2000, 1, 2, 1),
        maintenance_started_by=DeviceID("alice@dev1"),
        encryption_revision=8,
    )
)
serializer.rep_loads(serialized)
display("realm_status_rep_full", serialized, [])

serialized = serializer.rep_dumps(RealmStatusRepNotAllowed())
serializer.rep_loads(serialized)
display("realm_status_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(RealmStatusRepNotFound(reason="foobar"))
serializer.rep_loads(serialized)
display("realm_status_rep_not_found", serialized, [])

################### RealmStats ##################

serializer = realm_stats_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "realm_stats",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
    }
)
serializer.req_loads(serialized)
display("realm_stats_req", serialized, [])

serialized = serializer.rep_dumps(RealmStatsRepOk(blocks_size=8, vlobs_size=8))
serializer.rep_loads(serialized)
display("realm_stats_rep", serialized, [])

serialized = serializer.rep_dumps(RealmStatsRepNotAllowed())
serializer.rep_loads(serialized)
display("realm_stats_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(RealmStatsRepNotFound(reason="foobar"))
serializer.rep_loads(serialized)
display("realm_stats_rep_not_found", serialized, [])

################### RealmGetRoleCertificates ##################

serializer = realm_get_role_certificates_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "realm_get_role_certificates",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
    }
)
serializer.req_loads(serialized)
display("realm_get_role_certificates_req", serialized, [])

serialized = serializer.rep_dumps(RealmGetRoleCertificatesRepOk(certificates=[b"foobar"]))
serializer.rep_loads(serialized)
display("realm_get_role_certificates_rep", serialized, [])

serialized = serializer.rep_dumps(RealmGetRoleCertificatesRepNotAllowed())
serializer.rep_loads(serialized)
display("realm_get_role_certificates_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(RealmGetRoleCertificatesRepNotFound(reason="foobar"))
serializer.rep_loads(serialized)
display("realm_get_role_certificates_rep_not_found", serialized, [])

################### RealmUpdateRoles ##################

serializer = realm_update_roles_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "realm_update_roles",
        "role_certificate": b"foobar",
        "recipient_message": None,
    }
)
serializer.req_loads(serialized)
display("realm_update_roles_req_without", serialized, [])

serialized = serializer.req_dumps(
    {
        "cmd": "realm_update_roles",
        "role_certificate": b"foobar",
        "recipient_message": b"foobar",
    }
)
serializer.req_loads(serialized)
display("realm_update_roles_req_full", serialized, [])

serialized = serializer.rep_dumps(RealmUpdateRolesRepOk())
serializer.rep_loads(serialized)
display("realm_update_roles_rep", serialized, [])

serialized = serializer.rep_dumps(RealmUpdateRolesRepNotAllowed(reason="foobar"))
serializer.rep_loads(serialized)
display("realm_update_roles_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(RealmUpdateRolesRepInvalidCertification(reason="foobar"))
serializer.rep_loads(serialized)
display("realm_update_roles_rep_invalid_certification", serialized, [])

serialized = serializer.rep_dumps(RealmUpdateRolesRepInvalidData(reason="foobar"))
serializer.rep_loads(serialized)
display("realm_update_roles_rep_invalid_data", serialized, [])

serialized = serializer.rep_dumps(RealmUpdateRolesRepAlreadyGranted())
serializer.rep_loads(serialized)
display("realm_update_roles_rep_already_granted", serialized, [])

serialized = serializer.rep_dumps(RealmUpdateRolesRepIncompatibleProfile(reason="foobar"))
serializer.rep_loads(serialized)
display("realm_update_roles_rep_incompatible_profile", serialized, [])

serialized = serializer.rep_dumps(RealmUpdateRolesRepNotFound(reason="foobar"))
serializer.rep_loads(serialized)
display("realm_update_roles_rep_not_found", serialized, [])

serialized = serializer.rep_dumps(RealmUpdateRolesRepInMaintenance())
serializer.rep_loads(serialized)
display("realm_update_roles_rep_in_maintenance", serialized, [])

serialized = serializer.rep_dumps(RealmUpdateRolesRepUserRevoked())
serializer.rep_loads(serialized)
display("realm_update_roles_rep_user_revoked", serialized, [])

serialized = serializer.rep_dumps(
    RealmUpdateRolesRepRequireGreaterTimestamp(strictly_greater_than=DateTime(2000, 1, 2, 1))
)
serializer.rep_loads(serialized)
display("realm_update_roles_rep_require_greater_timestamp", serialized, [])

serialized = serializer.rep_dumps(
    RealmUpdateRolesRepBadTimestamp(
        reason=None,
        ballpark_client_early_offset=300.0,
        ballpark_client_late_offset=320.0,
        backend_timestamp=DateTime(2000, 1, 2, 1),
        client_timestamp=DateTime(2000, 1, 2, 1),
    )
)
serializer.rep_loads(serialized)
display("realm_update_roles_rep_bad_timestamp", serialized, [])

################### RealmStartReencryptionMaintenance ##################

serializer = realm_start_reencryption_maintenance_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "realm_start_reencryption_maintenance",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "encryption_revision": 8,
        "timestamp": DateTime(2000, 1, 2, 1),
        "per_participant_message": {UserID("109b68ba5cdf428ea0017fc6bcc04d4a"): b"foobar"},
    }
)
serializer.req_loads(serialized)
display("realm_start_reencryption_maintenance_req", serialized, [])

serialized = serializer.rep_dumps(RealmStartReencryptionMaintenanceRepOk())
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep", serialized, [])

serialized = serializer.rep_dumps(RealmStartReencryptionMaintenanceRepNotAllowed())
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(RealmStartReencryptionMaintenanceRepNotFound(reason="foobar"))
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep_not_found", serialized, [])

serialized = serializer.rep_dumps(RealmStartReencryptionMaintenanceRepBadEncryptionRevision())
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep_bad_encryption_revision", serialized, [])

serialized = serializer.rep_dumps(
    RealmStartReencryptionMaintenanceRepParticipantMismatch(reason="foobar")
)
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep_participant_mismatch", serialized, [])

serialized = serializer.rep_dumps(
    RealmStartReencryptionMaintenanceRepMaintenanceError(reason="foobar")
)
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep_maintenance_error", serialized, [])

serialized = serializer.rep_dumps(RealmStartReencryptionMaintenanceRepInMaintenance())
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep_in_maintenance", serialized, [])

serialized = serializer.rep_dumps(
    RealmStartReencryptionMaintenanceRepBadTimestamp(
        reason=None,
        ballpark_client_early_offset=300.0,
        ballpark_client_late_offset=320.0,
        backend_timestamp=DateTime(2000, 1, 2, 1),
        client_timestamp=DateTime(2000, 1, 2, 1),
    )
)
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep_bad_timestamp", serialized, [])

################### RealmFinishReencryptionMaintenance ##################

serializer = realm_finish_reencryption_maintenance_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "realm_finish_reencryption_maintenance",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "encryption_revision": 8,
    }
)
serializer.req_loads(serialized)
display("realm_finish_reencryption_maintenance_req", serialized, [])

serialized = serializer.rep_dumps(RealmFinishReencryptionMaintenanceRepOk())
serializer.rep_loads(serialized)
display("realm_finish_reencryption_maintenance_rep", serialized, [])

serialized = serializer.rep_dumps(RealmFinishReencryptionMaintenanceRepNotAllowed())
serializer.rep_loads(serialized)
display("realm_finish_reencryption_maintenance_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(RealmFinishReencryptionMaintenanceRepNotFound(reason="foobar"))
serializer.rep_loads(serialized)
display("realm_finish_reencryption_maintenance_rep_not_found", serialized, [])

serialized = serializer.rep_dumps(RealmFinishReencryptionMaintenanceRepBadEncryptionRevision())
serializer.rep_loads(serialized)
display("realm_finish_reencryption_maintenance_rep_bad_encryption_revision", serialized, [])

serialized = serializer.rep_dumps(
    RealmFinishReencryptionMaintenanceRepNotInMaintenance(reason="foobar")
)
serializer.rep_loads(serialized)
display("realm_finish_reencryption_maintenance_rep_not_in_maintenance", serialized, [])

serialized = serializer.rep_dumps(
    RealmFinishReencryptionMaintenanceRepMaintenanceError(reason="foobar")
)
serializer.rep_loads(serialized)
display("realm_finish_reencryption_maintenance_rep_maintenance_error", serialized, [])
