# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
    q_device,
)
from parsec.components.vlob import (
    VlobReadAsUserBadOutcome,
    VlobReadResult,
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


_q_get_latest_vlob = Q(
    f"""
SELECT
    key_index,
    { q_device(select="device_id", _id="author") } AS author,
    version,
    created_on,
    blob
FROM vlob_atom
WHERE
    realm = $realm_internal_id
    AND vlob_id = $vlob_id
ORDER BY version DESC
LIMIT 1
"""
)


_q_get_vlob_at_timestamp = Q(
    f"""
SELECT
    key_index,
    { q_device(select="device_id", _id="author") } AS author,
    version,
    created_on,
    blob
FROM vlob_atom
WHERE
    realm = $realm_internal_id
    AND vlob_id = $vlob_id
    AND vlob_id = $vlob_id
    AND created_on <= $timestamp
ORDER BY version DESC
LIMIT 1
"""
)


async def vlob_read_batch(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: VlobID,
    vlobs: list[VlobID],
    at: DateTime | None,
) -> VlobReadResult | VlobReadAsUserBadOutcome:
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
            return VlobReadAsUserBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["organization_is_expired"]:
        case False:
            pass
        case True:
            return VlobReadAsUserBadOutcome.ORGANIZATION_EXPIRED
        case unknown:
            assert False, repr(unknown)

    # 1.2) Check device & user

    match row["device_internal_id"]:
        case int():
            pass
        case None:
            return VlobReadAsUserBadOutcome.AUTHOR_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["user_is_frozen"]:
        case False:
            pass
        case True:
            return VlobReadAsUserBadOutcome.AUTHOR_REVOKED
        case unknown:
            assert False, repr(unknown)

    match row["user_is_revoked"]:
        case False:
            pass
        case True:
            return VlobReadAsUserBadOutcome.AUTHOR_REVOKED
        case unknown:
            assert False, repr(unknown)

    # 1.3) Check topics

    match row["last_common_certificate_timestamp"]:
        case DateTime() as last_common_certificate_timestamp:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["last_realm_certificate_timestamp"]:
        case DateTime() as last_realm_certificate_timestamp:
            pass
        case None:
            return VlobReadAsUserBadOutcome.REALM_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    # 1.4) Check realm access
    # (Note since realm topic exists, then the realm also exist !)

    match row["realm_internal_id"]:
        case int() as realm_internal_id:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["user_can_read"]:
        case True:
            pass
        case False:
            return VlobReadAsUserBadOutcome.AUTHOR_NOT_ALLOWED
        case unknown:
            assert False, repr(unknown)

    # 2) Checks are good, we can retrieve the vlobs

    output = []
    for vlob_id in vlobs:
        if at is None:
            row = await conn.fetchrow(
                *_q_get_latest_vlob(
                    realm_internal_id=realm_internal_id,
                    vlob_id=vlob_id,
                )
            )
        else:
            row = await conn.fetchrow(
                *_q_get_vlob_at_timestamp(
                    realm_internal_id=realm_internal_id,
                    vlob_id=vlob_id,
                    timestamp=at,
                )
            )
        if row is None:
            continue
        key_index = row["key_index"]
        vlob_author = DeviceID.from_hex(row["author"])
        version = row["version"]
        created_on = row["created_on"]
        blob = row["blob"]
        output.append(
            (
                vlob_id,
                key_index,
                vlob_author,
                version,
                created_on,
                blob,
            )
        )

    return VlobReadResult(
        items=output,
        needed_common_certificate_timestamp=last_common_certificate_timestamp,
        needed_realm_certificate_timestamp=last_realm_certificate_timestamp,
    )
