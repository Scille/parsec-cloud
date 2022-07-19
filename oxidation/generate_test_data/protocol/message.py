# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS
# flake8: noqa

from libparsec.types import DateTime
from utils import *
from parsec.crypto import *
from parsec.api.protocol import *
from parsec.api.data import *

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
