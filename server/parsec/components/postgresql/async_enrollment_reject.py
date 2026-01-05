# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    AsyncEnrollmentID,
    DateTime,
    DeviceID,
    OrganizationID,
    UserProfile,
)
from parsec.components.async_enrollment import (
    AsyncEnrollmentRejectBadOutcome,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.async_enrollment_submit import (
    _q_take_async_enrollment_create_write_lock,
)
from parsec.components.postgresql.handler import send_signal
from parsec.components.postgresql.queries import (
    AuthAndLockCommonOnlyBadOutcome,
    auth_and_lock_common_read,
)
from parsec.components.postgresql.utils import Q
from parsec.events import EventAsyncEnrollment

_q_get_enrollment = Q("""
SELECT
    _id AS enrollment_internal_id,
    state
FROM async_enrollment
WHERE
    organization = $organization_internal_id
    AND enrollment_id = $enrollment_id
LIMIT 1
""")


_q_reject_enrollment = Q("""
UPDATE async_enrollment
SET
    state = 'REJECTED',
    rejected_on = $now
WHERE
    _id = $enrollment_internal_id
""")


async def async_enrollment_reject(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    enrollment_id: AsyncEnrollmentID,
) -> None | AsyncEnrollmentRejectBadOutcome:
    # 1) We lock the common topic to ensure the user doesn't change its role
    # after we checked it.
    # Note the lock is not strictly mandatory since here we only modify data
    # that are not signed and don't require strong consistency with the
    # certificates (unlike e.g. blocks where the client actively checks that
    # no block has been created by a user after it has been revoked).
    # However since there is no hard performance requirements here we choose
    # to stay consistent with what do the `accept` part.
    match await auth_and_lock_common_read(conn, organization_id, author):
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return AsyncEnrollmentRejectBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return AsyncEnrollmentRejectBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return AsyncEnrollmentRejectBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return AsyncEnrollmentRejectBadOutcome.AUTHOR_REVOKED
        case auth_data:
            organization_internal_id = auth_data.organization_internal_id

    if auth_data.user_current_profile != UserProfile.ADMIN:
        return AsyncEnrollmentRejectBadOutcome.AUTHOR_NOT_ALLOWED

    # 2) Take lock to prevent any concurrent async enrollment creation

    await _q_take_async_enrollment_create_write_lock(conn, organization_id)

    # 3) Check enrollment exists and is in SUBMITTED state

    enrollment_row = await conn.fetchrow(
        *_q_get_enrollment(
            organization_internal_id=organization_internal_id,
            enrollment_id=enrollment_id,
        )
    )

    if enrollment_row is None:
        return AsyncEnrollmentRejectBadOutcome.ENROLLMENT_NOT_FOUND

    match enrollment_row["enrollment_internal_id"]:
        case int() as enrollment_internal_id:
            pass
        case _:
            assert False, enrollment_row

    match enrollment_row["state"]:
        case "SUBMITTED":
            pass
        case _:
            return AsyncEnrollmentRejectBadOutcome.ENROLLMENT_NO_LONGER_AVAILABLE

    # 5) Reject the enrollment

    await conn.execute(
        *_q_reject_enrollment(
            enrollment_internal_id=enrollment_internal_id,
            now=now,
        )
    )

    await send_signal(
        conn,
        EventAsyncEnrollment(
            organization_id=organization_id,
        ),
    )
