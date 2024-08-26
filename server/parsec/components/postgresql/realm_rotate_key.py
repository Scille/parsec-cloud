# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmKeyRotationCertificate,
    RealmRole,
    UserID,
    VerifyKey,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.events import EventBus
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.queries import (
    AuthAndLockCommonOnlyBadOutcome,
    AuthAndLockCommonOnlyData,
    LockRealmWriteRealmBadOutcome,
    LockRealmWriteRealmData,
    auth_and_lock_common_read,
    lock_realm_write,
)
from parsec.components.postgresql.utils import (
    Q,
    q_user,
    q_user_internal_id,
)
from parsec.components.realm import (
    BadKeyIndex,
    RealmRotateKeyStoreBadOutcome,
    RealmRotateKeyValidateBadOutcome,
    realm_rotate_key_validate,
)
from parsec.events import EventRealmCertificate

_q_get_realm_current_participants = Q(
    f"""
WITH per_user_last_role AS (
    SELECT DISTINCT ON(user_)
        { q_user(_id="realm_user_role.user_", select="user_id") } AS user_id,
        role
    FROM  realm_user_role
    WHERE realm = $realm_internal_id
    ORDER BY user_, certified_on DESC
)

SELECT user_id FROM per_user_last_role WHERE role IS NOT NULL
"""
)


_q_insert_keys_bundle = Q(
    """
WITH new_realm_keys_bundle AS (
    INSERT INTO realm_keys_bundle (
        realm,
        key_index,
        realm_key_rotation_certificate,
        certified_by,
        certified_on,
        key_canary,
        keys_bundle
    ) VALUES (
        $realm_internal_id,
        $key_index,
        $realm_key_rotation_certificate,
        $author_internal_id,
        $certified_on,
        $key_canary,
        $keys_bundle
    )
    RETURNING _id
),

updated_realm_key_index AS (
    UPDATE realm
    SET key_index = $key_index
    WHERE _id = $realm_internal_id
),

updated_realm_topic AS (
    UPDATE realm_topic
    SET last_timestamp = $certified_on
    WHERE
        realm = $realm_internal_id
        -- Sanity check
        AND last_timestamp < $certified_on
    RETURNING TRUE
)

SELECT
    COALESCE((SELECT * FROM updated_realm_topic), FALSE) AS update_realm_topic_ok,
    (SELECT _id FROM new_realm_keys_bundle) AS keys_bundle_internal_id
"""
)


_q_insert_keys_bundle_access = Q(
    f"""
INSERT INTO realm_keys_bundle_access (
    realm,
    user_,
    realm_keys_bundle,
    access
) VALUES (
    $realm_internal_id,
    { q_user_internal_id(organization="$organization_internal_id", user_id="$user_id") },
    $realm_keys_bundle_internal_id,
    $access
)
"""
)


async def realm_rotate_key(
    event_bus: EventBus,
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    realm_key_rotation_certificate: bytes,
    per_participant_keys_bundle_access: dict[UserID, bytes],
    keys_bundle: bytes,
) -> (
    RealmKeyRotationCertificate
    | BadKeyIndex
    | RealmRotateKeyValidateBadOutcome
    | TimestampOutOfBallpark
    | RealmRotateKeyStoreBadOutcome
    | RequireGreaterTimestamp
):
    match await auth_and_lock_common_read(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as db_common:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return RealmRotateKeyStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return RealmRotateKeyStoreBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return RealmRotateKeyStoreBadOutcome.AUTHOR_REVOKED

    match realm_rotate_key_validate(
        now=now,
        expected_author=author,
        author_verify_key=author_verify_key,
        realm_key_rotation_certificate=realm_key_rotation_certificate,
    ):
        case RealmKeyRotationCertificate() as certif:
            pass
        case error:
            return error

    match await lock_realm_write(
        conn,
        db_common.organization_internal_id,
        db_common.user_internal_id,
        certif.realm_id,
    ):
        case LockRealmWriteRealmData() as db_realm:
            if db_realm.realm_user_current_role != RealmRole.OWNER:
                return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_ALLOWED
        case LockRealmWriteRealmBadOutcome.REALM_NOT_FOUND:
            return RealmRotateKeyStoreBadOutcome.REALM_NOT_FOUND
        case LockRealmWriteRealmBadOutcome.USER_NOT_IN_REALM:
            return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_ALLOWED

    # We only accept the last key
    if db_realm.realm_key_index + 1 != certif.key_index:
        return BadKeyIndex(
            last_realm_certificate_timestamp=db_realm.last_realm_certificate_timestamp
        )

    # Ensure we are not breaking causality by adding a newer timestamp.

    last_certificate = max(
        db_common.last_common_certificate_timestamp,
        db_realm.last_realm_certificate_timestamp,
    )
    if last_certificate >= certif.timestamp:
        return RequireGreaterTimestamp(strictly_greater_than=last_certificate)

    rows = await conn.fetch(
        *_q_get_realm_current_participants(
            realm_internal_id=db_realm.realm_internal_id,
        )
    )
    participants = {UserID.from_hex(row["user_id"]) for row in rows}
    if per_participant_keys_bundle_access.keys() != participants:
        return RealmRotateKeyStoreBadOutcome.PARTICIPANT_MISMATCH

    # All checks are good, now we do the actual insertion

    row = await conn.fetchrow(
        *_q_insert_keys_bundle(
            realm_internal_id=db_realm.realm_internal_id,
            key_index=certif.key_index,
            realm_key_rotation_certificate=realm_key_rotation_certificate,
            author_internal_id=db_common.device_internal_id,
            certified_on=certif.timestamp,
            key_canary=certif.key_canary,
            keys_bundle=keys_bundle,
        )
    )
    assert row is not None

    match row["update_realm_topic_ok"]:
        case True:
            pass
        case unknown:
            assert False, unknown

    match row["keys_bundle_internal_id"]:
        case int() as keys_bundle_internal_id:
            pass
        case unknown:
            assert False, unknown

    def arg_gen():
        for user_id, access in per_participant_keys_bundle_access.items():
            x = _q_insert_keys_bundle_access.arg_only(
                organization_internal_id=db_common.organization_internal_id,
                realm_internal_id=db_realm.realm_internal_id,
                user_id=user_id,
                realm_keys_bundle_internal_id=keys_bundle_internal_id,
                access=access,
            )
            yield x

    await conn.executemany(
        _q_insert_keys_bundle_access.sql,
        arg_gen(),
    )

    await event_bus.send(
        EventRealmCertificate(
            organization_id=organization_id,
            timestamp=certif.timestamp,
            realm_id=certif.realm_id,
            user_id=db_common.user_id,
            role_removed=False,
        )
    )

    return certif
