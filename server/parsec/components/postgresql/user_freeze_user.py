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

_q_freeze_from_user_id = Q("""
WITH my_organization AS (
    SELECT
        _id,
        is_expired
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
),

updated_user AS (
    UPDATE user_ SET
        frozen = $frozen
    WHERE
        organization = (SELECT _id FROM my_organization)
        AND user_id = $user_id
    RETURNING
        user_.user_id,
        (SELECT human.email FROM human WHERE human._id = user_.human),
        (SELECT human.label FROM human WHERE human._id = user_.human)
)

SELECT
    (
        COALESCE(
            (SELECT TRUE FROM my_organization),
            FALSE
        )
    ) AS organization_exists,
    (SELECT user_id FROM updated_user) AS user_id,
    (SELECT email FROM updated_user) AS human_handle_email,
    (SELECT label FROM updated_user) AS human_handle_label
""")


_q_freeze_from_email = Q("""
WITH my_organization AS (
    SELECT
        _id,
        is_expired
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
),

my_human AS (
    SELECT
        _id,
        email,
        label
    FROM human
    WHERE
        organization = (SELECT _id FROM my_organization)
        AND email = $user_email
),

updated_user AS (
    UPDATE user_ SET
        frozen = $frozen
    WHERE user_.human = (SELECT my_human._id FROM my_human)
    RETURNING user_.user_id
)

SELECT
    (
        COALESCE(
            (SELECT TRUE FROM my_organization),
            FALSE
        )
    ) AS organization_exists,
    (SELECT user_id FROM updated_user) AS user_id,
    (SELECT email FROM my_human) AS human_handle_email,
    (SELECT label FROM my_human) AS human_handle_label
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

    match (user_id, user_email):
        case (None, None):
            return UserFreezeUserBadOutcome.NO_USER_ID_NOR_EMAIL
        case (UserID() as user_id, None):
            q = _q_freeze_from_user_id(
                organization_id=organization_id.str, user_id=user_id, frozen=frozen
            )
        case (None, EmailAddress() as user_email):
            q = _q_freeze_from_email(
                organization_id=organization_id.str, user_email=str(user_email), frozen=frozen
            )
        case (UserID(), EmailAddress()):
            return UserFreezeUserBadOutcome.BOTH_USER_ID_AND_EMAIL
        case never:  # pyright: ignore [reportUnnecessaryComparison]
            assert_never(never)

    row = await conn.fetchrow(*q)
    assert row is not None

    match row["organization_exists"]:
        case True:
            pass
        case False:
            return UserFreezeUserBadOutcome.ORGANIZATION_NOT_FOUND
        case _:
            assert False, row

    match row["user_id"]:
        case str() as raw_user_id:
            user_id = UserID.from_hex(raw_user_id)
        case None:
            return UserFreezeUserBadOutcome.USER_NOT_FOUND
        case _:
            assert False, row

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
                user_id=info.user_id,
            ),
        )
    else:
        await send_signal(
            conn,
            EventUserUnfrozen(
                organization_id=organization_id,
                user_id=info.user_id,
            ),
        )

    return info
