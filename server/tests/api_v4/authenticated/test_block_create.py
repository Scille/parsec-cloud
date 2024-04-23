# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import BlockID, DateTime, VlobID, authenticated_cmds
from parsec.components.block import BlockReadBadOutcome, BlockReadResult
from parsec.components.blockstore import BlockStoreCreateBadOutcome
from tests.common import Backend, CoolorgRpcClients, get_last_realm_certificate_timestamp


async def test_authenticated_block_create_ok(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    block_id = BlockID.new()
    block = b"<block content>"

    expected_dump = await backend.block.test_dump_blocks(coolorg.organization_id)
    expected_dump[block_id] = (ANY, coolorg.alice.device_id, coolorg.wksp1_id, 1, len(block))

    rep = await coolorg.alice.block_create(
        block_id=block_id, realm_id=coolorg.wksp1_id, key_index=1, block=block
    )
    assert rep == authenticated_cmds.v4.block_create.RepOk()

    content = await backend.block.read(
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        block_id=block_id,
    )
    assert isinstance(content, BlockReadResult)
    assert content.block == block
    assert content.key_index == 1

    dump = await backend.block.test_dump_blocks(coolorg.organization_id)
    assert dump == expected_dump


async def test_authenticated_block_create_bad_key_index(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    block_id = BlockID.new()
    block = b"<block content>"
    wksp1_last_certificate_timestamp = get_last_realm_certificate_timestamp(
        testbed_template=coolorg.testbed_template,
        realm_id=coolorg.wksp1_id,
    )

    expected_dump = await backend.block.test_dump_blocks(coolorg.organization_id)
    expected_dump[block_id] = (ANY, coolorg.alice.device_id, coolorg.wksp1_id, 1, len(block))

    rep = await coolorg.alice.block_create(
        block_id=block_id, realm_id=coolorg.wksp1_id, key_index=42, block=block
    )

    assert rep == authenticated_cmds.v4.block_create.RepBadKeyIndex(
        last_realm_certificate_timestamp=wksp1_last_certificate_timestamp,
    )


async def test_authenticated_block_create_realm_not_found(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    block_id = BlockID.new()
    block = b"<block content>"

    # Try to create block with an invalid realm ID
    rep = await coolorg.alice.block_create(
        block_id=block_id, realm_id=VlobID.new(), key_index=1, block=block
    )
    assert rep == authenticated_cmds.v4.block_create.RepRealmNotFound()

    # Block should not exist
    # This check is not strictly needed since the block existence
    # is already checked below with the test_dump_blocks call.
    # Anyway, let's leave it as an extra cheap check.
    content = await backend.block.read(
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        block_id=block_id,
    )
    assert content == BlockReadBadOutcome.BLOCK_NOT_FOUND

    dump = await backend.block.test_dump_blocks(coolorg.organization_id)
    assert not dump  # No changes!


async def test_authenticated_block_create_block_already_exists(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    block_id = BlockID.new()
    block = b"<block content>"

    await backend.block.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        block_id=block_id,
        realm_id=coolorg.wksp1_id,
        key_index=1,
        block=block,
    )

    expected_dump = await backend.block.test_dump_blocks(coolorg.organization_id)

    rep = await coolorg.alice.block_create(
        block_id=block_id, realm_id=coolorg.wksp1_id, key_index=1, block=block
    )
    assert rep == authenticated_cmds.v4.block_create.RepBlockAlreadyExists()

    dump = await backend.block.test_dump_blocks(coolorg.organization_id)
    assert dump == expected_dump  # No changes!


async def test_authenticated_block_create_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    block_id = BlockID.new()
    block = b"<block content>"

    expected_dump = await backend.block.test_dump_blocks(coolorg.organization_id)

    rep = await coolorg.mallory.block_create(
        block_id=block_id, realm_id=coolorg.wksp1_id, key_index=1, block=block
    )
    assert rep == authenticated_cmds.v4.block_create.RepAuthorNotAllowed()

    dump = await backend.block.test_dump_blocks(coolorg.organization_id)
    assert dump == expected_dump  # No changes!


async def test_authenticated_block_create_store_unavailable(
    coolorg: CoolorgRpcClients, backend: Backend, monkeypatch: pytest.MonkeyPatch
) -> None:
    block_id = BlockID.new()
    block = b"<block content>"

    async def mocked_blockstore_create(*args, **kwargs):
        return BlockStoreCreateBadOutcome.STORE_UNAVAILABLE

    monkeypatch.setattr(
        "parsec.components.memory.MemoryBlockStoreComponent.create", mocked_blockstore_create
    )
    monkeypatch.setattr(
        "parsec.components.postgresql.block.PGBlockStoreComponent.create", mocked_blockstore_create
    )

    rep = await coolorg.alice.block_create(
        block_id=block_id, realm_id=coolorg.wksp1_id, key_index=1, block=block
    )
    assert rep == authenticated_cmds.v4.block_create.RepStoreUnavailable()

    # Block should not exist
    # This check is not strictly needed since the block existence
    # is already checked below with the test_dump_blocks call.
    # Anyway, let's leave it as an extra cheap check.
    content = await backend.block.read(
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        block_id=block_id,
    )
    assert content == BlockReadBadOutcome.BLOCK_NOT_FOUND

    dump = await backend.block.test_dump_blocks(coolorg.organization_id)
    assert not dump  # No changes!
