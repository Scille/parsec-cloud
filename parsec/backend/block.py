# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from parsec.api.protocol import (
    OrganizationID,
    DeviceID,
    RealmID,
    BlockID,
    BlockReadReq,
    BlockReadRep,
    BlockCreateReq,
    BlockCreateRep,
    api_typed_msg_adapter,
)
from parsec.backend.utils import catch_protocol_errors, api


class BlockError(Exception):
    pass


class BlockAlreadyExistsError(BlockError):
    pass


class BlockNotFoundError(BlockError):
    pass


class BlockStoreError(BlockError):
    pass


class BlockAccessError(BlockError):
    pass


class BlockInMaintenanceError(BlockError):
    pass


class BaseBlockComponent:
    @api("block_read")
    @catch_protocol_errors
    @api_typed_msg_adapter(BlockReadReq, BlockReadRep)
    async def api_block_read(self, client_ctx, req: BlockReadReq) -> BlockReadRep:
        try:
            block = await self.read(client_ctx.organization_id, client_ctx.device_id, req.block_id)

        except BlockNotFoundError:
            return BlockReadRep.NotFound()

        except BlockStoreError:
            # For legacy reasons, block store error status is `timeout`
            return BlockReadRep.Timeout()

        except BlockAccessError:
            return BlockReadRep.NotAllowed()

        except BlockInMaintenanceError:
            return BlockReadRep.InMaintenance()

        return BlockReadRep.Ok(block=block)

    @api("block_create")
    @catch_protocol_errors
    @api_typed_msg_adapter(BlockCreateReq, BlockCreateRep)
    async def api_block_create(self, client_ctx, req: BlockCreateReq) -> BlockCreateRep:
        try:
            await self.create(
                client_ctx.organization_id,
                client_ctx.device_id,
                req.block_id,
                req.realm_id,
                req.block,
            )

        except BlockAlreadyExistsError:
            return BlockCreateRep.AlreadyExists()

        except BlockNotFoundError:
            return BlockCreateRep.NotFound()

        except BlockStoreError:
            # For legacy reasons, block store error status is `timeout`
            return BlockCreateRep.Timeout()

        except BlockAccessError:
            return BlockCreateRep.NotAllowed()

        except BlockInMaintenanceError:
            return BlockCreateRep.InMaintenance()

        return BlockCreateRep.Ok()

    async def read(
        self, organization_id: OrganizationID, author: DeviceID, block_id: BlockID
    ) -> bytes:
        """
        Raises:
            BlockNotFoundError
            BlockStoreError
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
            BlockStoreError
            BlockAccessError
            BlockInMaintenanceError
        """
        raise NotImplementedError()
