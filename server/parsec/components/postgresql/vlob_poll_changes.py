# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DeviceID,
    OrganizationID,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.components.vlob import (
    VlobPollChangesAsUserBadOutcome,
)

_q_poll_changes = Q(
    """
SELECT
    index,
    vlob_id,
    vlob_atom.version
FROM realm_vlob_update
LEFT JOIN vlob_atom ON realm_vlob_update.vlob_atom = vlob_atom._id
WHERE
    vlob_atom.realm = $realm_internal_id
    AND index > $checkpoint
ORDER BY index ASC
"""
)


_q_read_fetch_base_data = Q(
    """
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
        (revoked_on IS NOT NULL) AS revoked
    FROM user_
    WHERE _id = (SELECT user_ FROM my_device)
    LIMIT 1
),

my_realm AS (
    SELECT _id
    FROM realm
    WHERE
        realm_id = $realm_id
        AND organization = (SELECT _id FROM my_organization)
    LIMIT 1
)

SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT is_expired FROM my_organization) AS organization_is_expired,
    (SELECT _id FROM my_device) AS device_internal_id,
    (SELECT frozen FROM my_user) AS user_is_frozen,
    (SELECT revoked FROM my_user) AS user_is_revoked,
    (
        SELECT last_timestamp
        FROM common_topic
        WHERE organization = (SELECT _id FROM my_organization)
        LIMIT 1
    ) as last_common_certificate_timestamp,
    (SELECT _id FROM my_realm) AS realm_internal_id,
    (
        SELECT last_timestamp
        FROM realm_topic
        WHERE
            realm = (SELECT _id FROM my_realm)
        LIMIT 1
    ) AS last_realm_certificate_timestamp,
    COALESCE(
        (
            SELECT
                realm_user_role.role IS NOT NULL
            FROM realm_user_role
            WHERE
                realm_user_role.user_ = (SELECT _id FROM my_user)
                AND realm_user_role.realm = (SELECT _id FROM my_realm)
            ORDER BY certified_on DESC
            LIMIT 1
        ),
        False
    ) AS user_can_read
"""
)


async def vlob_poll_changes(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: VlobID,
    checkpoint: int,
) -> tuple[int, list[tuple[VlobID, int]]] | VlobPollChangesAsUserBadOutcome:
    row = await conn.fetchrow(
        *_q_read_fetch_base_data(
            organization_id=organization_id.str,
            device_id=author,
            realm_id=realm_id,
        )
    )
    assert row is not None

    # 1.1) Check organization

    match row["organization_internal_id"]:
        case int():
            pass
        case None:
            return VlobPollChangesAsUserBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["organization_is_expired"]:
        case False:
            pass
        case True:
            return VlobPollChangesAsUserBadOutcome.ORGANIZATION_EXPIRED
        case unknown:
            assert False, repr(unknown)

    # 1.2) Check device & user

    match row["device_internal_id"]:
        case int():
            pass
        case None:
            return VlobPollChangesAsUserBadOutcome.AUTHOR_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["user_is_frozen"]:
        case False:
            pass
        case True:
            return VlobPollChangesAsUserBadOutcome.AUTHOR_REVOKED
        case unknown:
            assert False, repr(unknown)

    match row["user_is_revoked"]:
        case False:
            pass
        case True:
            return VlobPollChangesAsUserBadOutcome.AUTHOR_REVOKED
        case unknown:
            assert False, repr(unknown)

    # 1.3) Check realm access

    match row["realm_internal_id"]:
        case int() as realm_internal_id:
            pass
        case None:
            return VlobPollChangesAsUserBadOutcome.REALM_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["user_can_read"]:
        case True:
            pass
        case False:
            return VlobPollChangesAsUserBadOutcome.AUTHOR_NOT_ALLOWED
        case unknown:
            assert False, repr(unknown)

    # 2) Checks are good, we can retrieve the vlobs

    rows = await conn.fetch(
        *_q_poll_changes(
            realm_internal_id=realm_internal_id,
            checkpoint=checkpoint,
        )
    )

    items = {}
    current_checkpoint = checkpoint
    for row in rows:
        current_checkpoint = row["index"]
        vlob_id = VlobID.from_hex(row["vlob_id"])
        version = row["version"]
        items[vlob_id] = version

    return current_checkpoint, list(items.items())
