# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from parsec.api.protocol import OrganizationID, BlockID
from parsec.backend.config import (
    BaseBlockStoreConfig,
    RAID0BlockStoreConfig,
    RAID1BlockStoreConfig,
    RAID5BlockStoreConfig,
    S3BlockStoreConfig,
    SWIFTBlockStoreConfig,
    PostgreSQLBlockStoreConfig,
    MockedBlockStoreConfig,
)


class BaseBlockStoreComponent:
    """
    BlockStoreComponent wraps a distributed object storage service, distributed implies
    there is no atomicity or object locking capability (that's how AWS S3 works).

    So for instance two consecutive `create` calls with the same orgID/ID couple can
    both succeed (and eventual consistency will later converge on a single truth).

    Hence it is the responsibility of the BlockComponent (which itself drives the
    BlockStoreComponent) to handle unicity and atomicity (typically using PostgreSQL
    transactions).

    The key takeaways here:
    - `BlockStoreComponent.create` must be implemented in an idempotent way
      (making the assumption multiple creates with the same orgID/ID couple
      always comes with the same block data).
    - BlockStoreComponent never raises business logic errors: for instance if a
      `BlockStoreComponent.read` raises a not found error, it shows the underlying
      storage is faulty (given `BlockComponent` has already checked the orgID/ID
      couple exists)
    - Each BlockStoreComponent should log any error of it underlying storage, as it
      most likely indicates some manual maintenance operation is required.
    - An object storage such as S3 has no overwrite protection, hence in case of a
    partially failed create operation, all object storages will write the new block data
    (and not only the ones that failed the first time)
    """

    async def read(self, organization_id: OrganizationID, block_id: BlockID) -> bytes:
        """
        Raises:
            BlockStoreError
        """
        raise NotImplementedError()

    async def create(
        self, organization_id: OrganizationID, block_id: BlockID, block: bytes
    ) -> None:
        """
        Raises:
            BlockStoreError
        """
        raise NotImplementedError()


def blockstore_factory(
    config: BaseBlockStoreConfig, postgresql_dbh=None
) -> BaseBlockStoreComponent:
    if isinstance(config, MockedBlockStoreConfig):
        from parsec.backend.memory import MemoryBlockStoreComponent

        return MemoryBlockStoreComponent()

    elif isinstance(config, PostgreSQLBlockStoreConfig):
        from parsec.backend.postgresql import PGBlockStoreComponent

        if not postgresql_dbh:
            raise ValueError("PostgreSQL block store is not available")
        return PGBlockStoreComponent(postgresql_dbh)

    elif isinstance(config, S3BlockStoreConfig):
        try:
            from parsec.backend.s3_blockstore import S3BlockStoreComponent

            return S3BlockStoreComponent(
                config.s3_region,
                config.s3_bucket,
                config.s3_key,
                config.s3_secret,
                config.s3_endpoint_url,
            )
        except ImportError as exc:
            raise ValueError("S3 block store is not available") from exc

    elif isinstance(config, SWIFTBlockStoreConfig):
        try:
            from parsec.backend.swift_blockstore import SwiftBlockStoreComponent

            return SwiftBlockStoreComponent(
                config.swift_authurl,
                config.swift_tenant,
                config.swift_container,
                config.swift_user,
                config.swift_password,
            )
        except ImportError as exc:
            raise ValueError("Swift block store is not available") from exc

    elif isinstance(config, RAID1BlockStoreConfig):
        from parsec.backend.raid1_blockstore import RAID1BlockStoreComponent

        blocks = [blockstore_factory(subconf, postgresql_dbh) for subconf in config.blockstores]

        return RAID1BlockStoreComponent(blocks, partial_create_ok=config.partial_create_ok)

    elif isinstance(config, RAID0BlockStoreConfig):
        from parsec.backend.raid0_blockstore import RAID0BlockStoreComponent

        blocks = [blockstore_factory(subconf, postgresql_dbh) for subconf in config.blockstores]

        return RAID0BlockStoreComponent(blocks)

    elif isinstance(config, RAID5BlockStoreConfig):
        from parsec.backend.raid5_blockstore import RAID5BlockStoreComponent

        if len(config.blockstores) < 3:
            raise ValueError(f"RAID5 block store needs at least 3 nodes")

        blocks = [blockstore_factory(subconf, postgresql_dbh) for subconf in config.blockstores]

        return RAID5BlockStoreComponent(blocks, partial_create_ok=config.partial_create_ok)

    else:
        raise ValueError(f"Unknown block store configuration `{config}`")
