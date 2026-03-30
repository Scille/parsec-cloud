# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    OrganizationID,
    VlobID,
)
from parsec.components.realm import (
    RealmDelete1GetBlocksBatchBadOutcome,
    RealmDeleteGetBlocksBatch,
)
from tests.common import (
    Backend,
    MinimalorgRpcClients,
    WorkspaceArchivedOrgRpcClients,
)


async def test_organization_not_found(
    backend: Backend,
) -> None:
    dummy_org = OrganizationID("NonExistent")
    outcome = await backend.realm.delete_1_get_blocks_batch(
        organization_id=dummy_org,
        realm_id=VlobID.new(),
        batch_offset_marker=0,
        batch_size=100,
    )
    assert outcome == RealmDelete1GetBlocksBatchBadOutcome.ORGANIZATION_NOT_FOUND


async def test_realm_not_found(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    outcome = await backend.realm.delete_1_get_blocks_batch(
        organization_id=minimalorg.organization_id,
        realm_id=VlobID.new(),
        batch_offset_marker=0,
        batch_size=100,
    )
    assert outcome == RealmDelete1GetBlocksBatchBadOutcome.REALM_NOT_FOUND


async def test_ok(
    workspace_archived_org: WorkspaceArchivedOrgRpcClients,
    backend: Backend,
) -> None:
    # `wksp_soon_to_delete` has no blocks
    outcome = await backend.realm.delete_1_get_blocks_batch(
        organization_id=workspace_archived_org.organization_id,
        realm_id=workspace_archived_org.wksp_soon_to_delete_id,
        batch_offset_marker=0,
        batch_size=100,
    )
    assert isinstance(outcome, RealmDeleteGetBlocksBatch)
    assert outcome.blocks == []

    # `wksp_ready_to_delete` has 5 blocks

    blocks = set()
    batch_offset_marker = 0
    for i in range(4):
        outcome = await backend.realm.delete_1_get_blocks_batch(
            organization_id=workspace_archived_org.organization_id,
            realm_id=workspace_archived_org.wksp_ready_to_delete_id,
            batch_offset_marker=batch_offset_marker,
            batch_size=2,
        )
        assert isinstance(outcome, RealmDeleteGetBlocksBatch)
        match i:
            case 3:
                expected_items = 0
            case 2:
                expected_items = 1
            case _:
                expected_items = 2
        assert len(outcome.blocks) == expected_items
        blocks |= set(outcome.blocks)
        batch_offset_marker = outcome.batch_offset_marker

    assert blocks == workspace_archived_org.wksp_ready_to_delete_blocks
