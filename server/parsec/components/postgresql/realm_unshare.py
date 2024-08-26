# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmRole,
    RealmRoleCertificate,
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
    CertificateBasedActionIdempotentOutcome,
    RealmUnshareStoreBadOutcome,
    RealmUnshareValidateBadOutcome,
    realm_unshare_validate,
)
from parsec.events import EventRealmCertificate

_q_get_recipient_user_and_realm_role_and_last_vlob_timestamp = Q(
    """
WITH my_user AS (
    SELECT _id
    FROM user_
    WHERE
        organization = $organization_internal_id
        AND user_id = $recipient_user_id
),

my_realm_role AS (
    SELECT role
    FROM realm_user_role
    WHERE
        user_ = (SELECT _id FROM my_user)
        AND realm = $realm_internal_id
    ORDER BY certified_on DESC
    LIMIT 1
),

my_last_vlob_timestamp AS (
    SELECT MAX(created_on) AS timestamp
    FROM vlob_atom
    WHERE realm = $realm_internal_id
)

SELECT
    (SELECT _id FROM my_user) AS user_internal_id,
    (SELECT role FROM my_realm_role) AS current_role,
    (SELECT timestamp FROM my_last_vlob_timestamp) AS last_vlob_timestamp
"""
)


_q_unshare = Q(
    """
WITH new_realm_user_role AS (
    INSERT INTO realm_user_role (
        realm,
        user_,
        role,
        certificate,
        certified_by,
        certified_on
    ) VALUES (
        $realm_internal_id,
        $recipient_internal_id,
        NULL,
        $certificate,
        $certified_by,
        $certified_on
    )
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
    COALESCE((SELECT * FROM updated_realm_topic), FALSE) AS update_realm_topic_ok
"""
)


async def realm_unshare(
    event_bus: EventBus,
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    realm_role_certificate: bytes,
) -> (
    RealmRoleCertificate
    | CertificateBasedActionIdempotentOutcome
    | RealmUnshareValidateBadOutcome
    | TimestampOutOfBallpark
    | RealmUnshareStoreBadOutcome
    | RequireGreaterTimestamp
):
    # 1) Read lock common topic

    match await auth_and_lock_common_read(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as db_common:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return RealmUnshareStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return RealmUnshareStoreBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return RealmUnshareStoreBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return RealmUnshareStoreBadOutcome.AUTHOR_REVOKED

    # 2) Validate certificate

    match realm_unshare_validate(
        now=now,
        expected_author_user_id=db_common.user_id,
        expected_author_device_id=author,
        author_verify_key=author_verify_key,
        realm_role_certificate=realm_role_certificate,
    ):
        case RealmRoleCertificate() as certif:
            assert certif.role is None
        case error:
            return error

    # 3) Write lock realm topic

    match await lock_realm_write(
        conn,
        db_common.organization_internal_id,
        db_common.user_internal_id,
        certif.realm_id,
    ):
        case LockRealmWriteRealmData() as db_realm:
            pass
        case LockRealmWriteRealmBadOutcome.REALM_NOT_FOUND:
            return RealmUnshareStoreBadOutcome.REALM_NOT_FOUND
        case LockRealmWriteRealmBadOutcome.USER_NOT_IN_REALM:
            return RealmUnshareStoreBadOutcome.AUTHOR_NOT_ALLOWED

    # 4) Fetch from database what is needed and do the checks

    row = await conn.fetchrow(
        *_q_get_recipient_user_and_realm_role_and_last_vlob_timestamp(
            organization_internal_id=db_common.organization_internal_id,
            realm_internal_id=db_realm.realm_internal_id,
            recipient_user_id=certif.user_id,
        )
    )
    assert row is not None

    # 4.1) Check recipient exists

    # Note we don't check if the recipient is revoked here: it is indeed allowed
    # to unshare with a revoked user. This allows for client to only check for
    # unshare event to detect when key rotation is needed.

    match row["user_internal_id"]:
        case int() as recipient_internal_id:
            pass
        case None:
            return RealmUnshareStoreBadOutcome.RECIPIENT_NOT_FOUND
        case unknown:
            assert False, unknown

    # 4.2) Check author is allowed to unshare recipient

    match row["current_role"]:
        case str() as raw_recipient_current_role:
            recipient_current_role = RealmRole.from_str(raw_recipient_current_role)
        case None as recipient_current_role:
            return CertificateBasedActionIdempotentOutcome(
                certificate_timestamp=db_realm.last_realm_certificate_timestamp
            )
        case unknown:
            assert False, unknown

    match (db_realm.realm_user_current_role, recipient_current_role):
        # OWNER can remove any role
        case (RealmRole.OWNER, _):
            pass
        # MANAGER cannot remove OWNER/MANAGER roles
        case (RealmRole.MANAGER, RealmRole.READER | RealmRole.CONTRIBUTOR):
            pass
        case _:
            return RealmUnshareStoreBadOutcome.AUTHOR_NOT_ALLOWED

    # 4.3) Ensure we are not breaking causality by adding a newer timestamp.

    match row["last_vlob_timestamp"]:
        case DateTime() as last_vlob_timestamp:
            last_timestamp = max(
                db_common.last_common_certificate_timestamp,
                db_realm.last_realm_certificate_timestamp,
                last_vlob_timestamp,
            )
        # `None` if the realm is totally empty
        case None:
            last_timestamp = max(
                db_common.last_common_certificate_timestamp,
                db_realm.last_realm_certificate_timestamp,
            )
        case unknown:
            assert False, unknown

    if last_timestamp >= certif.timestamp:
        return RequireGreaterTimestamp(strictly_greater_than=last_timestamp)

    # 5) All checks are good, now we do the actual insertion

    row = await conn.fetchrow(
        *_q_unshare(
            realm_internal_id=db_realm.realm_internal_id,
            recipient_internal_id=recipient_internal_id,
            certificate=realm_role_certificate,
            certified_by=db_common.device_internal_id,
            certified_on=certif.timestamp,
        )
    )
    assert row is not None

    match row["update_realm_topic_ok"]:
        case True:
            pass
        case unknown:
            assert False, unknown

    await event_bus.send(
        EventRealmCertificate(
            organization_id=organization_id,
            timestamp=certif.timestamp,
            realm_id=certif.realm_id,
            user_id=certif.user_id,
            role_removed=True,
        )
    )

    return certif
