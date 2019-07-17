# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.core.backend_connection import (
    backend_administration_cmds_factory,
    backend_anonymous_cmds_factory,
    backend_cmds_pool_factory,
)


@pytest.mark.trio
async def test_anonymous_ping(running_backend, coolorg):
    async with backend_anonymous_cmds_factory(coolorg.addr) as cmds:
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"


@pytest.mark.trio
async def test_administration_ping(running_backend, backend_addr, backend):
    async with backend_administration_cmds_factory(
        backend_addr, backend.config.administration_token
    ) as cmds:
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"


@pytest.mark.trio
async def test_ping(running_backend, alice):
    async with backend_cmds_pool_factory(
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"
