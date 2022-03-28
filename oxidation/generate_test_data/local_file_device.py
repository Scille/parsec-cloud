# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS
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
key = derivate_secret_key_from_recovery_passphrase(passphrase=passphrase)
display(f"recovery device file content", content["ciphertext"], [key])

with tempfile.NamedTemporaryFile(suffix=".keys") as fp:
    password = "P@ssw0rd."
    save_device_with_password(Path(fp.name), ALICE, password, True)
    fp.seek(0)
    raw = fp.read()
content = display(f"device file (password: {password})", raw, [])
key, _ = derivate_secret_key_from_password(password, salt=content["salt"])
content = display(f"device file (password: {password})", content["ciphertext"], [key])
