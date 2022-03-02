# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS
# flake8: noqa

from pendulum import datetime
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
        "trustchain": {"devices": [b"foobar"], "users": [b"foobar"], "revoked_users": [b"foobar"]},
    }
)
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

################### UserRevoke ##################

serializer = user_revoke_serializer

serialized = serializer.req_dumps({"cmd": "user_revoke", "revoked_user_certificate": b"foobar"})
serializer.req_loads(serialized)
display("user_revoke_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("user_revoke_rep", serialized, [])

################### UserCreate ##################

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
