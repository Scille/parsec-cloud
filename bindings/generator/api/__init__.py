# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
# ruff: noqa: F403

from .account import *
from .addr import *
from .cancel import *
from .client import *
from .common import *
from .config import *
from .device import *
from .events import *
from .invite import *
from .path import *
from .pki import *
from .platform import *
from .testbed import *
from .validation import *
from .workspace import *
from .workspace_history import *


def libparsec_init_set_on_event_callback(
    on_event_callback: OnClientEventCallback,  # noqa: F405
):
    raise NotImplementedError


async def libparsec_init_native_only_init(
    config: ClientConfig,  # noqa: F405
):
    raise NotImplementedError
