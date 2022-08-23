# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from utils import *
from parsec.crypto import *
from parsec.api.protocol import *
from parsec.api.data import *

################### BlockCreate ##################

serializer = block_create_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "block_create",
        "block_id": BlockID.from_hex("57c629b69d6c4abbaf651cafa46dbc93"),
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "block": b"foobar",
    }
)
serializer.req_loads(serialized)
display("block_create_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("block_create_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "already_exists"})
serializer.rep_loads(serialized)
display("block_create_rep_already_exists", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("block_create_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "timeout"})
serializer.rep_loads(serialized)
display("block_create_rep_timeout", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("block_create_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "in_maintenance"})
serializer.rep_loads(serialized)
display("block_create_rep_in_maintenance", serialized, [])

################### BlockRead ##################

serializer = block_read_serializer

serialized = serializer.req_dumps(
    {"cmd": "block_read", "block_id": BlockID.from_hex("57c629b69d6c4abbaf651cafa46dbc93")}
)
serializer.req_loads(serialized)
display("block_read_req", serialized, [])

serialized = serializer.rep_dumps({"block": b"foobar"})
serializer.rep_loads(serialized)
display("block_read_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("block_read_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "timeout"})
serializer.rep_loads(serialized)
display("block_read_rep_timeout", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("block_read_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "in_maintenance"})
serializer.rep_loads(serialized)
display("block_read_rep_in_maintenance", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_msg_format"})
serializer.rep_loads(serialized)
display("block_read_rep_invalid_msg_format", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_msg_format", "reason": "reason"})
serializer.rep_loads(serialized)
display("block_read_rep_invalid_msg_format", serialized, [])
