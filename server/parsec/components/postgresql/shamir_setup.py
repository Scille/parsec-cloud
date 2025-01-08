# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    DateTime,
    DeviceID,
    InvitationToken,
    OrganizationID,
    ShamirRecoveryBriefCertificate,
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
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.components.shamir import (
    ShamirSetupAlreadyExistsBadOutcome,
    ShamirSetupRevokedRecipientBadOutcome,
    ShamirSetupStoreBadOutcome,
    ShamirSetupValidateBadOutcome,
    shamir_setup_validate,
)

_q_check_recipient = Q(
    """
SELECT
    _id AS recipient_internal_id,
    (revoked_on IS NOT NULL) AS revoked
FROM user_
WHERE organization = $organization_internal_id
AND user_id = $recipient_id
LIMIT 1
"""
)

_q_insert_shamir_recovery_setup = Q(
    """
WITH shamir_recovery_topic_update AS (
    INSERT INTO shamir_recovery_topic (
        organization,
        last_timestamp
    ) VALUES (
        $organization_internal_id,
        $created_on
    )
    ON CONFLICT (organization)
    DO UPDATE SET
        last_timestamp = EXCLUDED.last_timestamp
    WHERE
        -- Sanity check
        shamir_recovery_topic.last_timestamp < EXCLUDED.last_timestamp
    RETURNING TRUE
)

INSERT INTO shamir_recovery_setup (
    organization,
    user_,
    created_on,
    brief_certificate,
    reveal_token,
    threshold,
    shares,
    ciphered_data
) VALUES (
    $organization_internal_id,
    $author_user_internal_id,
    $created_on,
    $brief_certificate,
    $reveal_token,
    $threshold,
    $shares,
    $ciphered_data
)
RETURNING _id AS shamir_recovery_setup_internal_id
"""
)


_q_insert_shamir_recovery_share = Q(
    """
INSERT INTO shamir_recovery_share (
    organization,
    shamir_recovery,
    recipient,
    share_certificate,
    shares
) VALUES (
    $organization_internal_id,
    $shamir_recovery_setup_internal_id,
    $recipient_internal_id,
    $share_certificate,
    $shares
)
"""
)


async def shamir_setup(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    ciphered_data: bytes,
    reveal_token: InvitationToken,
    shamir_recovery_brief_certificate: bytes,
    shamir_recovery_share_certificates: list[bytes],
) -> (
    ShamirRecoveryBriefCertificate
    | ShamirSetupStoreBadOutcome
    | ShamirSetupValidateBadOutcome
    | ShamirSetupAlreadyExistsBadOutcome
    | ShamirSetupRevokedRecipientBadOutcome
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
            return ShamirSetupStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return ShamirSetupStoreBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return ShamirSetupStoreBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return ShamirSetupStoreBadOutcome.AUTHOR_REVOKED

    match await lock_shamir_write(conn, organization_internal_id, author_user_internal_id):
        case LockShamirData(
            last_shamir_recovery_certificate_timestamp=last_shamir_recovery_certificate_timestamp,
            last_shamir_recovery_setup_created_on=last_shamir_recovery_setup_created_on,
            last_shamir_recovery_setup_deleted_on=last_shamir_recovery_setup_deleted_on,
        ):
            pass

    if (
        last_shamir_recovery_setup_created_on is not None
        and last_shamir_recovery_setup_deleted_on is None
    ):
        assert last_shamir_recovery_certificate_timestamp is not None
        return ShamirSetupAlreadyExistsBadOutcome(
            last_shamir_recovery_certificate_timestamp=last_shamir_recovery_certificate_timestamp
        )

    # 2) Validate the shamir certificates

    match shamir_setup_validate(
        now,
        author,
        author_user_id,
        author_verify_key,
        shamir_recovery_brief_certificate,
        shamir_recovery_share_certificates,
    ):
        case (cooked_brief, cooked_shares):
            pass
        case error:
            return error

    # 3) Check the recipients

    share_args = []
    for recipient_id, (share_certificate, _) in cooked_shares.items():
        row = await conn.fetchrow(
            *_q_check_recipient(
                organization_internal_id=organization_internal_id,
                recipient_id=recipient_id,
            )
        )
        if row is None:
            return ShamirSetupStoreBadOutcome.RECIPIENT_NOT_FOUND

        match row["revoked"]:
            case True:
                return ShamirSetupRevokedRecipientBadOutcome(
                    last_common_certificate_timestamp=last_common_certificate_timestamp
                )
            case False:
                pass
            case unknown:
                assert False, repr(unknown)

        match row["recipient_internal_id"]:
            case int() as recipient_internal_id:
                number_of_shares = cooked_brief.per_recipient_shares[recipient_id]
                share_args.append((recipient_internal_id, share_certificate, number_of_shares))
            case unknown:
                assert False, repr(unknown)

    # 4) Ensure we are not breaking causality by adding a newer timestamp

    timestamp = cooked_brief.timestamp
    if (err := timestamps_in_the_ballpark(timestamp, now)) is not None:
        return err

    last_certificate_timestamp = last_common_certificate_timestamp
    if last_shamir_recovery_certificate_timestamp is not None:
        last_certificate_timestamp = max(
            last_certificate_timestamp, last_shamir_recovery_certificate_timestamp
        )
    if last_certificate_timestamp >= cooked_brief.timestamp:
        return RequireGreaterTimestamp(strictly_greater_than=last_certificate_timestamp)

    # 5) All checks are good, now we do the actual insertion

    row = await conn.fetchrow(
        *_q_insert_shamir_recovery_setup(
            organization_internal_id=organization_internal_id,
            author_user_internal_id=author_user_internal_id,
            brief_certificate=shamir_recovery_brief_certificate,
            reveal_token=reveal_token.hex,
            threshold=cooked_brief.threshold,
            shares=sum(cooked_brief.per_recipient_shares.values()),
            ciphered_data=ciphered_data,
            created_on=timestamp,
        )
    )
    assert row is not None

    match row["shamir_recovery_setup_internal_id"]:
        case int() as shamir_recovery_setup_internal_id:
            pass
        case unknown:
            assert False, repr(unknown)

    def arg_gen():
        for recipient_internal_id, share_certificate, number_of_shares in share_args:
            yield _q_insert_shamir_recovery_share.arg_only(
                organization_internal_id=organization_internal_id,
                shamir_recovery_setup_internal_id=shamir_recovery_setup_internal_id,
                recipient_internal_id=recipient_internal_id,
                share_certificate=share_certificate,
                shares=number_of_shares,
            )

    await conn.executemany(
        _q_insert_shamir_recovery_share.sql,
        arg_gen(),
    )

    return cooked_brief
