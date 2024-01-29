# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from enum import Enum
from typing import assert_never

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceID,
    OrganizationID,
    UserID,
    VlobID,
    authenticated_cmds,
)
from parsec.api import api
from parsec.client_context import AuthenticatedClientContext

BlockReadBadOutcome = Enum(
    "BlockReadBadOutcome",
    (
        "ORGANIZATION_NOT_FOUND",
        "ORGANIZATION_EXPIRED",
        "AUTHOR_NOT_FOUND",
        "AUTHOR_REVOKED",
        "AUTHOR_NOT_ALLOWED",
        "BLOCK_NOT_FOUND",
        "STORE_UNAVAILABLE",
    ),
)

BlockCreateBadOutcome = Enum(
    "BlockCreateBadOutcome",
    (
        "ORGANIZATION_NOT_FOUND",
        "ORGANIZATION_EXPIRED",
        "AUTHOR_NOT_FOUND",
        "AUTHOR_REVOKED",
        "AUTHOR_NOT_ALLOWED",
        "REALM_NOT_FOUND",
        "BLOCK_ALREADY_EXISTS",
        "STORE_UNAVAILABLE",
    ),
)


class BaseBlockComponent:
    #
    # Public methods
    #

    async def read_as_user(
        self, organization_id: OrganizationID, author: UserID, block_id: BlockID
    ) -> bytes | BlockReadBadOutcome:
        raise NotImplementedError

    async def create(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        block_id: BlockID,
        realm_id: VlobID,
        block: bytes,
    ) -> None | BlockCreateBadOutcome:
        raise NotImplementedError

    async def test_dump_blocks(
        self, organization_id: OrganizationID
    ) -> dict[BlockID, tuple[DateTime, DeviceID, VlobID, int]]:
        raise NotImplementedError

    #
    # API commands
    #

    @api
    async def api_block_read(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.block_read.Req
    ) -> authenticated_cmds.latest.block_read.Rep:
        outcome = await self.read_as_user(
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id.user_id,
            block_id=req.block_id,
        )
        match outcome:
            case (bytes() | bytearray() | memoryview()) as block:
                return authenticated_cmds.latest.block_read.RepOk(block=block)
            case BlockReadBadOutcome.BLOCK_NOT_FOUND:
                return authenticated_cmds.latest.block_read.RepBlockNotFound()
            case BlockReadBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.block_read.RepAuthorNotAllowed()
            case BlockReadBadOutcome.STORE_UNAVAILABLE:
                return authenticated_cmds.latest.block_read.RepStoreUnavailable()
            case BlockReadBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case BlockReadBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case BlockReadBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case BlockReadBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
            case unknown:
                assert_never(unknown)

    @api
    async def api_block_create(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.block_create.Req,
    ) -> authenticated_cmds.latest.block_create.Rep:
        outcome = await self.create(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            block_id=req.block_id,
            realm_id=req.realm_id,
            block=req.block,
        )
        match outcome:
            case None:
                return authenticated_cmds.latest.block_create.RepOk()
            case BlockCreateBadOutcome.REALM_NOT_FOUND:
                return authenticated_cmds.latest.block_create.RepRealmNotFound()
            case BlockCreateBadOutcome.BLOCK_ALREADY_EXISTS:
                return authenticated_cmds.latest.block_create.RepBlockAlreadyExists()
            case BlockCreateBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.block_create.RepAuthorNotAllowed()
            case BlockCreateBadOutcome.STORE_UNAVAILABLE:
                return authenticated_cmds.latest.block_create.RepStoreUnavailable()
            case BlockCreateBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case BlockCreateBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case BlockCreateBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case BlockCreateBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
            case unknown:
                assert_never(unknown)
