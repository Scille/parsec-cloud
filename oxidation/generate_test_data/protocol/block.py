# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS


from parsec._parsec import (
    BlockCreateRepAlreadyExists,
    BlockCreateRepInMaintenance,
    BlockCreateRepNotAllowed,
    BlockCreateRepNotFound,
    BlockCreateRepOk,
    BlockCreateRepRealmArchived,
    BlockCreateRepRealmDeleted,
    BlockCreateRepTimeout,
    BlockReadRepInMaintenance,
    BlockReadRepNotAllowed,
    BlockReadRepNotFound,
    BlockReadRepOk,
    BlockReadRepRealmDeleted,
    BlockReadRepTimeout,
)
from parsec.api.data import *
from parsec.api.protocol import *

from .utils import *

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

serialized = serializer.rep_dumps(BlockCreateRepOk())
serializer.rep_loads(serialized)
display("block_create_rep", serialized, [])

serialized = serializer.rep_dumps(BlockCreateRepAlreadyExists())
serializer.rep_loads(serialized)
display("block_create_rep_already_exists", serialized, [])

serialized = serializer.rep_dumps(BlockCreateRepNotFound())
serializer.rep_loads(serialized)
display("block_create_rep_not_found", serialized, [])

serialized = serializer.rep_dumps(BlockCreateRepTimeout())
serializer.rep_loads(serialized)
display("block_create_rep_timeout", serialized, [])

serialized = serializer.rep_dumps(BlockCreateRepNotAllowed())
serializer.rep_loads(serialized)
display("block_create_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(BlockCreateRepInMaintenance())
serializer.rep_loads(serialized)
display("block_create_rep_in_maintenance", serialized, [])

serialized = serializer.rep_dumps(BlockCreateRepRealmArchived())
serializer.rep_loads(serialized)
display("block_create_rep_realm_archived", serialized, [])

serialized = serializer.rep_dumps(BlockCreateRepRealmDeleted())
serializer.rep_loads(serialized)
display("block_create_rep_realm_deleted", serialized, [])

################### BlockRead ##################

serializer = block_read_serializer

serialized = serializer.req_dumps(
    {"cmd": "block_read", "block_id": BlockID.from_hex("57c629b69d6c4abbaf651cafa46dbc93")}
)
serializer.req_loads(serialized)
display("block_read_req", serialized, [])

serialized = serializer.rep_dumps(BlockReadRepOk(block=b"foobar"))
serializer.rep_loads(serialized)
display("block_read_rep", serialized, [])

serialized = serializer.rep_dumps(BlockReadRepNotFound())
serializer.rep_loads(serialized)
display("block_read_rep_not_found", serialized, [])

serialized = serializer.rep_dumps(BlockReadRepTimeout())
serializer.rep_loads(serialized)
display("block_read_rep_timeout", serialized, [])

serialized = serializer.rep_dumps(BlockReadRepNotAllowed())
serializer.rep_loads(serialized)
display("block_read_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps(BlockReadRepInMaintenance())
serializer.rep_loads(serialized)
display("block_read_rep_in_maintenance", serialized, [])

serialized = serializer.rep_dumps(BlockReadRepRealmDeleted())
serializer.rep_loads(serialized)
display("block_read_rep_realm_deleted", serialized, [])
