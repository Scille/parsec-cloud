# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Awaitable, Callable, Optional, TypeVar

_R = TypeVar("_R")

def run(
    proc: Callable[..., Awaitable[_R]], *args: object, queue_len: Optional[int] = None
) -> _R: ...
