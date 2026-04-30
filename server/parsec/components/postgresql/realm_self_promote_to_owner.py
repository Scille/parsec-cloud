# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmRole,
    RealmRoleCertificate,
    UserProfile,
    VerifyKey,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.events import send_signal
from parsec.components.postgresql.queries import (
    AuthAndLockCommonOnlyBadOutcome,
    AuthAndLockCommonOnlyData,
    LockRealmWriteRealmBadOutcome,
    LockRealmWriteRealmData,
    auth_and_lock_common_read,
    lock_realm_write,
)
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.components.realm import (
    RealmSelfPromoteToOwnerActiveOwnerAlreadyExists,
    RealmSelfPromoteToOwnerStoreBadOutcome,
    RealmSelfPromoteToOwnerValidateBadOutcome,
    realm_self_promote_to_owner_validate,
)
from parsec.events import EventRealmCertificate

_q_get_info_for_checks = Q(
    """
WITH
realm_members_last_role AS (
    SELECT DISTINCT ON (user_)
        user_,
        role
    FROM realm_user_role
    WHERE realm = $realm_internal_id
    ORDER BY user_ ASC, certified_on DESC
),

active_realm_members AS (
    SELECT
        CASE realm_members_last_role.role
            WHEN 'READER' THEN 1
            WHEN 'CONTRIBUTOR' THEN 2
            WHEN 'MANAGER' THEN 3
            WHEN 'OWNER' THEN 4
        END AS priority
    FROM realm_members_last_role
    INNER JOIN user_ ON realm_members_last_role.user_ = user_._id
    WHERE
        realm_members_last_role.role IS NOT NULL
        AND user_.revoked_on IS NULL
),

last_vlob AS (
    SELECT MAX(created_on) AS last_vlob_timestamp
    FROM vlob_atom
    WHERE realm = $realm_internal_id
)

SELECT
    EXISTS(
        SELECT 1 FROM active_realm_members
        WHERE priority = 4
    ) AS has_active_owner,
    (SELECT MAX(priority) FROM active_realm_members) AS max_active_priority,
    (SELECT last_vlob_timestamp FROM last_vlob) AS last_vlob_timestamp
"""
)


_q_self_promote_to_owner = Q(
    """
WITH new_realm_user_role AS (
    INSERT INTO realm_user_role (
        realm,
        user_,
        role,
        certificate,
        certified_by,
        certified_on
    ) VALUES (
        $realm_internal_id,
        $author_internal_id,
        'OWNER',
        $certificate,
        $certified_by,
        $certified_on
    )
),

updated_realm_topic AS (
    UPDATE realm_topic
    SET last_timestamp = $certified_on
    WHERE
        realm = $realm_internal_id
        -- Sanity check
        AND last_timestamp < $certified_on
    RETURNING TRUE
)

SELECT COALESCE((SELECT * FROM updated_realm_topic), FALSE) AS update_realm_topic_ok
"""
)


def _role_to_priority(role: RealmRole) -> int:
    match role:
        case RealmRole.READER:
            return 1
        case RealmRole.CONTRIBUTOR:
            return 2
        case RealmRole.MANAGER:
            return 3
        case RealmRole.OWNER:
            return 4
        case unknown:
            # TODO: Implement `Enum` on `RealmRole` so we can use `assert_never` here
            # (see https://github.com/Scille/parsec-cloud/issues/12725)
            assert False, unknown


async def realm_self_promote_to_owner(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    realm_role_certificate: bytes,
) -> (
    RealmRoleCertificate
    | RealmSelfPromoteToOwnerActiveOwnerAlreadyExists
    | RealmSelfPromoteToOwnerValidateBadOutcome
    | TimestampOutOfBallpark
    | RealmSelfPromoteToOwnerStoreBadOutcome
    | RequireGreaterTimestamp
):
    # 1) Read lock common topic

    match await auth_and_lock_common_read(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as db_common:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return RealmSelfPromoteToOwnerStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return RealmSelfPromoteToOwnerStoreBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return RealmSelfPromoteToOwnerStoreBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return RealmSelfPromoteToOwnerStoreBadOutcome.AUTHOR_REVOKED

    if db_common.user_current_profile == UserProfile.OUTSIDER:
        return RealmSelfPromoteToOwnerStoreBadOutcome.AUTHOR_NOT_ALLOWED

    # 2) Validate certificate

    match realm_self_promote_to_owner_validate(
        now=now,
        expected_author_user_id=db_common.user_id,
        expected_author_device_id=author,
        author_verify_key=author_verify_key,
        realm_role_certificate=realm_role_certificate,
    ):
        case RealmRoleCertificate() as certif:
            assert certif.role == RealmRole.OWNER
        case error:
            return error

    # 3) Write lock realm topic

    match await lock_realm_write(
        conn,
        db_common.organization_internal_id,
        db_common.user_internal_id,
        certif.realm_id,
    ):
        case LockRealmWriteRealmData() as db_realm:
            pass
        case LockRealmWriteRealmBadOutcome.REALM_NOT_FOUND:
            return RealmSelfPromoteToOwnerStoreBadOutcome.REALM_NOT_FOUND
        case LockRealmWriteRealmBadOutcome.REALM_DELETED:
            return RealmSelfPromoteToOwnerStoreBadOutcome.REALM_DELETED
        case LockRealmWriteRealmBadOutcome.USER_NOT_IN_REALM:
            return RealmSelfPromoteToOwnerStoreBadOutcome.AUTHOR_NOT_ALLOWED

    # 4) Fetch from database what is needed and do the checks

    row = await conn.fetchrow(
        *_q_get_info_for_checks(
            realm_internal_id=db_realm.realm_internal_id,
        )
    )
    assert row is not None

    # 4.1) Check if there is already an active owner in the realm

    match row["has_active_owner"]:
        case True:
            return RealmSelfPromoteToOwnerActiveOwnerAlreadyExists(
                last_realm_certificate_timestamp=db_realm.last_realm_certificate_timestamp
            )
        case False:
            pass
        case _:
            assert False, row

    # 4.2) Check author has the highest role among active realm members

    match row["max_active_priority"]:
        case int() as max_active_priority:
            pass
        case None:
            # No active member in the realm at all (shouldn't happen since author is in realm)
            assert False, row
        case _:
            assert False, row

    author_priority = _role_to_priority(db_realm.realm_user_current_role)
    if author_priority < max_active_priority:
        return RealmSelfPromoteToOwnerStoreBadOutcome.AUTHOR_NOT_ALLOWED

    # 4.3) Ensure we are not breaking causality by adding a newer timestamp.

    match row["last_vlob_timestamp"]:
        case DateTime() as last_vlob_timestamp:
            last_timestamp = max(
                db_common.last_common_certificate_timestamp,
                db_realm.last_realm_certificate_timestamp,
                last_vlob_timestamp,
            )
        # `None` if the realm is totally empty
        case None:
            last_timestamp = max(
                db_common.last_common_certificate_timestamp,
                db_realm.last_realm_certificate_timestamp,
            )
        case _:
            assert False, row

    if last_timestamp >= certif.timestamp:
        return RequireGreaterTimestamp(strictly_greater_than=last_timestamp)

    # 5) All checks are good, now we do the actual insertion

    row = await conn.fetchrow(
        *_q_self_promote_to_owner(
            realm_internal_id=db_realm.realm_internal_id,
            author_internal_id=db_common.user_internal_id,
            certificate=realm_role_certificate,
            certified_by=db_common.device_internal_id,
            certified_on=certif.timestamp,
        )
    )
    assert row is not None

    match row["update_realm_topic_ok"]:
        case True:
            pass
        case _:
            assert False, row

    await send_signal(
        conn,
        EventRealmCertificate(
            organization_id=organization_id,
            timestamp=certif.timestamp,
            realm_id=certif.realm_id,
            user_id=certif.user_id,
            role_removed=False,
        ),
    )

    return certif
