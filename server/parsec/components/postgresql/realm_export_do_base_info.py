# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    OrganizationID,
    VerifyKey,
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
    RealmExportDoBaseInfo,
    RealmExportDoBaseInfoBadOutcome,
)

_q_get_org_and_realm = Q(
    """
WITH my_organization AS (
    SELECT
        _id,
        root_verify_key
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
    (SELECT root_verify_key FROM my_organization),
    (SELECT _id FROM my_realm) AS realm_internal_id
"""
)


_q_get_base_info = Q(
    f"""
WITH
    -- Note those fragments contain `$organization_internal_id` & `$redacted`
    all_common_certificates AS ({sql_fragment_all_common_certificates}),
    all_sequester_certificates AS ({sql_fragment_all_sequester_certificates}),
    all_realm_certificates AS ({sql_fragment_all_realm_certificates}),

my_vlobs AS (
    SELECT
        _id,
        size
    FROM vlob_atom
    WHERE
        realm = $realm_internal_id
        AND created_on <= $snapshot_timestamp
),

my_blocks AS (
    SELECT
        _id,
        size
    FROM block
    WHERE
        realm = $realm_internal_id
        AND created_on <= $snapshot_timestamp
)

SELECT
    COALESCE(
        (SELECT MAX(_id) FROM my_vlobs),
        0
    ) AS vlob_offset_marker_upper_bound,
    COALESCE(
        (SELECT MAX(_id) FROM my_blocks),
        0
    ) AS block_offset_marker_upper_bound,
    COALESCE(
        (SELECT SUM(size) FROM my_vlobs),
        0
    ) AS vlobs_total_bytes,
    COALESCE(
        (SELECT SUM(size) FROM my_blocks),
        0
    ) AS blocks_total_bytes,
    (
        SELECT MAX(timestamp)
        FROM all_realm_certificates
        WHERE
            realm = $realm_internal_id
            AND timestamp <= $snapshot_timestamp
    ) AS realm_certificate_timestamp_upper_bound,
    (
        SELECT MAX(timestamp) FROM all_common_certificates WHERE timestamp <= $snapshot_timestamp
    ) AS common_certificate_timestamp_upper_bound,
    (
        SELECT MAX(timestamp) FROM all_sequester_certificates WHERE timestamp <= $snapshot_timestamp
    ) AS sequester_certificate_timestamp_upper_bound
"""
)


async def realm_export_do_base_info(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    snapshot_timestamp: DateTime,
) -> RealmExportDoBaseInfo | RealmExportDoBaseInfoBadOutcome:
    row = await conn.fetchrow(
        *_q_get_org_and_realm(organization_id=organization_id.str, realm_id=realm_id)
    )
    assert row is not None

    match row["organization_internal_id"]:
        case int() as organization_internal_id:
            pass
        case None:
            return RealmExportDoBaseInfoBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, unknown

    match row["realm_internal_id"]:
        case int() as realm_internal_id:
            pass
        case None:
            return RealmExportDoBaseInfoBadOutcome.REALM_NOT_FOUND
        case unknown:
            assert False, unknown

    match row["root_verify_key"]:
        case bytes() as root_verify_key:
            root_verify_key = VerifyKey(root_verify_key)
        case unknown:
            assert False, unknown

    row = await conn.fetchrow(
        *_q_get_base_info(
            organization_internal_id=organization_internal_id,
            realm_internal_id=realm_internal_id,
            snapshot_timestamp=snapshot_timestamp,
            redacted=False,  # Dummy parameter to make happy the sql fragments
        )
    )
    assert row is not None

    match row["vlob_offset_marker_upper_bound"]:
        case int() as vlob_offset_marker_upper_bound:
            pass
        case unknown:
            assert False, unknown

    match row["block_offset_marker_upper_bound"]:
        case int() as block_offset_marker_upper_bound:
            pass
        case unknown:
            assert False, unknown

    match row["vlobs_total_bytes"]:
        case int() as vlobs_total_bytes:
            pass
        case unknown:
            assert False, unknown

    match row["blocks_total_bytes"]:
        case int() as blocks_total_bytes:
            pass
        case unknown:
            assert False, unknown

    match row["realm_certificate_timestamp_upper_bound"]:
        case None:
            return RealmExportDoBaseInfoBadOutcome.REALM_DIDNT_EXIST_AT_SNAPSHOT_TIMESTAMP
        case DateTime() as realm_certificate_timestamp_upper_bound:
            pass
        case unknown:
            assert False, unknown

    match row["common_certificate_timestamp_upper_bound"]:
        case DateTime() as common_certificate_timestamp_upper_bound:
            pass
        case unknown:
            assert False, unknown

    match row["sequester_certificate_timestamp_upper_bound"]:
        case DateTime() | None as sequester_certificate_timestamp_upper_bound:
            pass
        case unknown:
            assert False, unknown

    return RealmExportDoBaseInfo(
        root_verify_key=root_verify_key,
        vlob_offset_marker_upper_bound=vlob_offset_marker_upper_bound,
        block_offset_marker_upper_bound=block_offset_marker_upper_bound,
        blocks_total_bytes=blocks_total_bytes,
        vlobs_total_bytes=vlobs_total_bytes,
        common_certificate_timestamp_upper_bound=common_certificate_timestamp_upper_bound,
        realm_certificate_timestamp_upper_bound=realm_certificate_timestamp_upper_bound,
        sequester_certificate_timestamp_upper_bound=sequester_certificate_timestamp_upper_bound,
    )
