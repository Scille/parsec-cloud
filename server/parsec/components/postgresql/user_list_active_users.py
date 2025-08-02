# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    EmailAddress,
    HumanHandle,
    OrganizationID,
    UserID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q, q_organization_internal_id
from parsec.components.user import (
    UserInfo,
    UserListActiveUsersBadOutcome,
)

_q_get_organization_users = Q(
    f"""
SELECT
    user_.user_id,
    user_.frozen,
    human.email AS human_email,
    human.label AS human_label
FROM user_
INNER JOIN human ON user_.human = human._id
WHERE
    user_.organization = {q_organization_internal_id("$organization_id")}  -- noqa: LT05,LT14
    AND user_.revoked_on IS NULL
ORDER BY user_.created_on
"""
)


async def user_list_active_users(
    conn: AsyncpgConnection, organization_id: OrganizationID
) -> list[UserInfo] | UserListActiveUsersBadOutcome:
    rows = await conn.fetch(*_q_get_organization_users(organization_id=organization_id.str))
    if not rows:
        return UserListActiveUsersBadOutcome.ORGANIZATION_NOT_FOUND

    users = []
    for row in rows:
        match row["user_id"]:
            case str() as raw_user_id:
                user_id = UserID.from_hex(raw_user_id)
            case _:
                assert False, row

        match row["human_email"]:
            case str() as raw_human_email:
                human_email = EmailAddress(raw_human_email)
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
