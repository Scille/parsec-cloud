# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from parsec.api.protocol import (
    OrganizationID,
    DeviceID,
    RealmID,
    BlockID,
    block_create_serializer,
    block_read_serializer,
)
from parsec.backend.utils import catch_protocol_errors, api


class BlockError(Exception):
    pass


class BlockAlreadyExistsError(BlockError):
    pass


class BlockNotFoundError(BlockError):
    pass


class BlockTimeoutError(BlockError):
    pass


class BlockAccessError(BlockError):
    pass


class BlockInMaintenanceError(BlockError):
    pass


class BaseBlockComponent:
    @api("block_read")
    @catch_protocol_errors
    async def api_block_read(self, client_ctx, msg):
        msg = block_read_serializer.req_load(msg)

        try:
            block = await self.read(client_ctx.organization_id, client_ctx.device_id, **msg)

        except BlockNotFoundError:
            return block_read_serializer.rep_dump({"status": "not_found"})

        except BlockTimeoutError:
            return block_read_serializer.rep_dump({"status": "timeout"})

        except BlockAccessError:
            return block_read_serializer.rep_dump({"status": "not_allowed"})

        except BlockInMaintenanceError:
            return block_read_serializer.rep_dump({"status": "in_maintenance"})

        return block_read_serializer.rep_dump({"status": "ok", "block": block})

    @api("block_create")
    @catch_protocol_errors
    async def api_block_create(self, client_ctx, msg):
        msg = block_create_serializer.req_load(msg)

        try:
            await self.create(client_ctx.organization_id, client_ctx.device_id, **msg)

        except BlockAlreadyExistsError:
            return block_create_serializer.rep_dump({"status": "already_exists"})

        except BlockNotFoundError:
            return block_create_serializer.rep_dump({"status": "not_found"})

        except BlockTimeoutError:
            return block_create_serializer.rep_dump({"status": "timeout"})

        except BlockAccessError:
            return block_create_serializer.rep_dump({"status": "not_allowed"})

        except BlockInMaintenanceError:
            return block_create_serializer.rep_dump({"status": "in_maintenance"})

        return block_create_serializer.rep_dump({"status": "ok"})

    async def read(
        self, organization_id: OrganizationID, author: DeviceID, block_id: BlockID
    ) -> bytes:
        """
        Raises:
            BlockNotFoundError
            BlockTimeoutError
            BlockAccessError
            BlockInMaintenanceError
        """
        raise NotImplementedError()

    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        block_id: BlockID,
        realm_id: RealmID,
        block: bytes,
    ) -> None:
        """
        Raises:
            BlockNotFoundError: if cannot found realm
            BlockAlreadyExistsError
            BlockTimeoutError
            BlockAccessError
            BlockInMaintenanceError
        """
        raise NotImplementedError()
