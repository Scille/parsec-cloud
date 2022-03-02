# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS
# flake8: noqa

from pendulum import datetime
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
        "maintenance_started_on": datetime(2000, 1, 2, 1),
        "maintenance_started_by": DeviceID("alice@dev1"),
        "encryption_revision": 8,
    }
)
serializer.rep_loads(serialized)
display("realm_status_rep", serialized, [])

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

################### RealmGetRoleCertificates ##################

serializer = realm_get_role_certificates_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "realm_get_role_certificates",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "since": datetime(2000, 1, 2, 1),
    }
)
serializer.req_loads(serialized)
display("realm_get_role_certificates_req", serialized, [])

serialized = serializer.rep_dumps({"certificates": [b"foobar"]})
serializer.rep_loads(serialized)
display("realm_get_role_certificates_rep", serialized, [])

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

################### RealmStartReencryptionMaintenance ##################

serializer = realm_start_reencryption_maintenance_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "realm_start_reencryption_maintenance",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "encryption_revision": 8,
        "timestamp": datetime(2000, 1, 2, 1),
        "per_participant_message": {UserID("109b68ba5cdf428ea0017fc6bcc04d4a"): b"foobar"},
    }
)
serializer.req_loads(serialized)
display("realm_start_reencryption_maintenance_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep", serialized, [])

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
