# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import BlockID, DateTime, authenticated_cmds
from parsec.components.blockstore import BlockStoreReadBadOutcome
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    get_last_realm_certificate_timestamp,
    wksp1_bob_becomes_owner_and_changes_alice,
)


async def test_authenticated_block_read_ok(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    block_id = BlockID.new()
    block = b"<block content>"
    wksp1_last_certificate_timestamp = get_last_realm_certificate_timestamp(
        testbed_template=coolorg.testbed_template,
        realm_id=coolorg.wksp1_id,
    )

    await backend.block.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        block_id=block_id,
        realm_id=coolorg.wksp1_id,
        key_index=1,
        block=block,
    )

    rep = await coolorg.alice.block_read(block_id)
    assert rep == authenticated_cmds.v4.block_read.RepOk(
        block=block,
        key_index=1,
        needed_realm_certificate_timestamp=wksp1_last_certificate_timestamp,
    )


async def test_authenticated_block_read_block_not_found(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    block_id = BlockID.new()

    rep = await coolorg.alice.block_read(block_id)
    assert rep == authenticated_cmds.v4.block_read.RepBlockNotFound()


@pytest.mark.parametrize("kind", ("never_allowed", "no_longer_allowed"))
async def test_authenticated_block_read_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
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

    match kind:
        case "never_allowed":
            author = coolorg.mallory

        case "no_longer_allowed":
            await wksp1_bob_becomes_owner_and_changes_alice(
                coolorg=coolorg, backend=backend, new_alice_role=None
            )
            author = coolorg.alice

        case unknown:
            assert False, unknown

    rep = await author.block_read(block_id)
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
        key_index=1,
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
    monkeypatch.setattr(
        "parsec.components.postgresql.block.PGBlockStoreComponent.read", mocked_blockstore_read
    )

    rep = await coolorg.alice.block_read(block_id)
    assert rep == authenticated_cmds.v4.block_read.RepStoreUnavailable()


async def test_authenticated_block_read_http_common_errors(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
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

    async def do():
        await coolorg.alice.block_read(block_id)

    await authenticated_http_common_errors_tester(do)
