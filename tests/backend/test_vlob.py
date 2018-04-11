import pytest

from parsec.utils import to_jsonb64

from tests.common import connect_backend


async def populate_backend_vlob(backend):
    await backend.vlob.create("1", "<1 rts>", "<1 wts>", b"1 blob v1")
    await backend.vlob.update("1", "<1 wts>", 2, b"1 blob v2")
    await backend.vlob.create("2", "<2 rts>", "<2 wts>", b"2 blob v1")


def _get_existing_vlob(backend):
    # Backend must have been populated before that
    id, block = list(backend.test_populate_data["vlobs"].items())[0]
    return id, block["rts"], block["wts"], block["blobs"]


@pytest.mark.parametrize(
    "id,blob",
    [
        (None, None),
        (None, b"Initial commit."),
        ("foo", None),
        ("bar", b"Initial commit."),
    ],
    ids=lambda x: "id=%s, blob=%s" % x,
)
@pytest.mark.trio
async def test_vlob_create_and_read(backend, alice, id, blob):
    async with connect_backend(backend, auth_as=alice) as sock:
        payload = {}
        if id:
            payload["id"] = id
        if blob:
            payload["blob"] = to_jsonb64(blob)
        await sock.send({"cmd": "vlob_create", **payload})
        rep = await sock.recv()
        assert rep["status"] == "ok"
        assert rep["read_trust_seed"]
        assert rep["write_trust_seed"]
        if id:
            assert rep["id"] == id
        else:
            assert rep["id"]
            id = rep["id"]

    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send(
            {"cmd": "vlob_read", "id": rep["id"], "trust_seed": rep["read_trust_seed"]}
        )
        rep = await sock.recv()
        expected_content = to_jsonb64(b"" if not blob else blob)
        assert rep == {"status": "ok", "id": id, "version": 1, "blob": expected_content}


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"blob": to_jsonb64(b"..."), "bad_field": "foo"},
        {"blob": 42},
        {"blob": None},
        {"id": 42, "blob": to_jsonb64(b"...")},
        {"id": "", "blob": to_jsonb64(b"...")},  # Id is 1 long min
        {"id": "X" * 33, "blob": to_jsonb64(b"...")},  # Id is 32 long max
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
        {"cmd": "vlob_read", "id": "1234", "trust_seed": "TS4242"}
    )
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "not_found_error", "reason": "Vlob not found."}


@pytest.mark.trio
async def test_vlob_read_ok(backend, alice_backend_sock):
    await populate_backend_vlob(backend)

    await alice_backend_sock.send(
        {"cmd": "vlob_read", "id": "1", "trust_seed": "<1 rts>"}
    )
    rep = await alice_backend_sock.recv()

    assert rep == {
        "status": "ok", "id": "1", "blob": to_jsonb64(b"1 blob v2"), "version": 2
    }


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": "1234", "trust_seed": "TS4242", "bad_field": "foo"},
        {"id": "1234"},
        {"id": "1234", "trust_seed": 42},
        {"id": "1234", "trust_seed": None},
        {"id": 42, "trust_seed": "TS4242"},
        {"id": None, "trust_seed": "TS4242"},
        # {'id': '1234567890', 'trust_seed': 'TS4242'},  # TODO bad?
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
        {"cmd": "vlob_read", "id": "1", "trust_seed": "<1 rts>", "version": 3}
    )
    rep = await alice_backend_sock.recv()

    assert rep == {"status": "version_error", "reason": "Wrong blob version."}


@pytest.mark.trio
async def test_vlob_update_ok(backend, alice_backend_sock):
    await populate_backend_vlob(backend)

    await alice_backend_sock.send(
        {
            "cmd": "vlob_update",
            "id": "1",
            "trust_seed": "<1 wts>",
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
            "id": "123",
            "trust_seed": "WTS42",
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
            "id": "1234",
            "trust_seed": "WTS42",
            "version": "42",
            "blob": to_jsonb64(b"..."),
            "bad_field": "foo",
        },
        {"id": "1234", "trust_seed": "WTS42", "version": "42", "blob": None},
        {"id": "1234", "trust_seed": "WTS42", "version": "42", "blob": 42},
        {"id": "1234", "trust_seed": "WTS42", "version": "42"},
        {
            "id": "1234",
            "trust_seed": "WTS42",
            "version": None,
            "blob": to_jsonb64(b"..."),
        },
        {
            "id": "1234",
            "trust_seed": "WTS42",
            "version": -1,
            "blob": to_jsonb64(b"..."),
        },
        {"id": "1234", "trust_seed": None, "version": "42", "blob": to_jsonb64(b"...")},
        {"id": "1234", "trust_seed": 42, "version": "42", "blob": to_jsonb64(b"...")},
        {"id": "1234", "version": "42", "blob": to_jsonb64(b"...")},
        {"id": 42, "trust_seed": "WTS42", "version": "42", "blob": to_jsonb64(b"...")},
        {
            "id": None,
            "trust_seed": "WTS42",
            "version": "42",
            "blob": to_jsonb64(b"..."),
        },
        {"trust_seed": "WTS42", "version": "42", "blob": to_jsonb64(b"...")},
        {},
    ],
)
@pytest.mark.trio
async def test_vlob_update_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "vlob_update", **bad_msg})
    rep = await alice_backend_sock.recv()
    # Id and trust_seed are invalid anyway, but here we test another layer
    # so it's not important as long as we get our `bad_message` status
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_update_bad_version(backend, alice_backend_sock):
    await populate_backend_vlob(backend)

    await alice_backend_sock.send(
        {
            "cmd": "vlob_update",
            "id": "1",
            "trust_seed": "<1 wts>",
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
            "id": "1",
            "trust_seed": "dummy_seed",
            "version": 3,
            "blob": to_jsonb64(b"Next version."),
        }
    )
    rep = await alice_backend_sock.recv()

    assert rep == {"status": "trust_seed_error", "reason": "Invalid write trust seed."}
