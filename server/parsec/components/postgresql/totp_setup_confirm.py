# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import Literal

from parsec._parsec import AccessToken, DateTime, DeviceID, OrganizationID, UserID
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.queries import (
    AuthNoLockBadOutcome,
    AuthNoLockData,
    auth_no_lock,
)
from parsec.components.postgresql.utils import Q
from parsec.components.totp import (
    TOTPSetupConfirmBadOutcome,
    TOTPSetupConfirmWithTokenBadOutcome,
    verify_totp,
)

_q_get_totp = Q("""
SELECT
    totp_setup_completed,
    totp_secret
FROM user_
WHERE _id = $user_internal_id
LIMIT 1
FOR UPDATE
""")


_q_complete_totp_setup = Q("""
UPDATE user_
SET
    totp_setup_completed = TRUE,
    totp_reset_token = NULL
WHERE _id = $user_internal_id
""")


async def _totp_setup_confirm(
    conn: AsyncpgConnection,
    user_internal_id: int,
    one_time_password: str,
) -> (
    None
    | Literal[TOTPSetupConfirmBadOutcome.ALREADY_SETUP]
    | Literal[TOTPSetupConfirmBadOutcome.INVALID_ONE_TIME_PASSWORD]
):
    row = await conn.fetchrow(*_q_get_totp(user_internal_id=user_internal_id))
    assert row is not None

    match row["totp_setup_completed"]:
        case True:
            return TOTPSetupConfirmBadOutcome.ALREADY_SETUP
        case False:
            pass
        case _:
            assert False, row

    match row["totp_secret"]:
        case bytes() as totp_secret:
            pass
        case None:
            return TOTPSetupConfirmBadOutcome.INVALID_ONE_TIME_PASSWORD
        case _:
            assert False, row

    if not verify_totp(now=DateTime.now(), secret=totp_secret, one_time_password=one_time_password):
        return TOTPSetupConfirmBadOutcome.INVALID_ONE_TIME_PASSWORD

    await conn.execute(*_q_complete_totp_setup(user_internal_id=user_internal_id))


async def totp_setup_confirm(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author: DeviceID,
    one_time_password: str,
) -> None | TOTPSetupConfirmBadOutcome:
    match await auth_no_lock(conn, organization_id, author):
        case AuthNoLockData() as db_auth:
            pass
        case AuthNoLockBadOutcome.ORGANIZATION_NOT_FOUND:
            return TOTPSetupConfirmBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthNoLockBadOutcome.ORGANIZATION_EXPIRED:
            return TOTPSetupConfirmBadOutcome.ORGANIZATION_EXPIRED
        case AuthNoLockBadOutcome.AUTHOR_NOT_FOUND:
            return TOTPSetupConfirmBadOutcome.AUTHOR_NOT_FOUND
        case AuthNoLockBadOutcome.AUTHOR_REVOKED:
            return TOTPSetupConfirmBadOutcome.AUTHOR_REVOKED

    return await _totp_setup_confirm(
        conn, user_internal_id=db_auth.user_internal_id, one_time_password=one_time_password
    )


_q_get_organization_user_token = Q("""
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

my_user AS (
    SELECT
        _id,
        frozen,
        totp_reset_token,
        (revoked_on IS NOT NULL) AS revoked
    FROM user_
    WHERE
        organization = (SELECT my_organization._id FROM my_organization)
        AND user_id = $user_id
    LIMIT 1
    FOR UPDATE
)

SELECT
    (SELECT is_expired FROM my_organization) AS organization_expired,
    (SELECT _id FROM my_user) AS user_internal_id,
    (SELECT revoked FROM my_user) AS user_revoked,
    (SELECT frozen FROM my_user) AS user_frozen,
    (SELECT totp_reset_token FROM my_user) AS totp_reset_token
""")


async def totp_setup_confirm_with_token(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    user_id: UserID,
    token: AccessToken,
    one_time_password: str,
) -> None | TOTPSetupConfirmWithTokenBadOutcome:
    row = await conn.fetchrow(
        *_q_get_organization_user_token(organization_id=organization_id.str, user_id=user_id)
    )
    assert row is not None

    match row["organization_expired"]:
        case None:
            return TOTPSetupConfirmWithTokenBadOutcome.ORGANIZATION_NOT_FOUND
        case False:
            pass
        case True:
            return TOTPSetupConfirmWithTokenBadOutcome.ORGANIZATION_EXPIRED
        case _:
            assert False, row

    match row["user_internal_id"]:
        case int() as user_internal_id:
            pass
        case None:
            return TOTPSetupConfirmWithTokenBadOutcome.USER_NOT_FOUND
        case _:
            assert False, row

    match row["user_revoked"]:
        case False:
            pass
        case True:
            return TOTPSetupConfirmWithTokenBadOutcome.USER_REVOKED
        case _:
            assert False, row

    match row["user_frozen"]:
        case False:
            pass
        case True:
            return TOTPSetupConfirmWithTokenBadOutcome.USER_FROZEN
        case _:
            assert False, row

    match row["totp_reset_token"]:
        case None:
            return TOTPSetupConfirmWithTokenBadOutcome.BAD_TOKEN
        case str() as totp_reset_token:
            if totp_reset_token != token.hex:
                return TOTPSetupConfirmWithTokenBadOutcome.BAD_TOKEN
        case _:
            assert False, row

    outcome = await _totp_setup_confirm(
        conn, user_internal_id=user_internal_id, one_time_password=one_time_password
    )
    match outcome:
        case None:
            pass
        case TOTPSetupConfirmBadOutcome.ALREADY_SETUP:
            return TOTPSetupConfirmWithTokenBadOutcome.ALREADY_SETUP
        case TOTPSetupConfirmBadOutcome.INVALID_ONE_TIME_PASSWORD:
            return TOTPSetupConfirmWithTokenBadOutcome.INVALID_ONE_TIME_PASSWORD
