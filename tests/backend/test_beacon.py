import pytest


@pytest.mark.trio
async def test_beacon_read_any(alice_backend_sock):
    await alice_backend_sock.send({"cmd": "beacon_read", "id": "123", "offset": 0})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok", "id": "123", "offset": 0, "items": [], "count": 0}


@pytest.mark.trio
async def test_beacon_multimessages(backend, alice_backend_sock):
    await backend.beacon.update("1", src_id="a1", src_version=1, author="alice")
    await backend.beacon.update("1", src_id="b2", src_version=2, author="bob")
    await backend.beacon.update("1", src_id="b3", src_version=3, author="bob")

    await alice_backend_sock.send({"cmd": "beacon_read", "id": "1", "offset": 0})
    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "ok",
        "id": "1",
        "offset": 0,
        "items": [
            {"src_id": "a1", "src_version": 1},
            {"src_id": "b2", "src_version": 2},
            {"src_id": "b3", "src_version": 3},
        ],
        "count": 3,
    }

    # Also test offset
    await alice_backend_sock.send({"cmd": "beacon_read", "id": "1", "offset": 2})
    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "ok",
        "id": "1",
        "offset": 2,
        "items": [{"src_id": "b3", "src_version": 3}],
        "count": 1,
    }


@pytest.mark.trio
async def test_beacon_in_vlob_update(backend, alice_backend_sock):
    beacons_ids = ["123", "456", "789"]

    await backend.vlob.create("1", "<1 rts>", "<1 wts>", blob=b"foo")
    await backend.vlob.update("1", "<1 wts>", version=2, blob=b"bar", notify_beacons=beacons_ids)

    for beacon_id in beacons_ids:
        await alice_backend_sock.send({"cmd": "beacon_read", "id": beacon_id, "offset": 0})
        rep = await alice_backend_sock.recv()
        assert rep == {
            "status": "ok",
            "id": beacon_id,
            "offset": 0,
            "items": [{"src_id": "1", "src_version": 2}],
            "count": 1,
        }


@pytest.mark.trio
async def test_beacon_in_vlob_create(backend, alice_backend_sock):
    beacons_ids = ["123", "456", "789"]

    await backend.vlob.create("1", "<1 rts>", "<1 wts>", b"foo", notify_beacons=beacons_ids)

    for beacon_id in beacons_ids:
        await alice_backend_sock.send({"cmd": "beacon_read", "id": beacon_id, "offset": 0})
        rep = await alice_backend_sock.recv()
        assert rep == {
            "status": "ok",
            "id": beacon_id,
            "offset": 0,
            "items": [{"src_id": "1", "src_version": 1}],
            "count": 1,
        }
