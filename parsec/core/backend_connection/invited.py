# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import asynccontextmanager
from typing import TYPE_CHECKING, AsyncGenerator, AsyncIterator

import trio

from parsec._parsec import InvitedCmds as RsBackendInvitedCmds
from parsec.api.protocol import INVITED_CMDS
from parsec.core.backend_connection import BackendNotAvailable, cmds
from parsec.core.backend_connection.authenticated import OXIDIZED
from parsec.core.backend_connection.expose_cmds import Transport, expose_cmds_with_retrier
from parsec.core.backend_connection.transport import connect_as_invited
from parsec.core.types import BackendAddrType, BackendInvitationAddr

if TYPE_CHECKING:
    from parsec.core.backend_connection.authenticated import AcquireTransport


class BackendInvitedCmds:
    def __init__(
        self,
        addr: BackendAddrType,
        acquire_transport: AcquireTransport,
    ) -> None:
        self.addr = addr
        self.acquire_transport = acquire_transport

    ping = expose_cmds_with_retrier(cmds.invited_ping)
    invite_info = expose_cmds_with_retrier(cmds.invite_info)
    invite_1_claimer_wait_peer = expose_cmds_with_retrier(cmds.invite_1_claimer_wait_peer)
    invite_2a_claimer_send_hashed_nonce = expose_cmds_with_retrier(
        cmds.invite_2a_claimer_send_hashed_nonce
    )
    invite_2b_claimer_send_nonce = expose_cmds_with_retrier(cmds.invite_2b_claimer_send_nonce)
    invite_3a_claimer_signify_trust = expose_cmds_with_retrier(cmds.invite_3a_claimer_signify_trust)
    invite_3b_claimer_wait_peer_trust = expose_cmds_with_retrier(
        cmds.invite_3b_claimer_wait_peer_trust
    )
    invite_4_claimer_communicate = expose_cmds_with_retrier(cmds.invite_4_claimer_communicate)


for cmd in INVITED_CMDS:
    assert hasattr(BackendInvitedCmds, cmd)


@asynccontextmanager
async def backend_invited_cmds_factory(
    addr: BackendInvitationAddr, keepalive: int | None = None
) -> AsyncGenerator[BackendInvitedCmds | RsBackendInvitedCmds, None]:
    """
    Raises:
        BackendConnectionError
    """
    transport_lock = trio.Lock()
    transport: Transport | None = None
    closed = False

    async def _init_transport() -> Transport:
        nonlocal transport
        if not transport:
            if closed:
                raise trio.ClosedResourceError
            transport = await connect_as_invited(addr, keepalive=keepalive)
            transport.logger = transport.logger.bind(auth="<invited>")

        return transport

    async def _destroy_transport() -> None:
        nonlocal transport
        if transport:
            await transport.aclose()
            transport = None

    @asynccontextmanager
    async def _acquire_transport(
        force_fresh: bool = False, ignore_status: bool = False, allow_not_available: bool = False
    ) -> AsyncIterator[Transport]:
        nonlocal transport

        async with transport_lock:
            transport = await _init_transport()
            try:
                yield transport
            except BackendNotAvailable:
                await _destroy_transport()
                raise

    try:
        yield RsBackendInvitedCmds(addr) if OXIDIZED else BackendInvitedCmds(
            addr, _acquire_transport
        )

    finally:
        async with transport_lock:
            closed = True
            await _destroy_transport()
