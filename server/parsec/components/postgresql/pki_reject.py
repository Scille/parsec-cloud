# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    PKIEnrollmentID,
    UserProfile,
)
from parsec.components.pki import (
    PkiEnrollmentRejectBadOutcome,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.events import send_signal
from parsec.components.postgresql.queries import (
    AuthAndLockCommonOnlyBadOutcome,
    auth_and_lock_common_read,
)
from parsec.components.postgresql.utils import Q
from parsec.events import EventPkiEnrollment

_q_get_enrollment = Q("""
SELECT
    _id AS enrollment_internal_id,
    enrollment_state = 'SUBMITTED' AS in_submitted_state
FROM pki_enrollment
WHERE
    organization = $organization_internal_id
    AND enrollment_id = $enrollment_id
LIMIT 1
FOR UPDATE
""")


_q_update_enrollment = Q("""
UPDATE pki_enrollment
SET
    enrollment_state = 'REJECTED',
    info_rejected.rejected_on = $now
WHERE
    _id = $enrollment_internal_id
""")


async def pki_reject(
    conn: AsyncpgConnection,
    now: DateTime,
    author: DeviceID,
    organization_id: OrganizationID,
    enrollment_id: PKIEnrollmentID,
) -> None | PkiEnrollmentRejectBadOutcome:
    # We lock the common topic to ensure the user doesn't change its role
    # after we checked it.
    # Note the lock is not strictly mandatory since here we only modify data
    # that are not signed and don't require strong consistency with the
    # certificates (unlike e.g. blocks where the client actively checks that
    # no block has been created by a user after it has been revoked).
    # However since there is no hard performance requirements here we choose
    # to stay consistent with what do the `accept` part.
    match await auth_and_lock_common_read(conn, organization_id, author):
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return PkiEnrollmentRejectBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return PkiEnrollmentRejectBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return PkiEnrollmentRejectBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return PkiEnrollmentRejectBadOutcome.AUTHOR_REVOKED
        case auth_data:
            organization_internal_id = auth_data.organization_internal_id

    if auth_data.user_current_profile != UserProfile.ADMIN:
        return PkiEnrollmentRejectBadOutcome.AUTHOR_NOT_ALLOWED

    row = await conn.fetchrow(
        *_q_get_enrollment(
            organization_internal_id=organization_internal_id,
            enrollment_id=enrollment_id,
        )
    )

    if row is None:
        return PkiEnrollmentRejectBadOutcome.ENROLLMENT_NOT_FOUND

    match row["enrollment_internal_id"]:
        case int() as enrollment_internal_id:
            pass
        case _:
            assert False, row

    match row["in_submitted_state"]:
        case True:
            pass
        case False:
            return PkiEnrollmentRejectBadOutcome.ENROLLMENT_NO_LONGER_AVAILABLE
        case _:
            assert False, row

    await conn.execute(
        *_q_update_enrollment(
            enrollment_internal_id=enrollment_internal_id,
            now=now,
        )
    )

    await send_signal(
        conn,
        EventPkiEnrollment(
            organization_id=organization_id,
        ),
    )
