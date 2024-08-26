# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    OrganizationID,
    VlobID,
)
from parsec.components.organization import (
    OrganizationDumpTopics,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import (
    Q,
    q_realm,
)

_q_get_topics = Q(f"""
WITH my_organization AS (
    SELECT _id
    FROM organization
    WHERE organization_id = $organization_id
),

my_common AS (
    SELECT
        'common' AS topic,
        NULL::text AS discriminant,
        last_timestamp
    FROM common_topic
    WHERE organization = (SELECT _id FROM my_organization)
    LIMIT 1
),

my_sequester AS (
    SELECT
        'sequester' AS topic,
        NULL::text AS discriminant,
        last_timestamp
    FROM sequester_topic
    WHERE organization = (SELECT _id FROM my_organization)
    LIMIT 1
),

my_realms AS (
    SELECT
        'realm' AS topic,
        { q_realm(select="realm_id", _id="realm_topic.realm") }::text AS discriminant,
        last_timestamp
    FROM realm_topic
    WHERE organization = (SELECT _id FROM my_organization)
),

my_shamir_recovery AS (
    SELECT
        'shamir_recovery' AS topic,
        NULL::text AS discriminant,
        last_timestamp
    FROM shamir_recovery_topic
    WHERE organization = (SELECT _id FROM my_organization)
)

SELECT * FROM my_common
UNION ALL
SELECT * FROM my_sequester
UNION ALL
SELECT * FROM my_realms
UNION ALL
SELECT * FROM my_shamir_recovery
""")


async def organization_test_dump_topics(
    conn: AsyncpgConnection, id: OrganizationID
) -> OrganizationDumpTopics:
    rows = await conn.fetch(*_q_get_topics(organization_id=id.str))

    common_last_timestamp = None
    sequester_last_timestamp = None
    shamir_recovery_last_timestamp = None
    per_realm_last_timestamp = {}
    for row in rows:
        match row["last_timestamp"]:
            case DateTime() as last_timestamp:
                pass
            case unknown:
                assert False, unknown

        match row["discriminant"]:
            case str() | None as raw_discriminant:
                pass
            case unknown:
                assert False, unknown

        match row["topic"]:
            case "common":
                assert common_last_timestamp is None
                assert raw_discriminant is None
                common_last_timestamp = last_timestamp
            case "sequester":
                assert sequester_last_timestamp is None
                assert raw_discriminant is None
                sequester_last_timestamp = last_timestamp
            case "realm":
                assert raw_discriminant is not None
                realm_id = VlobID.from_hex(raw_discriminant)
                per_realm_last_timestamp[realm_id] = last_timestamp
            case "shamir_recovery":
                assert shamir_recovery_last_timestamp is None
                assert raw_discriminant is None
                shamir_recovery_last_timestamp = last_timestamp
            case unknown:
                assert False, unknown

    if common_last_timestamp is None:
        raise RuntimeError("Organization not found")

    return OrganizationDumpTopics(
        common=common_last_timestamp,
        sequester=sequester_last_timestamp,
        realms=per_realm_last_timestamp,
        shamir_recovery=shamir_recovery_last_timestamp,
    )
