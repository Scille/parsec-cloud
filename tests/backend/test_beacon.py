from uuid import UUID
import pytest


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


@pytest.mark.trio
async def test_beacon_read_any(alice_backend_sock):
    await alice_backend_sock.send({"cmd": "beacon_read", "id": BEACON_ID_1, "offset": 0})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok", "id": BEACON_ID_1.hex, "offset": 0, "items": [], "count": 0}


@pytest.mark.trio
async def test_beacon_multimessages(backend, alice_backend_sock, vlob_ids):
    await backend.beacon.update(BEACON_ID_1, src_id=vlob_ids[0], src_version=1, author="alice")
    await backend.beacon.update(BEACON_ID_1, src_id=vlob_ids[1], src_version=2, author="bob")
    await backend.beacon.update(BEACON_ID_1, src_id=vlob_ids[2], src_version=3, author="bob")

    await alice_backend_sock.send({"cmd": "beacon_read", "id": BEACON_ID_1, "offset": 0})
    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "ok",
        "id": BEACON_ID_1.hex,
        "offset": 0,
        "items": [
            {"src_id": vlob_ids[0].hex, "src_version": 1},
            {"src_id": vlob_ids[1].hex, "src_version": 2},
            {"src_id": vlob_ids[2].hex, "src_version": 3},
        ],
        "count": 3,
    }

    # Also test offset
    await alice_backend_sock.send({"cmd": "beacon_read", "id": BEACON_ID_1, "offset": 2})
    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "ok",
        "id": BEACON_ID_1.hex,
        "offset": 2,
        "items": [{"src_id": vlob_ids[2].hex, "src_version": 3}],
        "count": 1,
    }


@pytest.mark.trio
async def test_beacon_in_vlob_update(backend, alice_backend_sock):
    vlob_id = UUID("fc0dfa885c6c4d3781341e10bf94b080")
    beacons_ids = (BEACON_ID_1, BEACON_ID_2, BEACON_ID_3)

    await backend.vlob.create(vlob_id, "<1 rts>", "<1 wts>", blob=b"foo")
    await backend.vlob.update(
        vlob_id, "<1 wts>", version=2, blob=b"bar", notify_beacons=beacons_ids
    )

    for beacon_id in beacons_ids:
        await alice_backend_sock.send({"cmd": "beacon_read", "id": beacon_id, "offset": 0})
        rep = await alice_backend_sock.recv()
        assert rep == {
            "status": "ok",
            "id": beacon_id.hex,
            "offset": 0,
            "items": [{"src_id": vlob_id.hex, "src_version": 2}],
            "count": 1,
        }


@pytest.mark.trio
async def test_beacon_in_vlob_create(backend, alice_backend_sock):
    vlob_id = UUID("fc0dfa885c6c4d3781341e10bf94b080")
    beacons_ids = (BEACON_ID_1, BEACON_ID_2, BEACON_ID_3)

    await backend.vlob.create(vlob_id, "<1 rts>", "<1 wts>", b"foo", notify_beacons=beacons_ids)

    for beacon_id in beacons_ids:
        await alice_backend_sock.send({"cmd": "beacon_read", "id": beacon_id, "offset": 0})
        rep = await alice_backend_sock.recv()
        assert rep == {
            "status": "ok",
            "id": beacon_id.hex,
            "offset": 0,
            "items": [{"src_id": vlob_id.hex, "src_version": 1}],
            "count": 1,
        }
