# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from uuid import UUID

from parsec.api.protocole import (
    packb,
    vlob_group_check_serializer,
    vlob_create_serializer,
    vlob_read_serializer,
    vlob_update_serializer,
)


VLOB_ID = UUID("00000000000000000000000000000001")
GROUP_ID = UUID("00000000000000000000000000000002")
OTHER_VLOB_ID = UUID("00000000000000000000000000000003")


async def vlob_group_check(sock, to_check):
    await sock.send(
        vlob_group_check_serializer.req_dumps({"cmd": "vlob_group_check", "to_check": to_check})
    )
    raw_rep = await sock.recv()
    return vlob_group_check_serializer.rep_loads(raw_rep)


async def vlob_create(sock, group, id, blob, check_rep=True):
    await sock.send(
        vlob_create_serializer.req_dumps(
            {"cmd": "vlob_create", "group": group, "id": id, "blob": blob}
        )
    )
    raw_rep = await sock.recv()
    rep = vlob_create_serializer.rep_loads(raw_rep)
    if check_rep:
        assert rep == {"status": "ok"}
    return rep


async def vlob_read(sock, id, version=None):
    await sock.send(
        vlob_read_serializer.req_dumps({"cmd": "vlob_read", "id": id, "version": version})
    )
    raw_rep = await sock.recv()
    return vlob_read_serializer.rep_loads(raw_rep)


async def vlob_update(sock, id, version, blob, check_rep=True):
    await sock.send(
        vlob_update_serializer.req_dumps(
            {"cmd": "vlob_update", "id": id, "version": version, "blob": blob}
        )
    )
    raw_rep = await sock.recv()
    rep = vlob_update_serializer.rep_loads(raw_rep)
    if check_rep:
        assert rep == {"status": "ok"}
    return rep


@pytest.fixture
async def vlobs(backend, alice):
    ids = (UUID("00000000000000000000000000000001"), UUID("00000000000000000000000000000002"))
    await backend.vlob.create(
        alice.organization_id, alice.device_id, ids[0], GROUP_ID, b"1 blob v1"
    )
    await backend.vlob.update(alice.organization_id, alice.device_id, ids[0], 2, b"1 blob v2")
    await backend.vlob.create(
        alice.organization_id, alice.device_id, ids[1], GROUP_ID, b"2 blob v1"
    )
    return ids


@pytest.mark.trio
async def test_vlob_create_and_read(alice_backend_sock, alice2_backend_sock):
    blob = b"Initial commit."
    await vlob_create(alice_backend_sock, GROUP_ID, VLOB_ID, blob)

    rep = await vlob_read(alice2_backend_sock, VLOB_ID)
    assert rep == {"status": "ok", "version": 1, "blob": blob}


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"blob": b"...", "bad_field": "foo"},
        {"blob": 42},
        {"blob": None},
        {"id": "<not an uuid>", "blob": b"..."},
        {"id": 42, "blob": b"..."},
    ],
)
@pytest.mark.trio
async def test_vlob_create_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send(packb({"cmd": "vlob_create", **bad_msg}))
    raw_rep = await alice_backend_sock.recv()
    rep = vlob_create_serializer.rep_loads(raw_rep)
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_vlob_create_but_already_exists(alice_backend_sock):
    blob = b"Initial commit."

    await vlob_create(alice_backend_sock, GROUP_ID, VLOB_ID, blob)

    rep = await vlob_create(alice_backend_sock, GROUP_ID, VLOB_ID, blob, check_rep=False)
    assert rep["status"] == "already_exists"


@pytest.mark.trio
async def test_vlob_create_not_allowed_by_group(alice_backend_sock, bob_backend_sock):
    blob = b"Initial commit."

    await vlob_create(alice_backend_sock, GROUP_ID, VLOB_ID, blob)

    rep = await vlob_create(bob_backend_sock, GROUP_ID, OTHER_VLOB_ID, blob, check_rep=False)
    assert rep["status"] == "not_allowed"


@pytest.mark.trio
async def test_vlob_read_not_found(alice_backend_sock):
    rep = await vlob_read(alice_backend_sock, VLOB_ID)
    assert rep == {
        "status": "not_found",
        "reason": "Vlob `00000000-0000-0000-0000-000000000001` doesn't exist",
    }


@pytest.mark.trio
async def test_vlob_read_ok(alice_backend_sock, vlobs):
    rep = await vlob_read(alice_backend_sock, vlobs[0])
    assert rep == {"status": "ok", "blob": b"1 blob v2", "version": 2}


@pytest.mark.trio
async def test_vlob_read_not_allowed_by_group(bob_backend_sock, vlobs):
    rep = await vlob_read(bob_backend_sock, vlobs[0])
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
async def test_vlob_read_other_organization(backend, sock_from_other_organization_factory, vlobs):
    async with sock_from_other_organization_factory(backend) as sock:
        rep = await vlob_read(sock, vlobs[0])
    assert rep == {
        "status": "not_found",
        "reason": "Vlob `00000000-0000-0000-0000-000000000001` doesn't exist",
    }


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": str(VLOB_ID), "bad_field": "foo"},
        {"id": "<not an uuid>"},
        {"id": str(VLOB_ID)},  # TODO: really bad ?
        {"id": 42},
        {"id": None},
        {"id": str(VLOB_ID), "version": 0},
        {"id": str(VLOB_ID), "version": "foo"},
        {},
    ],
)
@pytest.mark.trio
async def test_vlob_read_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send(packb({"cmd": "vlob_read", **bad_msg}))
    raw_rep = await alice_backend_sock.recv()
    rep = vlob_read_serializer.rep_loads(raw_rep)
    # Id and trust_seed are invalid anyway, but here we test another layer
    # so it's not important as long as we get our `bad_message` status
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_read_bad_version(alice_backend_sock, vlobs):
    rep = await vlob_read(alice_backend_sock, vlobs[0], version=3)
    assert rep == {"status": "bad_version"}


@pytest.mark.trio
async def test_vlob_update_ok(alice_backend_sock, vlobs):
    await vlob_update(alice_backend_sock, vlobs[0], version=3, blob=b"Next version.")


@pytest.mark.trio
async def test_vlob_update_not_allowed_by_group(bob_backend_sock, vlobs):
    rep = await vlob_update(
        bob_backend_sock, vlobs[0], version=3, blob=b"Next version.", check_rep=False
    )
    assert rep == {"status": "not_allowed"}


@pytest.mark.trio
async def test_vlob_update_not_found(alice_backend_sock):
    rep = await vlob_update(
        alice_backend_sock, VLOB_ID, version=2, blob=b"Next version.", check_rep=False
    )
    assert rep == {
        "status": "not_found",
        "reason": "Vlob `00000000-0000-0000-0000-000000000001` doesn't exist",
    }


@pytest.mark.trio
async def test_vlob_update_other_organization(backend, sock_from_other_organization_factory, vlobs):
    async with sock_from_other_organization_factory(backend) as sock:
        rep = await vlob_update(sock, vlobs[0], version=3, blob=b"Next version.", check_rep=False)
    assert rep == {
        "status": "not_found",
        "reason": "Vlob `00000000-0000-0000-0000-000000000001` doesn't exist",
    }


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": str(VLOB_ID), "version": 42, "blob": b"...", "bad_field": "foo"},
        {"id": str(VLOB_ID), "version": 42, "blob": None},
        {"id": str(VLOB_ID), "version": 42, "blob": 42},
        {"id": str(VLOB_ID), "version": 42},
        {"id": str(VLOB_ID), "version": None, "blob": b"..."},
        {"id": str(VLOB_ID), "version": -1, "blob": b"..."},
        {"id": 42, "version": 42, "blob": b"..."},
        {"id": None, "version": 42, "blob": b"..."},
        {"version": 42, "blob": b"..."},
        {},
    ],
)
@pytest.mark.trio
async def test_vlob_update_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send(packb({"cmd": "vlob_update", **bad_msg}))
    raw_rep = await alice_backend_sock.recv()
    rep = vlob_update_serializer.rep_loads(raw_rep)
    # Id and version are invalid anyway, but here we test another layer
    # so it's not important as long as we get our `bad_message` status
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_update_bad_version(alice_backend_sock, vlobs):
    rep = await vlob_update(
        alice_backend_sock, vlobs[0], version=4, blob=b"Next version.", check_rep=False
    )
    assert rep == {"status": "bad_version"}


@pytest.mark.trio
async def test_group_check(bob_backend_sock, alice_backend_sock, vlobs):
    unknown_vlob_id = UUID("0000000000000000000000000000000A")
    placeholder_vlob_id = UUID("0000000000000000000000000000000B")
    bob_vlob_id = UUID("0000000000000000000000000000000C")
    bob_group_id = UUID("0000000000000000000000000000000D")

    await vlob_create(bob_backend_sock, bob_group_id, bob_vlob_id, b"")
    await vlob_update(bob_backend_sock, bob_vlob_id, 2, b"")

    rep = await vlob_group_check(
        alice_backend_sock,
        [
            # Ignore vlob with no read access
            {"id": bob_vlob_id, "version": 1},
            # Ignore unknown id
            {"id": unknown_vlob_id, "version": 1},
            # Version 0 is accepted
            {"id": placeholder_vlob_id, "version": 0},
            {"id": vlobs[0], "version": 1},
            {"id": vlobs[1], "version": 1},
        ],
    )
    assert rep == {
        "status": "ok",
        "changed": [{"id": placeholder_vlob_id, "version": 0}, {"id": vlobs[0], "version": 2}],
    }


@pytest.mark.trio
async def test_vlob_group_check_other_organization(
    backend, sock_from_other_organization_factory, vlobs
):
    async with sock_from_other_organization_factory(backend) as sock:
        rep = await vlob_group_check(sock, [{"id": vlobs[0], "version": 1}])
        assert rep == {"status": "ok", "changed": []}
