# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import assert_never

from parsec._parsec import (
    EmailAddress,
    HumanHandle,
    OrganizationID,
    UserID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.events import send_signal
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.components.user import (
    UserFreezeUserBadOutcome,
    UserInfo,
)
from parsec.events import (
    EventUserRevokedOrFrozen,
    EventUserUnfrozen,
)

_q_get_organization = Q("""
SELECT
    _id AS organization_internal_id,
    is_expired AS organization_expired
FROM organization
WHERE
    organization_id = $organization_id
    -- Only consider bootstrapped organizations
    AND root_verify_key IS NOT NULL
LIMIT 1
""")


_q_get_user_id_from_email = Q("""
SELECT user_.user_id
FROM user_
LEFT JOIN human ON user_.human = human._id
WHERE
    human.organization = $organization_internal_id
    AND human.email = $email
    AND user_.revoked_on IS NULL
""")


_q_get_user = Q("""
SELECT
    _id AS user_internal_id,
    (revoked_on IS NOT NULL) AS is_revoked
FROM user_
WHERE
    organization = $organization_internal_id
    AND user_id = $user_id
LIMIT 1
FOR UPDATE
""")


_q_freeze_from_user_id = Q("""
UPDATE user_ SET
    frozen = $frozen
WHERE
    _id = $user_internal_id
RETURNING
    (
        SELECT human.email
        FROM human
        WHERE human._id = user_.human
    ) AS human_handle_email,
    (
        SELECT human.label
        FROM human
        WHERE human._id = user_.human
    ) AS human_handle_label
""")


async def user_freeze_user(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    user_id: UserID | None,
    user_email: EmailAddress | None,
    frozen: bool,
) -> UserInfo | UserFreezeUserBadOutcome:
    # Note we don't need to lock the `common` topic here given setting the `frozen`
    # flag is totally unrelated to certificates.

    row = await conn.fetchrow(
        *_q_get_organization(
            organization_id=organization_id.str,
        )
    )
    if row is None:
        return UserFreezeUserBadOutcome.ORGANIZATION_NOT_FOUND

    match row["organization_internal_id"]:
        case int() as organization_internal_id:
            pass
        case _:
            assert False, row

    match (user_id, user_email):
        case (None, None):
            return UserFreezeUserBadOutcome.NO_USER_ID_NOR_EMAIL

        case (user_id, None):
            pass

        case (None, user_email):
            val = await conn.fetchval(
                *_q_get_user_id_from_email(
                    organization_internal_id=organization_internal_id, email=user_email.str
                )
            )
            match val:
                case None:
                    return UserFreezeUserBadOutcome.USER_NOT_FOUND
                case str() as raw_user_id:
                    user_id = UserID.from_hex(raw_user_id)
                case _:
                    assert False, val

        case (UserID(), EmailAddress()):
            return UserFreezeUserBadOutcome.BOTH_USER_ID_AND_EMAIL

        case never:  # pyright: ignore [reportUnnecessaryComparison]
            assert_never(never)

    row = await conn.fetchrow(
        *_q_get_user(organization_internal_id=organization_internal_id, user_id=user_id)
    )
    if row is None:
        return UserFreezeUserBadOutcome.USER_NOT_FOUND

    match row["user_internal_id"]:
        case int() as user_internal_id:
            pass
        case _:
            assert False, row

    match row["is_revoked"]:
        case False:
            pass
        case True:
            return UserFreezeUserBadOutcome.USER_REVOKED
        case _:
            assert False, row

    row = await conn.fetchrow(
        *_q_freeze_from_user_id(user_internal_id=user_internal_id, frozen=frozen)
    )
    assert row is not None  # `_q_get_user` has just locked the row for update

    match row["human_handle_email"]:
        case str() as raw_human_handle_email:
            human_handle_email = EmailAddress(raw_human_handle_email)
        case _:
            assert False, row

    match row["human_handle_label"]:
        case str() as human_handle_label:
            pass
        case _:
            assert False, row

    human_handle = HumanHandle(
        email=human_handle_email,
        label=human_handle_label,
    )
    info = UserInfo(
        user_id=user_id,
        human_handle=human_handle,
        frozen=frozen,
    )

    if info.frozen:
        await send_signal(
            conn,
            EventUserRevokedOrFrozen(
                organization_id=organization_id,
                user_id=user_id,
            ),
        )
    else:
        await send_signal(
            conn,
            EventUserUnfrozen(
                organization_id=organization_id,
                user_id=user_id,
            ),
        )

    return info
