# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
import pendulum

from parsec.api.data import UserCertificateContent, DeviceCertificateContent, UserClaimContent
from parsec.core.backend_connection import backend_cmds_pool_factory, backend_anonymous_cmds_factory


@pytest.mark.trio
async def test_user_invite_then_claim_ok(
    backend, running_backend, alice, alice_backend_cmds, mallory
):
    token = "424242"

    async def _alice_invite():
        encrypted_claim = await alice_backend_cmds.user_invite(mallory.user_id)
        claim = UserClaimContent.decrypt_and_load_for(
            encrypted_claim, recipient_privkey=alice.private_key
        )

        assert claim.token == token

        now = pendulum.now()
        user_certificate = UserCertificateContent(
            author=alice.device_id,
            timestamp=now,
            user_id=claim.device_id.user_id,
            public_key=claim.public_key,
            is_admin=False,
        ).dump_and_sign(alice.signing_key)
        device_certificate = DeviceCertificateContent(
            author=alice.device_id,
            timestamp=now,
            device_id=claim.device_id,
            verify_key=claim.verify_key,
        ).dump_and_sign(alice.signing_key)
        with trio.fail_after(1):
            await alice_backend_cmds.user_create(user_certificate, device_certificate)

    async def _mallory_claim():
        async with backend_anonymous_cmds_factory(mallory.organization_addr) as cmds:
            creator_user_certificate, creator_device_certificate, trustchain = await cmds.user_get_invitation_creator(
                mallory.user_id
            )
            assert trustchain == {"devices": [], "revoked_users": [], "users": []}
            creator = UserCertificateContent.unsecure_load(creator_user_certificate)
            creator_device = DeviceCertificateContent.unsecure_load(creator_device_certificate)
            assert creator_device.device_id.user_id == creator.user_id

            encrypted_claim = UserClaimContent(
                device_id=mallory.device_id,
                token=token,
                public_key=mallory.public_key,
                verify_key=mallory.verify_key,
            ).dump_and_encrypt_for(recipient_pubkey=creator.public_key)
            with trio.fail_after(1):
                await cmds.user_claim(mallory.user_id, encrypted_claim)

    with running_backend.backend.event_bus.listen() as spy:
        async with trio.open_service_nursery() as nursery:
            nursery.start_soon(_alice_invite)
            await spy.wait_with_timeout("event.connected", {"event_name": "user.claimed"})
            nursery.start_soon(_mallory_claim)

    # Now mallory should be able to connect to backend
    async with backend_cmds_pool_factory(
        mallory.organization_addr, mallory.device_id, mallory.signing_key
    ) as cmds:
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"
