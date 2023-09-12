# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    BlockCreateRepAlreadyExists,
    BlockCreateRepInMaintenance,
    BlockCreateRepNotAllowed,
    BlockCreateRepNotFound,
    BlockCreateRepOk,
    BlockCreateRepRealmArchived,
    BlockCreateRepRealmDeleted,
    BlockCreateRepTimeout,
    BlockReadRepInMaintenance,
    BlockReadRepNotAllowed,
    BlockReadRepNotFound,
    BlockReadRepOk,
    BlockReadRepRealmDeleted,
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


class BlockRealmDeletedError(BlockError):
    pass


class BlockRealmArchivedError(BlockError):
    pass


class BaseBlockComponent:
    @api("block_read")
    @catch_protocol_errors
    @api_typed_msg_adapter(BlockReadReq, BlockReadRep)
    async def api_block_read(
        self, client_ctx: AuthenticatedClientContext, req: BlockReadReq
    ) -> BlockReadRep:
        now = DateTime.now()
        try:
            block = await self.read(
                client_ctx.organization_id, client_ctx.device_id, req.block_id, now
            )

        except BlockNotFoundError:
            return BlockReadRepNotFound()

        except BlockStoreError:
            # For legacy reasons, block store error status is `timeout`
            return BlockReadRepTimeout()

        except BlockAccessError:
            return BlockReadRepNotAllowed()

        except BlockInMaintenanceError:
            return BlockReadRepInMaintenance()

        except BlockRealmDeletedError:
            return BlockReadRepRealmDeleted()

        return BlockReadRepOk(block=block)

    @api("block_create")
    @catch_protocol_errors
    @api_typed_msg_adapter(BlockCreateReq, BlockCreateRep)
    async def api_block_create(
        self, client_ctx: AuthenticatedClientContext, req: BlockCreateReq
    ) -> BlockCreateRep:
        now = DateTime.now()
        try:
            await self.create(
                organization_id=client_ctx.organization_id,
                author=client_ctx.device_id,
                block_id=req.block_id,
                realm_id=req.realm_id,
                block=req.block,
                now=now,
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

        except BlockRealmArchivedError:
            return BlockCreateRepRealmArchived()

        except BlockRealmDeletedError:
            return BlockCreateRepRealmDeleted()

        return BlockCreateRepOk()

    async def read(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        block_id: BlockID,
        now: DateTime,
    ) -> bytes:
        """
        Raises:
            BlockNotFoundError
            BlockStoreError
            BlockAccessError
            BlockInMaintenanceError
            BlockRealmDeletedError
        """
        raise NotImplementedError()

    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        block_id: BlockID,
        realm_id: RealmID,
        block: bytes,
        now: DateTime,
    ) -> None:
        """
        Raises:
            BlockNotFoundError: if cannot found realm
            BlockAlreadyExistsError
            BlockStoreError
            BlockAccessError
            BlockInMaintenanceError
            BlockRealmDeletedError
            BlockRealmArchivedError
        """
        raise NotImplementedError()
