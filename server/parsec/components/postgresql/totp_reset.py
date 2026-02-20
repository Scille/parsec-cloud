# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import assert_never

from parsec._parsec import AccessToken, EmailAddress, OrganizationID, UserID
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q
from parsec.components.totp import TOTPResetBadOutcome

_q_get_organization = Q("""
SELECT _id AS organization_internal_id
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
INNER JOIN human ON user_.human = human._id
WHERE
    human.organization = $organization_internal_id
    AND human.email = $email
    AND user_.revoked_on IS NULL
""")


_q_get_user = Q("""
SELECT
    user_._id AS user_internal_id,
    human.email AS user_email_addr,
    (user_.revoked_on IS NOT NULL) AS revoked
FROM user_
INNER JOIN human ON user_.human = human._id
WHERE
    user_.organization = $organization_internal_id
    AND user_.user_id = $user_id
LIMIT 1
FOR UPDATE
""")


_q_reset_totp = Q("""
UPDATE user_
SET
    totp_setup_completed = FALSE,
    totp_secret = NULL,
    totp_reset_token = $totp_reset_token
WHERE _id = $user_internal_id
""")


_q_reset_all_throttles = Q("""
UPDATE totp_opaque_key
SET
    last_failed_attempt = NULL,
    failed_attempts = 0
WHERE user_ = $user_internal_id
""")


async def totp_reset(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    user_id: UserID | None = None,
    user_email: EmailAddress | None = None,
) -> tuple[UserID, EmailAddress, AccessToken] | TOTPResetBadOutcome:
    row = await conn.fetchrow(
        *_q_get_organization(
            organization_id=organization_id.str,
        )
    )
    if row is None:
        return TOTPResetBadOutcome.ORGANIZATION_NOT_FOUND

    match row["organization_internal_id"]:
        case int() as organization_internal_id:
            pass
        case _:
            assert False, row

    match (user_id, user_email):
        case (None, None):
            return TOTPResetBadOutcome.NO_USER_ID_NOR_EMAIL

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
                    return TOTPResetBadOutcome.USER_NOT_FOUND
                case str() as raw_user_id:
                    user_id = UserID.from_hex(raw_user_id)
                case _:
                    assert False, val

        case (UserID(), EmailAddress()):
            return TOTPResetBadOutcome.BOTH_USER_ID_AND_EMAIL

        case never:  # pyright: ignore [reportUnnecessaryComparison]
            assert_never(never)

    row = await conn.fetchrow(
        *_q_get_user(organization_internal_id=organization_internal_id, user_id=user_id)
    )
    if row is None:
        return TOTPResetBadOutcome.USER_NOT_FOUND

    match row["user_internal_id"]:
        case int() as user_internal_id:
            pass
        case _:
            assert False, row

    match row["user_email_addr"]:
        case str() as raw_user_email_addr:
            user_email_addr = EmailAddress(raw_user_email_addr)
        case _:
            assert False, row

    match row["revoked"]:
        case False:
            pass
        case True:
            return TOTPResetBadOutcome.USER_REVOKED
        case _:
            assert False, row

    # All good, do the actual reset

    reset_token = AccessToken.new()

    await conn.execute(
        *_q_reset_totp(
            user_internal_id=user_internal_id,
            totp_reset_token=reset_token.hex,
        )
    )

    # Also reset all opaque key throttles

    await conn.execute(*_q_reset_all_throttles(user_internal_id=user_internal_id))

    return (user_id, user_email_addr, reset_token)
