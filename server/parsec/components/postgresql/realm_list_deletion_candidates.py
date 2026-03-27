# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    OrganizationID,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q
from parsec.components.realm import (
    RealmDeletionCandidate,
    RealmDeletionCandidateOrphaned,
    RealmDeletionCandidatePlanned,
    RealmListDeletionCandidatesBadOutcome,
)

_q_get_organization = Q("""
SELECT _id
FROM organization
WHERE
    organization_id = $organization_id
    -- Only consider bootstrapped organizations
    AND root_verify_key IS NOT NULL
LIMIT 1
""")


_q_get_planned_deletion_candidates = Q("""
SELECT
    realm.realm_id,
    my_current_archiving.deletion_date
FROM realm,
-- Lateral join to get the last archiving configuration for the current realm
    LATERAL (
        SELECT
            realm_archiving.configuration,
            realm_archiving.deletion_date
        FROM realm_archiving
        WHERE realm_archiving.realm = realm._id
        ORDER BY realm_archiving.certified_on DESC
        LIMIT 1
    ) AS my_current_archiving
WHERE
    realm.organization = $organization_internal_id
    AND realm.status = 'ARCHIVED_OR_DELETION_PLANNED'
    AND my_current_archiving.configuration = 'DELETION_PLANNED'
    AND my_current_archiving.deletion_date <= $now
""")


_q_get_orphaned_candidates = Q("""
WITH my_current_roles AS (
    SELECT DISTINCT ON (realm_user_role.realm, realm_user_role.user_)
        realm_user_role.realm,
        realm_user_role.user_,
        realm_user_role.role
    FROM realm_user_role
    INNER JOIN realm ON realm_user_role.realm = realm._id
    WHERE
        realm.organization = $organization_internal_id
        AND realm.status != 'DELETED'
    ORDER BY realm_user_role.realm ASC, realm_user_role.user_ ASC, realm_user_role.certified_on DESC
),

-- Realms that have at least one active (non-revoked) user with a non-NULL role
-- These are NOT orphaned
my_realms_with_active_members AS (
    SELECT DISTINCT my_current_roles.realm
    FROM my_current_roles
    INNER JOIN user_ ON my_current_roles.user_ = user_._id
    WHERE
        my_current_roles.role IS NOT NULL
        AND user_.revoked_on IS NULL
),

-- Orphaned realms: have members with non-NULL roles, but all are revoked
my_orphaned_realms AS (
    SELECT
        my_current_roles.realm,
        MAX(user_.revoked_on) AS orphaned_since
    FROM my_current_roles
    INNER JOIN user_ ON my_current_roles.user_ = user_._id
    WHERE
        my_current_roles.role IS NOT NULL
        AND my_current_roles.realm NOT IN (
            SELECT my_realms_with_active_members.realm FROM my_realms_with_active_members
        )
    GROUP BY my_current_roles.realm
)

SELECT
    realm.realm_id,
    my_orphaned_realms.orphaned_since
FROM my_orphaned_realms
INNER JOIN realm ON my_orphaned_realms.realm = realm._id
""")


async def realm_list_deletion_candidates(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    now: DateTime,
) -> list[RealmDeletionCandidate] | RealmListDeletionCandidatesBadOutcome:
    organization_internal_id = await conn.fetchval(
        *_q_get_organization(organization_id=organization_id.str)
    )
    if organization_internal_id is None:
        return RealmListDeletionCandidatesBadOutcome.ORGANIZATION_NOT_FOUND

    candidates: list[RealmDeletionCandidate] = []

    # Collect realms with planned deletion whose date has been reached
    rows = await conn.fetch(
        *_q_get_planned_deletion_candidates(
            organization_internal_id=organization_internal_id,
            now=now,
        )
    )
    for row in rows:
        match row["realm_id"]:
            case str() as raw_realm_id:
                realm_id = VlobID.from_hex(raw_realm_id)
            case _:
                assert False, row
        match row["deletion_date"]:
            case DateTime() as deletion_date:
                pass
            case _:
                assert False, row
        candidates.append(
            RealmDeletionCandidatePlanned(
                realm_id=realm_id,
                deletion_date=deletion_date,
            )
        )

    # Collect orphaned realms
    rows = await conn.fetch(
        *_q_get_orphaned_candidates(
            organization_internal_id=organization_internal_id,
        )
    )
    for row in rows:
        match row["realm_id"]:
            case str() as raw_realm_id:
                realm_id = VlobID.from_hex(raw_realm_id)
            case _:
                assert False, row
        match row["orphaned_since"]:
            case DateTime() as orphaned_since:
                pass
            case _:
                assert False, row
        candidates.append(
            RealmDeletionCandidateOrphaned(
                realm_id=realm_id,
                orphaned_since=orphaned_since,
            )
        )

    return candidates
