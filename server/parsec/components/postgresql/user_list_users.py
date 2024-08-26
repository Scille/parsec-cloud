# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    HumanHandle,
    OrganizationID,
    UserID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q, q_organization_internal_id
from parsec.components.user import (
    UserInfo,
    UserListUsersBadOutcome,
)

_q_get_organization_users = Q(
    f"""
SELECT
    user_.user_id,
    user_.frozen,
    human.email AS human_email,
    human.label AS human_label
FROM user_
INNER JOIN human ON human._id = user_.human
WHERE
    user_.organization = { q_organization_internal_id("$organization_id") }
ORDER BY user_.created_on
"""
)


async def user_list_users(
    conn: AsyncpgConnection, organization_id: OrganizationID
) -> list[UserInfo] | UserListUsersBadOutcome:
    rows = await conn.fetch(*_q_get_organization_users(organization_id=organization_id.str))
    if not rows:
        return UserListUsersBadOutcome.ORGANIZATION_NOT_FOUND

    users = []
    for row in rows:
        match row["user_id"]:
            case str() as raw_user_id:
                user_id = UserID.from_hex(raw_user_id)
            case _:
                assert False, row

        match row["human_email"]:
            case str() as human_email:
                pass
            case _:
                assert False, row

        match row["human_label"]:
            case str() as human_label:
                pass
            case _:
                assert False, row

        human_handle = HumanHandle(email=human_email, label=human_label)

        match row["frozen"]:
            case bool() as frozen:
                pass
            case _:
                assert False, row

        users.append(
            UserInfo(
                user_id=user_id,
                human_handle=human_handle,
                frozen=frozen,
            )
        )
    return users
