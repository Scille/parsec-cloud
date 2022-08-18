# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
import trio

from parsec.api.protocol import APIV1_ANONYMOUS_CMDS
from parsec.api.protocol.types import OrganizationID
from parsec.core.types import BackendOrganizationAddr
from parsec.core.backend_connection import (
    BackendNotAvailable,
    BackendConnectionRefused,
    apiv1_backend_anonymous_cmds_factory,
)

from parsec.backend.backend_events import BackendEvent
from tests.core.backend_connection.common import ALL_CMDS


@pytest.mark.trio
async def test_backend_offline(coolorg):
    with pytest.raises(BackendNotAvailable):
        async with apiv1_backend_anonymous_cmds_factory(coolorg.addr) as cmds:
            await cmds.ping()


@pytest.mark.trio
async def test_backend_switch_offline(running_backend, coolorg):
    async with apiv1_backend_anonymous_cmds_factory(coolorg.addr) as cmds:
        await cmds.ping()
        with running_backend.offline():
            with pytest.raises(BackendNotAvailable):
                await cmds.ping()


@pytest.mark.trio
async def test_backend_closed_cmds(running_backend, coolorg):
    async with apiv1_backend_anonymous_cmds_factory(coolorg.addr) as cmds:
        pass
    with pytest.raises(trio.ClosedResourceError):
        await cmds.ping()


@pytest.mark.trio
async def test_ping(running_backend, coolorg):
    async with apiv1_backend_anonymous_cmds_factory(coolorg.addr) as cmds:
        rep = await cmds.ping("Hello World !")
        assert rep == {"status": "ok", "pong": "Hello World !"}


@pytest.mark.trio
async def test_handshake_organization_expired(running_backend, expiredorg):
    with running_backend.backend.event_bus.listen() as spy:
        with pytest.raises(BackendConnectionRefused) as exc:
            async with apiv1_backend_anonymous_cmds_factory(expiredorg.addr) as cmds:
                await cmds.ping()
        await spy.wait_with_timeout(BackendEvent.ORGANIZATION_EXPIRED)
    assert str(exc.value) == "Trial organization has expired"


@pytest.mark.trio
async def test_handshake_unknown_organization(running_backend, coolorg):
    unknown_org_addr = BackendOrganizationAddr.build(
        backend_addr=coolorg.addr.get_backend_addr(),
        organization_id=OrganizationID("dummy"),
        root_verify_key=coolorg.root_verify_key,
    )
    with pytest.raises(BackendConnectionRefused) as exc:
        async with apiv1_backend_anonymous_cmds_factory(unknown_org_addr) as cmds:
            await cmds.ping()
    assert str(exc.value) == "Invalid handshake information"


@pytest.mark.trio
async def test_anonymous_cmds_has_right_methods(running_backend, coolorg):
    async with apiv1_backend_anonymous_cmds_factory(coolorg.addr) as cmds:
        for method_name in APIV1_ANONYMOUS_CMDS:
            assert hasattr(cmds, method_name)
        for method_name in ALL_CMDS - APIV1_ANONYMOUS_CMDS:
            assert not hasattr(cmds, method_name)
