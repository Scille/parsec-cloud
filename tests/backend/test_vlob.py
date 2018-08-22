import pytest
from uuid import UUID

from parsec.utils import to_jsonb64


async def populate_backend_vlob(backend):
    await backend.vlob.create(
        UUID("00000000000000000000000000000001"), "<1 rts>", "<1 wts>", b"1 blob v1"
    )
    await backend.vlob.update(UUID("00000000000000000000000000000001"), "<1 wts>", 2, b"1 blob v2")
    await backend.vlob.create(
        UUID("00000000000000000000000000000002"), "<2 rts>", "<2 wts>", b"2 blob v1"
    )


def _get_existing_vlob(backend):
    # Backend must have been populated before that
    id, block = list(backend.test_populate_data["vlobs"].items())[0]
    return id, block["rts"], block["wts"], block["blobs"]


@pytest.mark.trio
async def test_vlob_create_and_read(alice_backend_sock, bob_backend_sock):
    blob = to_jsonb64(b"Initial commit.")

    payload = {
        "id": "0000000000000000000000000000000f",
        "rts": "<123 rts>",
        "wts": "<123 wts>",
        "blob": blob,
    }
    await alice_backend_sock.send({"cmd": "vlob_create", **payload})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok"}

    await bob_backend_sock.send(
        {"cmd": "vlob_read", "id": "0000000000000000000000000000000f", "rts": "<123 rts>"}
    )
    rep = await bob_backend_sock.recv()
    assert rep == {
        "status": "ok",
        "id": "0000000000000000000000000000000f",
        "version": 1,
        "blob": blob,
    }


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": "0000000000000000000000000000000f", "wts": "<123 wts>", "blob": to_jsonb64(b"...")},
        {"id": "0000000000000000000000000000000f", "rts": "<123 rts>", "blob": to_jsonb64(b"...")},
        {
            "id": "0000000000000000000000000000000f",
            "rts": 42,
            "wts": "<123 wts>",
            "blob": to_jsonb64(b"..."),
        },
        {
            "id": "0000000000000000000000000000000f",
            "rts": "<123 rts>",
            "wts": 42,
            "blob": to_jsonb64(b"..."),
        },
        {"rts": "<123 rts>", "wts": "<123 wts>", "blob": to_jsonb64(b"..."), "bad_field": "foo"},
        {"rts": "<123 rts>", "wts": "<123 wts>", "blob": 42},
        {"rts": "<123 rts>", "wts": "<123 wts>", "blob": None},
        {"id": "<not an uuid>", "rts": "<123 rts>", "wts": "<123 wts>", "blob": to_jsonb64(b"...")},
        {"id": 42, "rts": "<123 rts>", "wts": "<123 wts>", "blob": to_jsonb64(b"...")},
    ],
)
@pytest.mark.trio
async def test_vlob_create_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "vlob_create", **bad_msg})
    rep = await alice_backend_sock.recv()
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_vlob_read_not_found(alice_backend_sock):
    await alice_backend_sock.send(
        {"cmd": "vlob_read", "id": "0000000000000000000000000000000f", "rts": "TS4242"}
    )
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "not_found_error", "reason": "Vlob not found."}


@pytest.mark.trio
async def test_vlob_read_ok(backend, alice_backend_sock):
    await populate_backend_vlob(backend)

    await alice_backend_sock.send(
        {"cmd": "vlob_read", "id": "00000000000000000000000000000001", "rts": "<1 rts>"}
    )
    rep = await alice_backend_sock.recv()

    assert rep == {
        "status": "ok",
        "id": "00000000000000000000000000000001",
        "blob": to_jsonb64(b"1 blob v2"),
        "version": 2,
    }


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": "0000000000000000000000000000000f", "rts": "TS4242", "bad_field": "foo"},
        {"id": "<not an uuid>", "rts": "TS4242"},
        {"id": "0000000000000000000000000000000f"},
        {"id": "0000000000000000000000000000000f", "rts": 42},
        {"id": "0000000000000000000000000000000f", "rts": None},
        {"id": 42, "rts": "TS4242"},
        {"id": None, "rts": "TS4242"},
        {},
    ],
)
@pytest.mark.trio
async def test_vlob_read_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "vlob_read", **bad_msg})
    rep = await alice_backend_sock.recv()
    # Id and trust_seed are invalid anyway, but here we test another layer
    # so it's not important as long as we get our `bad_message` status
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_read_bad_version(backend, alice_backend_sock):
    await populate_backend_vlob(backend)

    await alice_backend_sock.send(
        {
            "cmd": "vlob_read",
            "id": "00000000000000000000000000000001",
            "rts": "<1 rts>",
            "version": 3,
        }
    )
    rep = await alice_backend_sock.recv()

    assert rep == {"status": "version_error", "reason": "Wrong blob version."}


@pytest.mark.trio
async def test_vlob_update_ok(backend, alice_backend_sock):
    await populate_backend_vlob(backend)

    await alice_backend_sock.send(
        {
            "cmd": "vlob_update",
            "id": "00000000000000000000000000000001",
            "wts": "<1 wts>",
            "version": 3,
            "blob": to_jsonb64(b"Next version."),
        }
    )
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_vlob_update_not_found(alice_backend_sock):
    await alice_backend_sock.send(
        {
            "cmd": "vlob_update",
            "id": "0000000000000000000000000000000f",
            "wts": "WTS42",
            "version": 2,
            "blob": to_jsonb64(b"Next version."),
        }
    )
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "not_found_error", "reason": "Vlob not found."}


@pytest.mark.parametrize(
    "bad_msg",
    [
        {
            "id": "00000000000000000000000000000001",
            "wts": "WTS42",
            "version": "42",
            "blob": to_jsonb64(b"..."),
            "bad_field": "foo",
        },
        {"id": "0000000000000000000000000000000f", "wts": "WTS42", "version": "42", "blob": None},
        {"id": "0000000000000000000000000000000f", "wts": "WTS42", "version": "42", "blob": 42},
        {"id": "0000000000000000000000000000000f", "wts": "WTS42", "version": "42"},
        {
            "id": "0000000000000000000000000000000f",
            "wts": "WTS42",
            "version": None,
            "blob": to_jsonb64(b"..."),
        },
        {
            "id": "0000000000000000000000000000000f",
            "wts": "WTS42",
            "version": -1,
            "blob": to_jsonb64(b"..."),
        },
        {
            "id": "0000000000000000000000000000000f",
            "wts": None,
            "version": "42",
            "blob": to_jsonb64(b"..."),
        },
        {
            "id": "0000000000000000000000000000000f",
            "wts": 42,
            "version": "42",
            "blob": to_jsonb64(b"..."),
        },
        {"id": "0000000000000000000000000000000f", "version": "42", "blob": to_jsonb64(b"...")},
        {"id": 42, "wts": "WTS42", "version": "42", "blob": to_jsonb64(b"...")},
        {"id": None, "wts": "WTS42", "version": "42", "blob": to_jsonb64(b"...")},
        {"wts": "WTS42", "version": "42", "blob": to_jsonb64(b"...")},
        {},
    ],
)
@pytest.mark.trio
async def test_vlob_update_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "vlob_update", **bad_msg})
    rep = await alice_backend_sock.recv()
    # Id and wts are invalid anyway, but here we test another layer
    # so it's not important as long as we get our `bad_message` status
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_update_bad_version(backend, alice_backend_sock):
    await populate_backend_vlob(backend)

    await alice_backend_sock.send(
        {
            "cmd": "vlob_update",
            "id": "00000000000000000000000000000001",
            "wts": "<1 wts>",
            "version": 4,
            "blob": to_jsonb64(b"Next version."),
        }
    )
    rep = await alice_backend_sock.recv()

    assert rep == {"status": "version_error", "reason": "Wrong blob version."}


@pytest.mark.trio
async def test_update_bad_seed(backend, alice_backend_sock):
    await populate_backend_vlob(backend)

    await alice_backend_sock.send(
        {
            "cmd": "vlob_update",
            "id": "00000000000000000000000000000001",
            "wts": "dummy_seed",
            "version": 3,
            "blob": to_jsonb64(b"Next version."),
        }
    )
    rep = await alice_backend_sock.recv()

    assert rep == {"status": "trust_seed_error", "reason": "Invalid write trust seed."}


@pytest.mark.trio
async def test_group_check(monitor, backend, alice_backend_sock):
    await populate_backend_vlob(backend)

    await alice_backend_sock.send(
        {
            "cmd": "vlob_group_check",
            "to_check": [
                {"id": "0000000000000000000000000000000f", "rts": "<dummy>", "version": 0},
                {"id": "00000000000000000000000000000001", "rts": "<1 rts>", "version": 1},
                {"id": "00000000000000000000000000000002", "rts": "<2 rts>", "version": 1},
            ],
        }
    )
    rep = await alice_backend_sock.recv()

    assert rep == {
        "status": "ok",
        "changed": [
            {"id": "0000000000000000000000000000000f", "version": 0},
            {"id": "00000000000000000000000000000001", "version": 2},
        ],
    }
