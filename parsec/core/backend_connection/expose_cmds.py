# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from functools import wraps
from typing import TypeVar, Callable, Awaitable, List

from parsec.api.transport import Transport
from parsec.core.backend_connection.exceptions import BackendNotAvailable

try:
    # PEP 612 typing (available with Python>=3.10), This allow us to properly
    # check the types of the arguments of all the `cmds` commands
    from typing import ParamSpec, Concatenate

    P = ParamSpec("P")
    R = TypeVar("R")
    ExposedCmdInput = Callable[Concatenate[Transport, P], R]
    ExposedCmdOutput = Callable[P, R]
    PArgs = P.args
    PKwargs = P.kwargs
except ImportError:
    # Fallback typing
    P = TypeVar("P")
    R = TypeVar("R")
    ExposedCmdInput = Callable[..., Awaitable[R]]
    ExposedCmdOutput = Callable[..., Awaitable[R]]
    PArgs = object
    PKwargs = object

K = TypeVar("K", bound=List)
R = TypeVar("R")
Cmd = TypeVar("Cmd", bound=Callable)


def expose_cmds(cmd: ExposedCmdInput) -> ExposedCmdOutput:
    @wraps(cmd)
    async def wrapper(self, *args: PArgs, **kwargs: PKwargs) -> R:
        async with self.acquire_transport() as transport:
            return await cmd(transport, *args, **kwargs)

    return wrapper


def expose_cmds_with_retrier(cmd: ExposedCmdInput) -> ExposedCmdOutput:
    @wraps(cmd)
    async def wrapper(self, *args: PArgs, **kwargs: PKwargs) -> R:
        # Reusing the transports expose us to `BackendNotAvailable` exceptions
        # due to inactivity timeout while the transport was in the pool.
        try:
            async with self.acquire_transport(allow_not_available=True) as transport:
                return await cmd(transport, *args, **kwargs)

        except BackendNotAvailable:
            async with self.acquire_transport(force_fresh=True) as transport:
                return await cmd(transport, *args, **kwargs)

    return wrapper
