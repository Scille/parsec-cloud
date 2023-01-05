# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    BlockCreateRepAlreadyExists,
    BlockCreateRepInMaintenance,
    BlockCreateRepNotAllowed,
    BlockCreateRepNotFound,
    BlockCreateRepOk,
    BlockCreateRepTimeout,
    BlockReadRepInMaintenance,
    BlockReadRepNotAllowed,
    BlockReadRepNotFound,
    BlockReadRepOk,
    BlockReadRepTimeout,
    DateTime,
)
from parsec.api.protocol import (
    BlockCreateRep,
    BlockCreateReq,
    BlockID,
    BlockReadRep,
    BlockReadReq,
    DeviceID,
    OrganizationID,
    RealmID,
)
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.utils import api, api_typed_msg_adapter, catch_protocol_errors


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
    async def api_block_read(
        self, client_ctx: AuthenticatedClientContext, req: BlockReadReq
    ) -> BlockReadRep:
        try:
            block = await self.read(client_ctx.organization_id, client_ctx.device_id, req.block_id)

        except BlockNotFoundError:
            return BlockReadRepNotFound()

        except BlockStoreError:
            # For legacy reasons, block store error status is `timeout`
            return BlockReadRepTimeout()

        except BlockAccessError:
            return BlockReadRepNotAllowed()

        except BlockInMaintenanceError:
            return BlockReadRepInMaintenance()

        return BlockReadRepOk(block=block)

    @api("block_create")
    @catch_protocol_errors
    @api_typed_msg_adapter(BlockCreateReq, BlockCreateRep)
    async def api_block_create(
        self, client_ctx: AuthenticatedClientContext, req: BlockCreateReq
    ) -> BlockCreateRep:
        try:
            await self.create(
                organization_id=client_ctx.organization_id,
                author=client_ctx.device_id,
                block_id=req.block_id,
                realm_id=req.realm_id,
                block=req.block,
                created_on=DateTime.now(),
            )

        except BlockAlreadyExistsError:
            return BlockCreateRepAlreadyExists()

        except BlockNotFoundError:
            return BlockCreateRepNotFound()

        except BlockStoreError:
            # For legacy reasons, block store error status is `timeout`
            return BlockCreateRepTimeout()

        except BlockAccessError:
            return BlockCreateRepNotAllowed()

        except BlockInMaintenanceError:
            return BlockCreateRepInMaintenance()

        return BlockCreateRepOk()

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
        created_on: DateTime | None = None,
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
