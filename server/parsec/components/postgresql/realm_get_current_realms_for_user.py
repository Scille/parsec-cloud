# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    OrganizationID,
    RealmRole,
    UserID,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
    q_realm,
)
from parsec.components.realm import (
    RealmGetCurrentRealmsForUserBadOutcome,
)

_q_get_org_and_user = Q(
    """
WITH my_organization AS (
    SELECT _id
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
),

my_user AS (
    SELECT _id
    FROM user_
    WHERE
        organization = (SELECT _id FROM my_organization)
        AND user_id = $user_id
    LIMIT 1
)

SELECT
    (SELECT _id FROM my_user) AS user_internal_id,
    (SELECT _id FROM my_organization) AS organization_internal_id
"""
)


_q_get_realms_for_user = Q(
    f"""
SELECT DISTINCT ON (realm)
    { q_realm(_id="realm_user_role.realm", select="realm_id") } AS realm_id,
    role,
    certified_on
FROM realm_user_role
WHERE user_ = $user_internal_id
ORDER BY realm, certified_on DESC
"""
)


async def realm_get_current_realms_for_user(
    conn: AsyncpgConnection, organization_id: OrganizationID, user: UserID
) -> dict[VlobID, RealmRole] | RealmGetCurrentRealmsForUserBadOutcome:
    row = await conn.fetchrow(
        *_q_get_org_and_user(organization_id=organization_id.str, user_id=user)
    )
    assert row is not None

    match row["organization_internal_id"]:
        case int():
            pass
        case None:
            return RealmGetCurrentRealmsForUserBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, unknown

    match row["user_internal_id"]:
        case int() as user_internal_id:
            pass
        case None:
            return RealmGetCurrentRealmsForUserBadOutcome.USER_NOT_FOUND
        case unknown:
            assert False, unknown

    rows = await conn.fetch(*_q_get_realms_for_user(user_internal_id=user_internal_id))
    user_realms = {}
    for row in rows:
        if row["role"] is None:
            continue
        role = RealmRole.from_str(row["role"])
        realm_id = VlobID.from_hex(row["realm_id"])
        user_realms[realm_id] = role

    return user_realms
