# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmKeyRotationCertificate,
    RealmRole,
    SequesterServiceID,
    UserID,
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
    q_sequester_service_internal_id,
    q_user,
    q_user_internal_id,
)
from parsec.components.realm import (
    BadKeyIndex,
    ParticipantMismatch,
    RealmRotateKeyStoreBadOutcome,
    RealmRotateKeyValidateBadOutcome,
    SequesterServiceMismatch,
    realm_rotate_key_validate,
)
from parsec.components.sequester import RejectedBySequesterService, SequesterServiceUnavailable
from parsec.events import EventRealmCertificate

_q_get_realm_current_participants = Q(
    f"""
WITH per_user_last_role AS (
    SELECT DISTINCT ON(user_)
        {q_user(_id="realm_user_role.user_", select="user_id")} AS user_id,
        role
    FROM  realm_user_role
    WHERE realm = $realm_internal_id
    ORDER BY user_, certified_on DESC
)

SELECT user_id FROM per_user_last_role WHERE role IS NOT NULL
"""
)


_q_lock_sequester_read_and_get_active_services = Q("""
-- Sequester topic read lock
WITH my_locked_sequester_topic AS (
    SELECT last_timestamp
    FROM sequester_topic
    WHERE
        organization = $organization_internal_id
    LIMIT 1
    FOR SHARE
),
my_active_sequester_services AS (
    SELECT
        _id,
        service_id,
        webhook_url
    FROM sequester_service
    WHERE
        organization = $organization_internal_id
        AND revoked_on IS NULL
)

SELECT
    -- First row only: sequester topic info column
    last_timestamp AS last_sequester_certificate_timestamp,

    -- Non-first rows only: service info columns
    NULL AS service_internal_id,
    NULL AS service_id,
    NULL AS service_webhook_url
FROM my_locked_sequester_topic

UNION ALL -- Using UNION ALL is import to avoid sorting !

SELECT
    -- First row only: sequester topic info column
    NULL,

    -- Non-first rows only: service info columns
    _id,
    service_id,
    webhook_url
FROM my_active_sequester_services
""")


_q_insert_keys_bundle = Q(
    """
WITH new_realm_keys_bundle AS (
    INSERT INTO realm_keys_bundle (
        realm,
        key_index,
        realm_key_rotation_certificate,
        certified_by,
        certified_on,
        key_canary,
        keys_bundle
    ) VALUES (
        $realm_internal_id,
        $key_index,
        $realm_key_rotation_certificate,
        $author_internal_id,
        $certified_on,
        $key_canary,
        $keys_bundle
    )
    RETURNING _id
),

updated_realm_key_index AS (
    UPDATE realm
    SET key_index = $key_index
    WHERE _id = $realm_internal_id
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
    COALESCE((SELECT * FROM updated_realm_topic), FALSE) AS update_realm_topic_ok,
    (SELECT _id FROM new_realm_keys_bundle) AS keys_bundle_internal_id
"""
)


_q_insert_participants_keys_bundle_access = Q(
    f"""
INSERT INTO realm_keys_bundle_access (
    realm,
    user_,
    realm_keys_bundle,
    access,
    from_sharing
) VALUES (
    $realm_internal_id,
    {q_user_internal_id(organization="$organization_internal_id", user_id="$user_id")},
    $realm_keys_bundle_internal_id,
    $access,
    NULL
)
"""
)


_q_insert_sequester_services_keys_bundle_access = Q(
    f"""
INSERT INTO realm_sequester_keys_bundle_access (
    realm,
    sequester_service,
    realm_keys_bundle,
    access
) VALUES (
    $realm_internal_id,
    {q_sequester_service_internal_id(organization="$organization_internal_id", service_id="$service_id")},
    $realm_keys_bundle_internal_id,
    $access
)
"""
)


async def realm_rotate_key(
    conn: AsyncpgConnection,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    author_verify_key: VerifyKey,
    realm_key_rotation_certificate: bytes,
    per_participant_keys_bundle_access: dict[UserID, bytes],
    keys_bundle: bytes,
    per_sequester_service_keys_bundle_access: dict[SequesterServiceID, bytes] | None,
) -> (
    RealmKeyRotationCertificate
    | BadKeyIndex
    | RealmRotateKeyValidateBadOutcome
    | TimestampOutOfBallpark
    | RealmRotateKeyStoreBadOutcome
    | RequireGreaterTimestamp
    | ParticipantMismatch
    | SequesterServiceMismatch
    | SequesterServiceUnavailable
    | RejectedBySequesterService
):
    match await auth_and_lock_common_read(conn, organization_id, author):
        case AuthAndLockCommonOnlyData() as db_common:
            pass
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_NOT_FOUND:
            return RealmRotateKeyStoreBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.ORGANIZATION_EXPIRED:
            return RealmRotateKeyStoreBadOutcome.ORGANIZATION_EXPIRED
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_NOT_FOUND:
            return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_FOUND
        case AuthAndLockCommonOnlyBadOutcome.AUTHOR_REVOKED:
            return RealmRotateKeyStoreBadOutcome.AUTHOR_REVOKED

    match realm_rotate_key_validate(
        now=now,
        expected_author=author,
        author_verify_key=author_verify_key,
        realm_key_rotation_certificate=realm_key_rotation_certificate,
    ):
        case RealmKeyRotationCertificate() as certif:
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
                return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_ALLOWED
        case LockRealmWriteRealmBadOutcome.REALM_NOT_FOUND:
            return RealmRotateKeyStoreBadOutcome.REALM_NOT_FOUND
        case LockRealmWriteRealmBadOutcome.USER_NOT_IN_REALM:
            return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_ALLOWED

    # We only accept the last key
    if db_realm.realm_key_index + 1 != certif.key_index:
        return BadKeyIndex(
            last_realm_certificate_timestamp=db_realm.last_realm_certificate_timestamp
        )

    # Ensure we are not breaking causality by adding a newer timestamp.

    last_certificate = max(
        db_common.last_common_certificate_timestamp,
        db_realm.last_realm_certificate_timestamp,
    )
    if last_certificate >= certif.timestamp:
        return RequireGreaterTimestamp(strictly_greater_than=last_certificate)

    rows = await conn.fetch(
        *_q_get_realm_current_participants(
            realm_internal_id=db_realm.realm_internal_id,
        )
    )
    participants = {UserID.from_hex(row["user_id"]) for row in rows}
    if per_participant_keys_bundle_access.keys() != participants:
        return ParticipantMismatch(
            last_realm_certificate_timestamp=db_realm.last_realm_certificate_timestamp
        )

    if not db_common.organization_is_sequestered:
        if per_sequester_service_keys_bundle_access is not None:
            return RealmRotateKeyStoreBadOutcome.ORGANIZATION_NOT_SEQUESTERED

    else:
        # Organization is sequestered

        rows = await conn.fetch(
            *_q_lock_sequester_read_and_get_active_services(
                organization_internal_id=db_common.organization_internal_id,
            )
        )

        # First row is always present and provide info about the sequester topic
        first_row = rows[0]
        match first_row["last_sequester_certificate_timestamp"]:
            case DateTime() as last_sequester_certificate_timestamp:
                pass
            case unknown:
                assert False, repr(unknown)

        # Then following rows are services
        sequester_services = {}
        for row in rows[1:]:
            match row["service_id"]:
                case str() as raw_service_id:
                    service_id = SequesterServiceID.from_hex(raw_service_id)
                case unknown:
                    assert False, repr(unknown)

            match row["service_webhook_url"]:
                case None | str() as service_webhook_url:
                    pass
                case unknown:
                    assert False, repr(unknown)

            sequester_services[service_id] = service_webhook_url

        if (
            per_sequester_service_keys_bundle_access is None
            or per_sequester_service_keys_bundle_access.keys() != sequester_services.keys()
        ):
            return SequesterServiceMismatch(
                last_sequester_certificate_timestamp=last_sequester_certificate_timestamp,
            )

        # TODO: Webhook sequester service are complex to implement since they do HTTP requests.
        #       However we shouldn't keep the connection to PostgreSQL while doing those HTTP
        #       request to avoid famine.
        has_webhook_services = any(
            service_webhook_url is not None for service_webhook_url in sequester_services.values()
        )
        if has_webhook_services:
            raise NotImplementedError("Webhook sequester services are not supported yet !")

    # All checks are good, now we do the actual insertion

    row = await conn.fetchrow(
        *_q_insert_keys_bundle(
            realm_internal_id=db_realm.realm_internal_id,
            key_index=certif.key_index,
            realm_key_rotation_certificate=realm_key_rotation_certificate,
            author_internal_id=db_common.device_internal_id,
            certified_on=certif.timestamp,
            key_canary=certif.key_canary,
            keys_bundle=keys_bundle,
        )
    )
    assert row is not None

    match row["update_realm_topic_ok"]:
        case True:
            pass
        case unknown:
            assert False, unknown

    match row["keys_bundle_internal_id"]:
        case int() as keys_bundle_internal_id:
            pass
        case unknown:
            assert False, unknown

    def arg_gen():
        for user_id, access in per_participant_keys_bundle_access.items():
            yield _q_insert_participants_keys_bundle_access.arg_only(
                organization_internal_id=db_common.organization_internal_id,
                realm_internal_id=db_realm.realm_internal_id,
                user_id=user_id,
                realm_keys_bundle_internal_id=keys_bundle_internal_id,
                access=access,
            )

    await conn.executemany(
        _q_insert_participants_keys_bundle_access.sql,
        arg_gen(),
    )

    if per_sequester_service_keys_bundle_access:

        def arg_gen():
            for service_id, access in per_sequester_service_keys_bundle_access.items():
                x = _q_insert_sequester_services_keys_bundle_access.arg_only(
                    organization_internal_id=db_common.organization_internal_id,
                    realm_internal_id=db_realm.realm_internal_id,
                    service_id=service_id,
                    realm_keys_bundle_internal_id=keys_bundle_internal_id,
                    access=access,
                )
                yield x

        await conn.executemany(
            _q_insert_sequester_services_keys_bundle_access.sql,
            arg_gen(),
        )

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
