# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio

from parsec.types import DeviceID
from parsec.core.invite_claim import (
    generate_invitation_token,
    invite_and_create_user,
    claim_user,
    invite_and_create_device,
    claim_device,
)
from parsec.core.backend_connection import backend_cmds_factory


@pytest.mark.trio
async def test_invite_claim_user(
    running_backend, backend, alice, alice_backend_cmds, anonymous_backend_cmds
):
    new_device_id = DeviceID("zack@pc1")
    new_device = None
    token = generate_invitation_token()

    await backend.user.set_user_admin(alice.organization_id, alice.user_id, True)

    async def _from_alice():
        await invite_and_create_user(
            alice, alice_backend_cmds, new_device_id.user_id, token=token, is_admin=False
        )

    async def _from_mallory():
        nonlocal new_device
        new_device = await claim_user(anonymous_backend_cmds, new_device_id, token=token)

    async with trio.open_nursery() as nursery:
        with running_backend.backend.event_bus.listen() as spy:
            nursery.start_soon(_from_alice)
            with trio.fail_after(1):
                await spy.wait("event.connected", kwargs={"event_name": "user.claimed"})
        nursery.start_soon(_from_mallory)

    # Now connect as the new user
    async with backend_cmds_factory(
        new_device.organization_addr, new_device.device_id, new_device.signing_key
    ) as cmds:
        await cmds.ping("foo")


@pytest.mark.trio
async def test_invite_claim_device(
    running_backend, backend, alice, alice_backend_cmds, anonymous_backend_cmds
):
    new_device_id = DeviceID(f"{alice.user_id}@NewDevice")
    new_device = None
    token = generate_invitation_token()

    await backend.user.set_user_admin(alice.organization_id, alice.user_id, True)

    async def _from_alice():
        await invite_and_create_device(
            alice, alice_backend_cmds, new_device_id.device_name, token=token
        )

    async def _from_mallory():
        nonlocal new_device
        new_device = await claim_device(anonymous_backend_cmds, new_device_id, token=token)

    async with trio.open_nursery() as nursery:
        with running_backend.backend.event_bus.listen() as spy:
            nursery.start_soon(_from_alice)
            with trio.fail_after(1):
                await spy.wait("event.connected", kwargs={"event_name": "device.claimed"})
        nursery.start_soon(_from_mallory)

    # Now connect as the new device
    async with backend_cmds_factory(
        new_device.organization_addr, new_device.device_id, new_device.signing_key
    ) as cmds:
        await cmds.ping("foo")
