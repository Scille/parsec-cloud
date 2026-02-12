# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import DateTime, OrganizationID, SecretKey, TOTPOpaqueKeyID, UserID
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q
from parsec.components.totp import (
    TOTPFetchOpaqueKeyBadOutcome,
    TOTPFetchOpaqueKeyThrottled,
    compute_wait_until,
    verify_totp,
)

_q_get_organization_user_totp = Q("""
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
        totp_setup_completed,
        totp_secret,
        (revoked_on IS NOT NULL) AS revoked
    FROM user_
    WHERE
        organization = (SELECT my_organization._id FROM my_organization)
        AND user_id = $user_id
    LIMIT 1
),

my_opaque_key AS (
    SELECT
        _id,
        opaque_key,
        last_failed_attempt,
        failed_attempts
    FROM totp_opaque_key
    WHERE
        user_ = (SELECT my_user._id FROM my_user)
        AND opaque_key_id = $opaque_key_id
    LIMIT 1
    FOR UPDATE
)

SELECT
    (SELECT is_expired FROM my_organization) AS organization_expired,
    (SELECT _id FROM my_user) AS user_internal_id,
    (SELECT revoked FROM my_user) AS user_revoked,
    (SELECT frozen FROM my_user) AS user_frozen,
    (SELECT totp_setup_completed FROM my_user) AS totp_setup_completed,
    (SELECT totp_secret FROM my_user) AS totp_secret,
    (SELECT _id FROM my_opaque_key) AS opaque_key_internal_id,
    (SELECT opaque_key FROM my_opaque_key) AS opaque_key,
    (SELECT last_failed_attempt FROM my_opaque_key) AS opaque_key_last_failed_attempt,
    (SELECT failed_attempts FROM my_opaque_key) AS opaque_key_failed_attempts
""")


_q_update_throttle_failure = Q("""
UPDATE totp_opaque_key
SET
    last_failed_attempt = $now,
    failed_attempts = $failed_attempts
WHERE _id = $opaque_key_internal_id
""")


_q_reset_throttle = Q("""
UPDATE totp_opaque_key
SET
    last_failed_attempt = NULL,
    failed_attempts = 0
WHERE _id = $opaque_key_internal_id
""")


async def totp_fetch_opaque_key(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    user_id: UserID,
    opaque_key_id: TOTPOpaqueKeyID,
    one_time_password: str,
) -> SecretKey | TOTPFetchOpaqueKeyBadOutcome | TOTPFetchOpaqueKeyThrottled:
    row = await conn.fetchrow(
        *_q_get_organization_user_totp(
            organization_id=organization_id.str, user_id=user_id, opaque_key_id=opaque_key_id
        )
    )
    assert row is not None

    match row["organization_expired"]:
        case None:
            return TOTPFetchOpaqueKeyBadOutcome.ORGANIZATION_NOT_FOUND
        case False:
            pass
        case True:
            return TOTPFetchOpaqueKeyBadOutcome.ORGANIZATION_EXPIRED
        case _:
            assert False, row

    match row["user_internal_id"]:
        case int():
            pass
        case None:
            return TOTPFetchOpaqueKeyBadOutcome.USER_NOT_FOUND
        case _:
            assert False, row

    match row["user_revoked"]:
        case False:
            pass
        case True:
            return TOTPFetchOpaqueKeyBadOutcome.USER_REVOKED
        case _:
            assert False, row

    match row["user_frozen"]:
        case False:
            pass
        case True:
            return TOTPFetchOpaqueKeyBadOutcome.USER_FROZEN
        case _:
            assert False, row

    match row["totp_setup_completed"]:
        case True:
            pass
        case False:
            return TOTPFetchOpaqueKeyBadOutcome.NOT_SETUP
        case _:
            assert False, row

    # TOTP setup is completed, so the secret cannot be NULL
    match row["totp_secret"]:
        case bytes() as totp_secret:
            pass
        case _:
            assert False, row

    match row["opaque_key_internal_id"]:
        case int() as opaque_key_internal_id:
            pass
        case None:
            return TOTPFetchOpaqueKeyBadOutcome.OPAQUE_KEY_NOT_FOUND
        case _:
            assert False, row

    match row["opaque_key"]:
        case bytes() as opaque_key_raw:
            opaque_key = SecretKey(opaque_key_raw)
        case _:
            assert False, row

    match row["opaque_key_last_failed_attempt"]:
        case None | DateTime() as opaque_key_last_failed_attempt:
            pass
        case _:
            assert False, row

    match row["opaque_key_failed_attempts"]:
        case int() as opaque_key_failed_attempts:
            pass
        case _:
            assert False, row

    match compute_wait_until(opaque_key_failed_attempts, opaque_key_last_failed_attempt):
        case None:
            pass
        case wait_until:
            if now <= wait_until:
                return TOTPFetchOpaqueKeyThrottled(wait_until=wait_until)

    if not verify_totp(now=now, secret=totp_secret, one_time_password=one_time_password):
        await conn.execute(
            *_q_update_throttle_failure(
                opaque_key_internal_id=opaque_key_internal_id,
                now=now,
                failed_attempts=opaque_key_failed_attempts + 1,
            )
        )
        return TOTPFetchOpaqueKeyBadOutcome.INVALID_ONE_TIME_PASSWORD

    # Success - reset throttle and return key

    await conn.execute(*_q_reset_throttle(opaque_key_internal_id=opaque_key_internal_id))

    return opaque_key
