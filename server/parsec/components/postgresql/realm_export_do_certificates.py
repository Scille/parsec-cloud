# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    OrganizationID,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.user_get_certificates import (
    sql_fragment_all_common_certificates,
    sql_fragment_all_realm_certificates,
    sql_fragment_all_sequester_certificates,
)
from parsec.components.postgresql.utils import (
    Q,
)
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

    return RealmExportCertificates(
        common_certificates=common_certificates,
        sequester_certificates=sequester_certificates,
        realm_certificates=realm_certificates,
    )
