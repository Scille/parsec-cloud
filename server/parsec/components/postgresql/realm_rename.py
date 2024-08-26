# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmNameCertificate,
    RealmRole,
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
)
from parsec.components.realm import (
    BadKeyIndex,
    CertificateBasedActionIdempotentOutcome,
    RealmRenameStoreBadOutcome,
    RealmRenameValidateBadOutcome,
    realm_rename_validate,
)
from parsec.events import EventRealmCertificate

_q_realm_already_has_name_certificates = Q(
    """
SELECT COALESCE(
    (
        SELECT TRUE
        FROM realm_name
        WHERE realm = $realm_internal_id
    ),
    FALSE
)
"""
)


_q_rename_realm = Q(
    """
WITH new_realm_name AS (
    INSERT INTO realm_name (
        realm,
        realm_name_certificate,
        certified_by,
        certified_on
    ) VALUES (
        $realm_internal_id,
        $realm_name_certificate,
        $author_internal_id,
        $certified_on
    )
),

update_realm_topic AS (
    UPDATE realm_topic
    SET last_timestamp = $certified_on
    WHERE
        realm = $realm_internal_id
        -- Sanity check
        AND last_timestamp < $certified_on
    RETURNING TRUE
)

SELECT
    COALESCE((SELECT * FROM update_realm_topic), FALSE) AS update_realm_topic_ok
"""
)


async def realm_rename(
    event_bus: EventBus,
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    realm_name_certificate: bytes,
    initial_name_or_fail: bool,
) -> (
    RealmNameCertificate
    | BadKeyIndex
    | CertificateBasedActionIdempotentOutcome
    | RealmRenameValidateBadOutcome
    | TimestampOutOfBallpark
    | RealmRenameStoreBadOutcome
    | RequireGreaterTimestamp
):
    match await auth_and_lock_common_read(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as db_common_data:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return RealmRenameStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return RealmRenameStoreBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return RealmRenameStoreBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return RealmRenameStoreBadOutcome.AUTHOR_REVOKED

    match realm_rename_validate(
        now=now,
        expected_author=author,
        author_verify_key=author_verify_key,
        realm_name_certificate=realm_name_certificate,
    ):
        case RealmNameCertificate() as certif:
            pass
        case error:
            return error

    match await lock_realm_write(
        conn,
        db_common_data.organization_internal_id,
        db_common_data.user_internal_id,
        certif.realm_id,
    ):
        case LockRealmWriteRealmData() as db_realm:
            if db_realm.realm_user_current_role != RealmRole.OWNER:
                return RealmRenameStoreBadOutcome.AUTHOR_NOT_ALLOWED
        case LockRealmWriteRealmBadOutcome.REALM_NOT_FOUND:
            return RealmRenameStoreBadOutcome.REALM_NOT_FOUND
        case LockRealmWriteRealmBadOutcome.USER_NOT_IN_REALM:
            return RealmRenameStoreBadOutcome.AUTHOR_NOT_ALLOWED

    # We only accept the last key
    if db_realm.realm_key_index != certif.key_index:
        return BadKeyIndex(
            last_realm_certificate_timestamp=db_realm.last_realm_certificate_timestamp
        )

    if initial_name_or_fail:
        realm_already_has_name_certificates = await conn.fetchval(
            *_q_realm_already_has_name_certificates(
                realm_internal_id=db_realm.realm_internal_id,
            )
        )
        if realm_already_has_name_certificates:
            return CertificateBasedActionIdempotentOutcome(
                certificate_timestamp=db_realm.last_realm_certificate_timestamp
            )

    # Ensure we are not breaking causality by adding a newer timestamp.

    last_certificate = max(
        db_common_data.last_common_certificate_timestamp,
        db_realm.last_realm_certificate_timestamp,
    )
    if last_certificate >= certif.timestamp:
        return RequireGreaterTimestamp(strictly_greater_than=last_certificate)

    # All checks are good, now we do the actual insertion

    update_realm_topic_ok = await conn.fetchval(
        *_q_rename_realm(
            realm_internal_id=db_realm.realm_internal_id,
            realm_name_certificate=realm_name_certificate,
            author_internal_id=db_common_data.device_internal_id,
            certified_on=certif.timestamp,
        )
    )
    match update_realm_topic_ok:
        case True:
            pass
        case unknown:
            assert False, unknown

    await event_bus.send(
        EventRealmCertificate(
            organization_id=organization_id,
            timestamp=certif.timestamp,
            realm_id=certif.realm_id,
            user_id=db_common_data.user_id,
            role_removed=False,
        )
    )

    return certif
