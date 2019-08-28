# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio

from parsec.api.protocol import DeviceID
from parsec.core.invite_claim import (
    generate_invitation_token,
    invite_and_create_user,
    claim_user,
    invite_and_create_device,
    claim_device,
)
from parsec.core.backend_connection import backend_cmds_pool_factory


async def _invite_and_claim(running_backend, invite_func, claim_func, event_name="user.claimed"):
    with trio.fail_after(1):
        async with trio.open_nursery() as nursery:
            with running_backend.backend.event_bus.listen() as spy:
                nursery.start_soon(invite_func)
                await spy.wait("event.connected", {"event_name": event_name})
            nursery.start_soon(claim_func)


@pytest.mark.trio
async def test_invite_claim_non_admin_user(running_backend, backend, alice):
    new_device_id = DeviceID("zack@pc1")
    new_device = None
    token = generate_invitation_token()

    async def _from_alice():
        await invite_and_create_user(alice, new_device_id.user_id, token=token, is_admin=False)

    async def _from_mallory():
        nonlocal new_device
        new_device = await claim_user(alice.organization_addr, new_device_id, token=token)

    await _invite_and_claim(running_backend, _from_alice, _from_mallory)

    assert new_device.is_admin is False

    # Now connect as the new user
    async with backend_cmds_pool_factory(
        new_device.organization_addr, new_device.device_id, new_device.signing_key
    ) as cmds:
        await cmds.ping("foo")


@pytest.mark.trio
async def test_invite_claim_admin_user(running_backend, backend, alice):
    new_device_id = DeviceID("zack@pc1")
    new_device = None
    token = generate_invitation_token()

    async def _from_alice():
        await invite_and_create_user(alice, new_device_id.user_id, token=token, is_admin=True)

    async def _from_mallory():
        nonlocal new_device
        new_device = await claim_user(alice.organization_addr, new_device_id, token=token)

    await _invite_and_claim(running_backend, _from_alice, _from_mallory)

    assert new_device.is_admin

    # Now connect as the new user
    async with backend_cmds_pool_factory(
        new_device.organization_addr, new_device.device_id, new_device.signing_key
    ) as cmds:
        await cmds.ping("foo")


@pytest.mark.trio
async def test_invite_claim_3_chained_users(running_backend, backend, alice):
    # Zeta will be invited by Zoe, Zoe will be invited by Zack
    new_device_id_1 = DeviceID("zack@pc1")
    new_device_1 = None
    token_1 = generate_invitation_token()
    new_device_id_2 = DeviceID("zoe@pc2")
    new_device_2 = None
    token_2 = generate_invitation_token()
    new_device_id_3 = DeviceID("zeta@pc3")
    new_device_3 = None
    token_3 = generate_invitation_token()

    async def _invite_from_alice():
        await invite_and_create_user(alice, new_device_id_1.user_id, token=token_1, is_admin=True)

    async def _claim_from_1():
        nonlocal new_device_1
        new_device_1 = await claim_user(alice.organization_addr, new_device_id_1, token=token_1)

    async def _invite_from_1():
        await invite_and_create_user(
            new_device_1, new_device_id_2.user_id, token=token_2, is_admin=True
        )

    async def _claim_from_2():
        nonlocal new_device_2
        new_device_2 = await claim_user(alice.organization_addr, new_device_id_2, token=token_2)

    async def _invite_from_2():
        await invite_and_create_user(
            new_device_2, new_device_id_3.user_id, token=token_3, is_admin=False
        )

    async def _claim_from_3():
        nonlocal new_device_3
        new_device_3 = await claim_user(alice.organization_addr, new_device_id_3, token=token_3)

    await _invite_and_claim(running_backend, _invite_from_alice, _claim_from_1)
    await _invite_and_claim(running_backend, _invite_from_1, _claim_from_2)
    await _invite_and_claim(running_backend, _invite_from_2, _claim_from_3)

    assert new_device_1.is_admin
    assert new_device_2.is_admin
    assert not new_device_3.is_admin

    # Now connect as the last user
    async with backend_cmds_pool_factory(
        new_device_2.organization_addr, new_device_2.device_id, new_device_2.signing_key
    ) as cmds:
        await cmds.ping("foo")


@pytest.mark.trio
async def test_invite_claim_device(running_backend, backend, alice):
    new_device_id = DeviceID(f"{alice.user_id}@NewDevice")
    new_device = None
    token = generate_invitation_token()

    async def _from_alice():
        await invite_and_create_device(alice, new_device_id.device_name, token=token)

    async def _from_mallory():
        nonlocal new_device
        new_device = await claim_device(alice.organization_addr, new_device_id, token=token)

    await _invite_and_claim(
        running_backend, _from_alice, _from_mallory, event_name="device.claimed"
    )

    # Now connect as the new device
    async with backend_cmds_pool_factory(
        new_device.organization_addr, new_device.device_id, new_device.signing_key
    ) as cmds:
        await cmds.ping("foo")


@pytest.mark.trio
async def test_invite_claim_multiple_devices_from_chained_user(running_backend, backend, alice):
    # The devices are invited from one another
    new_device_id_1 = DeviceID("zack@pc1")
    new_device_1 = None
    token_1 = generate_invitation_token()

    new_device_id_2 = DeviceID("zack@pc2")
    new_device_2 = None
    token_2 = generate_invitation_token()

    new_device_id_3 = DeviceID("zack@pc3")
    new_device_3 = None
    token_3 = generate_invitation_token()

    async def _invite_from_alice():
        await invite_and_create_user(alice, new_device_id_1.user_id, token=token_1, is_admin=True)

    async def _claim_from_1():
        nonlocal new_device_1
        new_device_1 = await claim_user(alice.organization_addr, new_device_id_1, token=token_1)

    async def _invite_from_1():
        await invite_and_create_device(new_device_1, new_device_id_2.device_name, token=token_2)

    async def _claim_from_2():
        nonlocal new_device_2
        new_device_2 = await claim_device(alice.organization_addr, new_device_id_2, token=token_2)

    async def _invite_from_2():
        await invite_and_create_device(new_device_2, new_device_id_3.device_name, token=token_3)

    async def _claim_from_3():
        nonlocal new_device_3
        new_device_3 = await claim_device(alice.organization_addr, new_device_id_3, token=token_3)

    await _invite_and_claim(running_backend, _invite_from_alice, _claim_from_1)
    await _invite_and_claim(
        running_backend, _invite_from_1, _claim_from_2, event_name="device.claimed"
    )
    await _invite_and_claim(
        running_backend, _invite_from_2, _claim_from_3, event_name="device.claimed"
    )

    # Now connect as the last device
    async with backend_cmds_pool_factory(
        new_device_3.organization_addr, new_device_3.device_id, new_device_3.signing_key
    ) as cmds:
        await cmds.ping("foo")
