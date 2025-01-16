# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import (
    BlockID,
    DateTime,
    RealmRole,
    VlobID,
    authenticated_cmds,
)
from parsec.components.block import BlockReadBadOutcome, BlockReadResult
from parsec.components.blockstore import BlockStoreCreateBadOutcome
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    get_last_realm_certificate_timestamp,
    wksp1_alice_gives_role,
    wksp1_bob_becomes_owner_and_changes_alice,
)


@pytest.mark.parametrize(
    "kind",
    (
        "as_owner",
        "as_manager",
        "as_contributor",
    ),
)
async def test_authenticated_block_create_ok(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    match kind:
        case "as_owner":
            author = coolorg.alice

        case "as_manager":
            last_realm_certificate_timestamp = DateTime.now()
            await wksp1_alice_gives_role(
                coolorg,
                backend,
                coolorg.bob.user_id,
                RealmRole.MANAGER,
                now=last_realm_certificate_timestamp,
            )
            author = coolorg.bob

        case "as_contributor":
            last_realm_certificate_timestamp = DateTime.now()
            await wksp1_alice_gives_role(
                coolorg,
                backend,
                coolorg.bob.user_id,
                RealmRole.CONTRIBUTOR,
                now=last_realm_certificate_timestamp,
            )
            author = coolorg.bob

        case unknown:
            assert False, unknown

    block_id = BlockID.new()
    block = b"<block content>"

    expected_dump = await backend.block.test_dump_blocks(coolorg.organization_id)
    expected_dump[block_id] = (ANY, author.device_id, coolorg.wksp1_id, 1, len(block))

    rep = await author.block_create(
        block_id=block_id, realm_id=coolorg.wksp1_id, key_index=1, block=block
    )
    assert rep == authenticated_cmds.latest.block_create.RepOk()

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

    assert rep == authenticated_cmds.latest.block_create.RepBadKeyIndex(
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
    assert rep == authenticated_cmds.latest.block_create.RepRealmNotFound()

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
    assert rep == authenticated_cmds.latest.block_create.RepBlockAlreadyExists()

    dump = await backend.block.test_dump_blocks(coolorg.organization_id)
    assert dump == expected_dump  # No changes!


@pytest.mark.parametrize("kind", ("as_reader", "never_allowed", "no_longer_allowed"))
async def test_authenticated_block_create_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    match kind:
        case "as_reader":
            author = coolorg.bob

        case "never_allowed":
            author = coolorg.mallory

        case "no_longer_allowed":
            await wksp1_bob_becomes_owner_and_changes_alice(
                coolorg=coolorg, backend=backend, new_alice_role=RealmRole.READER
            )
            author = coolorg.alice

        case unknown:
            assert False, unknown

    block_id = BlockID.new()
    block = b"<block content>"

    expected_dump = await backend.block.test_dump_blocks(coolorg.organization_id)

    rep = await author.block_create(
        block_id=block_id, realm_id=coolorg.wksp1_id, key_index=1, block=block
    )
    assert rep == authenticated_cmds.latest.block_create.RepAuthorNotAllowed()

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
    assert rep == authenticated_cmds.latest.block_create.RepStoreUnavailable()

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


async def test_authenticated_block_create_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.block_create(
            block_id=BlockID.new(), realm_id=coolorg.wksp1_id, key_index=1, block=b"<block content>"
        )

    await authenticated_http_common_errors_tester(do)
