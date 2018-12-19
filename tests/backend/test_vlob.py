import pytest
from uuid import uuid4, UUID
from collections import namedtuple

from parsec.api.protocole import (
    vlob_group_check_serializer,
    vlob_create_serializer,
    vlob_read_serializer,
    vlob_update_serializer,
)


VLOB_ID = uuid4()
VLOB_RTS = "<rts>"
VLOB_WTS = "<wts>"


async def vlob_group_check(sock, to_check):
    await sock.send(
        vlob_group_check_serializer.req_dump({"cmd": "vlob_group_check", "to_check": to_check})
    )
    raw_rep = await sock.recv()
    return vlob_group_check_serializer.rep_load(raw_rep)


async def vlob_create(sock, id, rts, wts, blob, check_rep=True):
    await sock.send(
        vlob_create_serializer.req_dump(
            {"cmd": "vlob_create", "id": id, "rts": rts, "wts": wts, "blob": blob}
        )
    )
    raw_rep = await sock.recv()
    rep = vlob_create_serializer.rep_load(raw_rep)
    if check_rep:
        assert rep == {"status": "ok"}
    return rep


async def vlob_read(sock, id, rts, version=None):
    await sock.send(
        vlob_read_serializer.req_dump(
            {"cmd": "vlob_read", "id": id, "rts": rts, "version": version}
        )
    )
    raw_rep = await sock.recv()
    return vlob_read_serializer.rep_load(raw_rep)


async def vlob_update(sock, id, wts, version, blob, check_rep=True):
    await sock.send(
        vlob_update_serializer.req_dump(
            {"cmd": "vlob_update", "id": id, "wts": wts, "version": version, "blob": blob}
        )
    )
    raw_rep = await sock.recv()
    rep = vlob_update_serializer.rep_load(raw_rep)
    if check_rep:
        assert rep == {"status": "ok"}
    return rep


@pytest.fixture
async def vlobs(backend, alice):
    Access = namedtuple("Access", "id,rts,wts")
    accesses = (
        Access(UUID("00000000000000000000000000000001"), "<1 rts>", "<1 wts>"),
        Access(UUID("00000000000000000000000000000002"), "<2 rts>", "<2 wts>"),
    )
    await backend.vlob.create(*accesses[0], b"1 blob v1", author=alice.device_id)
    await backend.vlob.update(
        accesses[0].id, accesses[0].wts, 2, b"1 blob v2", author=alice.device_id
    )
    await backend.vlob.create(*accesses[1], b"2 blob v1", author=alice.device_id)
    return accesses


def _get_existing_vlob(backend):
    # Backend must have been populated before that
    id, block = list(backend.test_populate_data["vlobs"].items())[0]
    return id, block["rts"], block["wts"], block["blobs"]


@pytest.mark.trio
async def test_vlob_create_and_read(alice_backend_sock, bob_backend_sock):
    blob = b"Initial commit."
    await vlob_create(alice_backend_sock, VLOB_ID, VLOB_RTS, VLOB_WTS, blob)

    rep = await vlob_read(bob_backend_sock, VLOB_ID, VLOB_RTS)
    assert rep == {"status": "ok", "version": 1, "blob": blob}


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": VLOB_ID, "wts": VLOB_WTS, "blob": b"..."},
        {"id": VLOB_ID, "rts": VLOB_RTS, "blob": b"..."},
        {"id": VLOB_ID, "rts": 42, "wts": VLOB_WTS, "blob": b"..."},
        {"id": VLOB_ID, "rts": VLOB_RTS, "wts": 42, "blob": b"..."},
        {"rts": VLOB_RTS, "wts": VLOB_WTS, "blob": b"...", "bad_field": "foo"},
        {"rts": VLOB_RTS, "wts": VLOB_WTS, "blob": 42},
        {"rts": VLOB_RTS, "wts": VLOB_WTS, "blob": None},
        {"id": "<not an uuid>", "rts": VLOB_RTS, "wts": VLOB_WTS, "blob": b"..."},
        {"id": 42, "rts": VLOB_RTS, "wts": VLOB_WTS, "blob": b"..."},
    ],
)
@pytest.mark.trio
async def test_vlob_create_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "vlob_create", **bad_msg})
    raw_rep = await alice_backend_sock.recv()
    rep = vlob_create_serializer.rep_load(raw_rep)
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_vlob_create_but_already_exists(alice_backend_sock):
    blob = b"Initial commit."

    await vlob_create(alice_backend_sock, VLOB_ID, VLOB_RTS, VLOB_WTS, blob)

    rep = await vlob_create(alice_backend_sock, VLOB_ID, VLOB_RTS, VLOB_WTS, blob, check_rep=False)
    assert rep["status"] == "already_exists"


@pytest.mark.trio
async def test_vlob_read_not_found(alice_backend_sock):
    rep = await vlob_read(alice_backend_sock, VLOB_ID, VLOB_RTS)
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_vlob_read_ok(alice_backend_sock, vlobs):
    rep = await vlob_read(alice_backend_sock, vlobs[0].id, vlobs[0].rts)
    assert rep == {"status": "ok", "blob": b"1 blob v2", "version": 2}


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": VLOB_ID, "rts": VLOB_RTS, "bad_field": "foo"},
        {"id": "<not an uuid>", "rts": VLOB_RTS},
        {"id": VLOB_ID},
        {"id": VLOB_ID, "rts": 42},
        {"id": VLOB_ID, "rts": None},
        {"id": 42, "rts": VLOB_RTS},
        {"id": None, "rts": VLOB_RTS},
        {"id": VLOB_ID, "rts": VLOB_RTS, "version": 0},
        {"id": VLOB_ID, "rts": VLOB_RTS, "version": "foo"},
        {},
    ],
)
@pytest.mark.trio
async def test_vlob_read_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "vlob_read", **bad_msg})
    raw_rep = await alice_backend_sock.recv()
    rep = vlob_read_serializer.rep_load(raw_rep)
    # Id and trust_seed are invalid anyway, but here we test another layer
    # so it's not important as long as we get our `bad_message` status
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_read_bad_version(alice_backend_sock, vlobs):
    rep = await vlob_read(alice_backend_sock, vlobs[0].id, vlobs[0].rts, version=3)
    assert rep == {"status": "bad_version"}


@pytest.mark.trio
async def test_vlob_update_ok(alice_backend_sock, vlobs):
    await vlob_update(
        alice_backend_sock, vlobs[0].id, vlobs[0].wts, version=3, blob=b"Next version."
    )


@pytest.mark.trio
async def test_vlob_update_not_found(alice_backend_sock):
    rep = await vlob_update(
        alice_backend_sock, VLOB_ID, VLOB_WTS, version=2, blob=b"Next version.", check_rep=False
    )
    assert rep == {"status": "not_found"}


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": VLOB_ID, "wts": VLOB_WTS, "version": "42", "blob": b"...", "bad_field": "foo"},
        {"id": VLOB_ID, "wts": VLOB_WTS, "version": "42", "blob": None},
        {"id": VLOB_ID, "wts": VLOB_WTS, "version": "42", "blob": 42},
        {"id": VLOB_ID, "wts": VLOB_WTS, "version": "42"},
        {"id": VLOB_ID, "wts": VLOB_WTS, "version": None, "blob": b"..."},
        {"id": VLOB_ID, "wts": VLOB_WTS, "version": -1, "blob": b"..."},
        {"id": VLOB_ID, "wts": None, "version": "42", "blob": b"..."},
        {"id": VLOB_ID, "wts": 42, "version": "42", "blob": b"..."},
        {"id": VLOB_ID, "version": "42", "blob": b"..."},
        {"id": 42, "wts": VLOB_WTS, "version": "42", "blob": b"..."},
        {"id": None, "wts": VLOB_WTS, "version": "42", "blob": b"..."},
        {"wts": VLOB_WTS, "version": "42", "blob": b"..."},
        {},
    ],
)
@pytest.mark.trio
async def test_vlob_update_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "vlob_update", **bad_msg})
    raw_rep = await alice_backend_sock.recv()
    rep = vlob_update_serializer.rep_load(raw_rep)
    # Id and wts are invalid anyway, but here we test another layer
    # so it's not important as long as we get our `bad_message` status
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_update_bad_version(alice_backend_sock, vlobs):
    rep = await vlob_update(
        alice_backend_sock,
        vlobs[0].id,
        vlobs[0].wts,
        version=4,
        blob=b"Next version.",
        check_rep=False,
    )
    assert rep == {"status": "bad_version"}


@pytest.mark.trio
async def test_update_bad_seed(alice_backend_sock, vlobs):
    rep = await vlob_update(
        alice_backend_sock, vlobs[0].id, VLOB_WTS, version=3, blob=b"Next version.", check_rep=False
    )
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_group_check(alice_backend_sock, vlobs):
    rep = await vlob_group_check(
        alice_backend_sock,
        [
            # Ignore unknown id/rts
            {"id": VLOB_ID, "rts": VLOB_RTS, "version": 1},
            # Version 0 is accepted
            {"id": VLOB_ID, "rts": VLOB_RTS, "version": 0},
            {"id": vlobs[0].id, "rts": vlobs[0].rts, "version": 1},
            {"id": vlobs[1].id, "rts": vlobs[1].rts, "version": 1},
        ],
    )
    assert rep == {
        "status": "ok",
        "changed": [{"id": VLOB_ID, "version": 0}, {"id": vlobs[0].id, "version": 2}],
    }
