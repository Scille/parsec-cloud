# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import BlockID, DateTime, RealmRole, VlobID, authenticated_cmds
from parsec.components.blockstore import BlockStoreReadBadOutcome
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
        "as_reader",
        "as_contributor",
        "as_manager",
        "as_owner",
    ),
)
async def test_authenticated_block_read_ok(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    last_realm_certificate_timestamp = get_last_realm_certificate_timestamp(
        testbed_template=coolorg.testbed_template,
        realm_id=coolorg.wksp1_id,
    )

    match kind:
        case "as_reader":
            author = coolorg.bob

        case "as_contributor":
            last_realm_certificate_timestamp = DateTime(2019, 1, 1)
            await wksp1_alice_gives_role(
                coolorg,
                backend,
                coolorg.bob.user_id,
                RealmRole.CONTRIBUTOR,
                now=last_realm_certificate_timestamp,
            )
            author = coolorg.bob

        case "as_manager":
            last_realm_certificate_timestamp = DateTime(2019, 1, 1)
            await wksp1_alice_gives_role(
                coolorg,
                backend,
                coolorg.bob.user_id,
                RealmRole.MANAGER,
                now=last_realm_certificate_timestamp,
            )
            author = coolorg.bob

        case "as_owner":
            author = coolorg.alice

        case unknown:
            assert False, unknown

    realm_id = coolorg.wksp1_id
    block_id = BlockID.new()
    block = b"<block content>"

    await backend.block.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        block_id=block_id,
        realm_id=realm_id,
        key_index=1,
        block=block,
    )

    rep = await author.block_read(realm_id, block_id)
    assert rep == authenticated_cmds.latest.block_read.RepOk(
        block=block,
        key_index=1,
        needed_realm_certificate_timestamp=last_realm_certificate_timestamp,
    )


async def test_authenticated_block_read_realm_not_found(
    coolorg: CoolorgRpcClients,
):
    realm_id = VlobID.new()
    block_id = BlockID.new()
    await coolorg.alice.block_read(realm_id, block_id)


async def test_authenticated_block_read_block_not_found(coolorg: CoolorgRpcClients) -> None:
    realm_id = coolorg.wksp1_id
    block_id = BlockID.new()

    rep = await coolorg.alice.block_read(realm_id, block_id)
    assert rep == authenticated_cmds.latest.block_read.RepBlockNotFound()


@pytest.mark.parametrize("kind", ("never_allowed", "no_longer_allowed"))
async def test_authenticated_block_read_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    realm_id = coolorg.wksp1_id
    block_id = BlockID.new()
    block = b"<block content>"

    await backend.block.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        block_id=block_id,
        realm_id=realm_id,
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

    rep = await author.block_read(realm_id, block_id)
    assert rep == authenticated_cmds.latest.block_read.RepAuthorNotAllowed()


@pytest.mark.parametrize("kind", ("store_unavailable", "block_not_found"))
async def test_authenticated_block_read_store_unavailable(
    coolorg: CoolorgRpcClients, backend: Backend, monkeypatch: pytest.MonkeyPatch, kind: str
) -> None:
    realm_id = coolorg.wksp1_id
    block_id = BlockID.new()
    block = b"<block content>"

    await backend.block.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        block_id=block_id,
        realm_id=realm_id,
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

    rep = await coolorg.alice.block_read(realm_id, block_id)
    assert rep == authenticated_cmds.latest.block_read.RepStoreUnavailable()


async def test_authenticated_block_read_http_common_errors(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    realm_id = coolorg.wksp1_id
    block_id = BlockID.new()
    block = b"<block content>"

    await backend.block.create(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        block_id=block_id,
        realm_id=realm_id,
        key_index=1,
        block=block,
    )

    async def do():
        await coolorg.alice.block_read(realm_id, block_id)

    await authenticated_http_common_errors_tester(do)
