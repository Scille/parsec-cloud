# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from utils import *
from parsec.crypto import *
from parsec.api.protocol import *
from parsec.api.data import *

################### Ping ##################

serializer = ping_serializer

serialized = serializer.req_dumps({"cmd": "ping", "ping": "ping"})
serializer.req_loads(serialized)
display("ping req", serialized, [])

serialized = serializer.rep_dumps({"pong": "pong"})
serializer.rep_loads(serialized)
display("ping rep", serialized, [])
