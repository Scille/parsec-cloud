# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS


from parsec._parsec import DateTime
from parsec.api.data import *
from parsec.api.protocol import *

from .utils import *

################### MessageGet ##################

serializer = message_get_serializer

serialized = serializer.req_dumps({"cmd": "message_get", "offset": 8})
serializer.req_loads(serialized)
display("message_get_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "messages": {
            "count": 1,
            "sender": DeviceID("alice@dev1"),
            "timestamp": DateTime(2000, 1, 2, 1),
            "body": b"foobar",
        }
    }
)
serializer.rep_loads(serialized)
display("message_get_rep", serialized, [])
