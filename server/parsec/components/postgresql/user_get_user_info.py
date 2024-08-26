# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    HumanHandle,
    OrganizationID,
    UserID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
    q_organization_internal_id,
)
from parsec.components.user import (
    UserInfo,
)


def _make_q_get_user_for(condition: str) -> Q:
    return Q(f"""
SELECT
    user_.user_id,
    user_.frozen,
    human.email,
    human.label
FROM user_
INNER JOIN human ON user_.human = human._id
WHERE
    user_.organization = { q_organization_internal_id("$organization_id") }
    AND {condition}
""")


_q_get_user = _make_q_get_user_for("user_.user_id = $user_id")
_q_get_user_for_email = _make_q_get_user_for("human.email = $email")


async def user_get_user_info(
    conn: AsyncpgConnection, organization_id: OrganizationID, user_id: UserID
) -> UserInfo | None:
    row = await conn.fetchrow(*_q_get_user(organization_id=organization_id.str, user_id=user_id))

    if row is None:
        return None

    match row["user_id"]:
        case str() as raw_user_id:
            user_id = UserID.from_hex(raw_user_id)
        case unknown:
            assert False, unknown

    match row["frozen"]:
        case bool() as frozen:
            pass
        case unknown:
            assert False, unknown

    match row["email"]:
        case str() as email:
            pass
        case unknown:
            assert False, unknown

    match row["label"]:
        case str() as label:
            pass
        case unknown:
            assert False, unknown

    human_handle = HumanHandle(email, label)

    return UserInfo(user_id, human_handle, frozen)


async def user_get_user_info_from_email(
    conn: AsyncpgConnection, organization_id: OrganizationID, email: str
) -> UserInfo | None:
    row = await conn.fetchrow(
        *_q_get_user_for_email(organization_id=organization_id.str, email=email)
    )

    if row is None:
        return None

    match row["user_id"]:
        case str() as raw_user_id:
            user_id = UserID.from_hex(raw_user_id)
        case unknown:
            assert False, unknown

    match row["frozen"]:
        case bool() as frozen:
            pass
        case unknown:
            assert False, unknown

    match row["email"]:
        case str() as email:
            pass
        case unknown:
            assert False, unknown

    match row["label"]:
        case str() as label:
            pass
        case unknown:
            assert False, unknown

    human_handle = HumanHandle(email, label)

    return UserInfo(user_id, human_handle, frozen)
