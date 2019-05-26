# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import pytest
import pendulum
from uuid import UUID, uuid4

from parsec.backend.block import BlockTimeoutError
from parsec.api.protocole import block_create_serializer, block_read_serializer, packb, RealmRole


BLOCK_ID = UUID("00000000000000000000000000000001")
VLOB_ID = UUID("00000000000000000000000000000002")
BLOCK_DATA = b"Hodi ho !"


async def _create_realm(backend, user, vlob_id, realm_id):
    now = pendulum.now()
    await backend.vlob.create(
        organization_id=user.organization_id,
        author=user.device_id,
        realm_id=realm_id,
        encryption_revision=1,
        vlob_id=vlob_id,
        timestamp=now,
        blob=b"",
    )
    return realm_id


@pytest.fixture
async def block(backend, alice, realm):
    block_id = UUID("0000000000000000000000000000000C")

    await backend.block.create(alice.organization_id, alice.device_id, block_id, realm, BLOCK_DATA)
    return block_id


async def block_create(sock, block_id, realm_id, block):
    await sock.send(
        block_create_serializer.req_dumps(
            {"cmd": "block_create", "block_id": block_id, "realm_id": realm_id, "block": block}
        )
    )
    raw_rep = await sock.recv()
    return block_create_serializer.rep_loads(raw_rep)


async def block_read(sock, block_id):
    await sock.send(block_read_serializer.req_dumps({"cmd": "block_read", "block_id": block_id}))
    raw_rep = await sock.recv()
    return block_read_serializer.rep_loads(raw_rep)


@pytest.mark.trio
async def test_block_read_check_access_rights(backend, alice, bob, bob_backend_sock, realm, block):
    # User not part of the realm
    rep = await block_read(bob_backend_sock, block)
    assert rep == {"status": "not_allowed"}

    # User part of the realm with various role
    for role in (RealmRole.READER, RealmRole.CONTRIBUTOR, RealmRole.MANAGER, RealmRole.OWNER):
        await backend.realm.update_roles(
            alice.organization_id, alice.device_id, realm, bob.user_id, role
        )
        rep = await block_read(bob_backend_sock, block)
        assert rep == {"status": "ok", "block": b"Hodi ho !"}


@pytest.mark.trio
async def test_block_create_check_access_rights(backend, alice, bob, bob_backend_sock, realm):
    block_id = uuid4()

    # User not part of the realm
    rep = await block_create(bob_backend_sock, block_id, realm, BLOCK_DATA)
    assert rep == {"status": "not_allowed"}

    # User part of the realm with various role
    for role, access_granted in [
        (RealmRole.READER, False),
        (RealmRole.CONTRIBUTOR, True),
        (RealmRole.MANAGER, True),
        (RealmRole.OWNER, True),
    ]:
        await backend.realm.update_roles(
            alice.organization_id, alice.device_id, realm, bob.user_id, role
        )
        block_id = uuid4()
        rep = await block_create(bob_backend_sock, block_id, realm, BLOCK_DATA)
        if access_granted:
            assert rep == {"status": "ok"}

        else:
            assert rep == {"status": "not_allowed"}


@pytest.mark.trio
async def test_block_create_and_read(alice_backend_sock, realm):
    rep = await block_create(alice_backend_sock, BLOCK_ID, realm, BLOCK_DATA)
    assert rep == {"status": "ok"}

    rep = await block_read(alice_backend_sock, BLOCK_ID)
    assert rep == {"status": "ok", "block": BLOCK_DATA}

    # Test not found as well

    dummy_id = UUID("00000000000000000000000000000002")
    rep = await block_read(alice_backend_sock, dummy_id)
    assert rep == {"status": "not_found"}


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_block_create_and_read(alice_backend_sock, realm):
    await test_block_create_and_read(alice_backend_sock, realm)


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_block_create_partial_failure(alice_backend_sock, backend, realm):
    async def mock_create(organization_id, id, block):
        await trio.sleep(0)
        raise BlockTimeoutError()

    backend.blockstore.blockstores[1].create = mock_create

    rep = await block_create(alice_backend_sock, BLOCK_ID, realm, BLOCK_DATA)
    assert rep == {"status": "timeout"}


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_block_create_partial_exists(alice_backend_sock, alice, backend, realm):
    await backend.blockstore.blockstores[1].create(alice.organization_id, BLOCK_ID, BLOCK_DATA)

    rep = await block_create(alice_backend_sock, BLOCK_ID, realm, BLOCK_DATA)
    assert rep == {"status": "ok"}


@pytest.mark.trio
@pytest.mark.raid1_blockstore
async def test_raid1_block_read_partial_failure(alice_backend_sock, alice, backend, block):
    async def mock_read(organization_id, id):
        await trio.sleep(0)
        raise BlockTimeoutError()

    backend.blockstore.blockstores[1].read = mock_read

    rep = await block_read(alice_backend_sock, block)
    assert rep == {"status": "ok", "block": BLOCK_DATA}


@pytest.mark.trio
@pytest.mark.raid0_blockstore
async def test_raid0_block_create_and_read(alice_backend_sock, realm):
    await test_block_create_and_read(alice_backend_sock, realm)


@pytest.mark.trio
@pytest.mark.raid5_blockstore
async def test_raid5_block_create_and_read(alice_backend_sock, realm):
    await test_block_create_and_read(alice_backend_sock, realm)


@pytest.mark.trio
@pytest.mark.raid5_blockstore
async def test_raid5_block_create_partial_failure_0(alice_backend_sock, backend, realm):
    await _test_raid5_block_create_partial_failure(alice_backend_sock, backend, realm, 0)


@pytest.mark.trio
@pytest.mark.raid5_blockstore
async def test_raid5_block_create_partial_failure_1(alice_backend_sock, backend, realm):
    await _test_raid5_block_create_partial_failure(alice_backend_sock, backend, realm, 1)


@pytest.mark.trio
@pytest.mark.raid5_blockstore
async def test_raid5_block_create_partial_failure_2(alice_backend_sock, backend, realm):
    await _test_raid5_block_create_partial_failure(alice_backend_sock, backend, realm, 2)


async def _test_raid5_block_create_partial_failure(
    alice_backend_sock, backend, realm, failing_blockstore_pos
):
    async def mock_create(organization_id, id, block):
        await trio.sleep(0)
        raise BlockTimeoutError()

    backend.blockstore.blockstores[failing_blockstore_pos].create = mock_create

    rep = await block_create(alice_backend_sock, BLOCK_ID, realm, BLOCK_DATA)
    assert rep == {"status": "timeout"}


@pytest.mark.trio
@pytest.mark.raid5_blockstore
async def test_raid5_block_create_partial_exists_0(alice_backend_sock, alice, backend, realm):
    await _test_raid5_block_create_partial_exists(alice_backend_sock, alice, backend, realm, 0)


@pytest.mark.trio
@pytest.mark.raid5_blockstore
async def test_raid5_block_create_partial_exists_1(alice_backend_sock, alice, backend, realm):
    await _test_raid5_block_create_partial_exists(alice_backend_sock, alice, backend, realm, 1)


@pytest.mark.trio
@pytest.mark.raid5_blockstore
async def test_raid5_block_create_partial_exists_2(alice_backend_sock, alice, backend, realm):
    await _test_raid5_block_create_partial_exists(alice_backend_sock, alice, backend, realm, 2)


async def _test_raid5_block_create_partial_exists(
    alice_backend_sock, alice, backend, realm, failing_blockstore_pos
):
    await backend.blockstore.blockstores[1].create(alice.organization_id, BLOCK_ID, BLOCK_DATA)

    rep = await block_create(alice_backend_sock, BLOCK_ID, realm, BLOCK_DATA)
    assert rep == {"status": "ok"}


@pytest.mark.trio
@pytest.mark.raid5_blockstore
async def test_raid5_block_read_partial_failure_0(alice_backend_sock, alice, backend, block):
    await _test_raid5_block_read_partial_failure(alice_backend_sock, alice, backend, block, 0)


@pytest.mark.trio
@pytest.mark.raid5_blockstore
async def test_raid5_block_read_partial_failure_1(alice_backend_sock, alice, backend, block):
    await _test_raid5_block_read_partial_failure(alice_backend_sock, alice, backend, block, 1)


@pytest.mark.trio
@pytest.mark.raid5_blockstore
async def test_raid5_block_read_partial_failure_2(alice_backend_sock, alice, backend, block):
    await _test_raid5_block_read_partial_failure(alice_backend_sock, alice, backend, block, 2)


async def _test_raid5_block_read_partial_failure(
    alice_backend_sock, alice, backend, block, failing_blockstore_pos
):
    async def mock_read(organization_id, id):
        await trio.sleep(0)
        raise BlockTimeoutError()

    backend.blockstore.blockstores[failing_blockstore_pos].read = mock_read

    rep = await block_read(alice_backend_sock, block)
    assert rep == {"status": "ok", "block": BLOCK_DATA}


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
    rep = await block_read(alice_backend_sock, BLOCK_ID)
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
async def test_block_conflicting_id(alice_backend_sock, realm):
    block_v1 = b"v1"
    rep = await block_create(alice_backend_sock, BLOCK_ID, realm, block_v1)
    assert rep == {"status": "ok"}

    block_v2 = b"v2"
    rep = await block_create(alice_backend_sock, BLOCK_ID, realm, block_v2)
    assert rep == {"status": "already_exists"}

    rep = await block_read(alice_backend_sock, BLOCK_ID)
    assert rep == {"status": "ok", "block": block_v1}


@pytest.mark.trio
async def test_block_check_other_organization(
    backend, sock_from_other_organization_factory, realm, block
):
    async with sock_from_other_organization_factory(backend) as sock:
        rep = await block_read(sock, block)
        assert rep == {"status": "not_found"}

        await _create_realm(backend, sock.device, vlob_id=VLOB_ID, realm_id=realm)
        rep = await block_create(sock, block, realm, b"other org data")
        assert rep == {"status": "ok"}

        rep = await block_read(sock, block)
        assert rep == {"status": "ok", "block": b"other org data"}


@pytest.mark.trio
async def test_access_during_maintenance(backend, alice, alice_backend_sock, realm, block):
    await backend.realm.start_reencryption_maintenance(
        alice.organization_id, alice.device_id, realm, 2, {alice.user_id: b"whatever"}
    )

    rep = await block_create(alice_backend_sock, BLOCK_ID, realm, BLOCK_DATA)
    assert rep == {"status": "in_maintenance"}

    rep = await block_read(alice_backend_sock, block)
    assert rep == {"status": "in_maintenance"}
