# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from uuid import uuid4

from parsec.api.protocol import INVITED_CMDS, HandshakeInvitedOperation
from parsec.backend.invite import DeviceInvitation
from parsec.core.types import BackendInvitationAddr
from parsec.core.backend_connection import (
    BackendNotAvailable,
    BackendConnectionRefused,
    backend_invited_cmds_factory,
)

from tests.core.backend_connection.common import ALL_CMDS


@pytest.fixture
async def invitation_addr(backend, alice):
    invitation = DeviceInvitation(
        greeter_user_id=alice.user_id, greeter_human_handle=alice.human_handle
    )
    await backend.invite.new(organization_id=alice.organization_id, invitation=invitation)
    return BackendInvitationAddr.build(
        backend_addr=alice.organization_addr,
        organization_id=alice.organization_id,
        operation=HandshakeInvitedOperation.CLAIM_DEVICE,
        token=invitation.token,
    )


@pytest.mark.trio
async def test_backend_offline(invitation_addr):
    with pytest.raises(BackendNotAvailable):
        async with backend_invited_cmds_factory(invitation_addr) as cmds:
            await cmds.ping()


@pytest.mark.trio
async def test_backend_switch_offline(running_backend, invitation_addr):
    async with backend_invited_cmds_factory(invitation_addr) as cmds:
        await cmds.ping()
        with running_backend.offline():
            with pytest.raises(BackendNotAvailable):
                await cmds.ping()


@pytest.mark.trio
async def test_backend_closed_cmds(running_backend, invitation_addr):
    async with backend_invited_cmds_factory(invitation_addr) as cmds:
        pass
    with pytest.raises(trio.ClosedResourceError):
        await cmds.ping()


@pytest.mark.trio
async def test_ping(running_backend, invitation_addr):
    async with backend_invited_cmds_factory(invitation_addr) as cmds:
        rep = await cmds.ping("Hello World !")
        assert rep == {"status": "ok", "pong": "Hello World !"}


@pytest.mark.trio
async def test_handshake_organization_expired(running_backend, expiredorg, expiredorgalice):
    invitation = DeviceInvitation(
        greeter_user_id=expiredorgalice.user_id, greeter_human_handle=expiredorgalice.human_handle
    )
    await running_backend.backend.invite.new(
        organization_id=expiredorgalice.organization_id, invitation=invitation
    )
    invitation_addr = BackendInvitationAddr.build(
        backend_addr=running_backend.addr,
        organization_id=expiredorgalice.organization_id,
        operation=HandshakeInvitedOperation.CLAIM_DEVICE,
        token=invitation.token,
    )

    with pytest.raises(BackendConnectionRefused) as exc:
        async with backend_invited_cmds_factory(invitation_addr) as cmds:
            await cmds.ping()
    assert str(exc.value) == "Trial organization has expired"


@pytest.mark.trio
async def test_handshake_unknown_organization(running_backend, coolorg):
    invitation_addr = BackendInvitationAddr.build(
        backend_addr=running_backend.addr,
        organization_id=coolorg.organization_id,
        operation=HandshakeInvitedOperation.CLAIM_DEVICE,
        token=uuid4(),
    )
    with pytest.raises(BackendConnectionRefused) as exc:
        async with backend_invited_cmds_factory(invitation_addr) as cmds:
            await cmds.ping()
    assert str(exc.value) == "Invalid handshake information"


@pytest.mark.trio
async def test_invited_cmds_has_right_methods(running_backend, coolorg):
    async with backend_invited_cmds_factory(coolorg.addr) as cmds:
        for method_name in INVITED_CMDS:
            assert hasattr(cmds, method_name)
        for method_name in ALL_CMDS - INVITED_CMDS:
            assert not hasattr(cmds, method_name)
