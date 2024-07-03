# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from asyncpg.exceptions import UniqueViolationError

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceID,
    OrganizationID,
    RealmRole,
    VlobID,
)
from parsec.components.block import (
    BaseBlockComponent,
    BlockCreateBadOutcome,
    BlockReadBadOutcome,
    BlockReadResult,
)
from parsec.components.blockstore import (
    BaseBlockStoreComponent,
    BlockStoreCreateBadOutcome,
    BlockStoreReadBadOutcome,
)
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.realm import PGRealmComponent
from parsec.components.postgresql.user import PGUserComponent
from parsec.components.postgresql.utils import (
    Q,
    q_block,
    q_device,
    q_device_internal_id,
    q_organization_internal_id,
    q_realm,
    q_realm_internal_id,
    q_user_internal_id,
    transaction,
)
from parsec.components.realm import BadKeyIndex, RealmCheckBadOutcome
from parsec.components.user import CheckDeviceBadOutcome


def q_user_can_write_vlob(
    user: str | None = None,
    user_id: str | None = None,
    realm: str | None = None,
    realm_id: str | None = None,
    organization: str | None = None,
    organization_id: str | None = None,
) -> str:
    if user is None:
        assert organization_id is not None and user_id is not None
        _q_user = q_user_internal_id(
            organization=organization, organization_id=organization_id, user_id=user_id
        )
    else:
        _q_user = user

    if realm is None:
        assert organization_id is not None and realm_id is not None
        _q_realm = q_realm_internal_id(
            organization=organization, organization_id=organization_id, realm_id=realm_id
        )
    else:
        _q_realm = realm

    return f"""
COALESCE(
    (
        SELECT realm_user_role.role IN ('CONTRIBUTOR', 'MANAGER', 'OWNER')
        FROM realm_user_role
        WHERE
            realm_user_role.realm = { _q_realm }
            AND realm_user_role.user_ = { _q_user }
        ORDER BY certified_on DESC
        LIMIT 1
    ),
    False
)
"""


_q_get_block_info = Q(
    f"""
SELECT
    { q_realm(_id="block.realm", select="realm.realm_id") },
    key_index
FROM block
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND block_id = $block_id
"""
)

_q_get_block_write_right_and_unicity = Q(
    f"""
SELECT
    {
        q_user_can_write_vlob(
            organization_id="$organization_id",
            user_id="$user_id",
            realm_id="$realm_id"
        )
    } AS has_access,
    EXISTS({
        q_block(
            organization_id="$organization_id",
            block_id="$block_id"
        )
    }) AS exists
"""
)

_q_insert_block = Q(
    f"""
INSERT INTO block (organization, block_id, realm, author, size, created_on, key_index)
VALUES (
    { q_organization_internal_id("$organization_id") },
    $block_id,
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    { q_device_internal_id(organization_id="$organization_id", device_id="$author") },
    $size,
    $created_on,
    $key_index
)
"""
)

_q_get_all_block_meta = Q(
    f"""
SELECT
    block_id,
    { q_realm(_id="realm", select="realm.realm_id") } AS realm_id,
    { q_device(_id="author", select="device_id") } AS author,
    size,
    created_on,
    key_index
FROM block
WHERE
    organization = { q_organization_internal_id("$organization_id") }
"""
)


class PGBlockComponent(BaseBlockComponent):
    def __init__(self, pool: AsyncpgPool):
        self.pool = pool
        self.blockstore: BaseBlockStoreComponent
        self.user: PGUserComponent
        self.organization: PGOrganizationComponent
        self.realm: PGRealmComponent

    def register_components(
        self,
        blockstore: BaseBlockStoreComponent,
        user: PGUserComponent,
        organization: PGOrganizationComponent,
        realm: PGRealmComponent,
        **kwargs: object,
    ) -> None:
        self.blockstore = blockstore
        self.user = user
        self.organization = organization
        self.realm = realm

    @override
    @transaction
    async def read(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
        block_id: BlockID,
    ) -> BlockReadResult | BlockReadBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization():
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return BlockReadBadOutcome.ORGANIZATION_NOT_FOUND

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return BlockReadBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return BlockReadBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return BlockReadBadOutcome.AUTHOR_NOT_FOUND

        row = await conn.fetchrow(
            *_q_get_block_info(organization_id=organization_id.str, block_id=block_id)
        )

        if row is None:
            return BlockReadBadOutcome.BLOCK_NOT_FOUND
        realm_id = VlobID.from_hex(row["realm_id"])
        key_index = row["key_index"]

        match await self.realm._check_realm_topic(conn, organization_id, realm_id, author_user_id):
            case (_, _, last_realm_certificate_timestamp):
                pass
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                assert False, f"Realm not found: {realm_id}"
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return BlockReadBadOutcome.AUTHOR_NOT_ALLOWED

        outcome = await self.blockstore.read(organization_id, block_id)
        match outcome:
            case bytes() | bytearray() | memoryview() as block:
                return BlockReadResult(
                    block=block,
                    key_index=key_index,
                    needed_realm_certificate_timestamp=last_realm_certificate_timestamp,
                )
            case BlockStoreReadBadOutcome.BLOCK_NOT_FOUND:
                # Weird, the block exists in the database but not in the blockstore
                return BlockReadBadOutcome.STORE_UNAVAILABLE
            case BlockStoreReadBadOutcome.STORE_UNAVAILABLE:
                return BlockReadBadOutcome.STORE_UNAVAILABLE

    @override
    @transaction
    async def create(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        block_id: BlockID,
        key_index: int,
        block: bytes,
    ) -> None | BadKeyIndex | BlockCreateBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization():
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return BlockCreateBadOutcome.ORGANIZATION_NOT_FOUND

        match await self.user._check_device(conn, organization_id, author):
            case (author_user_id, _, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return BlockCreateBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return BlockCreateBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return BlockCreateBadOutcome.AUTHOR_NOT_FOUND

        match await self.realm._check_realm_topic(conn, organization_id, realm_id, author_user_id):
            case (role, current_key_index, last_realm_certificate_timestamp):
                if role not in (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR):
                    return BlockCreateBadOutcome.AUTHOR_NOT_ALLOWED
                if current_key_index != key_index:
                    return BadKeyIndex(
                        last_realm_certificate_timestamp=last_realm_certificate_timestamp,
                    )
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                return BlockCreateBadOutcome.REALM_NOT_FOUND
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return BlockCreateBadOutcome.AUTHOR_NOT_ALLOWED

        # Note it's important to check unicity here because blockstore create
        # overwrites existing data!
        ret = await conn.fetchrow(
            *_q_get_block_write_right_and_unicity(
                organization_id=organization_id.str,
                user_id=author_user_id,
                realm_id=realm_id,
                block_id=block_id,
            )
        )

        # TODO: this check is performed twice, which one do we want to remove?
        if ret is None or not ret["has_access"]:
            return BlockCreateBadOutcome.AUTHOR_NOT_ALLOWED

        elif ret["exists"]:
            return BlockCreateBadOutcome.BLOCK_ALREADY_EXISTS

        # 2) Upload block data in blockstore under an arbitrary id
        # Given block metadata and block data are stored on different storages,
        # being atomic is not easy here :(
        # For instance step 2) can be successful (or can be successful on *some*
        # blockstores in case of a RAID blockstores configuration) but step 4) fails.
        # This is solved by the fact blockstores are considered idempotent and two
        # create operations with the same orgID/ID couple are expected to have the
        # same block data.
        # Hence any blockstore create failure result in postgres transaction
        # cancellation, and blockstore create success can be overwritten by another
        # create in case the postgres transaction was cancelled in step 3)
        match await self.blockstore.create(organization_id, block_id, block):
            case None:
                pass
            case BlockStoreCreateBadOutcome.STORE_UNAVAILABLE:
                return BlockCreateBadOutcome.STORE_UNAVAILABLE

        # 3) Insert the block metadata into the database
        try:
            ret = await conn.execute(
                *_q_insert_block(
                    organization_id=organization_id.str,
                    block_id=block_id,
                    realm_id=realm_id,
                    author=author,
                    size=len(block),
                    created_on=now,
                    key_index=key_index,
                )
            )
        except UniqueViolationError:
            # Given step 1) hasn't locked anything in the database, concurrent block
            # create operations may end up with one of them in unique violation error
            return BlockCreateBadOutcome.BLOCK_ALREADY_EXISTS

        else:
            assert ret == "INSERT 0 1", f"Insertion error: {ret}"

    @override
    @transaction
    async def test_dump_blocks(
        self, conn: AsyncpgConnection, organization_id: OrganizationID
    ) -> dict[BlockID, tuple[DateTime, DeviceID, VlobID, int, int]]:
        rows = await conn.fetch(*_q_get_all_block_meta(organization_id=organization_id.str))

        items = {}
        for row in rows:
            block_id = BlockID.from_hex(row["block_id"])
            items[block_id] = (
                row["created_on"],
                DeviceID.from_hex(row["author"]),
                VlobID.from_hex(row["realm_id"]),
                int(row["key_index"]),
                int(row["size"]),
            )

        return items


_q_get_block_data = Q(
    """
SELECT data
FROM block_data
WHERE
    organization_id = $organization_id
    AND block_id = $block_id
"""
)

_q_insert_block_data = Q(
    """
INSERT INTO block_data (organization_id, block_id, data)
VALUES ($organization_id, $block_id, $data)
"""
)


class PGBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, pool: AsyncpgPool):
        self.pool = pool

    async def read(
        self, organization_id: OrganizationID, block_id: BlockID
    ) -> bytes | BlockStoreReadBadOutcome:
        async with self.pool.acquire() as conn:
            ret = await conn.fetchrow(
                *_q_get_block_data(organization_id=organization_id.str, block_id=block_id)
            )
            if not ret:
                return BlockStoreReadBadOutcome.BLOCK_NOT_FOUND

            return ret[0]

    async def create(
        self, organization_id: OrganizationID, block_id: BlockID, block: bytes
    ) -> None | BlockStoreCreateBadOutcome:
        async with self.pool.acquire() as conn:
            try:
                ret = await conn.execute(
                    *_q_insert_block_data(
                        organization_id=organization_id.str, block_id=block_id, data=block
                    )
                )
                if ret != "INSERT 0 1":
                    return BlockStoreCreateBadOutcome.STORE_UNAVAILABLE

            except UniqueViolationError:
                # Keep calm and stay idempotent
                pass
