from uuid import UUID
from typing import Tuple

from parsec.types import DeviceID, OrganizationID
from parsec.api.protocole import blockstore_create_serializer, blockstore_read_serializer
from parsec.backend.config import BaseBlockstoreConfig
from parsec.backend.utils import catch_protocole_errors


class BlockstoreError(Exception):
    pass


class BlockstoreAlreadyExistsError(BlockstoreError):
    pass


class BlockstoreNotFoundError(BlockstoreError):
    pass


class BlockstoreTimeoutError(BlockstoreError):
    pass


class BaseBlockstoreComponent:
    @catch_protocole_errors
    async def api_blockstore_read(self, client_ctx, msg):
        msg = blockstore_read_serializer.req_load(msg)

        try:
            block = await self.read(client_ctx.organization_id, msg["id"])

        except BlockstoreNotFoundError:
            return blockstore_read_serializer.rep_dump({"status": "not_found"})

        except BlockstoreTimeoutError:
            return blockstore_read_serializer.rep_dump({"status": "timeout"})

        return blockstore_read_serializer.rep_dump({"status": "ok", "block": block})

    @catch_protocole_errors
    async def api_blockstore_create(self, client_ctx, msg):
        msg = blockstore_create_serializer.req_load(msg)

        try:
            await self.create(client_ctx.organization_id, **msg, author=client_ctx.device_id)

        except BlockstoreAlreadyExistsError:
            return blockstore_read_serializer.rep_dump({"status": "already_exists"})

        except BlockstoreTimeoutError:
            return blockstore_read_serializer.rep_dump({"status": "timeout"})

        return blockstore_create_serializer.rep_dump({"status": "ok"})

    async def read(self, organization_id: OrganizationID, id: UUID) -> Tuple[bytes, DeviceID]:
        """
        Raises:
            BlockstoreNotFoundError
            BlockstoreTimeoutError
        """
        raise NotImplementedError()

    async def create(
        self, organization_id: OrganizationID, id: UUID, block: bytes, author: DeviceID
    ) -> None:
        """
        Raises:
            BlockstoreAlreadyExistsError
            BlockstoreTimeoutError
        """
        raise NotImplementedError()


def blockstore_factory(
    config: BaseBlockstoreConfig, postgresql_dbh=None
) -> BaseBlockstoreComponent:
    if config.type == "MOCKED":
        from parsec.backend.drivers.memory import MemoryBlockstoreComponent

        return MemoryBlockstoreComponent()

    elif config.type == "POSTGRESQL":
        from parsec.backend.drivers.postgresql import PGBlockstoreComponent

        if not postgresql_dbh:
            raise ValueError("PostgreSQL blockstore is not available")
        return PGBlockstoreComponent(postgresql_dbh)

    elif config.type == "S3":
        try:
            from parsec.backend.s3_blockstore import S3BlockstoreComponent

            return S3BlockstoreComponent(
                config.s3_region, config.s3_bucket, config.s3_key, config.s3_secret
            )
        except ImportError as exc:
            raise ValueError("S3 blockstore is not available") from exc

    elif config.type == "SWIFT":
        try:
            from parsec.backend.swift_blockstore import SwiftBlockstoreComponent

            return SwiftBlockstoreComponent(
                config.swift_authurl,
                config.swift_tenant,
                config.swift_container,
                config.swift_user,
                config.swift_password,
            )
        except ImportError as exc:
            raise ValueError("Swift blockstore is not available") from exc

    elif config.type == "RAID1":
        from parsec.backend.raid1_blockstore import RAID1BlockstoreComponent

        blockstores = [
            blockstore_factory(subconf, postgresql_dbh) for subconf in config.blockstores
        ]

        return RAID1BlockstoreComponent(blockstores)

    elif config.type == "RAID0":
        from parsec.backend.raid0_blockstore import RAID0BlockstoreComponent

        blockstores = [
            blockstore_factory(subconf, postgresql_dbh) for subconf in config.blockstores
        ]

        return RAID0BlockstoreComponent(blockstores)

    else:
        raise ValueError(f"Unknown blockstore type `{config.type}`")
