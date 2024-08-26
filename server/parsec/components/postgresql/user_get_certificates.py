# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    UserProfile,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.queries import (
    AuthNoLockBadOutcome,
    AuthNoLockData,
    auth_no_lock,
)
from parsec.components.postgresql.utils import Q
from parsec.components.user import (
    CertificatesBundle,
    UserGetCertificatesAsUserBadOutcome,
)

_q_get_certificates = Q("""
WITH all_common_certificates AS (
    -- User certificate
    (
        SELECT
            -- Certificates must be returned ordered by timestamp, however there is a trick
            -- for the common certificates: when a new user is created, the corresponding
            -- user and device certificates have the same timestamp, but we must return
            -- the user certificate first (given device references the user).
            -- Hence this priority field used to order with tuple (timestamp, priority).
            0 AS priority,
            created_on AS timestamp,
            (CASE WHEN $redacted THEN redacted_user_certificate ELSE user_certificate END) AS certificate
        FROM user_
        WHERE organization = $organization_internal_id
    )
    UNION
    -- Device certificate
    (
        SELECT
            1 AS priority,
            created_on AS timestamp,
            (CASE WHEN $redacted THEN redacted_device_certificate ELSE device_certificate END) AS certificate
        FROM device
        WHERE organization = $organization_internal_id
    )
    UNION
    -- User revoked certificate
    (
        SELECT
            1 AS priority,
            revoked_on AS timestamp,
            revoked_user_certificate AS certificate
        FROM user_
        WHERE organization = $organization_internal_id
        AND revoked_on IS NOT NULL
    )
    UNION
    -- User update certificate
    (
        SELECT
            1 AS priority,
            profile.certified_on AS timestamp,
            profile.profile_certificate AS certificate
        FROM profile
        INNER JOIN user_ ON profile.user_ = user_._id
        WHERE user_.organization = $organization_internal_id
    )
),

all_sequester_certificates AS (
    (
        SELECT
            _bootstrapped_on AS timestamp,
            sequester_authority_certificate AS certificate
        FROM organization
        WHERE
            _id = $organization_internal_id
            AND sequester_authority_certificate IS NOT NULL

    )
    UNION
    (
        SELECT
            created_on AS timestamp,
            service_certificate AS certificate
        FROM sequester_service
        WHERE organization = $organization_internal_id
    )
),

all_realm_certificates AS (
    -- Realm role certificate
    (
        SELECT
            realm,
            certified_on AS timestamp,
            certificate
        FROM realm_user_role
    )
    UNION
    -- Realm key rotation certificate
    (
        SELECT
            realm,
            certified_on AS timestamp,
            realm_key_rotation_certificate AS certificate
        FROM realm_keys_bundle
    )
    UNION
    -- Realm name certificate
    (
        SELECT
            realm,
            certified_on AS timestamp,
            realm_name_certificate AS certificate
        FROM realm_name
    )
),

-- Retrieve the last role for each realm the user is or used to be part of
my_realms_last_roles AS (
    SELECT DISTINCT ON (realm)
        realm,
        (role IS NOT NULL) AS currently_have_access,
        certified_on
    FROM realm_user_role
    WHERE user_ = $user_internal_id
    ORDER BY realm ASC, certified_on DESC
),

my_realm_certificates AS (
    SELECT
        (SELECT realm_id FROM realm WHERE _id = all_realm_certificates.realm) AS realm_id,
        all_realm_certificates.certificate,
        all_realm_certificates.timestamp
    FROM all_realm_certificates
    INNER JOIN my_realms_last_roles ON all_realm_certificates.realm = my_realms_last_roles.realm
    WHERE
        -- User can see all certificates from the realms he is part of...
        my_realms_last_roles.currently_have_access
        -- ...and all the certificates until he got revoked for realm he is not longer part of
        OR all_realm_certificates.timestamp <= my_realms_last_roles.certified_on
),

realm_after AS (
    SELECT
        UNNEST ($realm_after_ids::UUID[]) as realm_id,
        UNNEST ($realm_after_timestamps::TIMESTAMPTZ[]) as after
),

my_all_certificates AS (
    (
        SELECT
            'sequester' AS topic,
            NULL::TEXT AS discriminant,
            0 AS priority,
            timestamp,
            certificate
        FROM all_sequester_certificates
        WHERE COALESCE(timestamp > $sequester_after, TRUE)
        ORDER BY timestamp ASC
    )

    UNION ALL

    (
        SELECT
            'realm' AS topic,
            my_realm_certificates.realm_id::TEXT AS discriminant,
            0 AS priority,
            my_realm_certificates.timestamp,
            my_realm_certificates.certificate
        FROM my_realm_certificates
        LEFT JOIN realm_after ON my_realm_certificates.realm_id = realm_after.realm_id
        WHERE COALESCE(my_realm_certificates.timestamp > realm_after.after, TRUE)
    )


    UNION ALL

    -- Common must be fetch last given other topics depend on it.
    (
        SELECT
            'common' AS topic,
            NULL::TEXT AS discriminant,
            priority,
            timestamp,
            certificate
        FROM all_common_certificates
        WHERE COALESCE(timestamp > $common_after, TRUE)
    )
)

SELECT
    topic,
    discriminant,
    certificate
FROM my_all_certificates
-- ORDER BY must be done here given there is no guarantee on rows order after UNION
ORDER BY timestamp ASC, priority ASC
""")


async def user_get_certificates(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    author: DeviceID,
    common_after: DateTime | None,
    sequester_after: DateTime | None,
    shamir_recovery_after: DateTime | None,
    realm_after: dict[VlobID, DateTime],
) -> CertificatesBundle | UserGetCertificatesAsUserBadOutcome:
    # TODO
    assert shamir_recovery_after is None, "Shamir recovery not implemented yet"

    match await auth_no_lock(conn, organization_id, author):
        case AuthNoLockData() as db_auth:
            need_redacted = db_auth.user_current_profile == UserProfile.OUTSIDER
        case AuthNoLockBadOutcome.ORGANIZATION_NOT_FOUND:
            return UserGetCertificatesAsUserBadOutcome.ORGANIZATION_NOT_FOUND
        case AuthNoLockBadOutcome.ORGANIZATION_EXPIRED:
            return UserGetCertificatesAsUserBadOutcome.ORGANIZATION_EXPIRED
        case AuthNoLockBadOutcome.AUTHOR_NOT_FOUND:
            return UserGetCertificatesAsUserBadOutcome.AUTHOR_NOT_FOUND
        case AuthNoLockBadOutcome.AUTHOR_REVOKED:
            return UserGetCertificatesAsUserBadOutcome.AUTHOR_REVOKED

    rows = await conn.fetch(
        *_q_get_certificates(
            redacted=need_redacted,
            organization_internal_id=db_auth.organization_internal_id,
            user_internal_id=db_auth.user_internal_id,
            common_after=common_after,
            sequester_after=sequester_after,
            realm_after_ids=realm_after.keys(),
            realm_after_timestamps=realm_after.values(),
        )
    )

    common_certificates = []
    sequester_certificates = []
    realm_items = {}
    shamir_recovery_certificates = []

    for row in rows:
        match row["certificate"]:
            case bytes() as certificate:
                pass
            case _:
                assert False, row

        match row["discriminant"]:
            case str() | None as discriminant:
                pass
            case _:
                assert False, row

        match row["topic"]:
            # Note the rows are already ordered by timestamp
            case "common":
                assert discriminant is None, row
                common_certificates.append(certificate)
            case "sequester":
                assert discriminant is None, row
                sequester_certificates.append(certificate)
            case "realm":
                assert discriminant is not None, row
                realm_id = VlobID.from_hex(discriminant)
                try:
                    realm_items[realm_id].append(certificate)
                except KeyError:
                    realm_items[realm_id] = [certificate]
            case _:
                assert False, row

    return CertificatesBundle(
        common=common_certificates,
        sequester=sequester_certificates,
        realm=realm_items,
        shamir_recovery=shamir_recovery_certificates,
    )
