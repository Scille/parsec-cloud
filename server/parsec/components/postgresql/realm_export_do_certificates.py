# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    OrganizationID,
    SequesterServiceID,
    UserID,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.user_get_certificates import (
    sql_fragment_all_common_certificates,
    sql_fragment_all_realm_certificates,
    sql_fragment_all_sequester_certificates,
)
from parsec.components.postgresql.utils import Q, q_user
from parsec.components.realm import (
    RealmExportCertificates,
    RealmExportDoCertificatesBadOutcome,
)

_q_get_org_and_realm = Q(
    """
WITH my_organization AS (
    SELECT _id
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
),

my_realm AS (
    SELECT _id
    FROM realm
    WHERE
        organization = (SELECT _id FROM my_organization)
        AND realm_id = $realm_id
    LIMIT 1
)

SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT _id FROM my_realm) AS realm_internal_id
"""
)


_q_get_certificates = Q(f"""
WITH
    -- Note those fragments contain `$organization_internal_id` & `$redacted`
    all_common_certificates AS ({sql_fragment_all_common_certificates}),
    all_sequester_certificates AS ({sql_fragment_all_sequester_certificates}),
    all_realm_certificates AS ({sql_fragment_all_realm_certificates}),

my_all_certificates AS (
    (
        SELECT
            'common' AS topic,
            priority,
            timestamp,
            certificate
        FROM all_common_certificates
        WHERE timestamp <= $common_certificate_timestamp_upper_bound
    )

    UNION ALL

        (
        SELECT
            'sequester' AS topic,
            0 AS priority,
            timestamp,
            certificate
        FROM all_sequester_certificates
        WHERE timestamp <= $sequester_certificate_timestamp_upper_bound
    )

    UNION ALL

    (
        SELECT
            'realm' AS topic,
            0 AS priority,
            timestamp,
            certificate
        FROM all_realm_certificates
        WHERE
            realm = $realm_internal_id
            AND timestamp <= $realm_certificate_timestamp_upper_bound
    )
)

SELECT
    topic,
    certificate
FROM my_all_certificates
-- ORDER BY must be done here given there is no guarantee on rows order after UNION
ORDER BY timestamp ASC, priority ASC
""")


_q_get_realm_keys_bundles = Q("""
SELECT
    key_index,
    keys_bundle
FROM realm_keys_bundle
WHERE
    realm = $realm_internal_id
    AND certified_on <= $realm_certificate_timestamp_upper_bound
""")


_q_get_realm_keys_bundle_user_accesses = Q(f"""
SELECT
    {q_user(_id="realm_keys_bundle_access.user_", select="user_id")} AS user_id,
    realm_keys_bundle.key_index,
    realm_keys_bundle_access.access,
    realm_user_role.certified_on,
    realm_keys_bundle.certified_on,
    COALESCE(realm_user_role.certified_on, realm_keys_bundle.certified_on) AS foo
FROM realm_keys_bundle_access
LEFT JOIN realm_keys_bundle ON realm_keys_bundle_access.realm_keys_bundle = realm_keys_bundle._id
LEFT JOIN realm_user_role ON realm_keys_bundle_access.from_sharing = realm_user_role._id
WHERE
    realm_keys_bundle_access.realm = $realm_internal_id
    AND COALESCE(realm_user_role.certified_on, realm_keys_bundle.certified_on) <= $realm_certificate_timestamp_upper_bound
""")


_q_get_realm_keys_bundle_sequester_accesses = Q("""
SELECT
    sequester_service.service_id AS sequester_service_id,
    realm_keys_bundle.key_index,
    realm_sequester_keys_bundle_access.access
FROM realm_sequester_keys_bundle_access
LEFT JOIN realm_keys_bundle ON realm_sequester_keys_bundle_access.realm_keys_bundle = realm_keys_bundle._id
LEFT JOIN sequester_service ON sequester_service._id = realm_sequester_keys_bundle_access.sequester_service
WHERE
    realm_sequester_keys_bundle_access.realm = $realm_internal_id
    AND realm_keys_bundle.certified_on <= $realm_certificate_timestamp_upper_bound
""")


async def realm_export_do_certificates(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    common_certificate_timestamp_upper_bound: DateTime,
    realm_certificate_timestamp_upper_bound: DateTime,
    sequester_certificate_timestamp_upper_bound: DateTime | None,
) -> RealmExportCertificates | RealmExportDoCertificatesBadOutcome:
    row = await conn.fetchrow(
        *_q_get_org_and_realm(organization_id=organization_id.str, realm_id=realm_id)
    )
    assert row is not None

    match row["organization_internal_id"]:
        case int() as organization_internal_id:
            pass
        case None:
            return RealmExportDoCertificatesBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, unknown

    match row["realm_internal_id"]:
        case int() as realm_internal_id:
            pass
        case None:
            return RealmExportDoCertificatesBadOutcome.REALM_NOT_FOUND
        case unknown:
            assert False, unknown

    rows = await conn.fetch(
        *_q_get_certificates(
            organization_internal_id=organization_internal_id,
            realm_internal_id=realm_internal_id,
            common_certificate_timestamp_upper_bound=common_certificate_timestamp_upper_bound,
            realm_certificate_timestamp_upper_bound=realm_certificate_timestamp_upper_bound,
            sequester_certificate_timestamp_upper_bound=sequester_certificate_timestamp_upper_bound,
            redacted=False,  # Dummy parameter to make happy the sql fragments
        )
    )
    assert rows is not None

    common_certificates: list[bytes] = []
    sequester_certificates: list[bytes] = []
    realm_certificates: list[bytes] = []

    for row in rows:
        match row["certificate"]:
            case bytes() as certificate:
                pass
            case _:
                assert False, row

        match row["topic"]:
            # Note the rows are already ordered by timestamp
            case "common":
                common_certificates.append(certificate)
            case "sequester":
                sequester_certificates.append(certificate)
            case "realm":
                realm_certificates.append(certificate)
            case _:
                assert False, row

    rows = await conn.fetch(
        *_q_get_realm_keys_bundles(
            realm_internal_id=realm_internal_id,
            realm_certificate_timestamp_upper_bound=realm_certificate_timestamp_upper_bound,
        )
    )
    assert rows is not None

    realm_keys_bundles: list[tuple[int, bytes]] = []
    for row in rows:
        match row["key_index"]:
            case int() as key_index:
                pass
            case _:
                assert False, row

        match row["keys_bundle"]:
            case bytes() as keys_bundle:
                pass
            case _:
                assert False, row

        realm_keys_bundles.append((key_index, keys_bundle))

    rows = await conn.fetch(
        *_q_get_realm_keys_bundle_user_accesses(
            realm_internal_id=realm_internal_id,
            realm_certificate_timestamp_upper_bound=realm_certificate_timestamp_upper_bound,
        )
    )
    assert rows is not None

    realm_keys_bundle_user_accesses: list[tuple[UserID, int, bytes]] = []
    for row in rows:
        match row["user_id"]:
            case str() as raw_user_id:
                user_id = UserID.from_hex(raw_user_id)
            case _:
                assert False, row

        match row["key_index"]:
            case int() as key_index:
                pass
            case _:
                assert False, row

        match row["access"]:
            case bytes() as access:
                pass
            case _:
                assert False, row

        realm_keys_bundle_user_accesses.append((user_id, key_index, access))

    rows = await conn.fetch(
        *_q_get_realm_keys_bundle_sequester_accesses(
            realm_internal_id=realm_internal_id,
            realm_certificate_timestamp_upper_bound=realm_certificate_timestamp_upper_bound,
        )
    )
    assert rows is not None

    realm_keys_bundle_sequester_accesses: list[tuple[SequesterServiceID, int, bytes]] = []
    for row in rows:
        match row["sequester_service_id"]:
            case str() as raw_sequester_service_id:
                sequester_service_id = SequesterServiceID.from_hex(raw_sequester_service_id)
                pass
            case _:
                assert False, row

        match row["key_index"]:
            case int() as key_index:
                pass
            case _:
                assert False, row

        match row["access"]:
            case bytes() as access:
                pass
            case _:
                assert False, row

        realm_keys_bundle_sequester_accesses.append((sequester_service_id, key_index, access))

    return RealmExportCertificates(
        common_certificates=common_certificates,
        sequester_certificates=sequester_certificates,
        realm_certificates=realm_certificates,
        realm_keys_bundles=realm_keys_bundles,
        realm_keys_bundle_user_accesses=realm_keys_bundle_user_accesses,
        realm_keys_bundle_sequester_accesses=realm_keys_bundle_sequester_accesses,
    )
