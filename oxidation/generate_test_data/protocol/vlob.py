# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from parsec._parsec import (
    DateTime,
    VlobCreateRepOk,
    VlobCreateRepAlreadyExists,
    VlobCreateRepBadEncryptionRevision,
    VlobCreateRepBadTimestamp,
    VlobCreateRepInMaintenance,
    VlobCreateRepNotAllowed,
    VlobCreateRepNotASequesteredOrganization,
    VlobCreateRepRequireGreaterTimestamp,
    VlobCreateRepSequesterInconsistency,
    VlobReadRepOk,
    VlobReadRepBadEncryptionRevision,
    VlobReadRepBadVersion,
    VlobReadRepInMaintenance,
    VlobReadRepNotAllowed,
    VlobReadRepNotFound,
    VlobUpdateRepOk,
    VlobUpdateRepBadEncryptionRevision,
    VlobUpdateRepBadTimestamp,
    VlobUpdateRepBadVersion,
    VlobUpdateRepInMaintenance,
    VlobUpdateRepNotAllowed,
    VlobUpdateRepNotASequesteredOrganization,
    VlobUpdateRepNotFound,
    VlobUpdateRepRequireGreaterTimestamp,
    VlobUpdateRepSequesterInconsistency,
    VlobPollChangesRepOk,
    VlobPollChangesRepNotFound,
    VlobPollChangesRepNotAllowed,
    VlobPollChangesRepInMaintenance,
    VlobListVersionsRepOk,
    VlobListVersionsRepNotFound,
    VlobListVersionsRepInMaintenance,
    VlobListVersionsRepNotAllowed,
    VlobMaintenanceGetReencryptionBatchRepOk,
    VlobMaintenanceGetReencryptionBatchRepNotInMaintenance,
    VlobMaintenanceGetReencryptionBatchRepBadEncryptionRevision,
    VlobMaintenanceGetReencryptionBatchRepMaintenanceError,
    VlobMaintenanceGetReencryptionBatchRepNotAllowed,
    VlobMaintenanceGetReencryptionBatchRepNotFound,
    VlobMaintenanceSaveReencryptionBatchRepOk,
    VlobMaintenanceSaveReencryptionBatchRepBadEncryptionRevision,
    VlobMaintenanceSaveReencryptionBatchRepMaintenanceError,
    VlobMaintenanceSaveReencryptionBatchRepNotFound,
    VlobMaintenanceSaveReencryptionBatchRepNotAllowed,
    VlobMaintenanceSaveReencryptionBatchRepNotInMaintenance,
    ReencryptionBatchEntry,
)
from utils import *
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
        "sequester_blob": None,
    }
)
serializer.req_loads(serialized)
display("vlob_create_req_without", serialized, [])

serialized = serializer.req_dumps(
    {
        "cmd": "vlob_create",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "encryption_revision": 8,
        "vlob_id": VlobID.from_hex("2b5f314728134a12863da1ce49c112f6"),
        "timestamp": DateTime(2000, 1, 2, 1),
        "blob": b"foobar",
        "sequester_blob": {
            SequesterServiceID.from_hex("b5eb565343c442b3a26be44573813ff0"): b"foobar"
        },
    }
)
serializer.req_loads(serialized)
display("vlob_create_req_full", serialized, [])

serialized = serializer.rep_dumps(VlobCreateRepOk())
serializer.rep_loads(serialized)
display("vlob_create_rep", serialized, [])

serialized = serializer.rep_dumps(VlobCreateRepAlreadyExists(reason="foobar"))
serializer.rep_loads(serialized)
display("vlob_create_rep_already_exists", serialized, [])

serialized = serializer.rep_dumps(VlobCreateRepNotAllowed())
serializer.rep_loads(serialized)
display("vlob_create_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(VlobCreateRepBadEncryptionRevision())
serializer.rep_loads(serialized)
display("vlob_create_rep_bad_encryption_revision", serialized, [])

serialized = serializer.rep_dumps(VlobCreateRepInMaintenance())
serializer.rep_loads(serialized)
display("vlob_create_rep_in_maintenance", serialized, [])

serialized = serializer.rep_dumps(
    VlobCreateRepRequireGreaterTimestamp(strictly_greater_than=DateTime(2000, 1, 2, 1))
)
serializer.rep_loads(serialized)
display("vlob_create_rep_require_greater_timestamp", serialized, [])

serialized = serializer.rep_dumps(
    VlobCreateRepBadTimestamp(
        reason=None,
        ballpark_client_early_offset=300.0,
        ballpark_client_late_offset=320.0,
        backend_timestamp=DateTime(2000, 1, 2, 1),
        client_timestamp=DateTime(2000, 1, 2, 1),
    )
)
serializer.rep_loads(serialized)
display("vlob_create_rep_bad_timestamp", serialized, [])

serialized = serializer.rep_dumps(VlobCreateRepNotASequesteredOrganization())
serializer.rep_loads(serialized)
display("vlob_create_rep_not_a_sequestered_organization", serialized, [])

serialized = serializer.rep_dumps(
    VlobCreateRepSequesterInconsistency(
        sequester_authority_certificate=b"foobar",
        sequester_services_certificates=[b"foo", b"bar"],
    )
)
serializer.rep_loads(serialized)
display("vlob_create_rep_sequester_inconsistency", serialized, [])

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
    VlobReadRepOk(
        version=8,
        blob=b"foobar",
        author=DeviceID("alice@dev1"),
        timestamp=DateTime(2000, 1, 2, 1),
        author_last_role_granted_on=DateTime(2000, 1, 2, 1),
    )
)
serializer.rep_loads(serialized)
display("vlob_read_rep", serialized, [])

serialized = serializer.rep_dumps(VlobReadRepNotFound(reason="foobar"))
serializer.rep_loads(serialized)
display("vlob_read_rep_not_found", serialized, [])

serialized = serializer.rep_dumps(VlobReadRepNotAllowed())
serializer.rep_loads(serialized)
display("vlob_read_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(VlobReadRepBadVersion())
serializer.rep_loads(serialized)
display("vlob_read_rep_bad_version", serialized, [])

serialized = serializer.rep_dumps(VlobReadRepBadEncryptionRevision())
serializer.rep_loads(serialized)
display("vlob_read_rep_bad_encryption_revision", serialized, [])

serialized = serializer.rep_dumps(VlobReadRepInMaintenance())
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
        "sequester_blob": None,
    }
)
serializer.req_loads(serialized)
display("vlob_update_req_without", serialized, [])

serialized = serializer.req_dumps(
    {
        "cmd": "vlob_update",
        "encryption_revision": 8,
        "vlob_id": VlobID.from_hex("2b5f314728134a12863da1ce49c112f6"),
        "timestamp": DateTime(2000, 1, 2, 1),
        "version": 8,
        "blob": b"foobar",
        "sequester_blob": {
            SequesterServiceID.from_hex("b5eb565343c442b3a26be44573813ff0"): b"foobar"
        },
    }
)
serializer.req_loads(serialized)
display("vlob_update_req_full", serialized, [])

serialized = serializer.rep_dumps(VlobUpdateRepOk())
serializer.rep_loads(serialized)
display("vlob_update_rep", serialized, [])

serialized = serializer.rep_dumps(VlobUpdateRepNotFound(reason="foobar"))
serializer.rep_loads(serialized)
display("vlob_update_rep_not_found", serialized, [])

serialized = serializer.rep_dumps(VlobUpdateRepNotAllowed())
serializer.rep_loads(serialized)
display("vlob_update_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(VlobUpdateRepBadVersion())
serializer.rep_loads(serialized)
display("vlob_update_rep_bad_version", serialized, [])

serialized = serializer.rep_dumps(VlobUpdateRepBadEncryptionRevision())
serializer.rep_loads(serialized)
display("vlob_update_rep_bad_encryption_revision", serialized, [])

serialized = serializer.rep_dumps(VlobUpdateRepInMaintenance())
serializer.rep_loads(serialized)
display("vlob_update_rep_in_maintenance", serialized, [])

serialized = serializer.rep_dumps(
    VlobUpdateRepRequireGreaterTimestamp(strictly_greater_than=DateTime(2000, 1, 2, 1))
)
serializer.rep_loads(serialized)
display("vlob_update_rep_require_greater_timestamp", serialized, [])

serialized = serializer.rep_dumps(
    VlobUpdateRepBadTimestamp(
        reason=None,
        ballpark_client_early_offset=300.0,
        ballpark_client_late_offset=320.0,
        backend_timestamp=DateTime(2000, 1, 2, 1),
        client_timestamp=DateTime(2000, 1, 2, 1),
    )
)
serializer.rep_loads(serialized)
display("vlob_update_rep_bad_timestamp", serialized, [])

serialized = serializer.rep_dumps(VlobUpdateRepNotASequesteredOrganization())
serializer.rep_loads(serialized)
display("vlob_update_rep_not_a_sequestered_organization", serialized, [])

serialized = serializer.rep_dumps(
    VlobUpdateRepSequesterInconsistency(
        sequester_authority_certificate=b"foobar",
        sequester_services_certificates=[b"foo", b"bar"],
    )
)
serializer.rep_loads(serialized)
display("vlob_update_rep_sequester_inconsistency", serialized, [])

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
    VlobPollChangesRepOk(
        changes={VlobID.from_hex("2b5f314728134a12863da1ce49c112f6"): 8},
        current_checkpoint=8,
    )
)
serializer.rep_loads(serialized)
display("vlob_poll_changes_rep", serialized, [])

serialized = serializer.rep_dumps(VlobPollChangesRepNotAllowed())
serializer.rep_loads(serialized)
display("vlob_poll_changes_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(VlobPollChangesRepNotFound(reason="foobar"))
serializer.rep_loads(serialized)
display("vlob_poll_changes_rep_not_found", serialized, [])

serialized = serializer.rep_dumps(VlobPollChangesRepInMaintenance())
serializer.rep_loads(serialized)
display("vlob_poll_changes_rep_in_maintenance", serialized, [])

################### VlobListVersions ##################

serializer = vlob_list_versions_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "vlob_list_versions",
        "vlob_id": VlobID.from_hex("2b5f314728134a12863da1ce49c112f6"),
    }
)
serializer.req_loads(serialized)
display("vlob_list_versions_req", serialized, [])

serialized = serializer.rep_dumps(
    VlobListVersionsRepOk(versions={8: (DateTime(2000, 1, 2, 1), DeviceID("alice@dev1"))})
)
serializer.rep_loads(serialized)
display("vlob_list_versions_rep", serialized, [])

serialized = serializer.rep_dumps(VlobListVersionsRepNotAllowed())
serializer.rep_loads(serialized)
display("vlob_list_versions_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(VlobListVersionsRepNotFound(reason="foobar"))
serializer.rep_loads(serialized)
display("vlob_list_versions_rep_not_found", serialized, [])

serialized = serializer.rep_dumps(VlobListVersionsRepInMaintenance())
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
    VlobMaintenanceGetReencryptionBatchRepOk(
        batch=[
            ReencryptionBatchEntry(
                vlob_id=VlobID.from_hex("2b5f314728134a12863da1ce49c112f6"),
                version=8,
                blob=b"foobar",
            )
        ]
    )
)
serializer.rep_loads(serialized)
display("vlob_maintenance_get_reencryption_batch_rep", serialized, [])

serialized = serializer.rep_dumps(VlobMaintenanceGetReencryptionBatchRepNotAllowed())
serializer.rep_loads(serialized)
display("vlob_maintenance_get_reencryption_batch_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(VlobMaintenanceGetReencryptionBatchRepNotFound(reason="foobar"))
serializer.rep_loads(serialized)
display("vlob_maintenance_get_reencryption_batch_rep_not_found", serialized, [])

serialized = serializer.rep_dumps(
    VlobMaintenanceGetReencryptionBatchRepNotInMaintenance(reason="foobar")
)
serializer.rep_loads(serialized)
display("vlob_maintenance_get_reencryption_batch_rep_not_in_maintenance", serialized, [])

serialized = serializer.rep_dumps(VlobMaintenanceGetReencryptionBatchRepBadEncryptionRevision())
serializer.rep_loads(serialized)
display(
    "vlob_maintenance_get_reencryption_batch_rep_bad_encryption_revision",
    serialized,
    [],
)

serialized = serializer.rep_dumps(
    VlobMaintenanceGetReencryptionBatchRepMaintenanceError(reason="foobar")
)
serializer.rep_loads(serialized)
display("vlob_maintenance_get_reencryption_batch_rep_maintenance_error", serialized, [])

################### VlobMaintenanceSaveReencryptionBatch ##################

serializer = vlob_maintenance_save_reencryption_batch_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "vlob_maintenance_save_reencryption_batch",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "encryption_revision": 8,
        "batch": [
            ReencryptionBatchEntry(
                vlob_id=VlobID.from_hex("2b5f314728134a12863da1ce49c112f6"),
                version=8,
                blob=b"foobar",
            )
        ],
    }
)
serializer.req_loads(serialized)
display("vlob_maintenance_save_reencryption_batch_req", serialized, [])

serialized = serializer.rep_dumps(VlobMaintenanceSaveReencryptionBatchRepOk(total=8, done=8))
serializer.rep_loads(serialized)
display("vlob_maintenance_save_reencryption_batch_rep", serialized, [])

serialized = serializer.rep_dumps(VlobMaintenanceSaveReencryptionBatchRepNotAllowed())
serializer.rep_loads(serialized)
display("vlob_maintenance_save_reencryption_batch_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(VlobMaintenanceSaveReencryptionBatchRepNotFound(reason="foobar"))
serializer.rep_loads(serialized)
display("vlob_maintenance_save_reencryption_batch_rep_not_found", serialized, [])

serialized = serializer.rep_dumps(
    VlobMaintenanceSaveReencryptionBatchRepNotInMaintenance(reason="foobar")
)
serializer.rep_loads(serialized)
display("vlob_maintenance_save_reencryption_batch_rep_not_in_maintenance", serialized, [])

serialized = serializer.rep_dumps(VlobMaintenanceSaveReencryptionBatchRepBadEncryptionRevision())
serializer.rep_loads(serialized)
display(
    "vlob_maintenance_save_reencryption_batch_rep_bad_encryption_revision",
    serialized,
    [],
)

serialized = serializer.rep_dumps(
    VlobMaintenanceSaveReencryptionBatchRepMaintenanceError(reason="foobar")
)
serializer.rep_loads(serialized)
display("vlob_maintenance_save_reencryption_batch_rep_maintenance_error", serialized, [])
