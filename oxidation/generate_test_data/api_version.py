# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import msgpack

from parsec.api.version import ApiVersion

version = ApiVersion(2, 15)
raw_bytes = msgpack.packb(version)
unpacked = msgpack.unpackb(raw_bytes)
assert unpacked == [2, 15]

print(raw_bytes.hex())
