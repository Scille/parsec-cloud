import pytest
import trio

from parsec.trustchain import certify_user, certify_device
from parsec.core.types import RemoteDevice
from parsec.core.backend_connection2 import (
    BackendNotAvailable,
    backend_cmds_connect,
    backend_anonymous_cmds_connect,
)

from tests.common import freeze_time


@pytest.mark.trio
async def test_user_invite_backend_offline(_2_alice_core, _2_mallory):
    with pytest.raises(BackendNotAvailable):
        await _2_alice_core.user_invite(_2_mallory.user_id)


@pytest.mark.trio
async def test_user_invite_then_claim_ok(
    _2_alice,
    _2_alice_core,
    _2_mallory,
    running_backend
    # running_backend, core_factory, core_sock_factory, alice_core_sock
):
    async def _alice_invite():
        encrypted_claim = await _2_alice_core.user_invite(_2_mallory.user_id)
        claim = _2_alice.private_key.decrypt(encrypted_claim)
        assert claim == b"touille"

        certified_user = certify_user(
            _2_alice.device_id, _2_alice.signing_key, _2_mallory.user_id, _2_mallory.public_key
        )
        certified_device = certify_device(
            _2_alice.device_id, _2_alice.signing_key, _2_mallory.device_id, _2_mallory.verify_key
        )
        await _2_alice_core.user_create(certified_user, certified_device)

    async def _mallory_claim():
        async with backend_anonymous_cmds_connect(_2_mallory.backend_addr) as conn:
            invitation_creator = await conn.user_get_invitation_creator(_2_mallory.user_id)
            assert isinstance(invitation_creator, RemoteDevice)

            claim = b"touille"
            encrypted_claim = invitation_creator.public_key.encrypt(claim)
            await conn.user_claim(_2_mallory.user_id, encrypted_claim)

    async with trio.open_nursery() as nursery:
        nursery.start_soon(_alice_invite)
        nursery.start_soon(_mallory_claim)

    # Now mallory should be able to connect to backend
    conn = await backend_cmds_connect(
        _2_mallory.backend_addr, _2_mallory.device_id, _2_mallory.signing_key
    )
    await conn.ping()


@pytest.mark.trio
async def test_user_invite_then_claim_timeout(
    running_backend, core_factory, core_sock_factory, alice_core_sock
):
    with freeze_time("2017-01-01"):
        await alice_core_sock.send({"cmd": "user_invite", "user_id": "mallory"})
        rep = await alice_core_sock.recv()
    assert rep["status"] == "ok"
    assert rep["user_id"] == "mallory"
    assert rep["invitation_token"]
    invitation_token = rep["invitation_token"]

    # Create a brand new core and try to use the token to register
    async with core_factory(devices=[]) as new_core, core_sock_factory(new_core) as new_core_sock:
        with freeze_time("2017-01-02"):
            await new_core_sock.send(
                {
                    "cmd": "user_claim",
                    "id": "mallory@device1",
                    "invitation_token": invitation_token,
                    "password": "S3cr37",
                }
            )
            rep = await new_core_sock.recv()
        assert rep == {"status": "out_of_date_error", "reason": "Claim code is too old."}


@pytest.mark.trio
async def test_user_find(running_backend, alice_core):
    ret = await alice_core.user_find()
    assert ret == (["alice", "bob"], 2)

    ret = await alice_core.user_find(query="al")
    assert ret == (["alice"], 1)
