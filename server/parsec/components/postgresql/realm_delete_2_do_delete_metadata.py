# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    DateTime,
    OrganizationID,
    VlobID,
)
from parsec.components.postgresql import AsyncpgConnection
from parsec.components.postgresql.utils import Q
from parsec.components.realm import (
    RealmDelete2DoDeleteMetadataBadOutcome,
)

_q_get_realm_info = Q("""
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
    SELECT
        _id,
        status
    FROM realm
    WHERE
        organization = (SELECT my_organization._id FROM my_organization)
        AND realm_id = $realm_id
    LIMIT 1
),

my_last_archiving AS (
    SELECT
        configuration,
        deletion_date
    FROM realm_archiving
    WHERE realm = (SELECT my_realm._id FROM my_realm)
    ORDER BY _id DESC
    LIMIT 1
),

my_current_roles AS (
    SELECT DISTINCT ON (user_)
        user_,
        role
    FROM realm_user_role
    WHERE realm = (SELECT my_realm._id FROM my_realm)
    ORDER BY user_ ASC, certified_on DESC
)

SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT _id FROM my_realm) AS realm_internal_id,
    (SELECT status FROM my_realm) AS realm_status,
    (SELECT configuration FROM my_last_archiving) AS last_archiving_configuration,
    (SELECT deletion_date FROM my_last_archiving) AS last_archiving_deletion_date,
    -- A realm is orphaned if all users with a current non-NULL role are revoked.
    (
        NOT EXISTS (
            SELECT 1
            FROM my_current_roles
            INNER JOIN user_ ON my_current_roles.user_ = user_._id
            WHERE
                my_current_roles.role IS NOT NULL
                AND user_.revoked_on IS NULL
        )
    ) AS is_orphaned
""")


_q_do_delete = Q("""
-- Note we don't delete data from the `block_data` table.
-- This for three reasons:
-- - This table simulates an object storage and is only used for tests.
-- - Object storage are supposed to be cleaned in a separated operation.
-- - This table is isolated from the others (e.g. it doesn't have a `realm`
--   column) which makes cleaning it complex and slow.
WITH
delete_blocks AS (
    DELETE FROM block
    WHERE realm = $realm_internal_id
),

delete_realm_vlob_updates AS (
    DELETE FROM realm_vlob_update
    WHERE realm = $realm_internal_id
),

delete_vlob_atoms AS (
    DELETE FROM vlob_atom
    WHERE realm = $realm_internal_id
),

delete_realm_sequester_keys_bundle_access AS (
    DELETE FROM realm_sequester_keys_bundle_access
    WHERE realm = $realm_internal_id
),

delete_realm_keys_bundle_access AS (
    DELETE FROM realm_keys_bundle_access
    WHERE realm = $realm_internal_id
),

clear_realm_keys_bundle AS (
    UPDATE realm_keys_bundle
    SET keys_bundle = ''
    WHERE realm = $realm_internal_id
)

UPDATE realm
SET status = 'DELETED'
WHERE _id = $realm_internal_id
""")


async def realm_delete_2_do_delete_metadata(
    conn: AsyncpgConnection,
    organization_id: OrganizationID,
    realm_id: VlobID,
    now: DateTime,
) -> None | RealmDelete2DoDeleteMetadataBadOutcome:
    row = await conn.fetchrow(
        *_q_get_realm_info(organization_id=organization_id.str, realm_id=realm_id)
    )
    assert row is not None

    match row["organization_internal_id"]:
        case int():
            pass
        case None:
            return RealmDelete2DoDeleteMetadataBadOutcome.ORGANIZATION_NOT_FOUND
        case _:
            assert False, row

    match row["realm_internal_id"]:
        case int() as realm_internal_id:
            pass
        case None:
            return RealmDelete2DoDeleteMetadataBadOutcome.REALM_NOT_FOUND
        case _:
            assert False, row

    match row["realm_status"]:
        case "DELETED":
            return RealmDelete2DoDeleteMetadataBadOutcome.REALM_ALREADY_DELETED
        case str():
            pass
        case _:
            assert False, row

    match row["is_orphaned"]:
        case bool() as is_orphaned:
            pass
        case _:
            assert False, row

    # Check if the realm qualifies for deletion:
    # 1) orphaned realm (all members revoked), or
    # 2) deletion planned with date reached

    if not is_orphaned:
        # Not orphaned, check for planned deletion
        match row["last_archiving_configuration"]:
            case "DELETION_PLANNED":
                pass
            case _:
                return (
                    RealmDelete2DoDeleteMetadataBadOutcome.REALM_NOT_ORPHANED_NOR_DELETION_PLANNED
                )

        match row["last_archiving_deletion_date"]:
            case DateTime() as deletion_date:
                pass
            case _:
                assert False, row

        if deletion_date > now:
            return RealmDelete2DoDeleteMetadataBadOutcome.REALM_DELETION_DATE_NOT_REACHED

    # All checks passed, delete realm data and mark it as deleted
    await conn.execute(*_q_do_delete(realm_internal_id=realm_internal_id))
