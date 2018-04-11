import pytest
from unittest.mock import patch

from parsec.utils import to_jsonb64

from tests.common import freeze_time, connect_backend


@pytest.fixture
def mock_generate_token():
    with patch("parsec.backend.user._generate_token") as mock_generate_token:
        yield mock_generate_token


@pytest.mark.trio
async def test_user_get_ok(backend, alice, bob):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "user_get", "user_id": "bob"})
        rep = await sock.recv()
    assert rep == {
        "status": "ok",
        "user_id": "bob",
        "broadcast_key": "PJyDuQC6ZXlhLqnwP9cwLZE9ye18fydKmzAgT/dvS04=\n",
        "created_by": "<backend-fixture>",
        "created_on": "2000-01-01T00:00:00+00:00",
        "devices": {
            "test": {
                "created_on": "2000-01-01T00:00:00+00:00",
                "revocated_on": None,
                "verify_key": "p9KzHZjz4qjlFSTvMNNMU4QVD5CfdWEJ/Yprw/G9Xl4=\n",
            }
        },
    }


@pytest.mark.parametrize(
    "bad_msg",
    [{"user_id": 42}, {"user_id": None}, {"user_id": "alice", "unknown": "field"}, {}],
)
@pytest.mark.trio
async def test_user_get_bad_msg(backend, alice, bad_msg):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "user_get", **bad_msg})
        rep = await sock.recv()
        assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_user_get_not_found(backend, alice):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "user_get", "user_id": "dummy"})
        rep = await sock.recv()
        assert rep == {"status": "not_found", "reason": "No user with id `dummy`."}


@pytest.mark.trio
async def test_user_invite(backend, alice, mock_generate_token):
    mock_generate_token.side_effect = ["<token>"]
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "user_invite", "user_id": "john"})
        rep = await sock.recv()
    assert rep == {"status": "ok", "user_id": "john", "invitation_token": "<token>"}


@pytest.mark.trio
async def test_user_claim_unknown_token(backend, mallory):
    async with connect_backend(backend, auth_as="anonymous") as sock:
        await sock.send(
            {
                "cmd": "user_claim",
                "user_id": mallory.user_id,
                "invitation_token": "<token>",
                "broadcast_key": to_jsonb64(mallory.user_pubkey.encode()),
                "device_name": mallory.device_name,
                "device_verify_key": to_jsonb64(mallory.device_verifykey.encode()),
            }
        )
        rep = await sock.recv()
    assert rep == {
        "status": "not_found_error", "reason": "No invitation for user `mallory`"
    }


@pytest.fixture
async def invitation_token(backend, alice, mallory):
    token = "1234567890"
    with freeze_time("2017-07-07T00:00:00"):
        await backend.user.create_invitation(token, alice.id, mallory.user_id)
    return token


@pytest.mark.trio
async def test_user_claim_too_old_token(backend, invitation_token, mallory):
    async with connect_backend(backend, auth_as="anonymous") as sock:
        with freeze_time("2017-07-07T01:01:00"):
            await sock.send(
                {
                    "cmd": "user_claim",
                    "user_id": mallory.user_id,
                    "invitation_token": invitation_token,
                    "broadcast_key": to_jsonb64(mallory.user_pubkey.encode()),
                    "device_name": mallory.device_name,
                    "device_verify_key": to_jsonb64(mallory.device_verifykey.encode()),
                }
            )
            rep = await sock.recv()
    assert rep == {"status": "out_of_date_error", "reason": "Claim code is too old."}


@pytest.mark.trio
async def test_user_claim_token(backend, invitation_token, mallory):
    async with connect_backend(backend, auth_as="anonymous") as sock:
        with freeze_time("2017-07-07T00:59:00"):
            await sock.send(
                {
                    "cmd": "user_claim",
                    "user_id": mallory.user_id,
                    "invitation_token": invitation_token,
                    "broadcast_key": to_jsonb64(mallory.user_pubkey.encode()),
                    "device_name": mallory.device_name,
                    "device_verify_key": to_jsonb64(mallory.device_verifykey.encode()),
                }
            )
            rep = await sock.recv()
    assert rep == {"status": "ok"}

    # Finally make sure this user is accepted by the backend
    async with connect_backend(backend, auth_as=mallory) as sock:
        await sock.send({"cmd": "user_get", "user_id": "mallory"})
        rep = await sock.recv()
    assert rep == {
        "status": "ok",
        "user_id": "mallory",
        "created_by": "alice@test",
        "created_on": "2017-07-07T00:59:00+00:00",
        "broadcast_key": "KDXG2SYSdeTl+EBvZwPpsOfRkhEVVimOwH9hB459QWg=\n",
        "devices": {
            "test": {
                "created_on": "2017-07-07T00:59:00+00:00",
                "revocated_on": None,
                "verify_key": "YH9SOadvA66vf4GZn7wCbKVQ5RsiSkxW2deBeFVuzco=\n",
            }
        },
    }
