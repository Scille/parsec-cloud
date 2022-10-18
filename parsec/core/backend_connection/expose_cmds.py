# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from functools import wraps
from typing import Any, TypeVar, Callable, Awaitable, cast
from typing_extensions import Concatenate, ParamSpec

from parsec.api.transport import Transport
from parsec.core.backend_connection.authenticated import BackendAuthenticatedCmds
from parsec.core.backend_connection.exceptions import BackendNotAvailable


P = ParamSpec("P")
R = TypeVar("R", bound=Awaitable[Any])


def expose_cmds(cmd: Callable[Concatenate[Transport, P], Awaitable[R]]) -> Callable[P, R]:
    @wraps(cmd)
    async def wrapper(self, *args: P.args, **kwargs: P.kwargs) -> R:  # type: ignore[no-untyped-def, misc]
        async with self.acquire_transport() as transport:
            return await cmd(transport, *args, **kwargs)

    # because of wraps mypy does not infer a proper type
    return cast(Callable[P, R], wrapper)


def expose_cmds_with_retrier(cmd: Callable[Concatenate[Transport, P], R]) -> Callable[P, R]:
    @wraps(cmd)
    async def wrapper(self: BackendAuthenticatedCmds, *args: P.args, **kwargs: P.kwargs) -> R:  # type: ignore[misc]
        # Reusing the transports expose us to `BackendNotAvailable` exceptions
        # due to inactivity timeout while the transport was in the pool.
        try:
            async with self.acquire_transport(allow_not_available=True) as transport:
                return await cmd(transport, *args, **kwargs)

        except BackendNotAvailable:
            async with self.acquire_transport(force_fresh=True) as transport:
                return await cmd(transport, *args, **kwargs)

    # because of wraps mypy does not infer a proper type
    return cast(Callable[P, R], wrapper)
