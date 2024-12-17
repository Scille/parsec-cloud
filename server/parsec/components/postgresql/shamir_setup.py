# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    DateTime,
    DeviceID,
    InvitationToken,
    OrganizationID,
    ShamirRecoveryBriefCertificate,
    UserID,
    VerifyKey,
)
from parsec.ballpark import (
    RequireGreaterTimestamp,
    TimestampOutOfBallpark,
    timestamps_in_the_ballpark,
)
from parsec.components.postgresql import AsyncpgConnection
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

_q_lock_common_and_shamir_topic = Q(
    """
WITH my_organization AS (
    SELECT
        _id,
        is_expired
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
),


-- Common topic lock must occur ASAP
my_locked_common_topic AS (
    SELECT last_timestamp
    FROM common_topic
    WHERE organization = (SELECT _id FROM my_organization)
    LIMIT 1
    FOR SHARE
),

-- Common shamir recovery lock must occur ASAP, after common topic lock
my_locked_shamir_recovery_topic AS (
    SELECT last_timestamp
    FROM shamir_recovery_topic
    WHERE organization = (SELECT _id FROM my_organization)
    LIMIT 1
    FOR SHARE
),

my_device AS (
    SELECT
        device._id,
        device.user_
    FROM device
    INNER JOIN my_organization ON device.organization = my_organization._id
    WHERE device.device_id = $device_id
    LIMIT 1
),

my_user AS (
    SELECT
        user_._id,
        user_.user_id,
        (user_.revoked_on IS NOT NULL) AS revoked
    FROM user_
    INNER JOIN my_device ON user_._id = my_device.user_
    LIMIT 1
),

my_last_shamir_recovery AS (
    SELECT
        shamir_recovery_setup._id,
        shamir_recovery_setup.created_on,
        shamir_recovery_setup.deleted_on
    FROM shamir_recovery_setup
    INNER JOIN my_user ON shamir_recovery_setup.user_ = my_user._id
    INNER JOIN my_organization ON shamir_recovery_setup.organization = my_organization._id
    ORDER BY shamir_recovery_setup.created_on DESC
    LIMIT 1
)

SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT is_expired FROM my_organization) AS organization_is_expired,
    (SELECT _id FROM my_device) AS device_internal_id,
    (SELECT revoked FROM my_user) AS user_is_revoked,
    (SELECT user_id FROM my_user) AS author_user_id,
    (SELECT _id FROM my_user) AS author_user_internal_id,
    (SELECT last_timestamp FROM my_locked_common_topic) AS last_common_certificate_timestamp,
    (SELECT _id FROM my_last_shamir_recovery) AS last_shamir_recovery_setup_internal_id,
    (SELECT created_on FROM my_last_shamir_recovery) AS last_shamir_recovery_setup_created_on,
    (SELECT deleted_on FROM my_last_shamir_recovery) AS last_shamir_recovery_setup_deleted_on,
    (SELECT last_timestamp FROM my_locked_shamir_recovery_topic) AS last_shamir_recovery_certificate_timestamp
"""
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


async def setup(
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
            return ShamirSetupStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["organization_is_expired"]:
        case False:
            pass
        case True:
            return ShamirSetupStoreBadOutcome.ORGANIZATION_EXPIRED
        case unknown:
            assert False, repr(unknown)

    # 1.2) Check device & user

    match row["author_user_internal_id"]:
        case int() as author_user_internal_id:
            pass
        case None:
            return ShamirSetupStoreBadOutcome.AUTHOR_NOT_FOUND
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
            return ShamirSetupStoreBadOutcome.AUTHOR_REVOKED
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
    recipient_internal_ids = []
    for recipient_id in cooked_shares:
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
                recipient_internal_ids.append(recipient_internal_id)
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
        for recipient_internal_id, (recipient_id, (share_certificate, _)) in zip(
            recipient_internal_ids, cooked_shares.items()
        ):
            yield _q_insert_shamir_recovery_share.arg_only(
                organization_internal_id=organization_internal_id,
                shamir_recovery_setup_internal_id=shamir_recovery_setup_internal_id,
                recipient_internal_id=recipient_internal_id,
                share_certificate=share_certificate,
                shares=cooked_brief.per_recipient_shares[recipient_id],
            )

    await conn.executemany(
        _q_insert_shamir_recovery_share.sql,
        arg_gen(),
    )

    return cooked_brief
