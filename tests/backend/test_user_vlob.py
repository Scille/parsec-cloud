import pytest

from parsec.utils import to_jsonb64


async def populate_backend_user_vlob(backend, user):
    await backend.user_vlob.update(user.user_id, 1, b"blob v1")
    await backend.user_vlob.update(user.user_id, 2, b"blob v2")
    await backend.user_vlob.update(user.user_id, 3, b"blob v3")


@pytest.mark.trio
async def test_user_vlob_read_ok(backend, alice, alice_backend_sock):
    await populate_backend_user_vlob(backend, alice)

    await alice_backend_sock.send({"cmd": "user_vlob_read", "version": 2})
    rep = await alice_backend_sock.recv()

    assert rep == {"status": "ok", "blob": to_jsonb64(b"blob v2"), "version": 2}


@pytest.mark.trio
async def test_user_vlob_read_last_version(backend, alice, alice_backend_sock):
    await populate_backend_user_vlob(backend, alice)

    await alice_backend_sock.send({"cmd": "user_vlob_read"})
    rep = await alice_backend_sock.recv()

    assert rep == {"status": "ok", "blob": to_jsonb64(b"blob v3"), "version": 3}


@pytest.mark.trio
async def test_user_vlob_read_bad_version(backend, alice, alice_backend_sock):
    await populate_backend_user_vlob(backend, alice)

    await alice_backend_sock.send({"cmd": "user_vlob_read", "version": 42})
    rep = await alice_backend_sock.recv()

    assert rep == {"status": "version_error", "reason": "Wrong blob version."}


@pytest.mark.parametrize(
    "bad_msg", [{"version": None}, {"version": "42x"}, {"bad_field": "foo"}]
)
@pytest.mark.trio
async def test_user_vlob_read_bad_msg(backend, alice, alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "user_vlob_read", **bad_msg})
    rep = await alice_backend_sock.recv()
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_user_vlob_update_ok(backend, alice, alice_backend_sock):
    await populate_backend_user_vlob(backend, alice)

    await alice_backend_sock.send(
        {"cmd": "user_vlob_update", "version": 4, "blob": to_jsonb64(b"fooo")}
    )
    rep = await alice_backend_sock.recv()

    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_user_vlob_update_bad_version(backend, alice, alice_backend_sock):
    await populate_backend_user_vlob(backend, alice)

    await alice_backend_sock.send(
        {"cmd": "user_vlob_update", "version": 42, "blob": to_jsonb64(b"fooo")}
    )
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "version_error", "reason": "Wrong blob version."}


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"version": 42, "blob": to_jsonb64(b"..."), "bad_field": "foo"},
        {"version": "42x", "blob": to_jsonb64(b"...")},
        {"version": None, "blob": to_jsonb64(b"...")},
        {"version": 42, "blob": 42},
        {"version": 42, "blob": None},
        {"version": 42, "blob": "<not a b64>"},
        {},
    ],
)
@pytest.mark.trio
async def test_user_vlob_update_bad_msg(backend, bad_msg, alice, alice_backend_sock):
    await alice_backend_sock.send({"cmd": "user_vlob_update", **bad_msg})
    rep = await alice_backend_sock.recv()
    # Id and trust_seed are invalid anyway, but here we test another layer
    # so it's not important as long as we get our `bad_message` status
    assert rep["status"] == "bad_message"
