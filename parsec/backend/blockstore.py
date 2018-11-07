from parsec.backend.exceptions import AlreadyExistsError
from parsec.utils import to_jsonb64
from parsec.schema import BaseCmdSchema, fields


class _cmd_GET_Schema(BaseCmdSchema):
    id = fields.UUID(required=True)


cmd_GET_Schema = _cmd_GET_Schema()


class _cmd_POST_Schema(BaseCmdSchema):
    id = fields.UUID(required=True)
    block = fields.Base64Bytes(required=True)


cmd_POST_Schema = _cmd_POST_Schema()


class BaseBlockStoreComponent:
    async def api_blockstore_get(self, client_ctx, msg):
        msg = cmd_GET_Schema.load_or_abort(msg)
        block = await self.get(msg["id"])
        return {"status": "ok", "block": to_jsonb64(block)}

    async def api_blockstore_post(self, client_ctx, msg):
        msg = cmd_POST_Schema.load_or_abort(msg)
        try:
            await self.post(**msg)
        except AlreadyExistsError:
            pass
        return {"status": "ok"}

    async def get(self, id):
        """
        Raises:
            NotFoundError
        """
        raise NotImplementedError()

    async def post(self, id, block):
        """
        Raises:
            AlreadyExistsError
        """
        raise NotImplementedError()


def blockstore_factory(config, postgresql_dbh=None):
    return _blockstore_factory(config.blockstore_config, postgresql_dbh)


def _blockstore_factory(config, postgresql_dbh):
    if config.type == "MOCKED":
        from parsec.backend.drivers.memory import MemoryBlockStoreComponent

        return MemoryBlockStoreComponent()

    elif config.type == "POSTGRESQL":
        from parsec.backend.drivers.postgresql import PGBlockStoreComponent

        if not postgresql_dbh:
            raise ValueError("PostgreSQL blockstore is not available")
        return PGBlockStoreComponent(postgresql_dbh)

    elif config.type == "S3":
        try:
            from parsec.backend.s3_blockstore import S3BlockStoreComponent

            return S3BlockStoreComponent(
                config.s3_region, config.s3_bucket, config.s3_key, config.s3_secret
            )
        except ImportError as exc:
            raise ValueError("S3 blockstore is not available") from exc

    elif config.type == "SWIFT":
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
            raise ValueError("Swift blockstore is not available") from exc

    elif config.type == "RAID1":
        from parsec.backend.raid1_blockstore import RAID1BlockStoreComponent

        blockstores = [
            _blockstore_factory(subconf, postgresql_dbh) for subconf in config.blockstores
        ]

        return RAID1BlockStoreComponent(blockstores)

    else:
        raise ValueError(f"Unknown blockstore type `{config.type}`")
