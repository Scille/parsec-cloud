# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    UserProfile,
    UserUpdateCertificate,
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
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.components.user import (
    UserUpdateUserStoreBadOutcome,
    UserUpdateUserValidateBadOutcome,
    user_update_user_validate,
)
from parsec.events import (
    EventCommonCertificate,
    EventUserUpdated,
)

_q_get_info_for_checks = Q("""
SELECT
    _id AS recipient_internal_id,
    current_profile AS recipient_current_profile,
    (revoked_on IS NOT NULL) AS recipient_is_revoked
FROM user_
WHERE
    organization = $organization_internal_id
    AND user_id = $recipient_user_id
LIMIT 1
""")


_q_update_user = Q(
    """
WITH new_profile AS (
    INSERT INTO profile (
        user_,
        profile,
        profile_certificate,
        certified_by,
        certified_on
    )
    VALUES (
        $user_internal_id,
        $profile,
        $profile_certificate,
        $certified_by_internal_id,
        $certified_on
    )
),

updated_user AS (
    UPDATE user_
    SET current_profile = $profile
    WHERE
        _id = $user_internal_id
    RETURNING TRUE
),

updated_common_topic AS (
    UPDATE common_topic
    SET last_timestamp = $certified_on
    WHERE
        organization = $organization_internal_id
        -- Sanity check
        AND last_timestamp < $certified_on
    RETURNING TRUE
)

SELECT
    COALESCE((SELECT * FROM updated_common_topic), FALSE) AS update_user_ok,
    COALESCE((SELECT * FROM updated_common_topic), FALSE) AS update_common_topic_ok
"""
)


async def user_update_user(
    event_bus: EventBus,
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    user_update_certificate: bytes,
) -> (
    UserUpdateCertificate
    | UserUpdateUserValidateBadOutcome
    | UserUpdateUserStoreBadOutcome
    | TimestampOutOfBallpark
    | RequireGreaterTimestamp
):
    # 1) Write lock common topic

    match await auth_and_lock_common_write(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as db_common:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return UserUpdateUserStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return UserUpdateUserStoreBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return UserUpdateUserStoreBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return UserUpdateUserStoreBadOutcome.AUTHOR_REVOKED

    if db_common.user_current_profile != UserProfile.ADMIN:
        return UserUpdateUserStoreBadOutcome.AUTHOR_NOT_ALLOWED

    # 2) Validate certificate

    match user_update_user_validate(
        now=now,
        expected_author_user_id=db_common.user_id,
        expected_author_device_id=author,
        author_verify_key=author_verify_key,
        user_update_certificate=user_update_certificate,
    ):
        case UserUpdateCertificate() as certif:
            pass
        case error:
            return error

    # 3) Fetch from database what is needed and do the checks

    row = await conn.fetchrow(
        *_q_get_info_for_checks(
            organization_internal_id=db_common.organization_internal_id,
            recipient_user_id=certif.user_id,
        )
    )
    if row is None:
        return UserUpdateUserStoreBadOutcome.USER_NOT_FOUND

    # 3.1) Check the recipient exists & revoked & current profile

    match row["recipient_internal_id"]:
        case int() as recipient_internal_id:
            pass
        case unknown:
            assert False, unknown

    match row["recipient_is_revoked"]:
        case False:
            pass
        case True:
            return UserUpdateUserStoreBadOutcome.USER_REVOKED
        case unknown:
            assert False, unknown

    match row["recipient_current_profile"]:
        case str() as raw_recipient_current_profile:
            recipient_current_profile = UserProfile.from_str(raw_recipient_current_profile)
            if certif.new_profile == recipient_current_profile:
                return UserUpdateUserStoreBadOutcome.USER_NO_CHANGES
        case unknown:
            assert False, unknown

    # 3.2) Ensure we are not breaking causality by adding a newer timestamp.

    # Strictly speaking consistency only requires to ensure the profile change didn't
    # remove rights that have been used to add certificates/vlobs with posterior timestamp
    # (e.g. switching from OWNER to READER while a vlob has been created).
    #
    # However doing such precise checks is complex and error prone, so we take a simpler
    # approach by considering certificates don't change often so it's no big deal to
    # have a much more coarse approach.

    if certif.timestamp <= db_common.last_common_certificate_timestamp:
        return RequireGreaterTimestamp(
            strictly_greater_than=db_common.last_common_certificate_timestamp
        )

    # 4) All checks are good, now we do the actual insertion

    # TODO: validate it's okay not to check this
    # Note an OUTSIDER is not supposed to be OWNER/MANAGER of a shared realm. However this
    # is possible if the user's profile is updated to OUTSIDER here.
    # We don't try to prevent this given:
    # - It is complex and error prone to check.
    # - It is a very niche case.
    # - It is puzzling for the end user to understand why he cannot change a profile,
    #   and that he have to find somebody with access to a seemingly unrelated realm
    #   to change a role in order to be able to do it !

    row = await conn.fetchrow(
        *_q_update_user(
            organization_internal_id=db_common.organization_internal_id,
            user_internal_id=recipient_internal_id,
            profile=certif.new_profile.str,
            profile_certificate=user_update_certificate,
            certified_by_internal_id=db_common.device_internal_id,
            certified_on=certif.timestamp,
        )
    )
    assert row is not None

    match row["update_user_ok"]:
        case True:
            pass
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
    await event_bus.send(
        EventUserUpdated(
            organization_id=organization_id,
            user_id=certif.user_id,
            new_profile=certif.new_profile,
        )
    )

    return certif
