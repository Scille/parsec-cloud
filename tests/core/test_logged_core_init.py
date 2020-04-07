# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from unittest.mock import ANY

from parsec.core import logged_core_factory
from parsec.core.backend_connection import BackendConnStatus


@pytest.mark.trio
async def test_init_online_backend_late_reply(
    server_factory, core_config, alice, event_bus, backend
):
    can_serve_client = trio.Event()

    async def _handle_client(stream):
        await can_serve_client.wait()
        return await backend.handle_client(stream)

    async with server_factory(_handle_client, alice.organization_addr):
        with trio.fail_after(1):
            async with logged_core_factory(
                config=core_config, device=alice, event_bus=event_bus
            ) as core:
                # We don't want for backend to reply finish core init
                with core.event_bus.listen() as spy:
                    can_serve_client.set()
                    # Now backend reply, monitor should send events accordingly
                    await spy.wait(
                        "backend.connection.changed",
                        kwargs={"status": BackendConnStatus.READY, "status_exc": None},
                    )


@pytest.mark.trio
async def test_init_offline_backend_late_reply(server_factory, core_config, alice, event_bus):
    can_serve_client = trio.Event()

    async def _handle_client(stream):
        await can_serve_client.wait()
        await stream.aclose()

    async with server_factory(_handle_client, alice.organization_addr):
        with trio.fail_after(1):
            async with logged_core_factory(
                config=core_config, device=alice, event_bus=event_bus
            ) as core:
                with core.event_bus.listen() as spy:
                    can_serve_client.set()
                    await spy.wait(
                        "backend.connection.changed",
                        kwargs={"status": BackendConnStatus.LOST, "status_exc": ANY},
                    )
