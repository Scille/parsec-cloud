# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    ShamirRecoveryDeletionCertificate,
    UserID,
    VerifyKey,
)
from parsec.ballpark import (
    RequireGreaterTimestamp,
    TimestampOutOfBallpark,
    timestamps_in_the_ballpark,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.queries import (
    AuthAndLockCommonOnlyBadOutcome,
    AuthAndLockCommonOnlyData,
    LockShamirData,
    auth_and_lock_common_read,
    lock_shamir_write,
)
from parsec.components.postgresql.utils import Q
from parsec.components.shamir import (
    ShamirDeleteSetupAlreadyDeletedBadOutcome,
    ShamirDeleteStoreBadOutcome,
    ShamirDeleteValidateBadOutcome,
    shamir_delete_validate,
)

_q_check_recipients = Q(
    """
SELECT user_.user_id
FROM shamir_recovery_setup
INNER JOIN shamir_recovery_share ON shamir_recovery_share.shamir_recovery = shamir_recovery_setup._id
INNER JOIN user_ ON user_._id = shamir_recovery_share.recipient
WHERE shamir_recovery_setup.organization = $organization_internal_id
AND shamir_recovery_setup.user_ = $author_user_internal_id
AND shamir_recovery_setup.created_on = $timestamp
"""
)

_q_mark_shamir_recovery_setup_as_deleted = Q(
    """
WITH shamir_recovery_topic_update AS (
    INSERT INTO shamir_recovery_topic (
        organization,
        last_timestamp
    ) VALUES (
        $organization_internal_id,
        $deleted_on
    )
    ON CONFLICT (organization)
    DO UPDATE SET
        last_timestamp = EXCLUDED.last_timestamp
    WHERE
        -- Sanity check
        shamir_recovery_topic.last_timestamp < EXCLUDED.last_timestamp
    RETURNING TRUE
)

UPDATE shamir_recovery_setup SET
    deleted_on = $deleted_on,
    deletion_certificate = $deletion_certificate,
    -- Remove the ciphered data, the device keys should now be unrecoverable
    ciphered_data = NULL
WHERE organization = $organization_internal_id
AND _id = $shamir_recovery_setup_internal_id
-- Sanity check
AND deleted_on IS NULL
RETURNING TRUE
"""
)


async def shamir_delete(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    shamir_recovery_deletion_certificate: bytes,
) -> (
    ShamirRecoveryDeletionCertificate
    | ShamirDeleteStoreBadOutcome
    | ShamirDeleteValidateBadOutcome
    | ShamirDeleteSetupAlreadyDeletedBadOutcome
    | TimestampOutOfBallpark
    | RequireGreaterTimestamp
):
    # 1) Query the database to get all info about org/device/user
    #    and lock the common and shamir topics

    match await auth_and_lock_common_read(conn, organization_id, author):
        case AuthAndLockCommonOnlyData(
            organization_internal_id=organization_internal_id,
            user_id=author_user_id,
            user_internal_id=author_user_internal_id,
            last_common_certificate_timestamp=last_common_certificate_timestamp,
        ):
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return ShamirDeleteStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return ShamirDeleteStoreBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return ShamirDeleteStoreBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return ShamirDeleteStoreBadOutcome.AUTHOR_REVOKED

    match await lock_shamir_write(conn, organization_internal_id, author_user_internal_id):
        case LockShamirData(
            last_shamir_recovery_certificate_timestamp=last_shamir_recovery_certificate_timestamp,
            last_shamir_recovery_setup_internal_id=last_shamir_recovery_setup_internal_id,
            last_shamir_recovery_setup_created_on=last_shamir_recovery_setup_created_on,
            last_shamir_recovery_setup_deleted_on=last_shamir_recovery_setup_deleted_on,
        ):
            pass

    # 2) Validate the deletion certificate

    match shamir_delete_validate(
        now, author, author_user_id, author_verify_key, shamir_recovery_deletion_certificate
    ):
        case ShamirRecoveryDeletionCertificate() as cooked_deletion:
            pass
        case error:
            return error

    # 3) Check the recipients

    rows = await conn.fetch(
        *_q_check_recipients(
            organization_internal_id=organization_internal_id,
            author_user_internal_id=author_user_internal_id,
            timestamp=cooked_deletion.setup_to_delete_timestamp,
        )
    )
    expected_recipients = {UserID.from_hex(row["user_id"]) for row in rows}
    if not expected_recipients:
        return ShamirDeleteStoreBadOutcome.SETUP_NOT_FOUND

    if expected_recipients != cooked_deletion.share_recipients:
        return ShamirDeleteStoreBadOutcome.RECIPIENTS_MISMATCH

    if (
        last_shamir_recovery_setup_deleted_on is not None
        or last_shamir_recovery_setup_created_on != cooked_deletion.setup_to_delete_timestamp
    ):
        assert last_shamir_recovery_certificate_timestamp is not None
        return ShamirDeleteSetupAlreadyDeletedBadOutcome(
            last_shamir_recovery_certificate_timestamp=last_shamir_recovery_certificate_timestamp
        )

    # This is guaranteed by the previous checks
    assert last_shamir_recovery_setup_internal_id is not None

    # 4) Ensure we are not breaking causality by adding a newer timestamp

    if (err := timestamps_in_the_ballpark(cooked_deletion.timestamp, now)) is not None:
        return err

    last_certificate_timestamp = last_common_certificate_timestamp
    if last_shamir_recovery_certificate_timestamp is not None:
        last_certificate_timestamp = max(
            last_certificate_timestamp, last_shamir_recovery_certificate_timestamp
        )
    if last_certificate_timestamp >= cooked_deletion.timestamp:
        return RequireGreaterTimestamp(strictly_greater_than=last_certificate_timestamp)

    # 5) Mark the shamir recovery setup as deleted

    success = await conn.fetchval(
        *_q_mark_shamir_recovery_setup_as_deleted(
            organization_internal_id=organization_internal_id,
            shamir_recovery_setup_internal_id=last_shamir_recovery_setup_internal_id,
            deleted_on=cooked_deletion.timestamp,
            deletion_certificate=shamir_recovery_deletion_certificate,
        )
    )

    # Check that the update is successful
    assert success is True, success

    return cooked_deletion
