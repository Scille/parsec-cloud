# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmArchivingCertificate,
    RealmArchivingConfiguration,
    RealmRole,
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
    RealmUpdateArchivingStoreBadOutcome,
    RealmUpdateArchivingValidateBadOutcome,
    realm_update_archiving_validate,
)
from parsec.events import EventRealmCertificate

_q_get_org_and_realm_archiving_status = Q("""
WITH my_last_realm_archiving AS (
    SELECT
        configuration,
        deletion_date
    FROM realm_archiving
    WHERE realm = $realm_internal_id
    ORDER BY certified_on DESC
    LIMIT 1
)

SELECT
    organization.minimum_archiving_period,
    (SELECT my_last_realm_archiving.configuration FROM my_last_realm_archiving) AS last_archiving_configuration,
    (SELECT my_last_realm_archiving.deletion_date FROM my_last_realm_archiving) AS last_archiving_deletion_date
FROM organization
WHERE organization._id = $organization_internal_id
""")


_q_insert_realm_archiving = Q("""
WITH new_archiving AS (
    INSERT INTO realm_archiving (
        realm,
        configuration,
        deletion_date,
        certificate,
        certified_by,
        certified_on
    ) VALUES (
        $realm_internal_id,
        $configuration,
        $deletion_date,
        $certificate,
        $certified_by,
        $certified_on
    )
),

update_realm AS (
    UPDATE realm
    SET status = $status
    WHERE _id = $realm_internal_id
),

update_realm_topic AS (
    UPDATE realm_topic
    SET last_timestamp = $certified_on
    WHERE
        realm = $realm_internal_id
        -- Sanity check
        AND last_timestamp < $certified_on
    RETURNING TRUE
)

SELECT COALESCE((SELECT * FROM update_realm_topic), FALSE) AS update_realm_topic_ok
""")


async def realm_update_archiving(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    realm_archiving_certificate: bytes,
) -> (
    RealmArchivingCertificate
    | RealmUpdateArchivingValidateBadOutcome
    | TimestampOutOfBallpark
    | RealmUpdateArchivingStoreBadOutcome
    | RequireGreaterTimestamp
):
    match await auth_and_lock_common_read(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as db_common:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return RealmUpdateArchivingStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return RealmUpdateArchivingStoreBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return RealmUpdateArchivingStoreBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return RealmUpdateArchivingStoreBadOutcome.AUTHOR_REVOKED

    match realm_update_archiving_validate(
        now=now,
        expected_author=author,
        author_verify_key=author_verify_key,
        realm_archiving_certificate=realm_archiving_certificate,
    ):
        case RealmArchivingCertificate() as certif:
            pass
        case error:
            return error

    match await lock_realm_write(
        conn,
        db_common.organization_internal_id,
        db_common.user_internal_id,
        certif.realm_id,
    ):
        case LockRealmWriteRealmData() as db_realm:
            if db_realm.realm_user_current_role != RealmRole.OWNER:
                return RealmUpdateArchivingStoreBadOutcome.AUTHOR_NOT_ALLOWED
        case LockRealmWriteRealmBadOutcome.REALM_NOT_FOUND:
            return RealmUpdateArchivingStoreBadOutcome.REALM_NOT_FOUND
        case LockRealmWriteRealmBadOutcome.USER_NOT_IN_REALM:
            return RealmUpdateArchivingStoreBadOutcome.AUTHOR_NOT_ALLOWED

    row = await conn.fetchrow(
        *_q_get_org_and_realm_archiving_status(
            organization_internal_id=db_common.organization_internal_id,
            realm_internal_id=db_realm.realm_internal_id,
        )
    )
    assert row is not None

    match row["minimum_archiving_period"]:
        case int() as minimum_archiving_period:
            pass
        case _:
            assert False, row

    match row["last_archiving_configuration"]:
        case None:
            # No archiving record yet, realm is implicitly AVAILABLE
            pass
        case str() as last_archiving_configuration:
            if last_archiving_configuration == "DELETION_PLANNED":
                match row["last_archiving_deletion_date"]:
                    case DateTime() as last_deletion_date:
                        if last_deletion_date <= now:
                            return RealmUpdateArchivingStoreBadOutcome.REALM_DELETED
                    case _:
                        assert False, row
        case _:
            assert False, row

    # Check minimum archiving period for DeletionPlanned configuration
    min_deletion_date = now.add(seconds=minimum_archiving_period)
    if (
        certif.configuration.deletion_date is not None
        and certif.configuration.deletion_date < min_deletion_date
    ):
        return RealmUpdateArchivingStoreBadOutcome.ARCHIVING_PERIOD_TOO_SHORT

    # Ensure we are not breaking causality by adding a newer timestamp.

    last_certificate = max(
        db_common.last_common_certificate_timestamp,
        db_realm.last_realm_certificate_timestamp,
    )
    if last_certificate >= certif.timestamp:
        return RequireGreaterTimestamp(strictly_greater_than=last_certificate)

    # All checks are good, now we do the actual insertion

    if certif.configuration == RealmArchivingConfiguration.AVAILABLE:
        configuration_str = "AVAILABLE"
        deletion_date = None
        status = "AVAILABLE"
    elif certif.configuration == RealmArchivingConfiguration.ARCHIVED:
        configuration_str = "ARCHIVED"
        deletion_date = None
        status = "ARCHIVED_OR_DELETION_PLANNED"
    else:
        configuration_str = "DELETION_PLANNED"
        deletion_date = certif.configuration.deletion_date
        assert deletion_date is not None
        status = "ARCHIVED_OR_DELETION_PLANNED"

    update_realm_topic_ok = await conn.fetchval(
        *_q_insert_realm_archiving(
            realm_internal_id=db_realm.realm_internal_id,
            configuration=configuration_str,
            deletion_date=deletion_date,
            status=status,
            certificate=realm_archiving_certificate,
            certified_by=db_common.device_internal_id,
            certified_on=certif.timestamp,
        )
    )
    match update_realm_topic_ok:
        case True:
            pass
        case unknown:
            assert False, unknown

    await send_signal(
        conn,
        EventRealmCertificate(
            organization_id=organization_id,
            timestamp=certif.timestamp,
            realm_id=certif.realm_id,
            user_id=db_common.user_id,
            role_removed=False,
        ),
    )

    return certif
