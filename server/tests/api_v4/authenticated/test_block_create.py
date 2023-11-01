# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

from parsec._parsec import BlockID, DateTime, authenticated_cmds
from tests.common import Backend, CoolorgRpcClients


async def test_authenticated_block_create_ok(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    block_id = BlockID.new()
    block = b"<block content>"

    expected_dump = await backend.block.test_dump_blocks(coolorg.organization_id)
    expected_dump[block_id] = (ANY, coolorg.alice.device_id, coolorg.wksp1_id, len(block))

    rep = await coolorg.alice.block_create(block_id, coolorg.wksp1_id, block)
    assert rep == authenticated_cmds.v4.block_create.RepOk()

    content = await backend.block.read_as_user(
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id.user_id,
        block_id=block_id,
    )
    assert content == block

    dump = await backend.block.test_dump_blocks(coolorg.organization_id)
    assert dump == expected_dump


async def test_authenticated_block_create_already_exists(
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

    expected_dump = await backend.block.test_dump_blocks(coolorg.organization_id)

    rep = await coolorg.alice.block_create(block_id, coolorg.wksp1_id, block)
    assert rep == authenticated_cmds.v4.block_create.RepBlockAlreadyExists()

    dump = await backend.block.test_dump_blocks(coolorg.organization_id)
    assert dump == expected_dump  # No changes !


async def test_authenticated_block_create_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    block_id = BlockID.new()
    block = b"<block content>"

    expected_dump = await backend.block.test_dump_blocks(coolorg.organization_id)

    rep = await coolorg.mallory.block_create(block_id, coolorg.wksp1_id, block)
    assert rep == authenticated_cmds.v4.block_create.RepAuthorNotAllowed()

    dump = await backend.block.test_dump_blocks(coolorg.organization_id)
    assert dump == expected_dump  # No changes !
