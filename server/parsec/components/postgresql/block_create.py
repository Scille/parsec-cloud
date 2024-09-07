# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from asyncpg.exceptions import UniqueViolationError

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceID,
    OrganizationID,
    VlobID,
)
from parsec.components.block import (
    BlockCreateBadOutcome,
)
from parsec.components.blockstore import (
    BaseBlockStoreComponent,
    BlockStoreCreateBadOutcome,
)
from parsec.components.postgresql import AsyncpgPool
from parsec.components.postgresql.utils import (
    Q,
)
from parsec.components.realm import BadKeyIndex

_q_insert_block = Q(
    """
INSERT INTO block (organization, block_id, realm, author, size, created_on, key_index)
VALUES (
    $organization_internal_id,
    $block_id,
    $realm_internal_id,
    $device_internal_id,
    $size,
    $created_on,
    $key_index
)
"""
)

# `block_create` being performance critical, we rely on a single big query to both
# lock `common`/`realm` topics and fetches everything needed for access checks.
_q_create_fetch_data_and_lock_topics = Q(
    """
WITH my_organization AS (
    SELECT
        _id,
        is_expired
    FROM organization
    WHERE
        organization_id = $organization_id
        -- Only consider bootstrapped organizations
        AND root_verify_key IS NOT NULL
    LIMIT 1
),
-- Common topic lock must occur ASAP
my_locked_common_topic AS (
    SELECT last_timestamp
    FROM common_topic
    WHERE organization = (SELECT _id FROM my_organization)
    LIMIT 1
    FOR SHARE
),
my_realm AS (
    SELECT
        realm._id,
        key_index
    FROM realm
    WHERE
        organization = (SELECT _id FROM my_organization)
        AND realm_id = $realm_id
    LIMIT 1
),
-- Realm topic lock must occur ASAP
my_locked_realm_topic AS (
    SELECT last_timestamp
    FROM realm_topic
    WHERE
        organization = (SELECT _id FROM my_organization)
        AND realm = (SELECT _id FROM my_realm)
    LIMIT 1
    FOR SHARE
),
my_device AS (
    SELECT
        device._id,
        device.user_
    FROM device
    WHERE
        organization = (SELECT _id FROM my_organization)
        AND device.device_id = $device_id
    LIMIT 1
),
my_user AS (
    SELECT
        _id,
        frozen,
        (revoked_on IS NOT NULL) AS revoked
    FROM user_
    WHERE _id = (SELECT user_ FROM my_device)
    LIMIT 1
)
SELECT
    (SELECT _id FROM my_organization) AS organization_internal_id,
    (SELECT is_expired FROM my_organization) AS organization_is_expired,
    (SELECT _id FROM my_device) AS device_internal_id,
    (SELECT frozen FROM my_user) AS user_is_frozen,
    (SELECT revoked FROM my_user) AS user_is_revoked,
    (SELECT last_timestamp FROM my_locked_common_topic) AS last_common_certificate_timestamp,
    (SELECT last_timestamp FROM my_locked_realm_topic) AS last_realm_certificate_timestamp,
    (SELECT _id FROM my_realm) AS realm_internal_id,
    (SELECT key_index FROM my_realm) AS realm_key_index,
    COALESCE(
        (
            SELECT role IN ('CONTRIBUTOR', 'MANAGER', 'OWNER')
            FROM realm_user_role
            WHERE
                realm_user_role.user_ = (SELECT _id FROM my_user)
                AND realm_user_role.realm = (SELECT _id FROM my_realm)
            ORDER BY certified_on DESC
            LIMIT 1
        ),
        False
    ) AS user_can_write,
    EXISTS(
        SELECT True
        FROM block
        WHERE
            block.realm = (SELECT _id FROM my_realm)
            AND block.block_id = $block_id
        LIMIT 1
    ) AS block_already_exists
"""
)


async def block_create(
    blockstore: BaseBlockStoreComponent,
    pool: AsyncpgPool,
    now: DateTime,
    organization_id: OrganizationID,
    author: DeviceID,
    realm_id: VlobID,
    block_id: BlockID,
    key_index: int,
    block: bytes,
) -> None | BadKeyIndex | BlockCreateBadOutcome:
    # Given block metadata and block data are stored on different storages,
    # being atomic is not easy here :(
    #
    # Hence there is three distincts steps here:
    # 1) Fetch info from PostgreSQL to do access control & ensure the block doesn't
    #    already exists.
    # 2) Store the block in the blockstore (e.g. S3).
    # 3) Store the block in PostgreSQL.
    #
    # Importantly, we shouldn't keep topics lock during step 2:
    # - Step 2 can take a long time (e.g. with a RAID blockstore configuration).
    # - In case of PostgreSQL blockstore (only used for testing), this can create
    #   a deadlock in case of too many concurrent `block_create` given the
    #   blockstore is waiting on the PostgreSQL connection pool.
    #
    # On top of that, the blocks only have meaning as part of a file manifest (i.e. a
    # vlob from the server point of view), which is the one having timestamp.
    # Hence the the blocks have no concept of creation date (from the client points
    # point of view at least, as we store an insertion date in the database which
    # is a useful info for database admin).
    # This means a block inserted after its author is revoked (or lose his write
    # access to the realm) won't break causality.
    #
    # For those reasons, we ensure at one point in time T during step 1 the author had
    # the right to create the block, then do the insertion and pretend the step 3's
    # block insert in PostgreSQL also occurred exactly at time T.
    #
    # Notes:
    # - Concurrency is handled in step 3 given block has a unique constraint
    #   on organization + realm + block ID in PostgreSQL.
    # - Step 2 can be successful (or can be successful on *some* blockstores in case
    #   of a RAID blockstores configuration) but step 3 fails.
    #   This is solved by the fact blockstores follows eventual consistency (i.e. last
    #   write overwrite the previous ones) and two create operations with the same
    #   orgID/ID couple are expected to have the same block data.
    # - A possible performance optimization would be to keep result of step 1 in
    #   cache (this would also work for `vlob_read` which then wouldn't even have
    #   to query PostgreSQL !).

    # 1) Query the database to get all info about org/device/user/realm/block

    # Note the topics locked in this query that are going to be released right away
    # (since the PostgreSQL connection is released right after the query is done).
    # We keep it this way nevertheless (at least for now) to stay consistent with
    # the rest of the codebase and to simplify handling of concurrent insertions
    # of common & realm certificates.
    async with pool.acquire() as conn:
        row = await conn.fetchrow(
            *_q_create_fetch_data_and_lock_topics(
                organization_id=organization_id.str,
                device_id=author,
                realm_id=realm_id,
                block_id=block_id,
            )
        )
    assert row is not None

    # 1.1) Check organization

    match row["organization_internal_id"]:
        case int() as organization_internal_id:
            pass
        case None:
            return BlockCreateBadOutcome.ORGANIZATION_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    match row["organization_is_expired"]:
        case False:
            pass
        case True:
            return BlockCreateBadOutcome.ORGANIZATION_EXPIRED
        case unknown:
            assert False, repr(unknown)

    # 1.2) Check device & user

    match row["device_internal_id"]:
        case int() as device_internal_id:
            pass
        case None:
            return BlockCreateBadOutcome.AUTHOR_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    # Since device exists, it corresponding user must also exist

    match row["user_is_frozen"]:
        case False:
            pass
        case True:
            return BlockCreateBadOutcome.AUTHOR_REVOKED
        case unknown:
            assert False, repr(unknown)

    match row["user_is_revoked"]:
        case False:
            pass
        case True:
            return BlockCreateBadOutcome.AUTHOR_REVOKED
        case unknown:
            assert False, repr(unknown)

    # 1.3) Check topics

    match row["last_common_certificate_timestamp"]:
        case DateTime():
            pass
        case unknown:
            assert False, repr(unknown)

    match row["last_realm_certificate_timestamp"]:
        case DateTime() as last_realm_certificate_timestamp:
            pass
        case None:
            return BlockCreateBadOutcome.REALM_NOT_FOUND
        case unknown:
            assert False, repr(unknown)

    # 1.4) Check realm
    # (Note since realm's topic exists, the realm itself must also exist)

    match row["realm_internal_id"]:
        case int() as realm_internal_id:
            pass
        case unknown:
            assert False, repr(unknown)

    match row["realm_key_index"]:
        case int() as realm_current_key_index:
            if realm_current_key_index != key_index:
                return BadKeyIndex(
                    last_realm_certificate_timestamp=last_realm_certificate_timestamp,
                )
            pass
        case unknown:
            assert False, repr(unknown)

    match row["user_can_write"]:
        case True:
            pass
        case False:
            return BlockCreateBadOutcome.AUTHOR_NOT_ALLOWED
        case unknown:
            assert False, repr(unknown)

    # 1.5) Check block

    match row["block_already_exists"]:
        case False:
            pass
        case True:
            return BlockCreateBadOutcome.BLOCK_ALREADY_EXISTS
        case unknown:
            assert False, repr(unknown)

    # 2) Upload block data in blockstore

    match await blockstore.create(organization_id, block_id, block):
        case None:
            pass
        case BlockStoreCreateBadOutcome.STORE_UNAVAILABLE:
            return BlockCreateBadOutcome.STORE_UNAVAILABLE

    # 3) Insert the block metadata into the database

    # No need for explicit transaction here since we use this session for a single query
    async with pool.acquire() as conn:
        try:
            ret = await conn.execute(
                *_q_insert_block(
                    organization_internal_id=organization_internal_id,
                    block_id=block_id,
                    realm_internal_id=realm_internal_id,
                    device_internal_id=device_internal_id,
                    size=len(block),
                    created_on=now,
                    key_index=key_index,
                )
            )
        except UniqueViolationError:
            # Given concurrent block creation is allowed, unique violation may occur !
            return BlockCreateBadOutcome.BLOCK_ALREADY_EXISTS

        else:
            assert ret == "INSERT 0 1", f"Insertion error: {ret}"
