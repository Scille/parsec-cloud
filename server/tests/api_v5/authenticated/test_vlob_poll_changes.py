# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import DateTime, DeviceID, OrganizationID, RealmRole, VlobID, authenticated_cmds
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    wksp1_alice_gives_role,
    wksp1_bob_becomes_owner_and_changes_alice,
)


async def create_vlob(
    backend: Backend,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: VlobID,
    dt: DateTime,
    versions: int = 1,
) -> VlobID:
    vlob_id = VlobID.new()
    for version in range(1, versions + 1):
        v1_blob = f"<block content v{version}>".encode()
        if version == 1:
            outcome = await backend.vlob.create(
                now=dt,
                organization_id=organization_id,
                author=author,
                realm_id=realm_id,
                vlob_id=vlob_id,
                key_index=1,
                timestamp=dt,
                blob=v1_blob,
            )
        else:
            outcome = await backend.vlob.update(
                now=dt,
                organization_id=organization_id,
                author=author,
                vlob_id=vlob_id,
                key_index=1,
                timestamp=dt,
                version=version,
                blob=v1_blob,
            )
        assert outcome is None, outcome
        dt = dt.add(days=1)
    return vlob_id


@pytest.mark.parametrize(
    "kind",
    (
        "as_reader",
        "as_contributor",
        "as_manager",
        "as_owner",
    ),
)
async def test_authenticated_vlob_poll_changes_ok(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
    match kind:
        case "as_reader":
            author = coolorg.bob

        case "as_contributor":
            await wksp1_alice_gives_role(
                coolorg,
                backend,
                coolorg.bob.user_id,
                RealmRole.CONTRIBUTOR,
                now=DateTime(2019, 1, 1),
            )
            author = coolorg.bob

        case "as_manager":
            await wksp1_alice_gives_role(
                coolorg, backend, coolorg.bob.user_id, RealmRole.MANAGER, now=DateTime(2019, 1, 1)
            )
            author = coolorg.bob

        case "as_owner":
            author = coolorg.alice

        case unknown:
            assert False, unknown

    # Coolorg's wksp1 comes with already a vlob present (i.e. v1 of the workspace manifest)
    rep = await coolorg.alice.vlob_poll_changes(realm_id=coolorg.wksp1_id, last_checkpoint=0)
    assert isinstance(rep, authenticated_cmds.latest.vlob_poll_changes.RepOk)
    assert rep.current_checkpoint == 1
    checkpoint1_changes = rep.changes

    dt = DateTime(2020, 1, 1)
    vlob1_id = await create_vlob(
        backend=backend,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        dt=dt,
    )
    vlob2_id = await create_vlob(
        backend=backend,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        dt=dt,
        versions=3,
    )
    vlob3_id = await create_vlob(
        backend=backend,
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        realm_id=coolorg.wksp1_id,
        dt=dt,
    )

    rep = await author.vlob_poll_changes(realm_id=coolorg.wksp1_id, last_checkpoint=0)
    assert rep == authenticated_cmds.latest.vlob_poll_changes.RepOk(
        current_checkpoint=6,
        changes=[
            *checkpoint1_changes,
            (vlob1_id, 1),
            (vlob2_id, 3),
            (vlob3_id, 1),
        ],
    )

    rep = await author.vlob_poll_changes(realm_id=coolorg.wksp1_id, last_checkpoint=3)
    assert rep == authenticated_cmds.latest.vlob_poll_changes.RepOk(
        current_checkpoint=6,
        changes=[
            (vlob2_id, 3),
            (vlob3_id, 1),
        ],
    )

    rep = await author.vlob_poll_changes(realm_id=coolorg.wksp1_id, last_checkpoint=6)
    assert rep == authenticated_cmds.latest.vlob_poll_changes.RepOk(
        current_checkpoint=6, changes=[]
    )


@pytest.mark.parametrize("kind", ("never_allowed", "no_longer_allowed"))
async def test_authenticated_vlob_poll_changes_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
) -> None:
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

    rep = await author.vlob_poll_changes(realm_id=coolorg.wksp1_id, last_checkpoint=0)
    assert rep == authenticated_cmds.latest.vlob_poll_changes.RepAuthorNotAllowed()


async def test_authenticated_vlob_poll_changes_realm_not_found(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    bad_realm_id = VlobID.new()
    rep = await coolorg.alice.vlob_poll_changes(realm_id=bad_realm_id, last_checkpoint=0)
    assert rep == authenticated_cmds.latest.vlob_poll_changes.RepRealmNotFound()


async def test_authenticated_vlob_poll_changes_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.vlob_poll_changes(realm_id=coolorg.wksp1_id, last_checkpoint=0)

    await authenticated_http_common_errors_tester(do)
