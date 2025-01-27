# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmRole,
    RealmRoleCertificate,
    VerifyKey,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.events import EventBus
from parsec.components.postgresql import AsyncpgConnection
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
        { q_user_internal_id(organization="$organization_internal_id", user_id="$user_id") },
        'OWNER',
        $certificate,
        { q_device_internal_id(organization="$organization_internal_id", device_id="$certified_by") },
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
    event_bus: EventBus,
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

    # The realm already exists

    if realm_internal_id is None:
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

    # The realm has been created, fill the other realm-related tables

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

    await event_bus.send(
        EventRealmCertificate(
            organization_id=organization_id,
            timestamp=certif.timestamp,
            realm_id=certif.realm_id,
            user_id=certif.user_id,
            role_removed=certif.role is None,
        )
    )

    return certif
