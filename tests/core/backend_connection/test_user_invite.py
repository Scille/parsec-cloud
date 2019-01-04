import pytest
import trio
import pendulum

from parsec.trustchain import certify_user, certify_device
from parsec.core.types import RemoteUser
from parsec.core.backend_connection import backend_cmds_factory, backend_anonymous_cmds_factory
from parsec.core.invite_claim import generate_user_encrypted_claim, extract_user_encrypted_claim


@pytest.mark.trio
async def test_user_invite_then_claim_ok(
    backend, running_backend, alice, alice_backend_cmds, mallory
):
    token = "424242"

    await backend.user.set_user_admin(alice.user_id, True)

    async def _alice_invite():
        encrypted_claim = await alice_backend_cmds.user_invite(mallory.user_id)
        claim = extract_user_encrypted_claim(alice.private_key, encrypted_claim)

        assert claim["token"] == token

        now = pendulum.now()
        certified_user = certify_user(
            alice.device_id,
            alice.signing_key,
            claim["device_id"].user_id,
            claim["public_key"],
            now=now,
        )
        certified_device = certify_device(
            alice.device_id, alice.signing_key, claim["device_id"], claim["verify_key"], now=now
        )
        await alice_backend_cmds.user_create(certified_user, certified_device)

    async def _mallory_claim():
        async with backend_anonymous_cmds_factory(mallory.backend_addr) as cmds:
            invitation_creator = await cmds.user_get_invitation_creator(mallory.user_id)
            assert isinstance(invitation_creator, RemoteUser)

            encrypted_claim = generate_user_encrypted_claim(
                invitation_creator.public_key,
                token,
                mallory.device_id,
                mallory.public_key,
                mallory.verify_key,
            )
            await cmds.user_claim(mallory.user_id, encrypted_claim)

    async with trio.open_nursery() as nursery:
        nursery.start_soon(_alice_invite)
        await running_backend.backend.event_bus.spy.wait(
            "event.connected", kwargs={"event_name": "user.claimed"}
        )
        nursery.start_soon(_mallory_claim)

    # Now mallory should be able to connect to backend
    async with backend_cmds_factory(
        mallory.backend_addr, mallory.device_id, mallory.signing_key
    ) as cmds:
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"
