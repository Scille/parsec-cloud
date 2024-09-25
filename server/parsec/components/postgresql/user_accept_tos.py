# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.queries.q_auth_no_lock import (
    AuthNoLockBadOutcome,
    AuthNoLockData,
    auth_no_lock,
)
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.components.user import UserAcceptTosBadOutcome

_q_update_user_tos = Q("""
WITH my_organization AS (
    SELECT tos_updated_on
    FROM organization
    WHERE
        _id = $organization_internal_id
        AND tos_updated_on IS NOT NULL
),

updated_user AS (
    UPDATE user_
    SET
        tos_accepted_on = $tos_accepted_on
    WHERE
        _id = $user_internal_id
    RETURNING TRUE
)

SELECT
    COALESCE((SELECT * FROM updated_user), FALSE) AS update_user_ok,
    COALESCE(
        (SELECT FALSE FROM my_organization),
        TRUE
    ) AS no_tos,
    COALESCE(
        (SELECT $tos_updated_on != my_organization.tos_updated_on FROM my_organization),
        FALSE
    ) AS tos_mismatch
""")


async def user_accept_tos(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    tos_updated_on: DateTime,
) -> None | UserAcceptTosBadOutcome:
    match await auth_no_lock(conn, organization_id, author):
        case AuthNoLockData() as db_auth:
            pass
        case AuthNoLockBadOutcome.ORGANIZATION_NOT_FOUND:
            return UserAcceptTosBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthNoLockBadOutcome.ORGANIZATION_EXPIRED:
            return UserAcceptTosBadOutcome.ORGANIZATION_EXPIRED
        case AuthNoLockBadOutcome.AUTHOR_NOT_FOUND:
            return UserAcceptTosBadOutcome.AUTHOR_NOT_FOUND
        case AuthNoLockBadOutcome.AUTHOR_REVOKED:
            return UserAcceptTosBadOutcome.AUTHOR_REVOKED

    row = await conn.fetchrow(
        *_q_update_user_tos(
            organization_internal_id=db_auth.organization_internal_id,
            user_internal_id=db_auth.user_internal_id,
            tos_updated_on=tos_updated_on,
            tos_accepted_on=now,
        )
    )
    assert row is not None

    match row["update_user_ok"]:
        case True:
            pass
        case unknown:
            assert False, unknown

    match row["no_tos"]:
        case False:
            pass
        case True:
            return UserAcceptTosBadOutcome.NO_TOS
        case unknown:
            assert False, unknown

    match row["tos_mismatch"]:
        case False:
            pass
        case True:
            return UserAcceptTosBadOutcome.TOS_MISMATCH
        case unknown:
            assert False, unknown
