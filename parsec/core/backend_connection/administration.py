# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from async_generator import asynccontextmanager
from typing import Optional

from parsec.core.types import BackendOrganizationAddr
from parsec.core.backend_connection import cmds
from parsec.core.backend_connection.transport import connect
from parsec.core.backend_connection.exceptions import BackendNotAvailable


class BackendAdministrationCmds:
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

    organization_create = _expose_cmds("organization_create")
    organization_status = _expose_cmds("organization_status")
    organization_stats = _expose_cmds("organization_stats")


@asynccontextmanager
async def backend_administration_cmds_factory(
    addr: BackendOrganizationAddr, token: str, keepalive: Optional[int] = None
) -> BackendAdministrationCmds:
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
            transport = await connect(addr, administration_token=token, keepalive=keepalive)
            transport.logger = transport.logger.bind(auth="<administration>")

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
        yield BackendAdministrationCmds(addr, _acquire_transport)

    finally:
        async with transport_lock:
            closed = True
            await _destroy_transport()
