# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
import trio
from hypothesis import given
from hypothesis import strategies as st

from parsec._parsec import (
    BlockCreateRepAlreadyExists,
    BlockCreateRepInMaintenance,
    BlockCreateRepNotAllowed,
    BlockCreateRepOk,
    BlockCreateRepTimeout,
    BlockReadRepNotAllowed,
    BlockReadRepNotFound,
    BlockReadRepOk,
    BlockReadRepTimeout,
    DateTime,
)
from parsec.api.protocol import (
    BlockID,
    RealmRole,
    VlobID,
    block_create_serializer,
    block_read_serializer,
    packb,
)
from parsec.backend.block import BlockStoreError
from parsec.backend.raid5_blockstore import (
    generate_checksum_chunk,
    rebuild_block_from_chunks,
    split_block_in_chunks,
)
from parsec.backend.realm import RealmGrantedRole
from tests.backend.common import block_create, block_read
from tests.common import customize_fixtures

BLOCK_ID = BlockID.from_hex("00000000000000000000000000000001")
VLOB_ID = VlobID.from_hex("00000000000000000000000000000002")
BLOCK_DATA = b"Hodi ho !"


@pytest.fixture
async def block(backend, alice, realm):
    block_id = BlockID.from_hex("0000000000000000000000000000000C")

    await backend.block.create(
        organization_id=alice.organization_id,
        author=alice.device_id,
        block_id=block_id,
        realm_id=realm,
        block=BLOCK_DATA,
    )
    return block_id


@pytest.mark.trio
async def test_block_read_check_access_rights(
    backend, alice, bob, bob_ws, realm, block, next_timestamp
):
    # User not part of the realm
    rep = await block_read(bob_ws, block)
    assert isinstance(rep, BlockReadRepNotAllowed)

    # User part of the realm with various role
    for role in RealmRole.VALUES:
        await backend.realm.update_roles(
            alice.organization_id,
            RealmGrantedRole(
                certificate=b"<dummy>",
                realm_id=realm,
                user_id=bob.user_id,
                role=role,
                granted_by=alice.device_id,
                granted_on=next_timestamp(),
            ),
        )
        rep = await block_read(bob_ws, block)
        assert rep == BlockReadRepOk(BLOCK_DATA)

    # Ensure user that used to be part of the realm have no longer access
    await backend.realm.update_roles(
        alice.organization_id,
        RealmGrantedRole(
            certificate=b"<dummy>",
            realm_id=realm,
            user_id=bob.user_id,
            role=None,
            granted_by=alice.device_id,
            granted_on=next_timestamp(),
        ),
    )
    rep = await block_read(bob_ws, block)
    assert isinstance(rep, BlockReadRepNotAllowed)


@pytest.mark.trio
async def test_block_create_check_access_rights(backend, alice, bob, bob_ws, realm, next_timestamp):
    block_id = BlockID.new()

    # User not part of the realm
    rep = await block_create(bob_ws, block_id, realm, BLOCK_DATA, check_rep=False)
    assert isinstance(rep, BlockCreateRepNotAllowed)

    # User part of the realm with various role
    for role, access_granted in [
        (RealmRole.READER, False),
        (RealmRole.CONTRIBUTOR, True),
        (RealmRole.MANAGER, True),
        (RealmRole.OWNER, True),
    ]:
        await backend.realm.update_roles(
            alice.organization_id,
            RealmGrantedRole(
                certificate=b"<dummy>",
                realm_id=realm,
                user_id=bob.user_id,
                role=role,
                granted_by=alice.device_id,
                granted_on=next_timestamp(),
            ),
        )
        block_id = BlockID.new()
        rep = await block_create(bob_ws, block_id, realm, BLOCK_DATA, check_rep=False)
        if access_granted:
            assert isinstance(rep, BlockCreateRepOk)

        else:
            assert isinstance(rep, BlockCreateRepNotAllowed)

    # Ensure user that used to be part of the realm have no longer access
    await backend.realm.update_roles(
        alice.organization_id,
        RealmGrantedRole(
            certificate=b"<dummy>",
            realm_id=realm,
            user_id=bob.user_id,
            role=None,
            granted_by=alice.device_id,
            granted_on=next_timestamp(),
        ),
    )
    rep = await block_create(bob_ws, block_id, realm, BLOCK_DATA, check_rep=False)
    assert isinstance(rep, BlockCreateRepNotAllowed)


@pytest.mark.trio
async def test_block_create_and_read(alice_ws, realm):
    await block_create(alice_ws, BLOCK_ID, realm, BLOCK_DATA)

    rep = await block_read(alice_ws, BLOCK_ID)
    assert rep == BlockReadRepOk(BLOCK_DATA)

    # Test not found as well

    dummy_id = BlockID.from_hex("00000000000000000000000000000002")
    rep = await block_read(alice_ws, dummy_id)
    assert isinstance(rep, BlockReadRepNotFound)


@pytest.mark.trio
@customize_fixtures(blockstore_mode="RAID1")
async def test_raid1_block_create_and_read(alice_ws, realm):
    await test_block_create_and_read(alice_ws, realm)


@pytest.mark.trio
@customize_fixtures(blockstore_mode="RAID1")
async def test_raid1_block_create_partial_failure(caplog, alice_ws, backend, realm):
    async def mock_create(organization_id, id, block):
        await trio.sleep(0)
        raise BlockStoreError()

    backend.blockstore.blockstores[1].create = mock_create

    rep = await block_create(alice_ws, BLOCK_ID, realm, BLOCK_DATA, check_rep=False)
    assert isinstance(rep, BlockCreateRepTimeout)

    log = caplog.assert_occurred_once("[warning  ] Block create error: A node have failed")
    assert f"organization_id=CoolOrg" in log
    assert f"block_id={BLOCK_ID.hex}" in log


@pytest.mark.trio
@customize_fixtures(blockstore_mode="RAID1_PARTIAL_CREATE_OK")
async def test_raid1_partial_create_ok_block_create_partial_failure(alice_ws, backend, realm):
    async def mock_create(organization_id, id, block):
        await trio.sleep(0)
        raise BlockStoreError()

    backend.blockstore.blockstores[1].create = mock_create

    rep = await block_create(alice_ws, BLOCK_ID, realm, BLOCK_DATA)
    assert isinstance(rep, BlockCreateRepOk)

    rep = await block_read(alice_ws, BLOCK_ID)
    assert rep == BlockReadRepOk(BLOCK_DATA)


@pytest.mark.trio
@customize_fixtures(blockstore_mode="RAID1")
async def test_raid1_block_create_partial_exists(alice_ws, alice, backend, realm):
    await backend.blockstore.blockstores[1].create(alice.organization_id, BLOCK_ID, BLOCK_DATA)
    # Blockstore overwrite existing block without questions
    await block_create(alice_ws, BLOCK_ID, realm, BLOCK_DATA)


@pytest.mark.trio
@customize_fixtures(blockstore_mode="RAID1")
async def test_raid1_block_read_partial_failure(alice_ws, backend, block):
    async def mock_read(organization_id, id):
        await trio.sleep(0)
        raise BlockStoreError()

    backend.blockstore.blockstores[1].read = mock_read

    rep = await block_read(alice_ws, block)
    assert rep == BlockReadRepOk(BLOCK_DATA)


@pytest.mark.trio
@customize_fixtures(blockstore_mode="RAID0")
async def test_raid0_block_create_and_read(alice_ws, realm):
    await test_block_create_and_read(alice_ws, realm)


@pytest.mark.trio
@customize_fixtures(blockstore_mode="RAID5")
async def test_raid5_block_create_and_read(alice_ws, realm):
    await test_block_create_and_read(alice_ws, realm)


@pytest.mark.trio
@customize_fixtures(blockstore_mode="RAID5")
@pytest.mark.parametrize("failing_blockstore", (0, 1, 2))
async def test_raid5_block_create_single_failure(
    caplog, alice_ws, backend, realm, failing_blockstore
):
    async def mock_create(organization_id, id, block):
        await trio.sleep(0)
        raise BlockStoreError()

    backend.blockstore.blockstores[failing_blockstore].create = mock_create

    rep = await block_create(alice_ws, BLOCK_ID, realm, BLOCK_DATA, check_rep=False)
    assert isinstance(rep, BlockCreateRepTimeout)

    log = caplog.assert_occurred_once("[warning  ] Block create error: A node have failed")
    assert f"organization_id=CoolOrg" in log
    assert f"block_id={BLOCK_ID.hex}" in log


@pytest.mark.trio
@customize_fixtures(blockstore_mode="RAID5_PARTIAL_CREATE_OK")
@pytest.mark.parametrize("failing_blockstore", (0, 1, 2))
async def test_raid5_partial_create_ok_block_create_single_failure(
    caplog, alice_ws, backend, realm, failing_blockstore
):
    async def mock_create(organization_id, id, block):
        await trio.sleep(0)
        raise BlockStoreError()

    backend.blockstore.blockstores[failing_blockstore].create = mock_create

    await block_create(alice_ws, BLOCK_ID, realm, BLOCK_DATA)

    rep = await block_read(alice_ws, BLOCK_ID)
    assert rep == BlockReadRepOk(BLOCK_DATA)


@pytest.mark.trio
@customize_fixtures(blockstore_mode="RAID5_PARTIAL_CREATE_OK")
@pytest.mark.parametrize("failing_blockstores", [(0, 1), (0, 2)])
async def test_raid5_partial_create_ok_block_create_too_many_failures(
    caplog, alice_ws, backend, realm, failing_blockstores
):
    async def mock_create(organization_id, id, block):
        await trio.sleep(0)
        raise BlockStoreError()

    fb1, fb2 = failing_blockstores

    backend.blockstore.blockstores[fb1].create = mock_create
    backend.blockstore.blockstores[fb2].create = mock_create

    rep = await block_create(alice_ws, BLOCK_ID, realm, BLOCK_DATA, check_rep=False)
    assert isinstance(rep, BlockCreateRepTimeout)

    log = caplog.assert_occurred_once(
        "[warning  ] Block create error: More than 1 nodes have failed"
    )
    assert f"organization_id=CoolOrg" in log
    assert f"block_id={BLOCK_ID.hex}" in log


@pytest.mark.trio
@customize_fixtures(blockstore_mode="RAID5")
async def test_raid5_block_create_partial_exists(alice_ws, alice, backend, realm):
    await backend.blockstore.blockstores[1].create(alice.organization_id, BLOCK_ID, BLOCK_DATA)

    await block_create(alice_ws, BLOCK_ID, realm, BLOCK_DATA)


@pytest.mark.trio
@customize_fixtures(blockstore_mode="RAID5")
@pytest.mark.parametrize("failing_blockstore", (0, 1))  # Ignore checksum blockstore
async def test_raid5_block_read_single_failure(alice_ws, backend, block, failing_blockstore):
    async def mock_read(organization_id, id):
        await trio.sleep(0)
        raise BlockStoreError()

    backend.blockstore.blockstores[failing_blockstore].read = mock_read

    rep = await block_read(alice_ws, block)
    assert rep == BlockReadRepOk(BLOCK_DATA)


@pytest.mark.trio
@customize_fixtures(blockstore_mode="RAID5")
@pytest.mark.parametrize("bad_chunk", (b"", b"too big"))
async def test_raid5_block_read_single_invalid_chunk_size(
    alice_ws, alice, backend, block, bad_chunk
):
    async def mock_read(organization_id, id):
        return bad_chunk

    backend.blockstore.blockstores[1].read = mock_read

    rep = await block_read(alice_ws, block)
    # A bad chunk result in a bad block, which should be detected by the client
    assert isinstance(rep, BlockReadRepOk)


@pytest.mark.trio
@customize_fixtures(blockstore_mode="RAID5")
@pytest.mark.parametrize("failing_blockstores", [(0, 1), (0, 2)])
async def test_raid5_block_read_multiple_failure(
    caplog, alice_ws, backend, block, failing_blockstores
):
    async def mock_read(organization_id, id):
        await trio.sleep(0)
        raise BlockStoreError()

    fb1, fb2 = failing_blockstores
    backend.blockstore.blockstores[fb1].read = mock_read
    backend.blockstore.blockstores[fb2].read = mock_read

    rep = await block_read(alice_ws, block)
    assert isinstance(rep, BlockReadRepTimeout)

    log = caplog.assert_occurred_once("[warning  ] Block read error: More than 1 nodes have failed")
    assert f"organization_id=CoolOrg" in log
    assert f"block_id={block.hex}" in log


@pytest.mark.parametrize(
    "bad_msg",
    [
        {},
        {"id": BLOCK_ID.hex, "block": BLOCK_DATA, "bad_field": "foo"},
        {"id": "not an uuid", "block": BLOCK_DATA},
        {"id": 42, "block": BLOCK_DATA},
        {"id": None, "block": BLOCK_DATA},
        {"id": BLOCK_ID.hex, "block": 42},
        {"id": BLOCK_ID.hex, "block": None},
        {"block": BLOCK_DATA},
    ],
)
@pytest.mark.trio
async def test_block_create_bad_msg(alice_ws, bad_msg):
    await alice_ws.send(packb({"cmd": "block_create", **bad_msg}))
    raw_rep = await alice_ws.receive()
    rep = block_create_serializer.rep_loads(raw_rep)
    assert rep.status == "bad_message"


@pytest.mark.trio
async def test_block_read_not_found(alice_ws):
    rep = await block_read(alice_ws, BLOCK_ID)
    assert isinstance(rep, BlockReadRepNotFound)


@pytest.mark.parametrize(
    "bad_msg",
    [
        {"id": BLOCK_ID.hex, "bad_field": "foo"},
        {"id": "not_an_uuid"},
        {"id": 42},
        {"id": None},
        {},
    ],
)
@pytest.mark.trio
async def test_block_read_bad_msg(alice_ws, bad_msg):
    await alice_ws.send(packb({"cmd": "block_read", **bad_msg}))
    raw_rep = await alice_ws.receive()
    # Valid ID doesn't exists in database but this is ok given here we test
    # another layer so it's not important as long as we get our
    # `bad_message` status
    rep = block_read_serializer.rep_loads(raw_rep)
    assert rep.status == "bad_message"


@pytest.mark.trio
async def test_block_conflicting_id(alice_ws, realm):
    block_v1 = b"v1"
    await block_create(alice_ws, BLOCK_ID, realm, block_v1)

    block_v2 = b"v2"
    rep = await block_create(alice_ws, BLOCK_ID, realm, block_v2, check_rep=False)
    assert isinstance(rep, BlockCreateRepAlreadyExists)

    rep = await block_read(alice_ws, BLOCK_ID)
    assert rep == BlockReadRepOk(block_v1)


@pytest.mark.trio
async def test_block_check_other_organization(
    backend_asgi_app, ws_from_other_organization_factory, realm, block
):
    async with ws_from_other_organization_factory(backend_asgi_app) as sock:
        rep = await block_read(sock, block)
        assert isinstance(rep, BlockReadRepNotFound)

        await backend_asgi_app.backend.realm.create(
            sock.device.organization_id,
            RealmGrantedRole(
                certificate=b"<dummy>",
                realm_id=realm,
                user_id=sock.device.user_id,
                role=RealmRole.OWNER,
                granted_by=sock.device.device_id,
                granted_on=DateTime.now(),
            ),
        )
        await block_create(sock, block, realm, b"other org data")

        rep = await block_read(sock, block)
        assert rep == BlockReadRepOk(b"other org data")


@pytest.mark.trio
async def test_access_during_maintenance(backend, alice, alice_ws, realm, block):
    await backend.realm.start_reencryption_maintenance(
        alice.organization_id,
        alice.device_id,
        realm,
        2,
        {alice.user_id: b"whatever"},
        DateTime.now(),
    )
    rep = await block_create(alice_ws, BLOCK_ID, realm, BLOCK_DATA, check_rep=False)
    assert isinstance(rep, BlockCreateRepInMaintenance)

    # Reading while in reencryption is OK
    rep = await block_read(alice_ws, block)
    assert rep == BlockReadRepOk(BLOCK_DATA)


@given(block=st.binary(max_size=2**8), nb_blockstores=st.integers(min_value=3, max_value=16))
def test_split_block(block, nb_blockstores):
    nb_chunks = nb_blockstores - 1
    chunks = split_block_in_chunks(block, nb_chunks)
    assert len(chunks) == nb_chunks

    chunk_size = len(chunks[0])
    for chunk in chunks[1:]:
        assert len(chunk) == chunk_size

    rebuilt = rebuild_block_from_chunks(chunks, None)
    assert rebuilt == block

    checksum_chunk = generate_checksum_chunk(chunks)
    for missing in range(len(chunks)):
        partial_chunks = chunks.copy()
        partial_chunks[missing] = None
        rebuilt = rebuild_block_from_chunks(partial_chunks, checksum_chunk)
        assert rebuilt == block
