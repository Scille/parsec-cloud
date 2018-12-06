import pytest
from unittest.mock import ANY
from pendulum import Pendulum

from parsec.api.protocole import user_get_serializer, user_find_serializer


@pytest.mark.trio
async def test_api_user_get_ok(backend, alice_backend_sock, bob):
    await alice_backend_sock.send(
        user_get_serializer.req_dump({"cmd": "user_get", "user_id": "bob"})
    )
    raw_rep = await alice_backend_sock.recv()
    rep = user_get_serializer.rep_load(raw_rep)
    assert rep == {
        "status": "ok",
        "user_id": bob.user_id,
        "certified_user": ANY,
        "user_certifier": None,
        "created_on": Pendulum(2000, 1, 1),
        "devices": {
            bob.device_name: {
                "device_id": bob.id,
                "created_on": Pendulum(2000, 1, 1),
                "revocated_on": None,
                "certified_revocation": None,
                "revocation_certifier": None,
                "certified_device": ANY,
                "device_certifier": None,
            }
        },
        "trustchain": {},
    }


@pytest.mark.parametrize(
    "bad_msg", [{"user_id": 42}, {"user_id": None}, {"user_id": "alice", "unknown": "field"}, {}]
)
@pytest.mark.trio
async def test_api_user_get_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "user_get", **bad_msg})
    raw_rep = await alice_backend_sock.recv()
    rep = user_get_serializer.rep_load(raw_rep)
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_api_user_get_not_found(alice_backend_sock):
    await alice_backend_sock.send(
        user_get_serializer.req_dump({"cmd": "user_get", "user_id": "dummy"})
    )
    raw_rep = await alice_backend_sock.recv()
    rep = user_get_serializer.rep_load(raw_rep)
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_api_user_find(alice, backend, alice_backend_sock, root_key_certifier):
    # We won't use those keys anyway
    dpk = alice.user_pubkey
    dvk = alice.device_verifykey
    # Populate with cool guys
    for cool_guy, devices_names in [
        ("Philippe", ["p1", "p2"]),
        ("Mike", ["m1"]),
        ("Blacky", []),
        ("Philip_J_Fry", ["pe1"]),
    ]:
        user = root_key_certifier.user_factory(
            cool_guy, dpk, devices=[(d, dvk) for d in devices_names]
        )
        await backend.user.create_user(user)

    # Test exact match
    await alice_backend_sock.send(
        user_find_serializer.req_dump({"cmd": "user_find", "query": "Mike"})
    )
    raw_rep = await alice_backend_sock.recv()
    rep = user_find_serializer.rep_load(raw_rep)
    assert rep == {"status": "ok", "results": ["Mike"], "per_page": 100, "page": 1, "total": 1}

    # Test partial search
    await alice_backend_sock.send(
        user_find_serializer.req_dump({"cmd": "user_find", "query": "Phil"})
    )
    raw_rep = await alice_backend_sock.recv()
    rep = user_find_serializer.rep_load(raw_rep)
    assert rep == {
        "status": "ok",
        "results": ["Philip_J_Fry", "Philippe"],
        "per_page": 100,
        "page": 1,
        "total": 2,
    }

    # Test pagination
    await alice_backend_sock.send(
        user_find_serializer.req_dump(
            {"cmd": "user_find", "page": 1, "per_page": 1, "query": "Phil"}
        )
    )
    raw_rep = await alice_backend_sock.recv()
    rep = user_find_serializer.rep_load(raw_rep)
    assert rep == {
        "status": "ok",
        "results": ["Philip_J_Fry"],
        "per_page": 1,
        "page": 1,
        "total": 2,
    }

    # Test out of pagination
    await alice_backend_sock.send(
        user_find_serializer.req_dump(
            {"cmd": "user_find", "page": 2, "per_page": 5, "query": "Phil"}
        )
    )
    raw_rep = await alice_backend_sock.recv()
    rep = user_find_serializer.rep_load(raw_rep)
    assert rep == {"status": "ok", "results": [], "per_page": 5, "page": 2, "total": 2}

    # Test no params
    await alice_backend_sock.send(user_find_serializer.req_dump({"cmd": "user_find"}))
    raw_rep = await alice_backend_sock.recv()
    rep = user_find_serializer.rep_load(raw_rep)
    assert rep == {
        "status": "ok",
        "results": ["alice", "Blacky", "bob", "Mike", "Philip_J_Fry", "Philippe"],
        "per_page": 100,
        "page": 1,
        "total": 6,
    }

    # Test bad params
    for bad in [{"dummy": 42}, {"query": 42}, {"page": 0}, {"per_page": 0}, {"per_page": 101}]:
        await alice_backend_sock.send({"cmd": "user_find", **bad})
        raw_rep = await alice_backend_sock.recv()
        rep = user_find_serializer.rep_load(raw_rep)
        assert rep["status"] == "bad_message"
