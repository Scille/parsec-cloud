# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import OrganizationID
from parsec.components.organization import OrganizationEraseBadOutcome
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q

_q_erase_organization = Q(
    """
WITH
deleted_organizations AS (
    DELETE FROM organization
    WHERE organization_id = $organization_id
    RETURNING _id
),

deleted_sequester_service AS (
    DELETE FROM sequester_service
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_human AS (
    DELETE FROM human
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_users AS (
    DELETE FROM user_
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_devices AS (
    DELETE FROM device
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_profiles AS (
    DELETE FROM profile
    WHERE user_ IN (SELECT * FROM deleted_users)
    RETURNING _id
),

deleted_invitations AS (
    DELETE FROM invitation
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_realms AS (
    DELETE FROM realm
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_realm_user_roles AS (
    DELETE FROM realm_user_role
    WHERE realm IN (SELECT * FROM deleted_realms)
    RETURNING _id
),

deleted_vlob_atoms AS (
    DELETE FROM vlob_atom
    WHERE realm IN (SELECT * FROM deleted_realms)
    RETURNING _id
),

deleted_blocks AS (
    DELETE FROM block
    WHERE realm IN (SELECT * FROM deleted_realms)
    RETURNING _id, block_id
),

deleted_realm_keys_bundle AS (
    DELETE FROM realm_keys_bundle
    WHERE realm IN (SELECT * FROM deleted_realms)
    RETURNING _id
),

deleted_realm_keys_bundle_access AS (
    DELETE FROM realm_keys_bundle_access
    WHERE realm IN (SELECT * FROM deleted_realms)
    RETURNING _id
),

deleted_realm_sequester_keys_bundle_access AS (
    DELETE FROM realm_sequester_keys_bundle_access
    WHERE realm_keys_bundle IN (SELECT * FROM deleted_realm_keys_bundle)
    RETURNING _id
),

deleted_realm_names AS (
    DELETE FROM realm_name
    WHERE realm IN (SELECT * FROM deleted_realms)
    RETURNING _id
),

deleted_realm_vlob_updates AS (
    DELETE FROM realm_vlob_update
    WHERE realm IN (SELECT * FROM deleted_realms)
    RETURNING _id
),

deleted_block_data AS (
    DELETE FROM block_data
    WHERE organization_id = $organization_id
    RETURNING _id
),

deleted_topics_common AS (
    DELETE FROM common_topic
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_topics_sequester AS (
    DELETE FROM sequester_topic
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_topics_shamir_recovery AS (
    DELETE FROM shamir_recovery_topic
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_topics_realm AS (
    DELETE FROM realm_topic
    WHERE
        organization IN (SELECT * FROM deleted_organizations)
        OR realm IN (SELECT * FROM deleted_realms)
    RETURNING _id
),

deleted_greeting_sessions AS (
    DELETE FROM greeting_session
    WHERE invitation IN (SELECT * FROM deleted_invitations)
    RETURNING _id
),

deleted_greeting_attempts AS (
    DELETE FROM greeting_attempt
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_greeting_steps AS (
    DELETE FROM greeting_step
    WHERE greeting_attempt IN (SELECT * FROM deleted_greeting_attempts)
    RETURNING _id
),

deleted_shamir_recovery_setups AS (
    DELETE FROM shamir_recovery_setup
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
),

deleted_shamir_recovery_shares AS (
    DELETE FROM shamir_recovery_share
    WHERE organization IN (SELECT * FROM deleted_organizations)
    RETURNING _id
)

SELECT (SELECT COUNT(*) FROM deleted_organizations) AS deleted_count
"""
)


async def organization_erase(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
) -> None | OrganizationEraseBadOutcome:
    row = await conn.fetchrow(*_q_erase_organization(organization_id=organization_id.str))
    assert row is not None
    if row["deleted_count"] == 0:
        return OrganizationEraseBadOutcome.ORGANIZATION_NOT_FOUND
