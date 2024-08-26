# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from enum import auto

from parsec._parsec import DeviceID, OrganizationID, UserID, UserProfile
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.types import BadOutcomeEnum


class AuthNoLockBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()


@dataclass(slots=True)
class AuthNoLockData:
    organization_internal_id: int
    device_internal_id: int
    user_internal_id: int
    user_id: UserID
    user_current_profile: UserProfile


_q_auth = Q("""
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

my_device AS (
    SELECT
        _id,
        user_
    FROM device
    WHERE
        organization = (SELECT _id FROM my_organization)
        AND device_id = $device_id
    LIMIT 1
),

my_user AS (
    SELECT
        _id,
        frozen,
        (revoked_on IS NOT NULL) AS revoked,
        user_id,
        current_profile
    FROM user_
    WHERE _id = (SELECT user_ FROM my_device)
    LIMIT 1
)

SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT is_expired FROM my_organization) AS organization_is_expired,
    (SELECT _id FROM my_device) AS device_internal_id,
    (SELECT _id FROM my_user) AS user_internal_id,
    (SELECT frozen FROM my_user) AS user_is_frozen,
    (SELECT revoked FROM my_user) AS user_is_revoked,
    (SELECT user_id FROM my_user) AS user_id,
    (SELECT current_profile FROM my_user) AS user_current_profile
""")


async def auth_no_lock(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author: DeviceID,
) -> AuthNoLockData | AuthNoLockBadOutcome:
    row = await conn.fetchrow(
        *_q_auth(
            organization_id=organization_id.str,
            device_id=author,
        )
    )
    assert row is not None

    # 1) Check organization

    match row["organization_internal_id"]:
        case int() as organization_internal_id:
            pass
        case None:
            return AuthNoLockBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["organization_is_expired"]:
        case False:
            pass
        case True:
            return AuthNoLockBadOutcome.ORGANIZATION_EXPIRED
        case unknown:
            assert False, repr(unknown)

    # 2) Check device & user

    match row["device_internal_id"]:
        case int() as device_internal_id:
            pass
        case None:
            return AuthNoLockBadOutcome.AUTHOR_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    # Since device exists, its corresponding user must also exist

    match row["user_internal_id"]:
        case int() as user_internal_id:
            pass
        case None:
            return AuthNoLockBadOutcome.AUTHOR_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["user_id"]:
        case str() as raw_user_id:
            user_id = UserID.from_hex(raw_user_id)
        case unknown:
            assert False, repr(unknown)

    match row["user_is_frozen"]:
        case False:
            pass
        case True:
            return AuthNoLockBadOutcome.AUTHOR_REVOKED
        case unknown:
            assert False, repr(unknown)

    match row["user_is_revoked"]:
        case False:
            pass
        case True:
            return AuthNoLockBadOutcome.AUTHOR_REVOKED
        case unknown:
            assert False, repr(unknown)

    match row["user_current_profile"]:
        case str() as raw_user_current_profile:
            user_current_profile = UserProfile.from_str(raw_user_current_profile)
        case unknown:
            assert False, repr(unknown)

    return AuthNoLockData(
        organization_internal_id=organization_internal_id,
        device_internal_id=device_internal_id,
        user_internal_id=user_internal_id,
        user_id=user_id,
        user_current_profile=user_current_profile,
    )
