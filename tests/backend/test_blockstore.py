import trio
import pytest

from parsec.backend.exceptions import TimeoutError
from parsec.utils import to_jsonb64


def _get_existing_block(backend):
    # Backend must have been populated before that
    return list(backend.test_populate_data["blocks"].items())[0]


@pytest.mark.trio
async def test_blockstore_post_and_get(alice_backend_sock, bob_backend_sock):
    block_id = "b00008dba3834f08abc6eb3aec280c6a"

    block = to_jsonb64(b"Hodi ho !")
    await alice_backend_sock.send({"cmd": "blockstore_post", "id": block_id, "block": block})
    rep = await alice_backend_sock.recv()
    assert rep["status"] == "ok"

    await bob_backend_sock.send({"cmd": "blockstore_get", "id": block_id})
    rep = await bob_backend_sock.recv()
    assert rep == {"status": "ok", "block": block}

    # Test not found as well

    dummy_id = "6a8c947e3fac4a24850f857e44c6c50b"
    await bob_backend_sock.send({"cmd": "blockstore_get", "id": dummy_id})
    rep = await bob_backend_sock.recv()
    assert rep == {"status": "not_found_error", "reason": "Unknown block id."}


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_blockstore_post_and_get(alice_backend_sock, bob_backend_sock):
    await test_blockstore_post_and_get(alice_backend_sock, bob_backend_sock)


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_blockstore_post_partial_failure(alice_backend_sock, bob_backend_sock, backend):
    async def mock_post(id, block):
        await trio.sleep(0)
        raise TimeoutError()

    backend.blockstore.blockstores[1].post = mock_post

    block_id = "b00008dba3834f08abc6eb3aec280c6a"
    block = to_jsonb64(b"Hodi ho !")
    await alice_backend_sock.send({"cmd": "blockstore_post", "id": block_id, "block": block})
    rep = await alice_backend_sock.recv()
    assert rep["status"] == "timeout"


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_blockstore_post_partial_exists(alice_backend_sock, bob_backend_sock, backend):
    block_id = "b00008dba3834f08abc6eb3aec280c6a"
    await backend.blockstore.blockstores[1].post(block_id, b"Hodi ho !")

    block = to_jsonb64(b"Hodi ho !")
    await alice_backend_sock.send({"cmd": "blockstore_post", "id": block_id, "block": block})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok"}


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_blockstore_get_partial_failure(alice_backend_sock, bob_backend_sock, backend):
    block_id = "b00008dba3834f08abc6eb3aec280c6a"

    await backend.blockstore.blockstores[1].post(block_id, b"Hodi ho !")

    await bob_backend_sock.send({"cmd": "blockstore_get", "id": block_id})
    rep = await bob_backend_sock.recv()
    assert rep == {"status": "not_found_error", "reason": "Unknown block id."}


@pytest.mark.trio
@pytest.mark.raid0_blockstore
async def test_raid0_blockstore_post_and_get(alice_backend_sock, bob_backend_sock):
    await test_blockstore_post_and_get(alice_backend_sock, bob_backend_sock)


@pytest.mark.parametrize(
    "bad_msg",
    [
        {},
        {"id": "b00008dba3834f08abc6eb3aec280c6a", "blob": to_jsonb64(b"..."), "bad_field": "foo"},
        {"id": "not an uuid", "blob": to_jsonb64(b"...")},
        {"id": 42, "blob": to_jsonb64(b"...")},
        {"id": None, "blob": to_jsonb64(b"...")},
        {"id": "b00008dba3834f08abc6eb3aec280c6a", "blob": 42},
        {"id": "b00008dba3834f08abc6eb3aec280c6a", "blob": None},
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
    await alice_backend_sock.send(
        {"cmd": "blockstore_get", "id": "b00008dba3834f08abc6eb3aec280c6a"}
    )
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "not_found_error", "reason": "Unknown block id."}


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": "b00008dba3834f08abc6eb3aec280c6a", "bad_field": "foo"},
        {"id": "not_an_uuid"},
        {"id": 42},
        {"id": None},
        {},
    ],
)
@pytest.mark.trio
async def test_blockstore_get_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send({"cmd": "blockstore_get", **bad_msg})
    rep = await alice_backend_sock.recv()
    # Id and trust_seed are invalid anyway, but here we test another layer
    # so it's not important as long as we get our `bad_message` status
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_blockstore_conflicting_id(alice_backend_sock):
    block_id = "b00008dba3834f08abc6eb3aec280c6a"

    block_v1 = to_jsonb64(b"v1")
    await alice_backend_sock.send({"cmd": "blockstore_post", "id": block_id, "block": block_v1})
    rep = await alice_backend_sock.recv()
    assert rep["status"] == "ok"

    block_v2 = to_jsonb64(b"v2")
    await alice_backend_sock.send({"cmd": "blockstore_post", "id": block_id, "block": block_v2})
    rep = await alice_backend_sock.recv()
    assert rep["status"] == "ok"

    await alice_backend_sock.send({"cmd": "blockstore_get", "id": block_id})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok", "block": block_v1}
