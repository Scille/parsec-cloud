# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceID,
    OrganizationID,
    RealmID,
    authenticated_cmds,
)
from parsec.backend.client_context import AuthenticatedClientContext
from parsec.backend.utils import api


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
    @api
    async def api_block_read(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.block_read.Req
    ) -> authenticated_cmds.latest.block_read.Rep:
        try:
            block = await self.read(client_ctx.organization_id, client_ctx.device_id, req.block_id)

        except BlockNotFoundError:
            return authenticated_cmds.latest.block_read.RepNotFound()

        except BlockStoreError:
            # For legacy reasons, block store error status is `timeout`
            return authenticated_cmds.latest.block_read.RepTimeout()

        except BlockAccessError:
            return authenticated_cmds.latest.block_read.RepNotAllowed()

        except BlockInMaintenanceError:
            return authenticated_cmds.latest.block_read.RepInMaintenance()

        return authenticated_cmds.latest.block_read.RepOk(block=block)

    @api
    async def api_block_create(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.block_create.Req,
    ) -> authenticated_cmds.latest.block_create.Rep:
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
            return authenticated_cmds.latest.block_create.RepAlreadyExists()

        except BlockNotFoundError:
            return authenticated_cmds.latest.block_create.RepNotFound()

        except BlockStoreError:
            # For legacy reasons, block store error status is `timeout`
            return authenticated_cmds.latest.block_create.RepTimeout()

        except BlockAccessError:
            return authenticated_cmds.latest.block_create.RepNotAllowed()

        except BlockInMaintenanceError:
            return authenticated_cmds.latest.block_create.RepInMaintenance()

        return authenticated_cmds.latest.block_create.RepOk()

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
