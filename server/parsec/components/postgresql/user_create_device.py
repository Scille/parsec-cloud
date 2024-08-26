# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    OrganizationID,
    VerifyKey,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.events import EventBus
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.queries import (
    AuthAndLockCommonOnlyBadOutcome,
    AuthAndLockCommonOnlyData,
    auth_and_lock_common_write,
)
from parsec.components.postgresql.utils import Q
from parsec.components.user import (
    UserCreateDeviceStoreBadOutcome,
    UserCreateDeviceValidateBadOutcome,
    user_create_device_validate,
)
from parsec.events import (
    EventCommonCertificate,
)

_q_insert_device = Q("""
WITH new_device AS (
    INSERT INTO device (
        organization,
        user_,
        device_id,
        device_label,
        verify_key,
        device_certificate,
        redacted_device_certificate,
        device_certifier,
        created_on
    )
    VALUES (
        $organization_internal_id,
        $user_internal_id,
        $device_id,
        $device_label,
        $verify_key,
        $device_certificate,
        $redacted_device_certificate,
        $author_internal_id,
        $created_on
    )
    ON CONFLICT DO NOTHING
    RETURNING TRUE
),

updated_common_topic AS (
    UPDATE common_topic
    SET last_timestamp = $created_on
    WHERE
        organization = $organization_internal_id
        -- Sanity check
        AND last_timestamp < $created_on
    RETURNING TRUE
)

SELECT
    COALESCE((SELECT * FROM new_device), FALSE) AS insert_new_device_ok,
    COALESCE((SELECT * FROM updated_common_topic), FALSE) AS update_common_topic_ok
""")


async def user_create_device(
    event_bus: EventBus,
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    device_certificate: bytes,
    redacted_device_certificate: bytes,
) -> (
    DeviceCertificate
    | UserCreateDeviceValidateBadOutcome
    | UserCreateDeviceStoreBadOutcome
    | TimestampOutOfBallpark
    | RequireGreaterTimestamp
):
    # 1) Write lock common topic

    match await auth_and_lock_common_write(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as db_common:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return UserCreateDeviceStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return UserCreateDeviceStoreBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return UserCreateDeviceStoreBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return UserCreateDeviceStoreBadOutcome.AUTHOR_REVOKED

    # 2) Validate certificate

    match user_create_device_validate(
        now=now,
        expected_author_user_id=db_common.user_id,
        expected_author_device_id=author,
        author_verify_key=author_verify_key,
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    ):
        case DeviceCertificate() as certif:
            pass
        case error:
            return error

    # 3) Ensure we are not breaking causality by adding a newer timestamp.

    if certif.timestamp <= db_common.last_common_certificate_timestamp:
        return RequireGreaterTimestamp(
            strictly_greater_than=db_common.last_common_certificate_timestamp
        )

    # 4) (Almost) all checks are good, now we do the actual insertion
    # Almost because the case the device_id already exists is handled in the insert query

    row = await conn.fetchrow(
        *_q_insert_device(
            organization_internal_id=db_common.organization_internal_id,
            author_internal_id=db_common.device_internal_id,
            created_on=certif.timestamp,
            user_internal_id=db_common.user_internal_id,
            device_id=certif.device_id,
            device_label=certif.device_label.str,
            verify_key=certif.verify_key.encode(),
            device_certificate=device_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
    )
    assert row is not None
    match row["insert_new_device_ok"]:
        case True:
            pass
        case False:
            return UserCreateDeviceStoreBadOutcome.DEVICE_ALREADY_EXISTS
        case unknown:
            assert False, unknown

    match row["update_common_topic_ok"]:
        case True:
            pass
        case unknown:
            assert False, unknown

    await event_bus.send(
        EventCommonCertificate(
            organization_id=organization_id,
            timestamp=certif.timestamp,
        )
    )

    return certif
