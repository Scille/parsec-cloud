# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from async_generator import asynccontextmanager
from typing import Optional

from parsec.core.types import BackendOrganizationAddr
from parsec.core.backend_connection.transport import connect
from parsec.core.backend_connection.exceptions import BackendNotAvailable
from parsec.core.backend_connection.expose_cmds import expose_cmds
from parsec.api.protocol import ANONYMOUS_CMDS


class BackendAnonymousCmds:
    def __init__(self, addr, acquire_transport):
        self.addr = addr
        self.acquire_transport = acquire_transport

    for cmd_name in ANONYMOUS_CMDS:
        vars()[cmd_name] = expose_cmds(cmd_name)


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
