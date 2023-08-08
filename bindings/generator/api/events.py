# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import Callable

from .common import Variant


class ClientEvent(Variant):
    class Ping:
        ping: str


class OnClientEventCallback(Callable[[ClientEvent], None]):
    pass
