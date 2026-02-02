# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    AsyncEnrollmentID,
    DateTime,
    OrganizationID,
)
from parsec.components.async_enrollment import (
    AsyncEnrollmentCancelBadOutcome,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.async_enrollment_submit import (
    _q_take_async_enrollment_create_write_lock,
)
from parsec.components.postgresql.handler import send_signal
from parsec.components.postgresql.utils import Q
from parsec.events import EventAsyncEnrollment

_q_check_organization = Q("""
SELECT
    _id,
    is_expired
FROM organization
WHERE
    organization_id = $organization_id
    AND root_verify_key IS NOT NULL
LIMIT 1
""")

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


_q_cancel_enrollment = Q("""
UPDATE async_enrollment
SET
    state = 'CANCELLED',
    cancelled_on = $now
WHERE
    _id = $enrollment_internal_id
""")


async def async_enrollment_cancel(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    enrollment_id: AsyncEnrollmentID,
) -> None | AsyncEnrollmentCancelBadOutcome:
    # 1) Check organization exists and is not expired

    org_row = await conn.fetchrow(
        *_q_check_organization(
            organization_id=organization_id.str,
        )
    )
    assert org_row is not None

    match org_row["_id"]:
        case int() as organization_internal_id:
            pass
        case None:
            return AsyncEnrollmentCancelBadOutcome.ORGANIZATION_NOT_FOUND
        case _:
            assert False, org_row

    match org_row["is_expired"]:
        case False:
            pass
        case True:
            return AsyncEnrollmentCancelBadOutcome.ORGANIZATION_EXPIRED
        case _:
            assert False, org_row

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
        return AsyncEnrollmentCancelBadOutcome.ENROLLMENT_NOT_FOUND

    match enrollment_row["enrollment_internal_id"]:
        case int() as enrollment_internal_id:
            pass
        case _:
            assert False, enrollment_row

    match enrollment_row["state"]:
        case "SUBMITTED":
            pass
        case _:
            return AsyncEnrollmentCancelBadOutcome.ENROLLMENT_NO_LONGER_AVAILABLE

    # 4) Cancel the enrollment

    await conn.execute(
        *_q_cancel_enrollment(
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
