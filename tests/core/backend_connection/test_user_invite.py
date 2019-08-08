# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
import pendulum

from parsec.crypto import (
    build_user_certificate,
    build_device_certificate,
    unsecure_read_device_certificate,
    unsecure_read_user_certificate,
)
from parsec.core.types import UnverifiedRemoteUser, UnverifiedRemoteDevice
from parsec.core.backend_connection import backend_cmds_pool_factory, backend_anonymous_cmds_factory
from parsec.core.invite_claim import generate_user_encrypted_claim, extract_user_encrypted_claim


@pytest.mark.trio
async def test_user_invite_then_claim_ok(
    backend, running_backend, alice, alice_backend_cmds, mallory
):
    token = "424242"

    async def _alice_invite():
        encrypted_claim = await alice_backend_cmds.user_invite(mallory.user_id)
        claim = extract_user_encrypted_claim(alice.private_key, encrypted_claim)

        assert claim["token"] == token

        now = pendulum.now()
        user_certificate = build_user_certificate(
            alice.device_id,
            alice.signing_key,
            claim["device_id"].user_id,
            claim["public_key"],
            False,
            now,
        )
        device_certificate = build_device_certificate(
            alice.device_id, alice.signing_key, claim["device_id"], claim["verify_key"], now
        )
        with trio.fail_after(1):
            await alice_backend_cmds.user_create(user_certificate, device_certificate)

    async def _mallory_claim():
        async with backend_anonymous_cmds_factory(mallory.organization_addr) as cmds:
            invitation_creator_device, invitation_creator_user, trustchain = await cmds.user_get_invitation_creator(
                mallory.user_id
            )
            assert isinstance(invitation_creator_device, UnverifiedRemoteDevice)
            assert isinstance(invitation_creator_user, UnverifiedRemoteUser)
            assert trustchain == []

            creator = unsecure_read_user_certificate(invitation_creator_user.user_certificate)

            creator_device = unsecure_read_device_certificate(
                invitation_creator_device.device_certificate
            )
            assert creator_device.device_id.user_id == creator.user_id

            encrypted_claim = generate_user_encrypted_claim(
                creator.public_key, token, mallory.device_id, mallory.public_key, mallory.verify_key
            )
            with trio.fail_after(1):
                await cmds.user_claim(mallory.user_id, encrypted_claim)

    with running_backend.backend.event_bus.listen() as spy:
        async with trio.open_nursery() as nursery:
            nursery.start_soon(_alice_invite)
            await spy.wait_with_timeout("event.connected", {"event_name": "user.claimed"})
            nursery.start_soon(_mallory_claim)

    # Now mallory should be able to connect to backend
    async with backend_cmds_pool_factory(
        mallory.organization_addr, mallory.device_id, mallory.signing_key
    ) as cmds:
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"
