# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
import trio

from parsec.api.protocol import INVITED_CMDS, InvitationToken, InvitationType
from parsec.core.types import BackendInvitationAddr
from parsec.core.backend_connection import (
    BackendNotAvailable,
    BackendConnectionRefused,
    BackendInvitationNotFound,
    BackendInvitationAlreadyUsed,
    backend_invited_cmds_factory,
)
from parsec.backend.backend_events import BackendEvent
from tests.core.backend_connection.common import ALL_CMDS


@pytest.fixture
async def invitation_addr(backend, alice):
    invitation = await backend.invite.new_for_device(
        organization_id=alice.organization_id, greeter_user_id=alice.user_id
    )
    return BackendInvitationAddr.build(
        backend_addr=alice.organization_addr.get_backend_addr(),
        organization_id=alice.organization_id,
        invitation_type=InvitationType.DEVICE,
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
    invitation = await running_backend.backend.invite.new_for_device(
        organization_id=expiredorgalice.organization_id, greeter_user_id=expiredorgalice.user_id
    )
    invitation_addr = BackendInvitationAddr.build(
        backend_addr=running_backend.addr,
        organization_id=expiredorgalice.organization_id,
        invitation_type=InvitationType.DEVICE,
        token=invitation.token,
    )

    with running_backend.backend.event_bus.listen() as spy:
        with pytest.raises(BackendConnectionRefused) as exc:
            async with backend_invited_cmds_factory(invitation_addr) as cmds:
                await cmds.ping()
        await spy.wait_with_timeout(BackendEvent.ORGANIZATION_EXPIRED)
    assert str(exc.value) == "Trial organization has expired"


@pytest.mark.trio
async def test_handshake_unknown_organization(running_backend, coolorg):
    invitation_addr = BackendInvitationAddr.build(
        backend_addr=running_backend.addr,
        organization_id=coolorg.organization_id,
        invitation_type=InvitationType.DEVICE,
        token=InvitationToken.new(),
    )
    with pytest.raises(BackendInvitationNotFound) as exc:
        async with backend_invited_cmds_factory(invitation_addr) as cmds:
            await cmds.ping()
    assert str(exc.value) == "Invalid handshake: Invitation not found"


from parsec.api.protocol import InvitationDeletedReason
from pendulum import now as pendulum_now


@pytest.mark.trio
async def test_handshake_already_used_invitation(running_backend, coolorg, invitation_addr, alice):
    await running_backend.backend.invite.delete(
        organization_id=alice.organization_id,
        greeter=alice.user_id,
        token=invitation_addr.token,
        on=pendulum_now(),
        reason=InvitationDeletedReason.CANCELLED,
    )

    with pytest.raises(BackendInvitationAlreadyUsed) as exc:
        async with backend_invited_cmds_factory(invitation_addr) as cmds:
            await cmds.ping()
    assert str(exc.value) == "Invalid handshake: Invitation already deleted"


@pytest.mark.trio
async def test_invited_cmds_has_right_methods(running_backend, coolorg):
    async with backend_invited_cmds_factory(coolorg.addr) as cmds:
        for method_name in INVITED_CMDS:
            assert hasattr(cmds, method_name)
        for method_name in ALL_CMDS - INVITED_CMDS:
            assert not hasattr(cmds, method_name)
