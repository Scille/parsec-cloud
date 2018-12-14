from uuid import UUID
import pytest

from parsec.api.protocole import beacon_read_serializer

from tests.backend.test_events import events_subscribe, events_listen_nowait


BEACON_ID_1 = UUID("093393b2362042799652b05ee630070a")
BEACON_ID_2 = UUID("7f3f1dadab6d44978a72ad20620953d3")
BEACON_ID_3 = UUID("773c9971085d42d0bbcbf6fc186c445a")
VLOB_ID = UUID("fc0dfa885c6c4d3781341e10bf94b080")
VLOB_RTS = "<rts>"
VLOB_WTS = "<wts>"


@pytest.fixture
async def vlob_ids(backend):
    ids = (
        UUID("fc0dfa885c6c4d3781341e10bf94b080"),
        UUID("7a0efe58bad146df861a53207c550860"),
        UUID("af20bbfcc3294b96bb536fe65efc86b4"),
    )
    for id in ids:
        await backend.vlob.create(id, "<rts>", "<wts>", b"")
    return ids


async def beacon_read(sock, id, offset):
    raw_rep = await sock.send(
        beacon_read_serializer.req_dump({"cmd": "beacon_read", "id": id, "offset": offset})
    )
    raw_rep = await sock.recv()
    return beacon_read_serializer.rep_load(raw_rep)


@pytest.mark.trio
async def test_beacon_read_any(alice_backend_sock):
    rep = await beacon_read(alice_backend_sock, BEACON_ID_1, 0)
    assert rep == {"status": "ok", "items": []}


@pytest.mark.trio
async def test_beacon_multimessages(backend, alice_backend_sock, vlob_ids):
    await backend.beacon.update(BEACON_ID_1, src_id=vlob_ids[0], src_version=1, author="alice")
    await backend.beacon.update(BEACON_ID_1, src_id=vlob_ids[1], src_version=2, author="bob")
    await backend.beacon.update(BEACON_ID_1, src_id=vlob_ids[2], src_version=3, author="bob")

    rep = await beacon_read(alice_backend_sock, BEACON_ID_1, 0)
    assert rep == {
        "status": "ok",
        "items": [
            {"src_id": vlob_ids[0], "src_version": 1},
            {"src_id": vlob_ids[1], "src_version": 2},
            {"src_id": vlob_ids[2], "src_version": 3},
        ],
    }

    # Also test offset
    rep = await beacon_read(alice_backend_sock, BEACON_ID_1, 2)
    assert rep == {"status": "ok", "items": [{"src_id": vlob_ids[2], "src_version": 3}]}


@pytest.mark.trio
async def test_beacon_in_vlob_update(backend, alice_backend_sock, alice2):
    await backend.vlob.create(VLOB_ID, VLOB_RTS, VLOB_WTS, blob=b"foo")
    await backend.vlob.update(
        VLOB_ID,
        VLOB_WTS,
        version=2,
        blob=b"bar",
        notify_beacon=BEACON_ID_1,
        author=alice2.device_id,
    )

    rep = await beacon_read(alice_backend_sock, BEACON_ID_1, 0)
    assert rep == {"status": "ok", "items": [{"src_id": VLOB_ID, "src_version": 2}]}


@pytest.mark.trio
async def test_beacon_in_vlob_create(backend, alice_backend_sock, alice2):
    await backend.vlob.create(
        VLOB_ID, VLOB_RTS, VLOB_WTS, b"foo", notify_beacon=BEACON_ID_1, author=alice2.device_id
    )

    rep = await beacon_read(alice_backend_sock, BEACON_ID_1, 0)
    assert rep == {"status": "ok", "items": [{"src_id": VLOB_ID, "src_version": 1}]}


@pytest.mark.trio
async def test_beacon_updated_event(backend, alice_backend_sock, vlob_ids, alice, bob):
    await events_subscribe(alice_backend_sock, beacon_updated=[BEACON_ID_1, BEACON_ID_2])

    # Good events
    await backend.beacon.update(
        BEACON_ID_1, src_id=vlob_ids[0], src_version=1, author=bob.device_id
    )
    await backend.beacon.update(
        BEACON_ID_2, src_id=vlob_ids[1], src_version=2, author=bob.device_id
    )
    await backend.beacon.update(
        BEACON_ID_2, src_id=vlob_ids[1], src_version=3, author=bob.device_id
    )

    reps = [
        await events_listen_nowait(alice_backend_sock),
        await events_listen_nowait(alice_backend_sock),
        await events_listen_nowait(alice_backend_sock),
        await events_listen_nowait(alice_backend_sock),
    ]
    assert reps == [
        {
            "status": "ok",
            "event": "beacon.updated",
            "beacon_id": BEACON_ID_1,
            "index": 1,
            "src_id": vlob_ids[0],
            "src_version": 1,
        },
        {
            "status": "ok",
            "event": "beacon.updated",
            "beacon_id": BEACON_ID_2,
            "index": 1,
            "src_id": vlob_ids[1],
            "src_version": 2,
        },
        {
            "status": "ok",
            "event": "beacon.updated",
            "beacon_id": BEACON_ID_2,
            "index": 2,
            "src_id": vlob_ids[1],
            "src_version": 3,
        },
        {"status": "no_events"},
    ]

    # Ignore self events
    await backend.beacon.update(
        BEACON_ID_1, src_id=vlob_ids[0], src_version=1, author=alice.device_id
    )
    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "no_events"}

    # Beacon id not subscribed to
    await backend.beacon.update(
        BEACON_ID_3, src_id=vlob_ids[0], src_version=1, author=bob.device_id
    )
    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "no_events"}
