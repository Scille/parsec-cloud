# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from async_generator import asynccontextmanager
from typing import Optional

from parsec.core.types import BackendOrganizationAddr
from parsec.core.backend_connection import cmds
from parsec.core.backend_connection.transport import connect
from parsec.core.backend_connection.exceptions import BackendNotAvailable


class BackendAnonymousCmds:
    def __init__(self, addr, acquire_transport):
        self.addr = addr
        self.acquire_transport = acquire_transport

    def _expose_cmds(name):
        cmd = getattr(cmds, name)

        async def wrapper(self, *args, **kwargs):
            async with self.acquire_transport() as transport:
                return await cmd(transport, *args, **kwargs)

        wrapper.__name__ = name

        return wrapper

    ping = _expose_cmds("ping")

    organization_bootstrap = _expose_cmds("organization_bootstrap")

    user_get_invitation_creator = _expose_cmds("user_get_invitation_creator")
    user_claim = _expose_cmds("user_claim")

    device_get_invitation_creator = _expose_cmds("device_get_invitation_creator")
    device_claim = _expose_cmds("device_claim")


@asynccontextmanager
async def backend_anonymous_cmds_factory(
    addr: BackendOrganizationAddr, keepalive: Optional[int] = None
) -> BackendAnonymousCmds:
    """
    Raises:
        BackendConnectionError
    """
    transport_lock = trio.Lock()
    transport = None
    closed = False

    async def _init_transport():
        nonlocal transport
        if not transport:
            if closed:
                raise trio.ClosedResourceError
            transport = await connect(addr, keepalive=keepalive)
            transport.logger = transport.logger.bind(auth="<anonymous>")

    async def _destroy_transport():
        nonlocal transport
        if transport:
            await transport.aclose()
            transport = None

    @asynccontextmanager
    async def _acquire_transport(**kwargs):
        nonlocal transport

        async with transport_lock:
            await _init_transport()
            try:
                yield transport
            except BackendNotAvailable:
                await _destroy_transport()
                raise

    try:
        yield BackendAnonymousCmds(addr, _acquire_transport)

    finally:
        async with transport_lock:
            closed = True
            await _destroy_transport()
