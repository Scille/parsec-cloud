import pytest

from parsec.utils import to_jsonb64


def _get_existing_block(backend):
    # Backend must have been populated before that
    return list(backend.test_populate_data["blocks"].items())[0]


@pytest.mark.trio
async def test_blockstore_post_and_get(alice_backend_sock, bob_backend_sock):
    block_id = "123"

    block = to_jsonb64(b"Hodi ho !")
    await alice_backend_sock.send({"cmd": "blockstore_post", "id": block_id, "block": block})
    rep = await alice_backend_sock.recv()
    assert rep["status"] == "ok"

    await bob_backend_sock.send({"cmd": "blockstore_get", "id": block_id})
    rep = await bob_backend_sock.recv()
    assert rep == {"status": "ok", "block": block}


@pytest.mark.parametrize(
    "bad_msg",
    [
        {},
        {"id": "123", "blob": to_jsonb64(b"..."), "bad_field": "foo"},
        {"id": 42, "blob": to_jsonb64(b"...")},
        {"id": None, "blob": to_jsonb64(b"...")},
        {"id": "123", "blob": 42},
        {"id": "123", "blob": None},
        {"blob": to_jsonb64(b"...")},
    ],
)
@pytest.mark.trio
async def test_blockstore_post_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "blockstore_post", **bad_msg})
    rep = await alice_backend_sock.recv()
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_blockstore_get_not_found(alice_backend_sock):
    await alice_backend_sock.send({"cmd": "blockstore_get", "id": "1234"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "not_found_error", "reason": "Unknown block id."}


@pytest.mark.parametrize(
    "bad_msg", [{"id": "1234", "bad_field": "foo"}, {"id": 42}, {"id": None}, {}]
)
@pytest.mark.trio
async def test_blockstore_get_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "blockstore_get", **bad_msg})
    rep = await alice_backend_sock.recv()
    # Id and trust_seed are invalid anyway, but here we test another layer
    # so it's not important as long as we get our `bad_message` status
    assert rep["status"] == "bad_message"
