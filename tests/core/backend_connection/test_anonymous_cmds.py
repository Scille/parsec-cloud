# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio

from parsec.core.backend_connection import BackendNotAvailable, backend_anonymous_cmds_factory


@pytest.mark.trio
async def test_anonymous_backend_offline(coolorg):
    with pytest.raises(BackendNotAvailable):
        async with backend_anonymous_cmds_factory(coolorg.addr):
            pass


@pytest.mark.trio
async def test_anonymous_backend_switch_offline(running_backend, coolorg):
    async with backend_anonymous_cmds_factory(coolorg.addr) as cmds:
        with running_backend.offline():
            with pytest.raises(BackendNotAvailable):
                await cmds.ping("Whatever")


@pytest.mark.trio
async def test_anonymous_backend_closed_cmds(running_backend, coolorg):
    async with backend_anonymous_cmds_factory(coolorg.addr) as cmds:
        pass
    with pytest.raises(trio.ClosedResourceError):
        await cmds.ping("Whatever")
