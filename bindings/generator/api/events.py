# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Callable

from .common import Variant


class ClientEvent(Variant):
    class Ping:
        ping: str


class OnClientEventCallback(Callable[[ClientEvent], None]):  # type: ignore[misc]
    ...
