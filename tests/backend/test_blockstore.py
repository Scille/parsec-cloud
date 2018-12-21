import trio
import pytest
from uuid import uuid4

from parsec.backend.blockstore import BlockstoreTimeoutError
from parsec.api.protocole import blockstore_create_serializer, blockstore_read_serializer, packb


BLOCK_ID = uuid4()
BLOCK_DATA = b"Hodi ho !"


def _get_existing_block(backend):
    # Backend must have been populated before that
    return list(backend.test_populate_data["blocks"].items())[0]


async def create(sock, id, block, **kwargs):
    await sock.send(
        blockstore_create_serializer.req_dumps(
            {"cmd": "blockstore_create", "id": id, "block": block, **kwargs}
        )
    )
    raw_rep = await sock.recv()
    return blockstore_create_serializer.rep_loads(raw_rep)


async def read(sock, id, **kwargs):
    await sock.send(
        blockstore_read_serializer.req_dumps({"cmd": "blockstore_read", "id": id, **kwargs})
    )
    raw_rep = await sock.recv()
    return blockstore_read_serializer.rep_loads(raw_rep)


@pytest.mark.trio
async def test_blockstore_create_and_read(alice_backend_sock, bob_backend_sock):
    rep = await create(alice_backend_sock, BLOCK_ID, BLOCK_DATA)
    assert rep == {"status": "ok"}

    rep = await read(bob_backend_sock, BLOCK_ID)
    assert rep == {"status": "ok", "block": BLOCK_DATA}

    # Test not found as well

    dummy_id = uuid4()
    rep = await read(bob_backend_sock, dummy_id)
    assert rep == {"status": "not_found"}


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_blockstore_create_and_read(alice_backend_sock, bob_backend_sock):
    await test_blockstore_create_and_read(alice_backend_sock, bob_backend_sock)


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_blockstore_create_partial_failure(
    alice_backend_sock, bob_backend_sock, backend
):
    async def mock_create(id, block, author):
        await trio.sleep(0)
        raise BlockstoreTimeoutError()

    backend.blockstore.blockstores[1].create = mock_create

    rep = await create(alice_backend_sock, BLOCK_ID, BLOCK_DATA)
    rep == {"status": "timeout"}


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_blockstore_create_partial_exists(alice_backend_sock, alice, backend):
    await backend.blockstore.blockstores[1].create(BLOCK_ID, BLOCK_DATA, alice.device_id)

    rep = await create(alice_backend_sock, BLOCK_ID, BLOCK_DATA)
    assert rep == {"status": "ok"}


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_blockstore_read_partial_failure(alice_backend_sock, alice, backend):
    await backend.blockstore.blockstores[1].create(BLOCK_ID, BLOCK_DATA, alice.device_id)

    rep = await read(alice_backend_sock, BLOCK_ID)
    assert rep == {"status": "ok", "block": BLOCK_DATA}


@pytest.mark.trio
@pytest.mark.raid0_blockstore
async def test_raid0_blockstore_create_and_read(alice_backend_sock, bob_backend_sock):
    await test_blockstore_create_and_read(alice_backend_sock, bob_backend_sock)


@pytest.mark.parametrize(
    "bad_msg",
    [
        {},
        {"id": str(BLOCK_ID), "block": BLOCK_DATA, "bad_field": "foo"},
        {"id": "not an uuid", "block": BLOCK_DATA},
        {"id": 42, "block": BLOCK_DATA},
        {"id": None, "block": BLOCK_DATA},
        {"id": str(BLOCK_ID), "block": 42},
        {"id": str(BLOCK_ID), "block": None},
        {"block": BLOCK_DATA},
    ],
)
@pytest.mark.trio
async def test_blockstore_create_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send(packb({"cmd": "blockstore_create", **bad_msg}))
    raw_rep = await alice_backend_sock.recv()
    rep = blockstore_create_serializer.rep_loads(raw_rep)
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_blockstore_read_not_found(alice_backend_sock):
    rep = await read(alice_backend_sock, BLOCK_ID)
    assert rep == {"status": "not_found"}


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": str(BLOCK_ID), "bad_field": "foo"},
        {"id": "not_an_uuid"},
        {"id": 42},
        {"id": None},
        {},
    ],
)
@pytest.mark.trio
async def test_blockstore_read_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send(packb({"cmd": "blockstore_read", **bad_msg}))
    raw_rep = await alice_backend_sock.recv()
    rep = blockstore_read_serializer.rep_loads(raw_rep)
    # Valid ID doesn't exists in database but this is ok given here we test
    # another layer so it's not important as long as we get our
    # `bad_message` status
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_blockstore_conflicting_id(alice_backend_sock):
    block_v1 = b"v1"
    rep = await create(alice_backend_sock, BLOCK_ID, block_v1)
    assert rep == {"status": "ok"}

    block_v2 = b"v2"
    rep = await create(alice_backend_sock, BLOCK_ID, block_v2)
    assert rep == {"status": "already_exists"}

    rep = await read(alice_backend_sock, BLOCK_ID)
    assert rep == {"status": "ok", "block": block_v1}
