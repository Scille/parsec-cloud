# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from utils import *
from parsec.crypto import *
from parsec.api.protocol import *
from parsec.api.data import *

################### UserGet ##################

serializer = user_get_serializer

serialized = serializer.req_dumps(
    {"cmd": "user_get", "user_id": UserID("109b68ba5cdf428ea0017fc6bcc04d4a")}
)
serializer.req_loads(serialized)
display("user_get_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "user_certificate": b"foobar",
        "revoked_user_certificate": b"foobar",
        "device_certificates": [b"foobar"],
        "trustchain": {
            "devices": [b"foobar"],
            "users": [b"foobar"],
            "revoked_users": [b"foobar"],
        },
    }
)
serializer.rep_loads(serialized)
display("user_get_rep", serialized, [])

serialized = serializer.rep_dumps(
    {
        "user_certificate": b"foobar",
        "revoked_user_certificate": None,
        "device_certificates": [b"foobar"],
        "trustchain": {
            "devices": [b"foobar"],
            "users": [b"foobar"],
            "revoked_users": [b"foobar"],
        },
    }
)
serializer.rep_loads(serialized)
display("user_get_rep_null_revoked_user_cert", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("user_get_rep", serialized, [])

################### UserCreate ##################

serializer = user_create_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "user_create",
        "user_certificate": b"foobar",
        "device_certificate": b"foobar",
        "redacted_user_certificate": b"foobar",
        "redacted_device_certificate": b"foobar",
    }
)
serializer.req_loads(serialized)
display("user_create_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("user_create_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed", "reason": "foobar"})
serializer.rep_loads(serialized)
display("user_create_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_certification", "reason": "foobar"})
serializer.rep_loads(serialized)
display("user_create_rep_invalid_certification", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_data", "reason": "foobar"})
serializer.rep_loads(serialized)
display("user_create_rep_invalid_data", serialized, [])

serialized = serializer.rep_dumps({"status": "already_exists", "reason": "foobar"})
serializer.rep_loads(serialized)
display("user_create_rep_already_exists", serialized, [])

serialized = serializer.rep_dumps({"status": "active_users_limit_reached", "reason": "foobar"})
serializer.rep_loads(serialized)
display("user_create_rep_active_users_limit_reached", serialized, [])

################### UserRevoke ##################

serializer = user_revoke_serializer

serialized = serializer.req_dumps({"cmd": "user_revoke", "revoked_user_certificate": b"foobar"})
serializer.req_loads(serialized)
display("user_revoke_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("user_revoke_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed", "reason": "foobar"})
serializer.rep_loads(serialized)
display("user_revoke_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_certification", "reason": "foobar"})
serializer.rep_loads(serialized)
display("user_revoke_rep_invalid_certification", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("user_revoke_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "already_revoked", "reason": "foobar"})
serializer.rep_loads(serialized)
display("user_revoke_rep_already_revoked", serialized, [])

################### DeviceCreate ##################

serializer = device_create_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "device_create",
        "device_certificate": b"foobar",
        "redacted_device_certificate": b"foobar",
    }
)
serializer.req_loads(serialized)
display("device_create_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("device_create_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_certification", "reason": "foobar"})
serializer.rep_loads(serialized)
display("device_create_rep_invalid_certification", serialized, [])

serialized = serializer.rep_dumps({"status": "bad_user_id", "reason": "foobar"})
serializer.rep_loads(serialized)
display("device_create_rep_bad_user_id", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_data", "reason": "foobar"})
serializer.rep_loads(serialized)
display("device_create_rep_invalid_data", serialized, [])

serialized = serializer.rep_dumps({"status": "already_exists", "reason": "foobar"})
serializer.rep_loads(serialized)
display("device_create_rep_already_exists", serialized, [])

################### HumanFind ##################

serializer = human_find_serializer
serialized = serializer.req_dumps(
    {
        "cmd": "human_find",
        "query": "foobar",
        "omit_revoked": False,
        "omit_non_human": False,
        "page": 8,
        "per_page": 8,
    }
)
serializer.req_loads(serialized)
display("human_find_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "results": [
            {
                "user_id": UserID("109b68ba5cdf428ea0017fc6bcc04d4a"),
                "human_handle": HumanHandle("bob@dev1", "bob"),
                "revoked": False,
            }
        ],
        "page": 8,
        "per_page": 8,
        "total": 8,
    }
)
serializer.rep_loads(serialized)
display("human_find_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed", "reason": "foobar"})
serializer.rep_loads(serialized)
display("human_find_rep", serialized, [])
