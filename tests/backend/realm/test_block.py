# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import trio
import pytest
from unittest.mock import ANY
import pendulum
from uuid import UUID, uuid4

from parsec.backend.block import BlockTimeoutError
from parsec.backend.realm import RealmGrantedRole
from parsec.backend.raid5_blockstore import (
    split_block_in_chunks,
    generate_checksum_chunk,
    rebuild_block_from_chunks,
)
from parsec.api.protocole import block_create_serializer, block_read_serializer, packb, RealmRole

from hypothesis import given, strategies as st


BLOCK_ID = UUID("00000000000000000000000000000001")
VLOB_ID = UUID("00000000000000000000000000000002")
BLOCK_DATA = b"Hodi ho !"


@pytest.fixture
def get_raid5_records_msg(caplog):
    # Caplog doesn't works with logging configuration on windows
    if os.name == "nt":

        def _get_raid5_records_msg():
            return ANY

    else:

        def _get_raid5_records_msg():
            msgs = [
                record.msg
                for record in caplog.records
                if isinstance(record.msg, dict)
                and record.msg["logger"] == "parsec.backend.raid5_blockstore"
            ]
            return sorted(msgs, key=lambda msg: msg["event"])

    return _get_raid5_records_msg


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
            alice.organization_id,
            RealmGrantedRole(
                certificate=b"<dummy>",
                realm_id=realm,
                user_id=bob.user_id,
                role=role,
                granted_by=alice.device_id,
            ),
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
            alice.organization_id,
            RealmGrantedRole(
                certificate=b"<dummy>",
                realm_id=realm,
                user_id=bob.user_id,
                role=role,
                granted_by=alice.device_id,
            ),
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
@pytest.mark.parametrize("failing_blockstore", (0, 1, 2))
async def test_raid5_block_create_single_failure(
    get_raid5_records_msg, alice_backend_sock, backend, realm, failing_blockstore
):
    async def mock_create(organization_id, id, block):
        await trio.sleep(0)
        raise BlockTimeoutError()

    backend.blockstore.blockstores[failing_blockstore].create = mock_create

    rep = await block_create(alice_backend_sock, BLOCK_ID, realm, BLOCK_DATA)
    assert rep == {"status": "ok"}

    # Should be notified of blockstore malfunction
    assert get_raid5_records_msg() == [
        {
            "logger": "parsec.backend.raid5_blockstore",
            "level": "warning",
            "timestamp": ANY,
            "event": f"Cannot reach RAID5 blockstore #{failing_blockstore} to create block {BLOCK_ID}",
            "exception": ANY,
        }
    ]


@pytest.mark.trio
@pytest.mark.raid5_blockstore
@pytest.mark.parametrize("failing_blockstores", [(0, 1), (0, 2)])
async def test_raid5_block_create_multiple_failure(
    get_raid5_records_msg, alice_backend_sock, backend, realm, failing_blockstores
):
    async def mock_create(organization_id, id, block):
        await trio.sleep(0)
        raise BlockTimeoutError()

    fb1, fb2 = failing_blockstores

    backend.blockstore.blockstores[fb1].create = mock_create
    backend.blockstore.blockstores[fb2].create = mock_create

    rep = await block_create(alice_backend_sock, BLOCK_ID, realm, BLOCK_DATA)
    assert rep == {"status": "timeout"}

    # Should be notified of blockstore malfunction
    assert get_raid5_records_msg() == [
        {
            "logger": "parsec.backend.raid5_blockstore",
            "level": "error",
            "timestamp": ANY,
            "event": f"Block {BLOCK_ID} cannot be created: Too many failing blockstores in the RAID5 cluster",
        },
        {
            "logger": "parsec.backend.raid5_blockstore",
            "level": "warning",
            "timestamp": ANY,
            "event": f"Cannot reach RAID5 blockstore #{fb1} to create block {BLOCK_ID}",
            "exception": ANY,
        },
        {
            "logger": "parsec.backend.raid5_blockstore",
            "level": "warning",
            "timestamp": ANY,
            "event": f"Cannot reach RAID5 blockstore #{fb2} to create block {BLOCK_ID}",
            "exception": ANY,
        },
    ]


@pytest.mark.trio
@pytest.mark.raid5_blockstore
async def test_raid5_block_create_partial_exists(alice_backend_sock, alice, backend, realm):
    await backend.blockstore.blockstores[1].create(alice.organization_id, BLOCK_ID, BLOCK_DATA)

    rep = await block_create(alice_backend_sock, BLOCK_ID, realm, BLOCK_DATA)
    assert rep == {"status": "ok"}


@pytest.mark.trio
@pytest.mark.raid5_blockstore
@pytest.mark.parametrize("failing_blockstore", (0, 1))  # Ignore checksum blockstore
async def test_raid5_block_read_single_failure(
    get_raid5_records_msg, alice_backend_sock, alice, backend, block, failing_blockstore
):
    async def mock_read(organization_id, id):
        await trio.sleep(0)
        raise BlockTimeoutError()

    backend.blockstore.blockstores[failing_blockstore].read = mock_read

    rep = await block_read(alice_backend_sock, block)
    assert rep == {"status": "ok", "block": BLOCK_DATA}

    # Should be notified of blockstore malfunction
    assert get_raid5_records_msg() == [
        {
            "logger": "parsec.backend.raid5_blockstore",
            "level": "warning",
            "timestamp": ANY,
            "event": f"Cannot reach RAID5 blockstore #{failing_blockstore} to read block {block}",
            "exception": ANY,
        }
    ]


@pytest.mark.trio
@pytest.mark.raid5_blockstore
@pytest.mark.parametrize("failing_blockstores", [(0, 1), (0, 2)])
async def test_raid5_block_read_multiple_failure(
    get_raid5_records_msg, alice_backend_sock, alice, backend, block, failing_blockstores
):
    async def mock_read(organization_id, id):
        await trio.sleep(0)
        raise BlockTimeoutError()

    fb1, fb2 = failing_blockstores
    backend.blockstore.blockstores[fb1].read = mock_read
    backend.blockstore.blockstores[fb2].read = mock_read

    rep = await block_read(alice_backend_sock, block)
    assert rep == {"status": "timeout"}

    # Should be notified of blockstore malfunction
    assert get_raid5_records_msg() == [
        {
            "logger": "parsec.backend.raid5_blockstore",
            "level": "error",
            "timestamp": ANY,
            "event": f"Block {block} cannot be read: Too many failing blockstores in the RAID5 cluster",
        },
        {
            "logger": "parsec.backend.raid5_blockstore",
            "level": "warning",
            "timestamp": ANY,
            "event": f"Cannot reach RAID5 blockstore #{fb1} to read block {block}",
            "exception": ANY,
        },
        {
            "logger": "parsec.backend.raid5_blockstore",
            "level": "warning",
            "timestamp": ANY,
            "event": f"Cannot reach RAID5 blockstore #{fb2} to read block {block}",
            "exception": ANY,
        },
    ]


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

        await backend.realm.create(
            sock.device.organization_id,
            RealmGrantedRole(
                certificate=b"<dummy>",
                realm_id=realm,
                user_id=sock.device.user_id,
                role=RealmRole.OWNER,
                granted_by=sock.device.device_id,
            ),
        )
        rep = await block_create(sock, block, realm, b"other org data")
        assert rep == {"status": "ok"}

        rep = await block_read(sock, block)
        assert rep == {"status": "ok", "block": b"other org data"}


@pytest.mark.trio
async def test_access_during_maintenance(backend, alice, alice_backend_sock, realm, block):
    await backend.realm.start_reencryption_maintenance(
        alice.organization_id,
        alice.device_id,
        realm,
        2,
        {alice.user_id: b"whatever"},
        pendulum.now(),
    )

    rep = await block_create(alice_backend_sock, BLOCK_ID, realm, BLOCK_DATA)
    assert rep == {"status": "in_maintenance"}

    rep = await block_read(alice_backend_sock, block)
    assert rep == {"status": "in_maintenance"}


@given(block=st.binary(max_size=2 ** 8), nb_blockstores=st.integers(min_value=3, max_value=16))
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
