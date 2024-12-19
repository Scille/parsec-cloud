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
from parsec.components.postgresql.shamir_setup import _q_lock_common_and_shamir_topic
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


async def delete(
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

    row = await conn.fetchrow(
        *_q_lock_common_and_shamir_topic(
            organization_id=organization_id.str,
            device_id=author,
        )
    )
    assert row is not None

    # 1.1) Check organization

    match row["organization_internal_id"]:
        case int() as organization_internal_id:
            pass
        case None:
            return ShamirDeleteStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["organization_is_expired"]:
        case False:
            pass
        case True:
            return ShamirDeleteStoreBadOutcome.ORGANIZATION_EXPIRED
        case unknown:
            assert False, repr(unknown)

    # 1.2) Check device & user

    match row["author_user_internal_id"]:
        case int() as author_user_internal_id:
            pass
        case None:
            return ShamirDeleteStoreBadOutcome.AUTHOR_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["author_user_id"]:
        case str() as raw_author_user_id:
            author_user_id = UserID.from_hex(raw_author_user_id)
        case None:
            assert False, "Device exists but user does not"
        case unknown:
            assert False, repr(unknown)

    match row["user_is_revoked"]:
        case False:
            pass
        case True:
            return ShamirDeleteStoreBadOutcome.AUTHOR_REVOKED
        case unknown:
            assert False, repr(unknown)

    # 1.3) Check common and shamir topics

    match row["last_common_certificate_timestamp"]:
        case DateTime() as last_common_certificate_timestamp:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["last_shamir_recovery_certificate_timestamp"]:
        case DateTime() | None as last_shamir_recovery_certificate_timestamp:
            pass
        case unknown:
            assert False, repr(unknown)

    # 1.4) Check previous shamir setup

    match row["last_shamir_recovery_setup_internal_id"]:
        case int() | None as shamir_recovery_setup_internal_id:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["last_shamir_recovery_setup_created_on"]:
        case DateTime() | None as last_shamir_recovery_setup_created_on:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["last_shamir_recovery_setup_deleted_on"]:
        case DateTime() | None as last_shamir_recovery_setup_deleted_on:
            pass
        case unknown:
            assert False, repr(unknown)

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
    assert shamir_recovery_setup_internal_id is not None

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

    row = await conn.fetchrow(
        *_q_mark_shamir_recovery_setup_as_deleted(
            organization_internal_id=organization_internal_id,
            shamir_recovery_setup_internal_id=shamir_recovery_setup_internal_id,
            deleted_on=cooked_deletion.timestamp,
            deletion_certificate=shamir_recovery_deletion_certificate,
        )
    )

    # Check that the update is successful
    assert row is not None

    return cooked_deletion
