# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import DeviceID, OrganizationID, UserID, UserProfile
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.queries import (
    AuthNoLockBadOutcome,
    AuthNoLockData,
    auth_no_lock,
)
from parsec.components.postgresql.utils import Q
from parsec.components.user import (
    UserListFrozenUsersBadOutcome,
)

_q_get_organization_frozen_users = Q(
    """
SELECT
    user_.user_id

FROM user_
WHERE
    user_.organization = $internal_organization_id
    AND user_.frozen = TRUE
ORDER BY user_.created_on
"""
)


async def user_list_frozen_users(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    device_id: DeviceID,
) -> list[UserID] | UserListFrozenUsersBadOutcome:
    match await auth_no_lock(conn, organization_id, device_id):
        case AuthNoLockData() as db_auth:
            profile = db_auth.user_current_profile
            internal_organization_id = db_auth.organization_internal_id
        case AuthNoLockBadOutcome.ORGANIZATION_NOT_FOUND:
            return UserListFrozenUsersBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthNoLockBadOutcome.ORGANIZATION_EXPIRED:
            return UserListFrozenUsersBadOutcome.ORGANIZATION_EXPIRED
        case AuthNoLockBadOutcome.AUTHOR_NOT_FOUND:
            return UserListFrozenUsersBadOutcome.AUTHOR_NOT_FOUND
        case AuthNoLockBadOutcome.AUTHOR_REVOKED:
            return UserListFrozenUsersBadOutcome.AUTHOR_REVOKED

    if UserProfile.ADMIN != profile:
        return UserListFrozenUsersBadOutcome.AUTHOR_NOT_ALLOWED

    rows = await conn.fetch(
        *_q_get_organization_frozen_users(internal_organization_id=internal_organization_id)
    )

    users = []
    for row in rows:
        match row["user_id"]:
            case str() as raw_user_id:
                user_id = UserID.from_hex(raw_user_id)
            case unknown:
                assert False, repr(unknown)

        users.append(user_id)
    return users
