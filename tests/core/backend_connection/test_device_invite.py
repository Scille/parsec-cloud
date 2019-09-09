# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
import pendulum

from parsec.api.data import (
    UserCertificateContent,
    DeviceCertificateContent,
    DeviceClaimContent,
    DeviceClaimAnswerContent,
)
from parsec.api.protocol import DeviceID
from parsec.crypto import PrivateKey, SigningKey
from parsec.core.types import UnverifiedRemoteUser, UnverifiedRemoteDevice
from parsec.core.backend_connection import backend_cmds_pool_factory, backend_anonymous_cmds_factory


@pytest.mark.trio
async def test_device_invite_then_claim_ok(alice, alice_backend_cmds, running_backend):
    nd_id = DeviceID(f"{alice.user_id}@new_device")
    nd_signing_key = SigningKey.generate()
    token = "123456"
    device_certificate = None

    async def _alice_invite():
        nonlocal device_certificate

        encrypted_claim = await alice_backend_cmds.device_invite(nd_id.device_name)
        claim = DeviceClaimContent.decrypt_and_load_for(
            encrypted_claim, recipient_privkey=alice.private_key
        )

        assert claim.token == token

        device_certificate = DeviceCertificateContent(
            author=alice.device_id,
            timestamp=pendulum.now(),
            device_id=claim.device_id,
            verify_key=claim.verify_key,
        ).dump_and_sign(alice.signing_key)
        encrypted_answer = DeviceClaimAnswerContent(
            private_key=alice.private_key,
            user_manifest_id=alice.user_manifest_id,
            user_manifest_key=alice.user_manifest_key,
        ).dump_and_encrypt_for(recipient_pubkey=claim.answer_public_key)
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

            creator = UserCertificateContent.unsecure_load(invitation_creator_user.user_certificate)

            creator_device = DeviceCertificateContent.unsecure_load(
                invitation_creator_device.device_certificate
            )
            assert creator_device.device_id.user_id == creator.user_id

            answer_private_key = PrivateKey.generate()
            encrypted_claim = DeviceClaimContent(
                token=token,
                device_id=nd_id,
                verify_key=nd_signing_key.verify_key,
                answer_public_key=answer_private_key.public_key,
            ).dump_and_encrypt_for(recipient_pubkey=creator.public_key)
            with trio.fail_after(1):
                unverified_device, encrypted_answer = await cmds.device_claim(
                    nd_id, encrypted_claim
                )

            assert unverified_device.device_certificate == device_certificate
            answer = DeviceClaimAnswerContent.decrypt_and_load_for(
                encrypted_answer, recipient_privkey=answer_private_key
            )
            assert answer == DeviceClaimAnswerContent(
                private_key=alice.private_key,
                user_manifest_id=alice.user_manifest_id,
                user_manifest_key=alice.user_manifest_key,
            )

    with running_backend.backend.event_bus.listen() as spy:
        async with trio.open_nursery() as nursery:
            nursery.start_soon(_alice_invite)
            await spy.wait_with_timeout("event.connected", {"event_name": "device.claimed"})
            nursery.start_soon(_alice_nd_claim)

    # Now alice's new device should be able to connect to backend
    async with backend_cmds_pool_factory(alice.organization_addr, nd_id, nd_signing_key) as cmds:
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"
