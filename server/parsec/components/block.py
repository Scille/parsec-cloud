# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from dataclasses import dataclass
from enum import auto

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceID,
    OrganizationID,
    VlobID,
    authenticated_cmds,
)
from parsec.api import api
from parsec.client_context import AuthenticatedClientContext
from parsec.components.realm import BadKeyIndex
from parsec.types import BadOutcomeEnum


@dataclass(slots=True)
class BlockReadResult:
    block: bytes
    key_index: int
    needed_realm_certificate_timestamp: DateTime


class BlockReadBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    BLOCK_NOT_FOUND = auto()
    STORE_UNAVAILABLE = auto()


class BlockCreateBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    REALM_NOT_FOUND = auto()
    BLOCK_ALREADY_EXISTS = auto()
    STORE_UNAVAILABLE = auto()


class BaseBlockComponent:
    #
    # Public methods
    #

    async def read(
        self, organization_id: OrganizationID, author: DeviceID, block_id: BlockID
    ) -> BlockReadResult | BlockReadBadOutcome:
        raise NotImplementedError

    async def create(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        block_id: BlockID,
        key_index: int,
        block: bytes,
    ) -> None | BadKeyIndex | BlockCreateBadOutcome:
        raise NotImplementedError

    async def test_dump_blocks(
        self, organization_id: OrganizationID
    ) -> dict[BlockID, tuple[DateTime, DeviceID, VlobID, int, int]]:
        raise NotImplementedError

    #
    # API commands
    #

    @api
    async def api_block_read(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.block_read.Req
    ) -> authenticated_cmds.latest.block_read.Rep:
        outcome = await self.read(
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            block_id=req.block_id,
        )
        match outcome:
            case BlockReadResult() as result:
                return authenticated_cmds.latest.block_read.RepOk(
                    block=result.block,
                    key_index=result.key_index,
                    needed_realm_certificate_timestamp=result.needed_realm_certificate_timestamp,
                )
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
            key_index=req.key_index,
            block_id=req.block_id,
            realm_id=req.realm_id,
            block=req.block,
        )
        match outcome:
            case None:
                return authenticated_cmds.latest.block_create.RepOk()
            case BadKeyIndex() as error:
                return authenticated_cmds.latest.block_create.RepBadKeyIndex(
                    last_realm_certificate_timestamp=error.last_realm_certificate_timestamp,
                )
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
