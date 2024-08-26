# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RevokedUserCertificate,
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
from parsec.components.postgresql.utils import Q
from parsec.components.realm import CertificateBasedActionIdempotentOutcome
from parsec.components.user import (
    UserRevokeUserStoreBadOutcome,
    UserRevokeUserValidateBadOutcome,
    user_revoke_user_validate,
)
from parsec.events import (
    EventCommonCertificate,
    EventUserRevokedOrFrozen,
)

_q_get_info_for_checks_and_read_lock_realms = Q("""
WITH my_user AS (
    SELECT
        _id,
        (revoked_on IS NOT NULL) AS recipient_is_revoked
    FROM user_
    WHERE
        organization = $organization_internal_id
        AND user_id = $recipient_user_id
    LIMIT 1
),

-- Retrieve the last role for each realm the user is or used to be part of...
my_realms_last_roles AS (
    SELECT DISTINCT ON (realm)
        realm AS _id,
        role
    FROM realm_user_role
    WHERE user_ = (SELECT my_user._id FROM my_user)
    ORDER BY realm ASC, certified_on DESC
),

-- ...and only keep the realm the user is still part of
my_realms AS (
    SELECT _id
    FROM my_realms_last_roles
    WHERE role IS NOT NULL
),

-- Realms topic lock must occur ASAP
my_locked_realms_topics AS (
    SELECT last_timestamp
    FROM realm_topic
    WHERE realm IN (SELECT * FROM my_realms)
    -- Read lock
    FOR SHARE
),

-- We do a full scan on the `vlob_atom` table here :(
--
-- This is a very wasteful way of finding the last timestamp of all vlobs in the
-- realms the user is part of.
--
-- However this is *good enough* since user revocation should be rather rare.
-- If performances star being a trouble, we could optimize this by leveraging the
-- timestamp ballpark requirements: since a vlob create/update is only allowed
-- if its timestamp is within a ballpark, then we can deduce from the last inserted
-- vlob atom the further early and late timestamp than can concurrently occur.
my_vlobs_last_timestamp AS (
    SELECT
        MAX(created_on) AS last_timestamp
    FROM vlob_atom
    WHERE realm IN (SELECT * FROM my_realms)
)

SELECT
    (SELECT _id FROM my_user) AS recipient_internal_id,
    (SELECT recipient_is_revoked FROM my_user),
    (SELECT MAX(last_timestamp) FROM my_locked_realms_topics) AS realms_topics_last_timestamp,
    (SELECT last_timestamp FROM my_vlobs_last_timestamp) AS vlobs_last_timestamp
""")


_q_revoke_user = Q("""
WITH updated_user AS (
    UPDATE user_ SET
        revoked_user_certificate = $revoked_user_certificate,
        revoked_user_certifier = $author_internal_id,
        revoked_on = $revoked_on
    WHERE
        _id = $recipient_internal_id
        -- Sanity checks
        AND revoked_user_certificate IS NULL
        AND revoked_user_certifier IS NULL
        AND revoked_on IS NULL
    RETURNING TRUE
),

updated_common_topic AS (
    UPDATE common_topic
    SET last_timestamp = $revoked_on
    WHERE
        organization = $organization_internal_id
        -- Sanity check
        AND last_timestamp < $revoked_on
    RETURNING TRUE
)

SELECT
    COALESCE((SELECT * FROM updated_user), FALSE) AS update_user_ok,
    COALESCE((SELECT * FROM updated_common_topic), FALSE) AS update_common_topic_ok
""")


async def user_revoke_user(
    event_bus: EventBus,
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    revoked_user_certificate: bytes,
) -> (
    RevokedUserCertificate
    | CertificateBasedActionIdempotentOutcome
    | UserRevokeUserValidateBadOutcome
    | UserRevokeUserStoreBadOutcome
    | TimestampOutOfBallpark
    | RequireGreaterTimestamp
):
    # 1) Write lock common topic

    match await auth_and_lock_common_write(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as db_common:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return UserRevokeUserStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return UserRevokeUserStoreBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return UserRevokeUserStoreBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return UserRevokeUserStoreBadOutcome.AUTHOR_REVOKED

    if db_common.user_current_profile != UserProfile.ADMIN:
        return UserRevokeUserStoreBadOutcome.AUTHOR_NOT_ALLOWED

    # 2) Validate certificate

    match user_revoke_user_validate(
        now=now,
        expected_author_user_id=db_common.user_id,
        expected_author_device_id=author,
        author_verify_key=author_verify_key,
        revoked_user_certificate=revoked_user_certificate,
    ):
        case RevokedUserCertificate() as certif:
            pass
        case error:
            return error

    # 3) Fetch from database what is needed and do the checks

    row = await conn.fetchrow(
        *_q_get_info_for_checks_and_read_lock_realms(
            organization_internal_id=db_common.organization_internal_id,
            recipient_user_id=certif.user_id,
        )
    )
    assert row is not None

    # 3.1) Check the recipient exists & revoked & current profile

    match row["recipient_internal_id"]:
        case int() as recipient_internal_id:
            pass
        case None:
            return UserRevokeUserStoreBadOutcome.USER_NOT_FOUND
        case unknown:
            assert False, unknown

    match row["recipient_is_revoked"]:
        case False:
            pass
        case True:
            return CertificateBasedActionIdempotentOutcome(
                certificate_timestamp=db_common.last_common_certificate_timestamp
            )
        case unknown:
            assert False, unknown

    # 3.2) Ensure we are not breaking causality by adding a newer timestamp.

    # Given a revoked user is not allowed to modify a realm, we must check timestamp on:
    # - The common topic
    # - For each realm the user is part of: the realm topic
    # - For each realm the user is part of: the last vlob

    last_timestamp = db_common.last_common_certificate_timestamp

    match row["realms_topics_last_timestamp"]:
        case DateTime() as realms_topics_last_timestamp:
            last_timestamp = max(realms_topics_last_timestamp, last_timestamp)
        # Can be `None` if the user to revoke is not part of any realm
        case None:
            pass
        case unknown:
            assert False, unknown

    match row["vlobs_last_timestamp"]:
        case DateTime() as vlobs_last_timestamp:
            last_timestamp = max(vlobs_last_timestamp, last_timestamp)
        # Can be `None` if the user to revoke is part realms which are all empty
        case None:
            pass
            pass
        case unknown:
            assert False, unknown

    if certif.timestamp <= last_timestamp:
        return RequireGreaterTimestamp(strictly_greater_than=last_timestamp)

    # 4) All checks are good, now we do the actual insertion

    row = await conn.fetchrow(
        *_q_revoke_user(
            organization_internal_id=db_common.organization_internal_id,
            revoked_user_certificate=revoked_user_certificate,
            author_internal_id=db_common.device_internal_id,
            revoked_on=certif.timestamp,
            recipient_internal_id=recipient_internal_id,
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
        EventUserRevokedOrFrozen(
            organization_id=organization_id,
            user_id=certif.user_id,
        )
    )

    return certif
