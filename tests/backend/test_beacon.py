from uuid import UUID
import pytest

from parsec.api.protocole import beacon_read_serializer


BEACON_ID_1 = UUID("093393b2362042799652b05ee630070a")
BEACON_ID_2 = UUID("7f3f1dadab6d44978a72ad20620953d3")
BEACON_ID_3 = UUID("773c9971085d42d0bbcbf6fc186c445a")


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


async def send(sock, id, offset):
    raw_rep = await sock.send(
        beacon_read_serializer.req_dump({"cmd": "beacon_read", "id": id, "offset": offset})
    )
    raw_rep = await sock.recv()
    return beacon_read_serializer.rep_load(raw_rep)


@pytest.mark.trio
async def test_beacon_read_any(alice_backend_sock):
    rep = await send(alice_backend_sock, BEACON_ID_1, 0)
    assert rep == {"status": "ok", "items": []}


@pytest.mark.trio
async def test_beacon_multimessages(backend, alice_backend_sock, vlob_ids):
    await backend.beacon.update(BEACON_ID_1, src_id=vlob_ids[0], src_version=1, author="alice")
    await backend.beacon.update(BEACON_ID_1, src_id=vlob_ids[1], src_version=2, author="bob")
    await backend.beacon.update(BEACON_ID_1, src_id=vlob_ids[2], src_version=3, author="bob")

    rep = await send(alice_backend_sock, BEACON_ID_1, 0)
    assert rep == {
        "status": "ok",
        "items": [
            {"src_id": vlob_ids[0], "src_version": 1},
            {"src_id": vlob_ids[1], "src_version": 2},
            {"src_id": vlob_ids[2], "src_version": 3},
        ],
    }

    # Also test offset
    rep = await send(alice_backend_sock, BEACON_ID_1, 2)
    assert rep == {"status": "ok", "items": [{"src_id": vlob_ids[2], "src_version": 3}]}


@pytest.mark.trio
async def test_beacon_in_vlob_update(backend, alice_backend_sock, alice2):
    vlob_id = UUID("fc0dfa885c6c4d3781341e10bf94b080")

    await backend.vlob.create(vlob_id, "<1 rts>", "<1 wts>", blob=b"foo")
    await backend.vlob.update(
        vlob_id,
        "<1 wts>",
        version=2,
        blob=b"bar",
        notify_beacon=BEACON_ID_1,
        author=alice2.device_id,
    )

    rep = await send(alice_backend_sock, BEACON_ID_1, 0)
    assert rep == {"status": "ok", "items": [{"src_id": vlob_id, "src_version": 2}]}


@pytest.mark.trio
async def test_beacon_in_vlob_create(backend, alice_backend_sock, alice2):
    vlob_id = UUID("fc0dfa885c6c4d3781341e10bf94b080")

    await backend.vlob.create(
        vlob_id, "<1 rts>", "<1 wts>", b"foo", notify_beacon=BEACON_ID_1, author=alice2.device_id
    )

    rep = await send(alice_backend_sock, BEACON_ID_1, 0)
    assert rep == {"status": "ok", "items": [{"src_id": vlob_id, "src_version": 1}]}
