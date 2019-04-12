# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import pytest
import pendulum
from uuid import UUID

from parsec.backend.block import BlockTimeoutError
from parsec.api.protocole import block_create_serializer, block_read_serializer, packb


BLOCK_ID = UUID("00000000000000000000000000000001")
VLOB_ID = UUID("00000000000000000000000000000002")
BLOCK_DATA = b"Hodi ho !"


async def _create_vlob_group(backend, user, vlob_id, vlob_group_id):
    now = pendulum.now()
    await backend.vlob.create(
        user.organization_id, user.device_id, vlob_id, vlob_group_id, now, b""
    )
    return vlob_group_id


@pytest.fixture
async def vlob_group(backend, alice):
    return await _create_vlob_group(
        backend,
        alice,
        UUID("0000000000000000000000000000000A"),
        UUID("0000000000000000000000000000000B"),
    )


@pytest.fixture
async def block(backend, alice, vlob_group):
    block_id = UUID("0000000000000000000000000000000C")

    await backend.block.create(
        alice.organization_id, alice.device_id, block_id, vlob_group, BLOCK_DATA
    )
    return block_id


async def create(sock, id, vlob_group, block):
    await sock.send(
        block_create_serializer.req_dumps(
            {"cmd": "block_create", "id": id, "vlob_group": vlob_group, "block": block}
        )
    )
    raw_rep = await sock.recv()
    return block_create_serializer.rep_loads(raw_rep)


async def read(sock, id):
    await sock.send(block_read_serializer.req_dumps({"cmd": "block_read", "id": id}))
    raw_rep = await sock.recv()
    return block_read_serializer.rep_loads(raw_rep)


@pytest.mark.trio
async def test_block_no_read_access(backend, bob_backend_sock, alice, bob, vlob_group, block):
    rep = await read(bob_backend_sock, block)
    assert rep == {"status": "not_allowed"}

    # Give access to bob
    await backend.vlob.update_group_rights(
        alice.organization_id,
        author=alice.user_id,
        id=vlob_group,
        user=bob.user_id,
        read_right=True,
        write_right=False,
        admin_right=False,
    )

    rep = await read(bob_backend_sock, block)
    assert rep == {"status": "ok", "block": BLOCK_DATA}


@pytest.mark.trio
async def test_block_create_and_read(alice_backend_sock, vlob_group):
    rep = await create(alice_backend_sock, BLOCK_ID, vlob_group, BLOCK_DATA)
    assert rep == {"status": "ok"}

    rep = await read(alice_backend_sock, BLOCK_ID)
    assert rep == {"status": "ok", "block": BLOCK_DATA}

    # Test not found as well

    dummy_id = UUID("00000000000000000000000000000002")
    rep = await read(alice_backend_sock, dummy_id)
    assert rep == {"status": "not_found"}


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_block_create_and_read(alice_backend_sock, vlob_group):
    await test_block_create_and_read(alice_backend_sock, vlob_group)


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_block_create_partial_failure(alice_backend_sock, backend, vlob_group):
    async def mock_create(organization_id, id, block):
        await trio.sleep(0)
        raise BlockTimeoutError()

    backend.blockstore.blockstores[1].create = mock_create

    rep = await create(alice_backend_sock, BLOCK_ID, vlob_group, BLOCK_DATA)
    assert rep == {"status": "timeout"}


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_block_create_partial_exists(alice_backend_sock, alice, backend, vlob_group):
    await backend.blockstore.blockstores[1].create(alice.organization_id, BLOCK_ID, BLOCK_DATA)

    rep = await create(alice_backend_sock, BLOCK_ID, vlob_group, BLOCK_DATA)
    assert rep == {"status": "ok"}


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_block_read_partial_failure(alice_backend_sock, alice, backend, block):
    async def mock_read(organization_id, id):
        await trio.sleep(0)
        raise BlockTimeoutError()

    backend.blockstore.blockstores[1].read = mock_read

    rep = await read(alice_backend_sock, block)
    assert rep == {"status": "ok", "block": BLOCK_DATA}


@pytest.mark.trio
@pytest.mark.raid0_blockstore
async def test_raid0_block_create_and_read(alice_backend_sock, vlob_group):
    await test_block_create_and_read(alice_backend_sock, vlob_group)


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
async def test_block_create_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send(packb({"cmd": "block_create", **bad_msg}))
    raw_rep = await alice_backend_sock.recv()
    rep = block_create_serializer.rep_loads(raw_rep)
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_block_read_not_found(alice_backend_sock):
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
async def test_block_read_bad_msg(alice_backend_sock, bad_msg):
    await alice_backend_sock.send(packb({"cmd": "block_read", **bad_msg}))
    raw_rep = await alice_backend_sock.recv()
    rep = block_read_serializer.rep_loads(raw_rep)
    # Valid ID doesn't exists in database but this is ok given here we test
    # another layer so it's not important as long as we get our
    # `bad_message` status
    assert rep["status"] == "bad_message"


@pytest.mark.trio
async def test_block_conflicting_id(alice_backend_sock, vlob_group):
    block_v1 = b"v1"
    rep = await create(alice_backend_sock, BLOCK_ID, vlob_group, block_v1)
    assert rep == {"status": "ok"}

    block_v2 = b"v2"
    rep = await create(alice_backend_sock, BLOCK_ID, vlob_group, block_v2)
    assert rep == {"status": "already_exists"}

    rep = await read(alice_backend_sock, BLOCK_ID)
    assert rep == {"status": "ok", "block": block_v1}


@pytest.mark.trio
async def test_block_check_other_organization(
    backend, sock_from_other_organization_factory, vlob_group, block
):
    async with sock_from_other_organization_factory(backend) as sock:
        rep = await read(sock, block)
        assert rep == {"status": "not_found"}

        await _create_vlob_group(backend, sock.device, vlob_id=VLOB_ID, vlob_group_id=vlob_group)
        rep = await create(sock, block, vlob_group, b"other org data")
        assert rep == {"status": "ok"}

        rep = await read(sock, block)
        assert rep == {"status": "ok", "block": b"other org data"}
