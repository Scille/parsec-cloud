# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from enum import auto
from typing import Any

from parsec._parsec import DateTime, DeviceID, OrganizationID, UserID, UserProfile
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.types import BadOutcomeEnum


class AuthAndLockCommonOnlyBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()


@dataclass(slots=True)
class AuthAndLockCommonOnlyData:
    organization_internal_id: int
    organization_is_sequestered: bool
    device_internal_id: int
    user_id: UserID
    user_internal_id: int
    user_current_profile: UserProfile
    last_common_certificate_timestamp: DateTime


_Q_AUTH_AND_LOCK_COMMON_ONLY_TEMPLATE = """
WITH my_organization AS (
    SELECT
        _id,
        is_expired,
        sequester_authority_certificate IS NOT NULL AS is_sequestered
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
),

-- Common topic lock must occur ASAP
my_locked_common_topic AS (
    SELECT last_timestamp
    FROM common_topic
    WHERE organization = (SELECT my_organization._id FROM my_organization)
    LIMIT 1
    -- Read or write lock?
    {common_row_lock}
),

my_device AS (
    SELECT
        _id,
        user_
    FROM device
    WHERE
        organization = (SELECT my_organization._id FROM my_organization)
        AND device_id = $device_id
    LIMIT 1
),

my_user AS (
    SELECT
        _id,
        user_id,
        current_profile,
        (revoked_on IS NOT NULL) AS revoked
    FROM user_
    WHERE _id = (SELECT my_device.user_ FROM my_device)
    LIMIT 1
)

SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT is_expired FROM my_organization) AS organization_is_expired,
    (SELECT is_sequestered FROM my_organization) AS organization_is_sequestered,
    (SELECT last_timestamp FROM my_locked_common_topic) AS last_common_certificate_timestamp,
    (SELECT _id FROM my_device) AS device_internal_id,
    (SELECT _id FROM my_user) AS user_internal_id,
    (SELECT revoked FROM my_user) AS user_is_revoked,
    (SELECT user_id FROM my_user) AS user_id,
    (SELECT current_profile FROM my_user) AS user_current_profile
"""


_q_auth_and_lock_common_write = Q(
    _Q_AUTH_AND_LOCK_COMMON_ONLY_TEMPLATE.format(common_row_lock="FOR UPDATE")
)
_q_auth_and_lock_common_read = Q(
    _Q_AUTH_AND_LOCK_COMMON_ONLY_TEMPLATE.format(common_row_lock="FOR SHARE")
)


async def auth_and_lock_common_write(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author: DeviceID,
) -> AuthAndLockCommonOnlyData | AuthAndLockCommonOnlyBadOutcome:
    """
    "Auth" in the name might be confusing:
    - When doing an RPC request, actual authentication is done by
      `BaseAuthComponent.authenticated_auth` which is implemented with
      different queries.
    - This function is only to be used to implement the internal methods the
      RPC request implementation relies on (e.g. `BaseBlockComponent`'s `api_block_read`
      vs `block_read`).

    The key difference here is this auth doesn't check if the user is frozen, given
    1) it is already checked by the RPC request authentication, and 2) it is not
    an issue to accept an operation while concurrent operation has just frozen the
    user (since freeze is just a server-side access control flag, unlike
    user revocation which involve certificates which timestamp are required to
    be consistent with other certificates&data).
    """
    return await _do_query(
        conn,
        _q_auth_and_lock_common_write(
            organization_id=organization_id.str,
            device_id=author,
        ),
    )


async def auth_and_lock_common_read(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author: DeviceID,
) -> AuthAndLockCommonOnlyData | AuthAndLockCommonOnlyBadOutcome:
    """
    "Auth" in the name might be confusing:
    - When doing an RPC request, actual authentication is done by
      `BaseAuthComponent.authenticated_auth` which is implemented with
      different queries.
    - This function is only to be used to implement the internal methods the
      RPC request implementation relies on (e.g. `BaseBlockComponent`'s `api_block_read`
      vs `block_read`).

    The key difference here is this auth doesn't check if the user is frozen, given
    1) it is already checked by the RPC request authentication, and 2) it is not
    an issue to accept an operation while concurrent operation has just frozen the
    user (since freeze is just a server-side access control flag, unlike
    user revocation which involve certificates which timestamp are required to
    be consistent with other certificates&data).
    """
    return await _do_query(
        conn,
        _q_auth_and_lock_common_read(
            organization_id=organization_id.str,
            device_id=author,
        ),
    )


async def _do_query(
    conn: AsyncpgConnection, args: tuple[Any, ...]
) -> AuthAndLockCommonOnlyData | AuthAndLockCommonOnlyBadOutcome:
    row = await conn.fetchrow(*args)
    assert row is not None

    # 1) Check organization

    match row["organization_internal_id"]:
        case int() as organization_internal_id:
            pass
        case None:
            return AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND
        case _:
            assert False, row

    match row["organization_is_expired"]:
        case False:
            pass
        case True:
            return AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED
        case _:
            assert False, row

    match row["organization_is_sequestered"]:
        case bool() as organization_is_sequestered:
            pass
        case _:
            assert False, row

    # 2) Check device & user

    match row["device_internal_id"]:
        case int() as device_internal_id:
            pass
        case None:
            return AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND
        case _:
            assert False, row

    # Since device exists, its corresponding user must also exist

    match row["user_internal_id"]:
        case int() as user_internal_id:
            pass
        case _:
            assert False, row

    match row["user_id"]:
        case str() as raw_user_id:
            user_id = UserID.from_hex(raw_user_id)
        case _:
            assert False, row

    match row["user_is_revoked"]:
        case False:
            pass
        case True:
            return AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED
        case _:
            assert False, row

    match row["user_current_profile"]:
        case str() as raw_user_current_profile:
            user_current_profile = UserProfile.from_str(raw_user_current_profile)
        case _:
            assert False, row

    # 3) Check topics

    match row["last_common_certificate_timestamp"]:
        case DateTime() as last_common_certificate_timestamp:
            pass
        case _:
            assert False, row

    return AuthAndLockCommonOnlyData(
        organization_internal_id=organization_internal_id,
        organization_is_sequestered=organization_is_sequestered,
        last_common_certificate_timestamp=last_common_certificate_timestamp,
        device_internal_id=device_internal_id,
        user_internal_id=user_internal_id,
        user_id=user_id,
        user_current_profile=user_current_profile,
    )
