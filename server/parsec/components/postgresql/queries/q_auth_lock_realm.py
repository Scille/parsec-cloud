# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from enum import auto
from typing import Any

from parsec._parsec import (
    DateTime,
    RealmRole,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.types import BadOutcomeEnum


class LockRealmWriteRealmBadOutcome(BadOutcomeEnum):
    REALM_NOT_FOUND = auto()
    USER_NOT_IN_REALM = auto()


@dataclass(slots=True)
class LockRealmWriteRealmData:
    last_realm_certificate_timestamp: DateTime
    realm_internal_id: int
    realm_key_index: int
    realm_user_current_role: RealmRole


_Q_LOCK_REALM_TEMPLATE = """
WITH my_realm AS (
    SELECT
        realm._id,
        key_index
    FROM realm
    WHERE
        organization = $organization_internal_id
        AND realm_id = $realm_id
    LIMIT 1
),

-- Realm topic lock must occur ASAP
my_locked_realm_topic AS (
    SELECT last_timestamp
    FROM realm_topic
    WHERE
        organization = $organization_internal_id
        AND realm = (SELECT _id FROM my_realm)
    LIMIT 1
    -- Read or write lock ?
    {common_row_lock}
)

SELECT
    (SELECT last_timestamp FROM my_locked_realm_topic) AS last_realm_certificate_timestamp,
    (SELECT _id FROM my_realm) AS realm_internal_id,
    (SELECT key_index FROM my_realm) AS realm_key_index,
    (
        SELECT role
        FROM realm_user_role
        WHERE
            user_ = $user_internal_id
            AND realm = (SELECT _id FROM my_realm)
        ORDER BY certified_on DESC
        LIMIT 1
    ) AS realm_user_current_role
"""


_q_lock_realm_write = Q(_Q_LOCK_REALM_TEMPLATE.format(common_row_lock="FOR UPDATE"))
_q_lock_realm_read = Q(_Q_LOCK_REALM_TEMPLATE.format(common_row_lock="FOR SHARE"))


async def lock_realm_write(
    conn: AsyncpgConnection,
    organization_internal_id: int,
    user_internal_id: int,
    realm_id: VlobID,
) -> LockRealmWriteRealmData | LockRealmWriteRealmBadOutcome:
    return await _do_query(
        conn,
        _q_lock_realm_write(
            organization_internal_id=organization_internal_id,
            user_internal_id=user_internal_id,
            realm_id=realm_id,
        ),
    )


async def lock_realm_read(
    conn: AsyncpgConnection,
    organization_internal_id: int,
    user_internal_id: int,
    realm_id: VlobID,
) -> LockRealmWriteRealmData | LockRealmWriteRealmBadOutcome:
    return await _do_query(
        conn,
        _q_lock_realm_read(
            organization_internal_id=organization_internal_id,
            user_internal_id=user_internal_id,
            realm_id=realm_id,
        ),
    )


async def _do_query(
    conn: AsyncpgConnection, args: tuple[Any]
) -> LockRealmWriteRealmData | LockRealmWriteRealmBadOutcome:
    row = await conn.fetchrow(*args)
    assert row is not None

    # 1) Check realm

    match row["realm_internal_id"]:
        case int() as realm_internal_id:
            pass
        case None:
            return LockRealmWriteRealmBadOutcome.REALM_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["realm_key_index"]:
        case int() as realm_key_index:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["realm_user_current_role"]:
        case str() as raw_realm_user_current_role:
            realm_user_current_role = RealmRole.from_str(raw_realm_user_current_role)
        case None:
            return LockRealmWriteRealmBadOutcome.USER_NOT_IN_REALM
        case unknown:
            assert False, repr(unknown)

    # 2) Check topics

    match row["last_realm_certificate_timestamp"]:
        case DateTime() as last_realm_certificate_timestamp:
            pass
        case unknown:
            assert False, repr(unknown)

    return LockRealmWriteRealmData(
        last_realm_certificate_timestamp=last_realm_certificate_timestamp,
        realm_internal_id=realm_internal_id,
        realm_key_index=realm_key_index,
        realm_user_current_role=realm_user_current_role,
    )
