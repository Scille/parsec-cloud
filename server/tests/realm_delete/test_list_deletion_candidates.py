# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    OrganizationID,
)
from parsec.components.realm import (
    RealmDeletionCandidateOrphaned,
    RealmDeletionCandidatePlanned,
    RealmListDeletionCandidatesBadOutcome,
)
from tests.common import (
    Backend,
    WorkspaceArchivedOrgRpcClients,
)


async def test_organization_not_found(
    backend: Backend,
) -> None:
    dummy_org = OrganizationID("NonExistent")
    outcome = await backend.realm.list_deletion_candidates(
        organization_id=dummy_org,
        now=DateTime.now(),
    )
    assert outcome == RealmListDeletionCandidatesBadOutcome.ORGANIZATION_NOT_FOUND


async def test_ok(
    workspace_archived_org: WorkspaceArchivedOrgRpcClients,
    backend: Backend,
) -> None:
    outcome = await backend.realm.list_deletion_candidates(
        organization_id=workspace_archived_org.organization_id,
        now=DateTime.now(),
    )
    assert isinstance(outcome, list)
    assert sorted(outcome, key=lambda x: (x.realm_id, type(x).__name__)) == [
        # wksp_archived/wksp_soon_to_delete/wksp_not_longer_to_delete are ignored
        RealmDeletionCandidatePlanned(
            realm_id=workspace_archived_org.wksp_ready_to_delete_id,
            deletion_date=DateTime(2010, 1, 1),
        ),
        RealmDeletionCandidateOrphaned(
            realm_id=workspace_archived_org.wksp_orphaned_id, orphaned_since=DateTime(2000, 2, 12)
        ),
        RealmDeletionCandidateOrphaned(
            realm_id=workspace_archived_org.wksp_orphaned_and_ready_to_delete_id,
            orphaned_since=DateTime(2000, 2, 12),
        ),
        RealmDeletionCandidatePlanned(
            realm_id=workspace_archived_org.wksp_orphaned_and_ready_to_delete_id,
            deletion_date=DateTime(2010, 1, 1),
        ),
    ]
