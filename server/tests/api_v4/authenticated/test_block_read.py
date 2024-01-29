# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import BlockID, DateTime, authenticated_cmds
from parsec.components.blockstore import BlockStoreReadBadOutcome
from tests.common import Backend, CoolorgRpcClients


async def test_authenticated_block_read_ok(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    block_id = BlockID.new()
    block = b"<block content>"

    await backend.block.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        block_id=block_id,
        realm_id=coolorg.wksp1_id,
        block=block,
    )

    rep = await coolorg.alice.block_read(block_id)
    assert rep == authenticated_cmds.v4.block_read.RepOk(block=block)


async def test_authenticated_block_read_block_not_found(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    block_id = BlockID.new()

    rep = await coolorg.alice.block_read(block_id)
    assert rep == authenticated_cmds.v4.block_read.RepBlockNotFound()


async def test_authenticated_block_read_author_not_allowed(
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
        block=block,
    )

    rep = await coolorg.mallory.block_read(block_id)
    assert rep == authenticated_cmds.v4.block_read.RepAuthorNotAllowed()


@pytest.mark.parametrize("kind", ("store_unavailable", "block_not_found"))
async def test_authenticated_block_read_store_unavailable(
    coolorg: CoolorgRpcClients, backend: Backend, monkeypatch: pytest.MonkeyPatch, kind: str
) -> None:
    block_id = BlockID.new()
    block = b"<block content>"

    await backend.block.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        block_id=block_id,
        realm_id=coolorg.wksp1_id,
        block=block,
    )

    async def mocked_blockstore_read(*args, **kwargs):
        match kind:
            case "store_unavailable":
                return BlockStoreReadBadOutcome.STORE_UNAVAILABLE
            case "block_not_found":
                return BlockStoreReadBadOutcome.BLOCK_NOT_FOUND
            case _:
                assert False

    monkeypatch.setattr(
        "parsec.components.memory.MemoryBlockStoreComponent.read", mocked_blockstore_read
    )

    rep = await coolorg.alice.block_read(block_id)
    assert rep == authenticated_cmds.v4.block_read.RepStoreUnavailable()
