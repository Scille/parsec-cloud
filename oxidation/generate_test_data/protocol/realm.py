# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from parsec._parsec import DateTime
from utils import *
from parsec.crypto import *
from parsec.api.protocol import *
from parsec.api.data import *

################### RealmCreate ##################

serializer = realm_create_serializer

serialized = serializer.req_dumps({"cmd": "realm_create", "role_certificate": b"foobar"})
serializer.req_loads(serialized)
display("realm_create_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("realm_create_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_certification", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_create_rep_invalid_certification", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_data", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_create_rep_invalid_data", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_create_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "already_exists"})
serializer.rep_loads(serialized)
display("realm_create_rep_already_exists", serialized, [])

################### RealmStatus ##################

serializer = realm_status_serializer

serialized = serializer.req_dumps(
    {"cmd": "realm_status", "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5")}
)
serializer.req_loads(serialized)
display("realm_status_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "in_maintenance": True,
        "maintenance_type": MaintenanceType.GARBAGE_COLLECTION,
        "maintenance_started_on": DateTime(2000, 1, 2, 1),
        "maintenance_started_by": DeviceID("alice@dev1"),
        "encryption_revision": 8,
    }
)
serializer.rep_loads(serialized)
display("realm_status_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("realm_status_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_status_rep_not_found", serialized, [])

################### RealmStats ##################

serializer = realm_stats_serializer

serialized = serializer.req_dumps(
    {"cmd": "realm_stats", "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5")}
)
serializer.req_loads(serialized)
display("realm_stats_req", serialized, [])

serialized = serializer.rep_dumps({"blocks_size": 8, "vlobs_size": 8})
serializer.rep_loads(serialized)
display("realm_stats_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("realm_stats_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found", "reason": "foobar"})
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

serialized = serializer.rep_dumps({"certificates": [b"foobar"]})
serializer.rep_loads(serialized)
display("realm_get_role_certificates_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("realm_get_role_certificates_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_get_role_certificates_rep_not_found", serialized, [])

################### RealmUpdateRoles ##################

serializer = realm_update_roles_serializer

serialized = serializer.req_dumps(
    {"cmd": "realm_update_roles", "role_certificate": b"foobar", "recipient_message": b"foobar"}
)
serializer.req_loads(serialized)
display("realm_update_roles_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("realm_update_roles_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_update_roles_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_certification", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_update_roles_rep_invalid_certification", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_data", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_update_roles_rep_invalid_data", serialized, [])

serialized = serializer.rep_dumps({"status": "already_granted"})
serializer.rep_loads(serialized)
display("realm_update_roles_rep_already_granted", serialized, [])

serialized = serializer.rep_dumps({"status": "incompatible_profile", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_update_roles_rep_incompatible_profile", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_update_roles_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "in_maintenance"})
serializer.rep_loads(serialized)
display("realm_update_roles_rep_in_maintenance", serialized, [])

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

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "bad_encryption_revision"})
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep_bad_encryption_revision", serialized, [])

serialized = serializer.rep_dumps({"status": "participant_mismatch", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep_participant_mismatch", serialized, [])

serialized = serializer.rep_dumps({"status": "maintenance_error", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep_maintenance_error", serialized, [])

serialized = serializer.rep_dumps({"status": "in_maintenance"})
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep_in_maintenance", serialized, [])

################### RealmFinishReecryptionMaintenance ##################

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

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("realm_finish_reencryption_maintenance_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("realm_finish_reencryption_maintenance_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_finish_reencryption_maintenance_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "bad_encryption_revision"})
serializer.rep_loads(serialized)
display("realm_finish_reencryption_maintenance_rep_bad_encryption_revision", serialized, [])

serialized = serializer.rep_dumps({"status": "not_in_maintenance", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_finish_reencryption_maintenance_rep_not_in_maintenance", serialized, [])

serialized = serializer.rep_dumps({"status": "maintenance_error", "reason": "foobar"})
serializer.rep_loads(serialized)
display("realm_finish_reencryption_maintenance_rep_maintenance_error", serialized, [])
