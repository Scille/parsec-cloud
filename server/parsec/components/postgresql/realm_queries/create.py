# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncpg

from parsec._parsec import OrganizationID, RealmRoleCertificate, RealmRole
from parsec.components.postgresql.handler import send_signal
from parsec.components.postgresql.utils import (
    Q,
    q_device_internal_id,
    q_organization_internal_id,
    q_user_internal_id,
    query,
)
from parsec.components.realm import CertificateBasedActionIdempotentOutcome

_q_insert_realm = Q(
    f"""
INSERT INTO realm (
    organization,
    realm_id,
    created_on
) SELECT
    { q_organization_internal_id("$organization_id") },
    $realm_id,
    $created_on
ON CONFLICT (organization, realm_id) DO NOTHING
RETURNING _id
"""
)


_q_insert_realm_role = Q(
    f"""
INSERT INTO realm_user_role(
    realm,
    user_,
    role,
    certificate,
    certified_by,
    certified_on
)
SELECT
    $realm_internal_id,
    { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") },
    'OWNER',
    $certificate,
    { q_device_internal_id(organization_id="$organization_id", device_id="$certified_by") },
    $certified_on
"""
)


_q_insert_realm_encryption_revision = Q(
    """
INSERT INTO vlob_encryption_revision (
    realm,
    encryption_revision
)
SELECT
    $_id,
    1
"""
)


async def query_create(
    conn: asyncpg.Connection,
    organization_id: OrganizationID,
    realm_role_certificate: bytes,
    realm_role_certificate_cooked: RealmRoleCertificate,
) -> None | CertificateBasedActionIdempotentOutcome:
    assert realm_role_certificate_cooked.role == RealmRole.OWNER

    realm_internal_id = await conn.fetchval(
        *_q_insert_realm(
            organization_id=organization_id.str,
            realm_id=realm_role_certificate_cooked.realm_id,
            created_on=realm_role_certificate_cooked.timestamp,
        )
    )
    if not realm_internal_id:
        return CertificateBasedActionIdempotentOutcome(
            certificate_timestamp=realm_role_certificate_cooked.timestamp  # TODO: fix with proper timestamp
        )

    # Ensure certificate consistency: our certificate must be the newest thing on the server.
    #
    # Strictly speaking there is no consistency requirement here (the new empty realm
    # has no impact on existing data).
    #
    # However we still use the same check that is applied everywhere else in order to be
    # consistent.

    # TODO: migrate this:
    # assert (
    #     org.last_certificate_or_vlob_timestamp is not None
    # )  # Bootstrap has created the first certif
    # if org.last_certificate_or_vlob_timestamp >= certif.timestamp:
    #     return RequireGreaterTimestamp(
    #         strictly_greater_than=org.last_certificate_or_vlob_timestamp
    #     )

    await conn.execute(
        *_q_insert_realm_role(
            realm_internal_id=realm_internal_id,
            organization_id=organization_id.str,
            user_id=realm_role_certificate_cooked.user_id.str,
            certificate=realm_role_certificate,
            certified_by=realm_role_certificate_cooked.author.str,
            certified_on=realm_role_certificate_cooked.timestamp,
        )
    )
