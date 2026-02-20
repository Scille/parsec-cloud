# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import Literal

from parsec._parsec import AccessToken, DeviceID, OrganizationID, UserID
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.queries import (
    AuthNoLockBadOutcome,
    AuthNoLockData,
    auth_no_lock,
)
from parsec.components.postgresql.utils import Q
from parsec.components.totp import (
    TOTPSetupGetSecretBadOutcome,
    TOTPSetupGetSecretWithTokenBadOutcome,
    generate_totp_secret,
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


_q_update_totp_secret = Q("""
UPDATE user_
SET totp_secret = $totp_secret
WHERE _id = $user_internal_id
""")


async def _totp_setup_retrieve_or_create_secret(
    conn: AsyncpgConnection,
    user_internal_id: int,
) -> bytes | Literal[TOTPSetupGetSecretBadOutcome.ALREADY_SETUP]:
    row = await conn.fetchrow(*_q_get_totp(user_internal_id=user_internal_id))
    assert row is not None

    match row["totp_setup_completed"]:
        case True:
            return TOTPSetupGetSecretBadOutcome.ALREADY_SETUP
        case False:
            pass
        case _:
            assert False, row

    match row["totp_secret"]:
        case bytes() as totp_secret:
            return totp_secret  # Secret already exists, just return it
        case None:
            pass
        case _:
            assert False, row

    totp_secret = generate_totp_secret()
    await conn.execute(
        *_q_update_totp_secret(
            user_internal_id=user_internal_id,
            totp_secret=totp_secret,
        )
    )
    return totp_secret


async def totp_setup_get_secret(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author: DeviceID,
) -> bytes | TOTPSetupGetSecretBadOutcome:
    match await auth_no_lock(conn, organization_id, author):
        case AuthNoLockData() as db_auth:
            pass
        case AuthNoLockBadOutcome.ORGANIZATION_NOT_FOUND:
            return TOTPSetupGetSecretBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthNoLockBadOutcome.ORGANIZATION_EXPIRED:
            return TOTPSetupGetSecretBadOutcome.ORGANIZATION_EXPIRED
        case AuthNoLockBadOutcome.AUTHOR_NOT_FOUND:
            return TOTPSetupGetSecretBadOutcome.AUTHOR_NOT_FOUND
        case AuthNoLockBadOutcome.AUTHOR_REVOKED:
            return TOTPSetupGetSecretBadOutcome.AUTHOR_REVOKED

    return await _totp_setup_retrieve_or_create_secret(
        conn, user_internal_id=db_auth.user_internal_id
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


async def totp_setup_get_secret_with_token(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    user_id: UserID,
    token: AccessToken,
) -> bytes | TOTPSetupGetSecretWithTokenBadOutcome:
    row = await conn.fetchrow(
        *_q_get_organization_user_token(organization_id=organization_id.str, user_id=user_id)
    )
    assert row is not None

    match row["organization_expired"]:
        case None:
            return TOTPSetupGetSecretWithTokenBadOutcome.ORGANIZATION_NOT_FOUND
        case False:
            pass
        case True:
            return TOTPSetupGetSecretWithTokenBadOutcome.ORGANIZATION_EXPIRED
        case _:
            assert False, row

    match row["user_internal_id"]:
        case int() as user_internal_id:
            pass
        case None:
            return TOTPSetupGetSecretWithTokenBadOutcome.USER_NOT_FOUND
        case _:
            assert False, row

    match row["user_revoked"]:
        case False:
            pass
        case True:
            return TOTPSetupGetSecretWithTokenBadOutcome.USER_REVOKED
        case _:
            assert False, row

    match row["user_frozen"]:
        case False:
            pass
        case True:
            return TOTPSetupGetSecretWithTokenBadOutcome.USER_FROZEN
        case _:
            assert False, row

    match row["totp_reset_token"]:
        case None:
            return TOTPSetupGetSecretWithTokenBadOutcome.BAD_TOKEN
        case str() as totp_reset_token:
            if totp_reset_token != token.hex:
                return TOTPSetupGetSecretWithTokenBadOutcome.BAD_TOKEN
        case _:
            assert False, row

    outcome = await _totp_setup_retrieve_or_create_secret(conn, user_internal_id=user_internal_id)
    match outcome:
        case bytes() as totp_secret:
            pass
        case TOTPSetupGetSecretBadOutcome.ALREADY_SETUP:
            return TOTPSetupGetSecretWithTokenBadOutcome.ALREADY_SETUP

    return totp_secret
