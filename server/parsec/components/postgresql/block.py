# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import assert_never, override

import asyncpg
from asyncpg.exceptions import UniqueViolationError

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceID,
    OrganizationID,
    RealmRole,
    UserProfile,
    VlobID,
)
from parsec.components.block import (
    BaseBlockComponent,
    BlockCreateBadOutcome,
    BlockReadBadOutcome,
)
from parsec.components.blockstore import (
    BaseBlockStoreComponent,
    BlockStoreCreateBadOutcome,
    BlockStoreReadBadOutcome,
)
from parsec.components.organization import Organization, OrganizationGetBadOutcome
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
    q_user_can_read_vlob,
    q_user_can_write_vlob,
    q_user_internal_id,
    transaction,
)
from parsec.components.realm import RealmCheckBadOutcome
from parsec.components.user import CheckDeviceBadOutcome

_q_get_realm_id_from_block_id = Q(
    f"""
SELECT
    { q_realm(_id="block.realm", select="realm.realm_id") }
FROM block
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND block_id = $block_id
"""
)


_q_get_block_meta = Q(
    f"""
SELECT
    deleted_on,
    {
        q_user_can_read_vlob(
            user=q_user_internal_id(
                organization_id="$organization_id",
                user_id="$user_id"
            ),
            realm="block.realm"
        )
    } as has_access
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
    } as has_access,
    EXISTS({
        q_block(
            organization_id="$organization_id",
            block_id="$block_id"
        )
    }) as exists
"""
)


_q_insert_block = Q(
    f"""
INSERT INTO block (organization, block_id, realm, author, size, created_on)
VALUES (
    { q_organization_internal_id("$organization_id") },
    $block_id,
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    { q_device_internal_id(organization_id="$organization_id", device_id="$author") },
    $size,
    $created_on
)
"""
)

_q_get_all_block_meta = Q(
    f"""
SELECT
    block_id,
    { q_realm(_id="realm", select="realm.realm_id") } as realm_id,
    { q_device(_id="author", select="device_id") } as author,
    size,
    created_on
FROM block
WHERE
    organization = { q_organization_internal_id("$organization_id") }
"""
)


# async def _check_realm(
#     conn: asyncpg.Connection,
#     organization_id: OrganizationID,
#     realm_id: VlobID,
#     operation_kind: OperationKind,
# ) -> None:
#     # Fetch the realm status maintenance type
#     try:
#         status = await get_realm_status(conn, organization_id, realm_id)
#     except RealmNotFoundError as exc:
#         raise BlockNotFoundError(*exc.args) from exc

#     # Special case of reading while in reencryption is authorized
#     if operation_kind == OperationKind.DATA_READ and status.in_reencryption:
#         pass

#     # Access is not allowed while in maintenance
#     elif status.in_maintenance:
#         raise BlockInMaintenanceError("Data realm is currently under maintenance")


class PGBlockComponent(BaseBlockComponent):
    def __init__(self, pool: asyncpg.Pool):
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
        conn: asyncpg.Connection,
        organization_id: OrganizationID,
        author: DeviceID,
        block_id: BlockID,
    ) -> bytes | BlockReadBadOutcome:
        match await self.organization._get(conn, organization_id):
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return BlockReadBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization():
                pass
            case unknown:
                assert_never(unknown)

        match await self.user._check_device(conn, organization_id, author):
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return BlockReadBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return BlockReadBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return BlockReadBadOutcome.AUTHOR_NOT_FOUND
            case UserProfile():
                pass
            case unknown:
                assert_never(unknown)

        realm_id_uuid = await conn.fetchval(
            *_q_get_realm_id_from_block_id(organization_id=organization_id.str, block_id=block_id)
        )

        if not realm_id_uuid:
            return BlockReadBadOutcome.BLOCK_NOT_FOUND
        realm_id = VlobID.from_hex(realm_id_uuid)

        match await self.realm._check_realm(conn, organization_id, realm_id, author):
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                assert False, f"Realm not found: {realm_id}"
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return BlockReadBadOutcome.AUTHOR_NOT_ALLOWED
            case (RealmRole(), _):
                pass
            case unknown:
                assert_never(unknown)

        outcome = await self.blockstore.read(organization_id, block_id)
        match outcome:
            case bytes() | bytearray() | memoryview() as block:
                return block
            case BlockStoreReadBadOutcome.BLOCK_NOT_FOUND:
                # Weird, the block exists in the database but not in the blockstore
                return BlockReadBadOutcome.STORE_UNAVAILABLE
            case BlockStoreReadBadOutcome.STORE_UNAVAILABLE:
                return BlockReadBadOutcome.STORE_UNAVAILABLE
            case unknown:
                assert_never(unknown)

    @override
    @transaction
    async def create(
        self,
        conn: asyncpg.Connection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        block_id: BlockID,
        realm_id: VlobID,
        block: bytes,
    ) -> None | BlockCreateBadOutcome:
        match await self.organization._get(conn, organization_id):
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return BlockCreateBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization():
                pass
            case unknown:
                assert_never(unknown)

        match await self.user._check_device(conn, organization_id, author):
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return BlockCreateBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return BlockCreateBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return BlockCreateBadOutcome.AUTHOR_NOT_FOUND
            case UserProfile():
                pass
            case unknown:
                assert_never(unknown)

        match await self.realm._check_realm(conn, organization_id, realm_id, author):
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                return BlockCreateBadOutcome.REALM_NOT_FOUND
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return BlockCreateBadOutcome.AUTHOR_NOT_ALLOWED
            case (RealmRole() as role, _):
                if role not in (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR):
                    return BlockCreateBadOutcome.AUTHOR_NOT_ALLOWED
            case unknown:
                assert_never(unknown)

        # Note it's important to check unicity here because blockstore create
        # overwrite existing data !
        ret = await conn.fetchrow(
            *_q_get_block_write_right_and_unicity(
                organization_id=organization_id.str,
                user_id=author.user_id.str,
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
            case BlockStoreCreateBadOutcome.STORE_UNAVAILABLE:
                return BlockCreateBadOutcome.STORE_UNAVAILABLE
            case None:
                pass
            case unknown:
                assert_never(unknown)

        # 3) Insert the block metadata into the database
        try:
            ret = await conn.execute(
                *_q_insert_block(
                    organization_id=organization_id.str,
                    block_id=block_id,
                    realm_id=realm_id,
                    author=author.str,
                    size=len(block),
                    created_on=now,
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
        self, conn: asyncpg.Connection, organization_id: OrganizationID
    ) -> dict[BlockID, tuple[DateTime, DeviceID, VlobID, int]]:
        ret = await conn.fetch(*_q_get_all_block_meta(organization_id=organization_id.str))

        items = {}
        for item in ret:
            block_id = BlockID.from_hex(item["block_id"])
            items[block_id] = (
                item["created_on"],
                DeviceID(item["author"]),
                VlobID.from_hex(item["realm_id"]),
                int(item["size"]),
            )

        return items


_q_get_block_data = Q(
    """
SELECT
    data
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
    def __init__(self, pool: asyncpg.Pool):
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
