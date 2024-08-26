# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    OrganizationID,
    UserCertificate,
    UserProfile,
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
from parsec.components.postgresql.utils import Q, q_human_internal_id
from parsec.components.user import (
    UserCreateUserStoreBadOutcome,
    UserCreateUserValidateBadOutcome,
    user_create_user_validate,
)
from parsec.events import (
    EventCommonCertificate,
)

_q_insert_user_and_device = Q(f"""
WITH new_human AS (
    INSERT INTO human (organization, email, label)
    VALUES (
        $organization_internal_id,
        $email,
        $label
    )
    ON CONFLICT DO NOTHING
    RETURNING _id
),

my_human AS (
    SELECT
        COALESCE(
            (SELECT _id FROM new_human),
            ({ q_human_internal_id(organization="$organization_internal_id", email="$email") })
        ) AS _id
),

my_organization AS (
    SELECT active_users_limit FROM organization WHERE _id = $organization_internal_id LIMIT 1
),

my_checks AS (
    SELECT
        COALESCE(
            (
                SELECT TRUE
                FROM user_
                WHERE
                    human = (SELECT _id FROM my_human)
                    AND revoked_on IS NULL
                LIMIT 1
            ),
            FALSE
        ) AS human_already_taken,

        (
            CASE WHEN (SELECT active_users_limit FROM my_organization) IS NULL
            THEN FALSE
            ELSE
                (
                    SELECT active_users_limit FROM my_organization
                ) <= (
                    SELECT COUNT(*)
                    FROM user_
                    WHERE
                        organization = $organization_internal_id
                        AND revoked_on IS NULL
                )
            END
        ) AS active_users_limit_reached
),

new_user AS (
    INSERT INTO user_ (
        organization,
        user_id,
        initial_profile,
        current_profile,
        user_certificate,
        redacted_user_certificate,
        user_certifier,
        created_on,
        human
    )
    VALUES (
        $organization_internal_id,
        $user_id,
        $initial_profile,
        $initial_profile,
        $user_certificate,
        $redacted_user_certificate,
        $author_internal_id,
        $created_on,
        (SELECT _id FROM my_human)
    )
    ON CONFLICT DO NOTHING
    RETURNING _id
),

new_device AS (
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
    (
        SELECT
            $organization_internal_id,
            new_user._id,
            $device_id,
            $device_label,
            $verify_key,
            $device_certificate,
            $redacted_device_certificate,
            $author_internal_id,
            $created_on
        FROM new_user
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
    (SELECT human_already_taken FROM my_checks) AS human_already_taken,
    (SELECT active_users_limit_reached FROM my_checks) AS active_users_limit_reached,
    COALESCE((SELECT TRUE FROM new_user), FALSE) AS insert_new_user_ok,
    COALESCE((SELECT * FROM new_device), FALSE) AS insert_new_device_ok,
    COALESCE((SELECT * FROM updated_common_topic), FALSE) AS update_common_topic_ok
""")


async def user_create_user(
    event_bus: EventBus,
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    user_certificate: bytes,
    redacted_user_certificate: bytes,
    device_certificate: bytes,
    redacted_device_certificate: bytes,
) -> (
    tuple[UserCertificate, DeviceCertificate]
    | UserCreateUserValidateBadOutcome
    | UserCreateUserStoreBadOutcome
    | TimestampOutOfBallpark
    | RequireGreaterTimestamp
):
    # 1) Write lock common topic

    match await auth_and_lock_common_write(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as db_common:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return UserCreateUserStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return UserCreateUserStoreBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return UserCreateUserStoreBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return UserCreateUserStoreBadOutcome.AUTHOR_REVOKED

    if db_common.user_current_profile != UserProfile.ADMIN:
        return UserCreateUserStoreBadOutcome.AUTHOR_NOT_ALLOWED

    # 2) Validate certificates

    match user_create_user_validate(
        now=now,
        expected_author=author,
        author_verify_key=author_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
    ):
        case (user_certif, device_certif):
            pass
        case error:
            return error

    # 3) Ensure we are not breaking causality by adding a newer timestamp.

    if user_certif.timestamp <= db_common.last_common_certificate_timestamp:
        return RequireGreaterTimestamp(
            strictly_greater_than=db_common.last_common_certificate_timestamp
        )

    # 4) (Almost) all checks are good, now we do the actual insertion
    # Almost because the cases user_id/device_id already exists and human_handle
    # already taken are handled in the insert query

    row = await conn.fetchrow(
        *_q_insert_user_and_device(
            organization_internal_id=db_common.organization_internal_id,
            author_internal_id=db_common.device_internal_id,
            created_on=user_certif.timestamp,
            email=user_certif.human_handle.email,
            label=user_certif.human_handle.label,
            user_id=user_certif.user_id,
            initial_profile=user_certif.profile.str,
            user_certificate=user_certificate,
            redacted_user_certificate=redacted_user_certificate,
            device_id=device_certif.device_id,
            device_label=device_certif.device_label.str,
            verify_key=device_certif.verify_key.encode(),
            device_certificate=device_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
    )
    assert row is not None

    match row["human_already_taken"]:
        case False:
            pass
        case True:
            return UserCreateUserStoreBadOutcome.HUMAN_HANDLE_ALREADY_TAKEN
        case unknown:
            assert False, unknown

    match row["active_users_limit_reached"]:
        case False:
            pass
        case True:
            return UserCreateUserStoreBadOutcome.ACTIVE_USERS_LIMIT_REACHED
        case unknown:
            assert False, unknown

    match row["insert_new_user_ok"]:
        case True:
            pass
        case False:
            return UserCreateUserStoreBadOutcome.USER_ALREADY_EXISTS
        case unknown:
            assert False, unknown

    match row["insert_new_device_ok"]:
        case True:
            pass
        case False:
            return UserCreateUserStoreBadOutcome.USER_ALREADY_EXISTS
        case unknown:
            assert False, unknown

    match row["update_common_topic_ok"]:
        case True:
            pass
        case unknown:
            assert False, unknown

    await event_bus.send(
        EventCommonCertificate(organization_id=organization_id, timestamp=user_certif.timestamp)
    )

    return user_certif, device_certif
