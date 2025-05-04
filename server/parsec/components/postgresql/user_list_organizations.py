# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    OrganizationID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q
from parsec.components.user import (
    UserListOrganizationsBadOutcome,
    UserOrgInfo,
)

_q_get_organizations = Q("""
SELECT
    organization,
FROM human
WHERE
    human.email = $user_email
""")


async def user_list_organizations(
    conn: AsyncpgConnection, user_email: str
) -> list[UserOrgInfo] | UserListOrganizationsBadOutcome:
    rows = await conn.fetch(*_q_get_organizations(user_email=user_email))
    if not rows:
        return UserListOrganizationsBadOutcome.USER_NOT_FOUND

    orgs = []
    for row in rows:
        match row["organization"]:
            case str() as raw_org_id:
                org_id = OrganizationID(raw_org_id)
            case _:
                assert False, row

        orgs.append(
            UserOrgInfo(
                org_id=org_id,
            )
        )
    return orgs
