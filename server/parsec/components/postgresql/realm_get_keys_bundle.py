# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DeviceID,
    OrganizationID,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.queries import (
    AuthNoLockBadOutcome,
    AuthNoLockData,
    auth_no_lock,
)
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.components.realm import (
    KeysBundle,
    RealmGetKeysBundleBadOutcome,
)

_g_get_last_keys_bundle_and_access = Q(
    """
WITH my_realm AS (
    SELECT
        realm._id,
        realm_user_role.role IS NOT NULL AS user_has_role
    FROM realm
    -- Left join so that we get a line even if user has never been part of the realm
    LEFT JOIN realm_user_role ON realm._id = realm_user_role.realm
    WHERE
        realm_user_role.user_ = $user_internal_id
        AND realm.realm_id = $realm_id
    ORDER BY realm_user_role.certified_on DESC
    LIMIT 1
),

my_keys_bundle AS (
    SELECT
        _id,
        key_index,
        keys_bundle
    FROM realm_keys_bundle
    WHERE realm = (SELECT _id FROM my_realm)
    ORDER BY certified_on DESC
    LIMIT 1
),

my_keys_bundle_access AS (
    SELECT access
    FROM realm_keys_bundle_access
    WHERE
        realm_keys_bundle = (SELECT _id FROM my_keys_bundle)
        AND user_ = $user_internal_id
)

SELECT
    (SELECT _id FROM my_realm) AS realm_internal_id,
    (SELECT user_has_role FROM my_realm) AS user_has_role,
    (SELECT key_index FROM my_keys_bundle) AS key_index,
    (SELECT keys_bundle FROM my_keys_bundle) AS keys_bundle,
    (SELECT access FROM my_keys_bundle_access) AS keys_bundle_access
"""
)


_g_get_keys_bundle_and_access = Q(
    """
WITH my_realm AS (
    SELECT
        realm._id,
        realm_user_role.role IS NOT NULL AS user_has_role
    FROM realm
    -- Left join so that we get a line even if user has never been part of the realm
    LEFT JOIN realm_user_role ON realm._id = realm_user_role.realm
    WHERE
        realm_user_role.user_ = $user_internal_id
        AND realm.realm_id = $realm_id
    ORDER BY realm_user_role.certified_on DESC
    LIMIT 1
),

my_keys_bundle AS (
    SELECT
        _id,
        key_index,
        keys_bundle
    FROM realm_keys_bundle
    WHERE
        realm = (SELECT _id FROM my_realm)
        AND key_index = $key_index
    LIMIT 1
),

my_keys_bundle_access AS (
    SELECT access
    FROM realm_keys_bundle_access
    WHERE
        realm_keys_bundle = (SELECT _id FROM my_keys_bundle)
        AND user_ = $user_internal_id
)

SELECT
    (SELECT _id FROM my_realm) AS realm_internal_id,
    (SELECT user_has_role FROM my_realm) AS user_has_role,
    (SELECT key_index FROM my_keys_bundle) AS key_index,
    (SELECT keys_bundle FROM my_keys_bundle) AS keys_bundle,
    (SELECT access FROM my_keys_bundle_access) AS keys_bundle_access
"""
)


async def realm_get_keys_bundle(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: VlobID,
    key_index: int | None,
) -> KeysBundle | RealmGetKeysBundleBadOutcome:
    match await auth_no_lock(conn, organization_id, author):
        case AuthNoLockData() as db_auth:
            pass
        case AuthNoLockBadOutcome.ORGANIZATION_NOT_FOUND:
            return RealmGetKeysBundleBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthNoLockBadOutcome.ORGANIZATION_EXPIRED:
            return RealmGetKeysBundleBadOutcome.ORGANIZATION_EXPIRED
        case AuthNoLockBadOutcome.AUTHOR_NOT_FOUND:
            return RealmGetKeysBundleBadOutcome.AUTHOR_NOT_FOUND
        case AuthNoLockBadOutcome.AUTHOR_REVOKED:
            return RealmGetKeysBundleBadOutcome.AUTHOR_REVOKED

    # `key_index` starts at 1, but the array starts at 0
    match key_index:
        case 0:
            return RealmGetKeysBundleBadOutcome.BAD_KEY_INDEX
        case None:
            q = _g_get_last_keys_bundle_and_access(
                user_internal_id=db_auth.user_internal_id,
                realm_id=realm_id,
            )
        case int() as key_index:
            q = _g_get_keys_bundle_and_access(
                user_internal_id=db_auth.user_internal_id,
                realm_id=realm_id,
                key_index=key_index,
            )

    row = await conn.fetchrow(*q)
    assert row is not None

    match row["realm_internal_id"]:
        case int():
            pass
        case None:
            return RealmGetKeysBundleBadOutcome.REALM_NOT_FOUND
        case unknown:
            assert False, unknown

    match row["user_has_role"]:
        case True:
            pass
        # `None` if user never had role, `False` if he has been unshared
        case False | None:
            return RealmGetKeysBundleBadOutcome.AUTHOR_NOT_ALLOWED
        case unknown:
            assert False, unknown

    match row["key_index"]:
        case int() as current_key_index:
            pass
        case None:
            return RealmGetKeysBundleBadOutcome.BAD_KEY_INDEX
        case unknown:
            assert False, unknown

    match row["keys_bundle"]:
        case bytes() as keys_bundle:
            pass
        case unknown:
            assert False, unknown

    match row["keys_bundle_access"]:
        case bytes() as keys_bundle_access:
            pass
        case None:
            return RealmGetKeysBundleBadOutcome.ACCESS_NOT_AVAILABLE_FOR_AUTHOR
        case unknown:
            assert False, unknown

    return KeysBundle(
        key_index=current_key_index,
        keys_bundle_access=keys_bundle_access,
        keys_bundle=keys_bundle,
    )
