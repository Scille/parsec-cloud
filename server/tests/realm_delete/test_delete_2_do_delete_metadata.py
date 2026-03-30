# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import (
    BlockID,
    DateTime,
    OrganizationID,
    VlobID,
)
from parsec.components.realm import (
    RealmDelete2DoDeleteMetadataBadOutcome,
)
from tests.common import (
    Backend,
    CoolorgRpcClients,
    MinimalorgRpcClients,
    WorkspaceArchivedOrgRpcClients,
)


async def test_organization_not_found(
    backend: Backend,
) -> None:
    dummy_org = OrganizationID("NonExistent")
    outcome = await backend.realm.delete_2_do_delete_metadata(
        organization_id=dummy_org,
        realm_id=VlobID.new(),
        now=DateTime.now(),
    )
    assert outcome == RealmDelete2DoDeleteMetadataBadOutcome.ORGANIZATION_NOT_FOUND


async def test_realm_not_found(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    outcome = await backend.realm.delete_2_do_delete_metadata(
        organization_id=minimalorg.organization_id,
        realm_id=VlobID.new(),
        now=DateTime.now(),
    )
    assert outcome == RealmDelete2DoDeleteMetadataBadOutcome.REALM_NOT_FOUND


async def test_realm_not_orphaned_nor_deletion_planned_never_archived(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    outcome = await backend.realm.delete_2_do_delete_metadata(
        organization_id=coolorg.organization_id,
        realm_id=coolorg.wksp1_id,
        now=DateTime.now(),
    )
    assert outcome == RealmDelete2DoDeleteMetadataBadOutcome.REALM_NOT_ORPHANED_NOR_DELETION_PLANNED


@pytest.mark.parametrize("kind", ("no_longer_planned_for_deletion", "archived"))
async def test_realm_not_orphaned_nor_deletion_planned(
    workspace_archived_org: WorkspaceArchivedOrgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    match kind:
        case "no_longer_planned_for_deletion":
            wksp_id = workspace_archived_org.wksp_not_longer_to_delete_id
        case "archived":
            wksp_id = workspace_archived_org.wksp_archived_id
        case unknown:
            assert False, unknown

    outcome = await backend.realm.delete_2_do_delete_metadata(
        organization_id=workspace_archived_org.organization_id,
        realm_id=wksp_id,
        now=DateTime.now(),
    )
    assert outcome == RealmDelete2DoDeleteMetadataBadOutcome.REALM_NOT_ORPHANED_NOR_DELETION_PLANNED


async def test_realm_deletion_date_not_reached(
    workspace_archived_org: WorkspaceArchivedOrgRpcClients,
    backend: Backend,
) -> None:
    outcome = await backend.realm.delete_2_do_delete_metadata(
        organization_id=workspace_archived_org.organization_id,
        realm_id=workspace_archived_org.wksp_soon_to_delete_id,
        now=DateTime.now(),
    )
    assert outcome == RealmDelete2DoDeleteMetadataBadOutcome.REALM_DELETION_DATE_NOT_REACHED


async def test_realm_already_deleted(
    workspace_archived_org: WorkspaceArchivedOrgRpcClients,
    backend: Backend,
) -> None:
    outcome = await backend.realm.delete_2_do_delete_metadata(
        organization_id=workspace_archived_org.organization_id,
        realm_id=workspace_archived_org.wksp_deleted_id,
        now=DateTime.now(),
    )
    assert outcome == RealmDelete2DoDeleteMetadataBadOutcome.REALM_ALREADY_DELETED


@pytest.mark.parametrize("kind", ("deletion_planned", "orphaned"))
async def test_ok(
    workspace_archived_org: WorkspaceArchivedOrgRpcClients,
    backend: Backend,
    kind: str,
) -> None:
    """Deleting a realm with reached deletion date marks it as deleted."""

    match kind:
        case "deletion_planned":
            wksp_id = workspace_archived_org.wksp_ready_to_delete_id
            active_device_id = workspace_archived_org.alice.device_id
        case "orphaned":
            wksp_id = workspace_archived_org.wksp_orphaned_id
            active_device_id = None
        case unknown:
            assert False, unknown

    outcome = await backend.realm.delete_2_do_delete_metadata(
        organization_id=workspace_archived_org.organization_id,
        realm_id=wksp_id,
        now=DateTime.now(),
    )
    assert outcome is None

    # Check deletion
    if active_device_id:
        await backend.block.read(
            organization_id=workspace_archived_org.organization_id,
            realm_id=wksp_id,
            author=active_device_id,
            block_id=BlockID.new(),  # Realm deleted is checked before checking block existence
        )
    else:
        outcome = await backend.realm.delete_2_do_delete_metadata(
            organization_id=workspace_archived_org.organization_id,
            realm_id=wksp_id,
            now=DateTime.now(),
        )
        assert outcome == RealmDelete2DoDeleteMetadataBadOutcome.REALM_ALREADY_DELETED
