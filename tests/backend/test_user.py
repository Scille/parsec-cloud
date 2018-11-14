import pytest
from unittest.mock import patch

from parsec.utils import to_jsonb64

from tests.common import freeze_time


@pytest.fixture
def mock_generate_token():
    with patch("parsec.backend.user._generate_token") as mock_generate_token:
        yield mock_generate_token


@pytest.mark.trio
async def test_user_get_ok(backend, alice_backend_sock, bob):
    await alice_backend_sock.send({"cmd": "user_get", "user_id": "bob"})
    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "ok",
        "user_id": "bob",
        "broadcast_key": to_jsonb64(bob.user_pubkey.encode()),
        "created_on": "2000-01-01T00:00:00+00:00",
        "devices": {
            bob.device_name: {
                "created_on": "2000-01-01T00:00:00+00:00",
                "revocated_on": None,
                "verify_key": to_jsonb64(bob.device_verifykey.encode()),
            }
        },
    }


@pytest.mark.trio
async def test_user_find(backend, alice_backend_sock):
    # Populate with cool guys
    dk = b"<dummy key>"
    await backend.user.create("Philippe", devices=[("p1", dk), ("p2", dk)], broadcast_key=dk)
    await backend.user.create("Mike", devices=[("m1", dk)], broadcast_key=dk)
    await backend.user.create("Blacky", devices=[], broadcast_key=dk)
    await backend.user.create("Philippe2", devices=[("pe1", dk)], broadcast_key=dk)

    # Test exact match
    await alice_backend_sock.send({"cmd": "user_find", "query": "Mike"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok", "results": ["Mike"], "per_page": 100, "page": 1, "total": 1}

    # Test partial search
    await alice_backend_sock.send({"cmd": "user_find", "query": "Phil"})
    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "ok",
        "results": ["Philippe", "Philippe2"],
        "per_page": 100,
        "page": 1,
        "total": 2,
    }

    # Test pagination
    await alice_backend_sock.send({"cmd": "user_find", "page": 1, "per_page": 1, "query": "Phil"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok", "results": ["Philippe"], "per_page": 1, "page": 1, "total": 2}

    # Test out of pagination
    await alice_backend_sock.send({"cmd": "user_find", "page": 2, "per_page": 5, "query": "Phil"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok", "results": [], "per_page": 5, "page": 2, "total": 2}

    # Test no params
    await alice_backend_sock.send({"cmd": "user_find"})
    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "ok",
        "results": ["alice", "Blacky", "bob", "Mike", "Philippe", "Philippe2"],
        "per_page": 100,
        "page": 1,
        "total": 6,
    }

    # Test bad params
    for bad in [{"dummy": 42}, {"query": 42}, {"page": 0}, {"per_page": 0}, {"per_page": 101}]:
        await alice_backend_sock.send({"cmd": "user_find", **bad})
        rep = await alice_backend_sock.recv()
        assert rep["status"] == "bad_message"


@pytest.mark.parametrize(
    "bad_msg", [{"user_id": 42}, {"user_id": None}, {"user_id": "alice", "unknown": "field"}, {}]
)
@pytest.mark.trio
async def test_user_get_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "user_get", **bad_msg})
    rep = await alice_backend_sock.recv()
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_user_get_not_found(alice_backend_sock):
    await alice_backend_sock.send({"cmd": "user_get", "user_id": "dummy"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "not_found", "reason": "No user with id `dummy`."}


@pytest.mark.trio
async def test_user_invite(alice_backend_sock, mock_generate_token):
    mock_generate_token.side_effect = ["<token>"]
    await alice_backend_sock.send({"cmd": "user_invite", "user_id": "john"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok", "user_id": "john", "invitation_token": "<token>"}


@pytest.mark.trio
async def test_user_claim_unknown_token(anonymous_backend_sock, mallory):
    await anonymous_backend_sock.send(
        {
            "cmd": "user_claim",
            "user_id": mallory.user_id,
            "invitation_token": "<token>",
            "broadcast_key": to_jsonb64(mallory.user_pubkey.encode()),
            "device_name": mallory.device_name,
            "device_verify_key": to_jsonb64(mallory.device_verifykey.encode()),
        }
    )
    rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "not_found_error", "reason": "No invitation for user `mallory`"}


@pytest.fixture
async def invitation_token(backend, mallory):
    token = "1234567890"
    with freeze_time("2017-07-07T00:00:00"):
        await backend.user.create_invitation(token, mallory.user_id)
    return token


@pytest.mark.trio
async def test_user_claim_too_old_token(anonymous_backend_sock, invitation_token, mallory):
    with freeze_time("2017-07-07T01:01:00"):
        await anonymous_backend_sock.send(
            {
                "cmd": "user_claim",
                "user_id": mallory.user_id,
                "invitation_token": invitation_token,
                "broadcast_key": to_jsonb64(mallory.user_pubkey.encode()),
                "device_name": mallory.device_name,
                "device_verify_key": to_jsonb64(mallory.device_verifykey.encode()),
            }
        )
        rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "out_of_date_error", "reason": "Claim code is too old."}


@pytest.mark.trio
async def test_user_claim_token(
    backend, alice, invitation_token, mallory, anonymous_backend_sock, backend_sock_factory
):
    with freeze_time("2017-07-07T00:59:00"):
        await anonymous_backend_sock.send(
            {
                "cmd": "user_claim",
                "user_id": mallory.user_id,
                "invitation_token": invitation_token,
                "broadcast_key": to_jsonb64(mallory.user_pubkey.encode()),
                "device_name": mallory.device_name,
                "device_verify_key": to_jsonb64(mallory.device_verifykey.encode()),
            }
        )
        rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "ok"}

    # Finally make sure this user is accepted by the backend
    async with backend_sock_factory(backend, mallory) as mallory_sock:
        await mallory_sock.send({"cmd": "user_get", "user_id": mallory.user_id})
        rep = await mallory_sock.recv()
        assert rep == {
            "status": "ok",
            "user_id": "mallory",
            "created_on": "2017-07-07T00:59:00+00:00",
            "broadcast_key": to_jsonb64(mallory.user_pubkey.encode()),
            "devices": {
                mallory.device_name: {
                    "created_on": "2017-07-07T00:59:00+00:00",
                    "revocated_on": None,
                    "verify_key": to_jsonb64(mallory.device_verifykey.encode()),
                }
            },
        }
