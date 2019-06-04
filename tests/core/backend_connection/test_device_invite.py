# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
import pendulum

from parsec.types import DeviceID
from parsec.crypto import (
    PrivateKey,
    SigningKey,
    build_device_certificate,
    unsecure_read_device_certificate,
    unsecure_read_user_certificate,
)
from parsec.core.types import UnverifiedRemoteUser, UnverifiedRemoteDevice
from parsec.core.backend_connection import backend_cmds_pool_factory, backend_anonymous_cmds_factory
from parsec.core.invite_claim import (
    generate_device_encrypted_claim,
    extract_device_encrypted_claim,
    generate_device_encrypted_answer,
    extract_device_encrypted_answer,
)


@pytest.mark.trio
async def test_device_invite_then_claim_ok(alice, alice_backend_cmds, running_backend):
    nd_id = DeviceID(f"{alice.user_id}@new_device")
    nd_signing_key = SigningKey.generate()
    token = "123456"

    async def _alice_invite():
        encrypted_claim = await alice_backend_cmds.device_invite(nd_id.device_name)
        claim = extract_device_encrypted_claim(alice.private_key, encrypted_claim)

        assert claim["token"] == token
        device_certificate = build_device_certificate(
            alice.device_id,
            alice.signing_key,
            claim["device_id"],
            claim["verify_key"],
            pendulum.now(),
        )
        encrypted_answer = generate_device_encrypted_answer(
            claim["answer_public_key"],
            alice.private_key,
            alice.user_manifest_id,
            alice.user_manifest_key,
        )
        with trio.fail_after(1):
            await alice_backend_cmds.device_create(device_certificate, encrypted_answer)

    async def _alice_nd_claim():
        async with backend_anonymous_cmds_factory(alice.organization_addr) as cmds:
            invitation_creator_device, invitation_creator_user, trustchain = await cmds.device_get_invitation_creator(
                nd_id
            )
            assert isinstance(invitation_creator_device, UnverifiedRemoteDevice)
            assert isinstance(invitation_creator_user, UnverifiedRemoteUser)
            assert trustchain == []

            creator = unsecure_read_user_certificate(invitation_creator_user.user_certificate)

            creator_device = unsecure_read_device_certificate(
                invitation_creator_device.device_certificate
            )
            assert creator_device.device_id.split("@")[0] == creator.user_id

            answer_private_key = PrivateKey.generate()
            encrypted_claim = generate_device_encrypted_claim(
                creator_public_key=creator.public_key,
                token=token,
                device_id=nd_id,
                verify_key=nd_signing_key.verify_key,
                answer_public_key=answer_private_key.public_key,
            )
            with trio.fail_after(1):
                encrypted_answer = await cmds.device_claim(nd_id, encrypted_claim)

            answer = extract_device_encrypted_answer(answer_private_key, encrypted_answer)
            assert answer["private_key"] == alice.private_key
            assert answer == {
                "type": "device_claim_answer",
                "private_key": alice.private_key,
                "user_manifest_id": alice.user_manifest_id,
                "user_manifest_key": alice.user_manifest_key,
            }

    async with trio.open_nursery() as nursery:
        nursery.start_soon(_alice_invite)
        await running_backend.backend.event_bus.spy.wait(
            "event.connected", kwargs={"event_name": "device.claimed"}
        )
        nursery.start_soon(_alice_nd_claim)

    # Now alice's new device should be able to connect to backend
    async with backend_cmds_pool_factory(alice.organization_addr, nd_id, nd_signing_key) as cmds:
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"
