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
from parsec.components.events import EventBus
from parsec.components.postgresql import AsyncpgConnection
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
    BadKeyIndex,
    CertificateBasedActionIdempotentOutcome,
    RealmShareStoreBadOutcome,
    RealmShareValidateBadOutcome,
    realm_share_validate,
)
from parsec.events import EventRealmCertificate

_q_get_info_for_checks = Q(
    """
WITH my_recipient_user AS (
    SELECT
        _id,
        current_profile,
        (revoked_on IS NOT NULL) AS is_revoked
    FROM user_
    WHERE
        organization = $organization_internal_id
        AND user_id = $recipient_user_id
),

my_recipient_realm_role AS (
    SELECT role
    FROM realm_user_role
    WHERE
        user_ = (SELECT _id FROM my_recipient_user)
        AND realm = $realm_internal_id
    ORDER BY certified_on DESC
    LIMIT 1
),

my_last_keys_bundle AS (
    SELECT
        _id,
        key_index
    FROM realm_keys_bundle
    WHERE realm = $realm_internal_id
    ORDER BY certified_on DESC
    LIMIT 1
),

my_last_vlob_timestamp AS (
    SELECT MAX(created_on) AS timestamp
    FROM vlob_atom
    WHERE realm = $realm_internal_id
)

SELECT
    (SELECT _id FROM my_recipient_user) AS recipient_internal_id,
    (SELECT current_profile FROM my_recipient_user) AS recipient_current_profile,
    (SELECT is_revoked FROM my_recipient_user) AS recipient_is_revoked,
    (SELECT role FROM my_recipient_realm_role) AS recipient_current_role,
    (SELECT _id FROM my_last_keys_bundle) AS last_keys_bundle_internal_id,
    (SELECT key_index FROM my_last_keys_bundle) AS last_key_index,
    (SELECT timestamp FROM my_last_vlob_timestamp) AS last_vlob_timestamp
"""
)


_q_share = Q(
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
        $recipient_internal_id,
        $role,
        $certificate,
        $certified_by,
        $certified_on
    )
    RETURNING _id
),

new_realm_keys_bundle_access AS (
    INSERT INTO realm_keys_bundle_access (
        realm,
        user_,
        realm_keys_bundle,
        access
    ) VALUES (
        $realm_internal_id,
        $recipient_internal_id,
        $keys_bundle_internal_id,
        $keys_bundle_access
    )
    -- Overwriting the previous keys bundle access, this way a corrupted access
    -- can be corrected by re-sharing.
    ON CONFLICT (realm, user_, realm_keys_bundle) DO
        UPDATE SET
            access = excluded.access
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

SELECT
    COALESCE((SELECT * FROM updated_realm_topic), FALSE) AS update_realm_topic_ok
"""
)


async def realm_share(
    event_bus: EventBus,
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    realm_role_certificate: bytes,
    recipient_keys_bundle_access: bytes,
    key_index: int,
) -> (
    RealmRoleCertificate
    | BadKeyIndex
    | CertificateBasedActionIdempotentOutcome
    | RealmShareValidateBadOutcome
    | TimestampOutOfBallpark
    | RealmShareStoreBadOutcome
    | RequireGreaterTimestamp
):
    # 1) Read lock common topic

    match await auth_and_lock_common_read(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as db_common:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return RealmShareStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return RealmShareStoreBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return RealmShareStoreBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return RealmShareStoreBadOutcome.AUTHOR_REVOKED

    # 2) Validate certificate

    match realm_share_validate(
        now=now,
        expected_author_user_id=db_common.user_id,
        expected_author_device_id=author,
        author_verify_key=author_verify_key,
        realm_role_certificate=realm_role_certificate,
    ):
        case RealmRoleCertificate() as certif:
            assert certif.role is not None
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
            return RealmShareStoreBadOutcome.REALM_NOT_FOUND
        case LockRealmWriteRealmBadOutcome.USER_NOT_IN_REALM:
            return RealmShareStoreBadOutcome.AUTHOR_NOT_ALLOWED

    # 4) Fetch from database what is needed and do the checks

    row = await conn.fetchrow(
        *_q_get_info_for_checks(
            organization_internal_id=db_common.organization_internal_id,
            realm_internal_id=db_realm.realm_internal_id,
            recipient_user_id=certif.user_id,
        )
    )
    assert row is not None

    # 4.1) Check recipient exists and is not revoked

    match row["recipient_internal_id"]:
        case int() as recipient_internal_id:
            pass
        case None:
            return RealmShareStoreBadOutcome.RECIPIENT_NOT_FOUND
        case unknown:
            assert False, unknown

    match row["recipient_is_revoked"]:
        case False:
            pass
        case True:
            return RealmShareStoreBadOutcome.RECIPIENT_REVOKED
        case unknown:
            assert False, unknown

    match row["recipient_current_profile"]:
        case str() as raw_recipient_current_profile:
            recipient_current_profile = UserProfile.from_str(raw_recipient_current_profile)
        case unknown:
            assert False, unknown

    # 4.2) Check author is allowed to share recipient

    match row["recipient_current_role"]:
        case str() as raw_recipient_current_role:
            recipient_current_role = RealmRole.from_str(raw_recipient_current_role)
        case None as recipient_current_role:
            pass
        case unknown:
            assert False, unknown

    owner_only = (RealmRole.OWNER,)
    owner_or_manager = (RealmRole.OWNER, RealmRole.MANAGER)
    new_recipient_role = certif.role
    needed_author_roles: tuple[RealmRole, ...]
    if recipient_current_role in owner_or_manager or new_recipient_role in owner_or_manager:
        needed_author_roles = owner_only
    else:
        needed_author_roles = owner_or_manager

    if db_realm.realm_user_current_role not in needed_author_roles:
        return RealmShareStoreBadOutcome.AUTHOR_NOT_ALLOWED

    if recipient_current_profile == UserProfile.OUTSIDER and new_recipient_role in (
        RealmRole.MANAGER,
        RealmRole.OWNER,
    ):
        return RealmShareStoreBadOutcome.ROLE_INCOMPATIBLE_WITH_OUTSIDER

    if recipient_current_role == new_recipient_role:
        return CertificateBasedActionIdempotentOutcome(
            certificate_timestamp=db_realm.last_realm_certificate_timestamp
        )

    # 4.3) Check realm's last keys bundle & key index

    match row["last_keys_bundle_internal_id"]:
        case int() as last_keys_bundle_internal_id:
            pass
        case None:
            # Realm has no key rotation yet
            return BadKeyIndex(
                last_realm_certificate_timestamp=db_realm.last_realm_certificate_timestamp
            )
        case unknown:
            assert False, unknown

    match row["last_key_index"]:
        case int() as last_key_index:
            pass
        case unknown:
            assert False, unknown

    if key_index != last_key_index:
        return BadKeyIndex(
            last_realm_certificate_timestamp=db_realm.last_realm_certificate_timestamp
        )

    # 4.4) Ensure we are not breaking causality by adding a newer timestamp.

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
        case unknown:
            assert False, unknown

    if last_timestamp >= certif.timestamp:
        return RequireGreaterTimestamp(strictly_greater_than=last_timestamp)

    # 5) All checks are good, now we do the actual insertion

    row = await conn.fetchrow(
        *_q_share(
            realm_internal_id=db_realm.realm_internal_id,
            recipient_internal_id=recipient_internal_id,
            certificate=realm_role_certificate,
            certified_by=db_common.device_internal_id,
            certified_on=certif.timestamp,
            role=new_recipient_role.str,
            keys_bundle_internal_id=last_keys_bundle_internal_id,
            keys_bundle_access=recipient_keys_bundle_access,
        )
    )
    assert row is not None

    match row["update_realm_topic_ok"]:
        case True:
            pass
        case unknown:
            assert False, unknown

    await event_bus.send(
        EventRealmCertificate(
            organization_id=organization_id,
            timestamp=certif.timestamp,
            realm_id=certif.realm_id,
            user_id=certif.user_id,
            role_removed=False,
        )
    )

    return certif
