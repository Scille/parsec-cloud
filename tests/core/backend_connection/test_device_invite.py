import pytest
import trio

from parsec.types import DeviceID
from parsec.crypto import PrivateKey, SigningKey
from parsec.trustchain import certify_device
from parsec.core.types import RemoteUser
from parsec.core.backend_connection2 import (
    BackendNotAvailable,
    backend_cmds_connect,
    backend_anonymous_cmds_connect,
)
from parsec.core.invite_claim import (
    generate_device_encrypted_claim,
    extract_device_encrypted_claim,
    generate_device_encrypted_answer,
    extract_device_encrypted_answer,
)


@pytest.mark.trio
async def test_device_invite_backend_offline(alice_core, alice2):
    with pytest.raises(BackendNotAvailable):
        await alice_core.device_invite(alice2.device_id)


@pytest.mark.trio
async def test_device_invite_then_claim_ok(alice, alice_core, running_backend):
    nd_id = DeviceID("alice@new_device")
    nd_signing_key = SigningKey.generate()

    async def _alice_invite():
        encrypted_claim = await alice_core.device_invite(nd_id)
        claim = extract_device_encrypted_claim(alice.private_key, encrypted_claim)

        certified_device = certify_device(
            alice.device_id, alice.signing_key, claim["device_id"], claim["verify_key"]
        )
        encrypted_answer = generate_device_encrypted_answer(
            claim["answer_public_key"], alice.private_key, alice.user_manifest_access
        )
        await alice_core.device_create(certified_device, encrypted_answer)

    async def _alice_nd_claim():
        async with backend_anonymous_cmds_connect(running_backend.addr) as conn:
            invitation_creator = await conn.device_get_invitation_creator(nd_id)
            assert isinstance(invitation_creator, RemoteUser)

            answer_private_key = PrivateKey.generate()
            encrypted_claim = generate_device_encrypted_claim(
                invitation_creator.public_key,
                nd_id,
                nd_signing_key.verify_key,
                answer_private_key.public_key,
            )
            encrypted_answer = await conn.device_claim(nd_id, encrypted_claim)

            answer = extract_device_encrypted_answer(answer_private_key, encrypted_answer)
            assert answer["private_key"] == alice.private_key
            assert answer == {
                "type": "device_claim_answer",
                "private_key": alice.private_key,
                "user_manifest_access": alice.user_manifest_access,
            }

    async with trio.open_nursery() as nursery:
        nursery.start_soon(_alice_invite)
        await running_backend.backend.event_bus.spy.wait(
            "event.connected", kwargs={"event_name": "device.claimed"}
        )
        nursery.start_soon(_alice_nd_claim)

    # Now alice's new device should be able to connect to backend
    async with backend_cmds_connect(running_backend.addr, nd_id, nd_signing_key) as conn:
        pong = await conn.ping("Hello World !")
        assert pong == "Hello World !"
