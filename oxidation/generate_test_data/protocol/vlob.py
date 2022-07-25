# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from libparsec.types import DateTime
from utils import *
from parsec.crypto import *
from parsec.api.protocol import *
from parsec.api.data import *

################### VlobCreate ##################

serializer = vlob_create_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "vlob_create",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "encryption_revision": 8,
        "vlob_id": VlobID.from_hex("2b5f314728134a12863da1ce49c112f6"),
        "timestamp": DateTime(2000, 1, 2, 1),
        "blob": b"foobar",
    }
)
serializer.req_loads(serialized)
display("vlob_create_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("vlob_create_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "already_exists", "reason": "foobar"})
serializer.rep_loads(serialized)
display("vlob_create_rep_already_exists", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("vlob_create_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "bad_encryption_revision"})
serializer.rep_loads(serialized)
display("vlob_create_rep_bad_encryption_revision", serialized, [])

serialized = serializer.rep_dumps({"status": "in_maintenance"})
serializer.rep_loads(serialized)
display("vlob_create_rep_in_maintenance", serialized, [])

################### VlobRead ##################

serializer = vlob_read_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "vlob_read",
        "encryption_revision": 8,
        "vlob_id": VlobID.from_hex("2b5f314728134a12863da1ce49c112f6"),
        "version": 8,
        "timestamp": DateTime(2000, 1, 2, 1),
    }
)
serializer.req_loads(serialized)
display("vlob_read_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "version": 8,
        "blob": b"foobar",
        "author": DeviceID("alice@dev1"),
        "timestamp": DateTime(2000, 1, 2, 1),
        "author_last_role_granted_on": DateTime(2000, 1, 2, 1),
    }
)
serializer.rep_loads(serialized)
display("vlob_read_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found", "reason": "foobar"})
serializer.rep_loads(serialized)
display("vlob_read_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("vlob_read_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "bad_version"})
serializer.rep_loads(serialized)
display("vlob_read_rep_bad_version", serialized, [])

serialized = serializer.rep_dumps({"status": "bad_encryption_revision"})
serializer.rep_loads(serialized)
display("vlob_read_rep_bad_encryption_revision", serialized, [])

serialized = serializer.rep_dumps({"status": "in_maintenance"})
serializer.rep_loads(serialized)
display("vlob_read_rep_in_maintenance", serialized, [])

################### VlobUpdate ##################

serializer = vlob_update_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "vlob_update",
        "encryption_revision": 8,
        "vlob_id": VlobID.from_hex("2b5f314728134a12863da1ce49c112f6"),
        "timestamp": DateTime(2000, 1, 2, 1),
        "version": 8,
        "blob": b"foobar",
    }
)
serializer.req_loads(serialized)
display("vlob_update_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("vlob_update_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found", "reason": "foobar"})
serializer.rep_loads(serialized)
display("vlob_update_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("vlob_update_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "bad_version"})
serializer.rep_loads(serialized)
display("vlob_update_rep_bad_version", serialized, [])

serialized = serializer.rep_dumps({"status": "bad_encryption_revision"})
serializer.rep_loads(serialized)
display("vlob_update_rep_bad_encryption_revision", serialized, [])

serialized = serializer.rep_dumps({"status": "in_maintenance"})
serializer.rep_loads(serialized)
display("vlob_update_rep_in_maintenance", serialized, [])

################### VlobPollChanges ##################

serializer = vlob_poll_changes_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "vlob_poll_changes",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "last_checkpoint": 8,
    }
)
serializer.req_loads(serialized)
display("vlob_poll_changes_req", serialized, [])

serialized = serializer.rep_dumps(
    {"changes": {VlobID.from_hex("2b5f314728134a12863da1ce49c112f6"): 8}, "current_checkpoint": 8}
)
serializer.rep_loads(serialized)
display("vlob_poll_changes_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("vlob_poll_changes_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found", "reason": "foobar"})
serializer.rep_loads(serialized)
display("vlob_poll_changes_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "in_maintenance"})
serializer.rep_loads(serialized)
display("vlob_poll_changes_rep_in_maintenance", serialized, [])

################### VlobListVersions ##################

serializer = vlob_list_versions_serializer

serialized = serializer.req_dumps(
    {"cmd": "vlob_list_versions", "vlob_id": VlobID.from_hex("2b5f314728134a12863da1ce49c112f6")}
)
serializer.req_loads(serialized)
display("vlob_list_versions_req", serialized, [])

serialized = serializer.rep_dumps(
    {"versions": {8: (DateTime(2000, 1, 2, 1), DeviceID("alice@dev1"))}}
)
serializer.rep_loads(serialized)
display("vlob_list_versions_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("vlob_list_versions_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found", "reason": "foobar"})
serializer.rep_loads(serialized)
display("vlob_list_versions_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "in_maintenance"})
serializer.rep_loads(serialized)
display("vlob_list_versions_rep_in_maintenance", serialized, [])

################### VlobMaintenanceGetReencryptionBatch ##################

serializer = vlob_maintenance_get_reencryption_batch_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "vlob_maintenance_get_reencryption_batch",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "encryption_revision": 8,
        "size": 8,
    }
)
serializer.req_loads(serialized)
display("vlob_maintenance_get_reencryption_batch_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "batch": {
            "vlob_id": VlobID.from_hex("2b5f314728134a12863da1ce49c112f6"),
            "version": 8,
            "blob": b"foobar",
        }
    }
)
serializer.rep_loads(serialized)
display("vlob_maintenance_get_reencryption_batch_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("vlob_maintenance_get_reencryption_batch_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found", "reason": "foobar"})
serializer.rep_loads(serialized)
display("vlob_maintenance_get_reencryption_batch_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "not_in_maintenance", "reason": "foobar"})
serializer.rep_loads(serialized)
display("vlob_maintenance_get_reencryption_batch_rep_not_in_maintenance", serialized, [])

serialized = serializer.rep_dumps({"status": "bad_encryption_revision"})
serializer.rep_loads(serialized)
display("vlob_maintenance_get_reencryption_batch_rep_bad_encryption_revision", serialized, [])

serialized = serializer.rep_dumps({"status": "maintenance_error", "reason": "foobar"})
serializer.rep_loads(serialized)
display("vlob_maintenance_get_reencryption_batch_rep_maintenance_error", serialized, [])

################### VlobMaintenanceSaveReencryptionBatch ##################

serializer = vlob_maintenance_save_reencryption_batch_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "vlob_maintenance_save_reencryption_batch",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "encryption_revision": 8,
        "batch": {
            "vlob_id": VlobID.from_hex("2b5f314728134a12863da1ce49c112f6"),
            "version": 8,
            "blob": b"foobar",
        },
    }
)
serializer.req_loads(serialized)
display("vlob_maintenance_save_reencryption_batch_req", serialized, [])

serialized = serializer.rep_dumps({"total": 8, "done": 8})
serializer.rep_loads(serialized)
display("vlob_maintenance_save_reencryption_batch_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("vlob_maintenance_save_reencryption_batch_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found", "reason": "foobar"})
serializer.rep_loads(serialized)
display("vlob_maintenance_save_reencryption_batch_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "not_in_maintenance", "reason": "foobar"})
serializer.rep_loads(serialized)
display("vlob_maintenance_save_reencryption_batch_rep_not_in_maintenance", serialized, [])

serialized = serializer.rep_dumps({"status": "bad_encryption_revision"})
serializer.rep_loads(serialized)
display("vlob_maintenance_save_reencryption_batch_rep_bad_encryption_revision", serialized, [])

serialized = serializer.rep_dumps({"status": "maintenance_error", "reason": "foobar"})
serializer.rep_loads(serialized)
display("vlob_maintenance_save_reencryption_batch_rep_maintenance_error", serialized, [])
