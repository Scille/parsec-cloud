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
    auth_and_lock_common_read,
)
from parsec.components.postgresql.utils import (
    Q,
    q_device_internal_id,
    q_user_internal_id,
)
from parsec.components.realm import (
    CertificateBasedActionIdempotentOutcome,
    RealmCreateStoreBadOutcome,
    RealmCreateValidateBadOutcome,
    realm_create_validate,
)
from parsec.events import EventRealmCertificate

# Two important notes about preventing race conditions during realm creation:
# - 1. Inserting a row with an `ON CONFLICT DO NOTHING` clause allows us to
#   effectively lock the realm creation, as conflicting concurrent queries
#   will block until the first transaction owning the lock is done (similar to
#   what happens with a `SELECT ... FOR UPDATE` statement).
# - 2. It is tempting to use a `WITH` clause to have a single query that both
#   locks the realm creation and fills the realm-related tables. However, this
#   would expose use to race conditions, as the `WITH` clause does not provide
#   any guarantee on the order of execution of its subqueries. More generally,
#   the order of execution of subqueries in an SQL query is not guaranteed, whether
#   it is using a `WITH` clause, the `(SELECT ...)` syntax, or the `FROM ... JOIN ...`.
#   This is why we have to split the realm creation in two queries, one to lock the
#   realm creation and one to fill the realm-related tables.

_q_lock_realm_creation = Q(
    """
INSERT INTO realm (
    organization,
    realm_id,
    created_on,
    key_index
) VALUES (
    $organization_internal_id,
    $realm_id,
    $timestamp,
    0
)
ON CONFLICT (organization, realm_id) DO NOTHING
RETURNING _id
"""
)

_q_get_last_realm_certificate_timestamp = Q(
    """
SELECT last_timestamp
FROM realm_topic
INNER JOIN realm ON realm._id = realm_topic.realm
WHERE
    realm.organization = $organization_internal_id
    AND realm_id = $realm_id
"""
)

_q_create_realm = Q(
    f"""
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
        {q_user_internal_id(organization="$organization_internal_id", user_id="$user_id")},
        'OWNER',
        $certificate,
        {q_device_internal_id(organization="$organization_internal_id", device_id="$certified_by")},
        $timestamp
    )
    RETURNING TRUE AS success
),

new_realm_topic AS (
    INSERT INTO realm_topic (
        organization,
        realm,
        last_timestamp
    ) VALUES (
        $organization_internal_id,
        $realm_internal_id,
        $timestamp
    )
    RETURNING TRUE AS success
)

SELECT new_realm_user_role.success AND new_realm_topic.success AS success
FROM new_realm_user_role, new_realm_topic
"""
)


async def realm_create(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    realm_role_certificate: bytes,
) -> (
    RealmRoleCertificate
    | CertificateBasedActionIdempotentOutcome
    | RealmCreateValidateBadOutcome
    | TimestampOutOfBallpark
    | RealmCreateStoreBadOutcome
    | RequireGreaterTimestamp
):
    # Note there is no realm topic lock here, this is because there is no topic
    # to lock since the realm doesn't exist yet !
    # Instead we rely on the database's organization_id+realm_id unique constraint
    # to protect us against concurrent creation of the same realm.

    match await auth_and_lock_common_read(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as db_common:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return RealmCreateStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return RealmCreateStoreBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return RealmCreateStoreBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return RealmCreateStoreBadOutcome.AUTHOR_REVOKED

    if db_common.user_current_profile == UserProfile.OUTSIDER:
        return RealmCreateStoreBadOutcome.AUTHOR_NOT_ALLOWED

    match realm_create_validate(
        now=now,
        expected_author_device_id=author,
        expected_author_user_id=db_common.user_id,
        author_verify_key=author_verify_key,
        realm_role_certificate=realm_role_certificate,
    ):
        case RealmRoleCertificate() as certif:
            pass
        case error:
            return error
    assert certif.role == RealmRole.OWNER

    # Ensure we are not breaking causality by adding a newer timestamp.

    if db_common.last_common_certificate_timestamp >= certif.timestamp:
        return RequireGreaterTimestamp(
            strictly_greater_than=db_common.last_common_certificate_timestamp
        )

    # Lock the realm creation by trying to insert a new row in the realm table

    realm_internal_id = await conn.fetchval(
        *_q_lock_realm_creation(
            organization_internal_id=db_common.organization_internal_id,
            realm_id=certif.realm_id,
            timestamp=certif.timestamp,
        )
    )

    match realm_internal_id:
        # The realm has been successfully created and locked
        case int() as realm_internal_id:
            pass

        # The realm has already been created, return the last realm certificate timestamp
        case None:
            last_realm_certificate_timestamp = await conn.fetchval(
                *_q_get_last_realm_certificate_timestamp(
                    organization_internal_id=db_common.organization_internal_id,
                    realm_id=certif.realm_id,
                )
            )
            assert isinstance(last_realm_certificate_timestamp, DateTime)
            return CertificateBasedActionIdempotentOutcome(
                certificate_timestamp=last_realm_certificate_timestamp
            )

        case unknown:
            assert False, repr(unknown)

    # Fill the other realm-related tables

    success = await conn.fetchval(
        *_q_create_realm(
            organization_internal_id=db_common.organization_internal_id,
            realm_internal_id=realm_internal_id,
            timestamp=certif.timestamp,
            user_id=certif.user_id,
            certificate=realm_role_certificate,
            certified_by=certif.author,
        )
    )
    assert success, success

    # Send the corresponding event

    await send_signal(
        conn,
        EventRealmCertificate(
            organization_id=organization_id,
            timestamp=certif.timestamp,
            realm_id=certif.realm_id,
            user_id=certif.user_id,
            role_removed=certif.role is None,
        ),
    )

    return certif
