# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import DateTime, DeviceID, OrganizationID, VlobID, authenticated_cmds
from tests.common import Backend, CoolorgRpcClients


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
                sequester_blob=None,
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
                sequester_blob=None,
            )
        assert outcome is None, outcome
        dt = dt.add(days=1)
    return vlob_id


async def test_authenticated_vlob_poll_changes_ok(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # Coolorg's wksp1 comes with already a vlob present (i.e. v1 of the workspace manifest)
    rep = await coolorg.alice.vlob_poll_changes(realm_id=coolorg.wksp1_id, last_checkpoint=0)
    assert isinstance(rep, authenticated_cmds.v4.vlob_poll_changes.RepOk)
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

    rep = await coolorg.alice.vlob_poll_changes(realm_id=coolorg.wksp1_id, last_checkpoint=0)
    assert rep == authenticated_cmds.v4.vlob_poll_changes.RepOk(
        current_checkpoint=6,
        changes=[
            *checkpoint1_changes,
            (vlob1_id, 1),
            (vlob2_id, 3),
            (vlob3_id, 1),
        ],
    )

    rep = await coolorg.alice.vlob_poll_changes(realm_id=coolorg.wksp1_id, last_checkpoint=3)
    assert rep == authenticated_cmds.v4.vlob_poll_changes.RepOk(
        current_checkpoint=6,
        changes=[
            (vlob2_id, 3),
            (vlob3_id, 1),
        ],
    )

    rep = await coolorg.alice.vlob_poll_changes(realm_id=coolorg.wksp1_id, last_checkpoint=6)
    assert rep == authenticated_cmds.v4.vlob_poll_changes.RepOk(current_checkpoint=6, changes=[])
