# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from async_generator import asynccontextmanager
from typing import Optional

from parsec.core.types import BackendOrganizationAddr
from parsec.core.backend_connection.transport import connect
from parsec.core.backend_connection.exceptions import BackendNotAvailable
from parsec.core.backend_connection.expose_cmds import expose_cmds
from parsec.api.protocol import ADMINISTRATION_CMDS


class BackendAdministrationCmds:
    def __init__(self, addr, acquire_transport):
        self.addr = addr
        self.acquire_transport = acquire_transport

    for cmd_name in ADMINISTRATION_CMDS:
        vars()[cmd_name] = expose_cmds(cmd_name)


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
