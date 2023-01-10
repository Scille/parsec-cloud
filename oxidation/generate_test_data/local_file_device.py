# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

import trio
import tempfile
from protocol.utils import *

from parsec.crypto import *
from parsec.api.protocol import *
from parsec.api.data import *
from parsec.core.types import *
from parsec.core.local_device import *


with tempfile.NamedTemporaryFile(suffix=".psrk") as fp:
    passphrase = trio.run(save_recovery_device, Path(fp.name), ALICE, True)
    fp.seek(0)
    raw = fp.read()
content = display(f"recovery device file (passphrase: {passphrase})", raw, [])
key = SecretKey.from_recovery_passphrase(passphrase=passphrase)
display(f"recovery device file content", content["ciphertext"], [key])

with tempfile.NamedTemporaryFile(suffix=".keys") as fp:
    password = "P@ssw0rd."
    save_device_with_password(Path(fp.name), ALICE, password, True)
    fp.seek(0)
    raw = fp.read()
content = display(f"device file (password: {password})", raw, [])
key = SecretKey.from_password(password, salt=content["salt"])

# Legacy device key file without `human_handle` and `device_label`
raw = legacy_key_file_serializer.dumps(
    {
        "type": "password",
        "salt": content["salt"],
        "ciphertext": content["ciphertext"],
    }
)

display("legacy device file without `human_handle` and `device_label`", raw, [])

# With `human_handle` and `device_label`
raw = legacy_key_file_serializer.dumps(
    {
        "type": "password",
        "salt": content["salt"],
        "ciphertext": content["ciphertext"],
        "human_handle": ALICE.human_handle,
        "device_label": ALICE.device_label,
    }
)

display("legacy device file with `human_handle` and `device_label`", raw, [])

content = display(f"device file (password: {password})", content["ciphertext"], [key])
