# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import BlockID, RealmRole, UserProfile, VlobID, authenticated_cmds
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    alice_gives_profile,
    wksp1_alice_gives_role,
)


async def _wksp1_metadata_size(coolorg: CoolorgRpcClients, backend: Backend) -> int:
    dump = await backend.vlob.test_dump_vlobs(organization_id=coolorg.organization_id)
    return sum(
        len(blob) for versions in dump[coolorg.wksp1_id].values() for (_, _, blob) in versions
    )


def _wksp1_stats(
    realm_id: VlobID,
    metadata_size: int,
    data_size: int = 0,
    owners: int = 0,
    managers: int = 0,
    contributors: int = 0,
    readers: int = 0,
) -> authenticated_cmds.latest.realm_list_stats.RealmStats:
    return authenticated_cmds.latest.realm_list_stats.RealmStats(
        realm_id=realm_id,
        metadata_size=metadata_size,
        data_size=data_size,
        owners=owners,
        managers=managers,
        contributors=contributors,
        readers=readers,
    )


async def test_authenticated_realm_list_stats_ok(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    wksp1_metadata_size = await _wksp1_metadata_size(coolorg, backend)
    assert wksp1_metadata_size > 0

    # Note Alice's and Bob's user realms are not listed (they have no key rotation)
    rep = await coolorg.alice.realm_list_stats()
    assert rep == authenticated_cmds.latest.realm_list_stats.RepOk(
        realms=[
            _wksp1_stats(
                coolorg.wksp1_id,
                metadata_size=wksp1_metadata_size,
                owners=1,  # Alice
                readers=1,  # Bob
            )
        ]
    )

    # Add some data...

    rep = await coolorg.alice.block_create(
        block_id=BlockID.new(), realm_id=coolorg.wksp1_id, key_index=1, block=b"a" * 42
    )
    assert rep == authenticated_cmds.latest.block_create.RepOk()
    rep = await coolorg.alice.block_create(
        block_id=BlockID.new(), realm_id=coolorg.wksp1_id, key_index=1, block=b"b" * 8
    )
    assert rep == authenticated_cmds.latest.block_create.RepOk()

    # ...and change roles

    await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER)
    await wksp1_alice_gives_role(coolorg, backend, coolorg.mallory.user_id, RealmRole.CONTRIBUTOR)

    rep = await coolorg.alice.realm_list_stats()
    assert rep == authenticated_cmds.latest.realm_list_stats.RepOk(
        realms=[
            _wksp1_stats(
                coolorg.wksp1_id,
                metadata_size=wksp1_metadata_size,
                data_size=50,
                owners=1,  # Alice
                managers=1,  # Bob
                contributors=1,  # Mallory
            )
        ]
    )

    # Unshared and revoked users are no longer counted

    await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, None)
    await alice_gives_profile(coolorg, backend, coolorg.mallory.user_id, None)

    rep = await coolorg.alice.realm_list_stats()
    assert rep == authenticated_cmds.latest.realm_list_stats.RepOk(
        realms=[
            _wksp1_stats(
                coolorg.wksp1_id,
                metadata_size=wksp1_metadata_size,
                data_size=50,
                owners=1,  # Alice
            )
        ]
    )


async def test_authenticated_realm_list_stats_includes_realms_author_is_not_part_of(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    wksp1_metadata_size = await _wksp1_metadata_size(coolorg, backend)

    # Bob becomes ADMIN, but is still not part of wksp1 (only Alice is)
    await alice_gives_profile(coolorg, backend, coolorg.bob.user_id, UserProfile.ADMIN)
    await wksp1_alice_gives_role(coolorg, backend, coolorg.bob.user_id, None)

    rep = await coolorg.bob.realm_list_stats()
    assert rep == authenticated_cmds.latest.realm_list_stats.RepOk(
        realms=[
            _wksp1_stats(
                coolorg.wksp1_id,
                metadata_size=wksp1_metadata_size,
                owners=1,  # Alice
            )
        ]
    )


@pytest.mark.parametrize("kind", ("as_standard", "as_outsider"))
async def test_authenticated_realm_list_stats_author_not_allowed(
    coolorg: CoolorgRpcClients, kind: str
) -> None:
    match kind:
        case "as_standard":
            author = coolorg.bob
        case "as_outsider":
            author = coolorg.mallory
        case unknown:
            assert False, unknown

    rep = await author.realm_list_stats()
    assert rep == authenticated_cmds.latest.realm_list_stats.RepAuthorNotAllowed()


async def test_authenticated_realm_list_stats_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.realm_list_stats()

    await authenticated_http_common_errors_tester(do)
