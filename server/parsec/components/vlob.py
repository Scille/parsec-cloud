# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import json
from dataclasses import dataclass
from enum import auto

import httpx

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    SequesterServiceID,
    VlobID,
    authenticated_cmds,
)
from parsec.api import api
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.client_context import AuthenticatedClientContext
from parsec.components.realm import BadKeyIndex
from parsec.logging import get_logger
from parsec.types import BadOutcomeEnum

logger = get_logger()

# Maximum number of vlobs that can be read in a single request
VLOB_READ_REQUEST_ITEMS_LIMIT: int = 1000


@dataclass(slots=True)
class SequesterInconsistency:
    last_common_certificate_timestamp: DateTime


@dataclass(slots=True)
class SequesterServiceNotAvailable:
    service_id: SequesterServiceID


@dataclass(slots=True)
class RejectedBySequesterService:
    service_id: SequesterServiceID
    reason: str


@dataclass(slots=True)
class VlobReadResult:
    # Fields are: vlob ID, key index, author, version, created on, blob
    items: list[tuple[VlobID, int, DeviceID, int, DateTime, bytes]]
    needed_common_certificate_timestamp: DateTime
    needed_realm_certificate_timestamp: DateTime


class VlobCreateBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    REALM_NOT_FOUND = auto()
    VLOB_ALREADY_EXISTS = auto()
    ORGANIZATION_NOT_SEQUESTERED = auto()


class VlobUpdateBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    VLOB_NOT_FOUND = auto()
    BAD_VLOB_VERSION = auto()
    ORGANIZATION_NOT_SEQUESTERED = auto()


class VlobReadAsUserBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    REALM_NOT_FOUND = auto()


class VlobPollChangesAsUserBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    REALM_NOT_FOUND = auto()


class VlobListVersionsBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()


class BaseVlobComponent:
    def __init__(self, http_client: httpx.AsyncClient):
        self._http_client = http_client

    #
    # Public methods
    #

    async def create(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        vlob_id: VlobID,
        key_index: int,
        timestamp: DateTime,
        blob: bytes,
        # Sequester is a special case, so gives it a default version to simplify tests
        sequester_blob: dict[SequesterServiceID, bytes] | None = None,
    ) -> (
        None
        | BadKeyIndex
        | VlobCreateBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
        | RejectedBySequesterService
        | SequesterServiceNotAvailable
        | SequesterInconsistency
    ):
        raise NotImplementedError

    async def update(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        vlob_id: VlobID,
        key_index: int,
        version: int,
        timestamp: DateTime,
        blob: bytes,
        # Sequester is a special case, so gives it a default version to simplify tests
        sequester_blob: dict[SequesterServiceID, bytes] | None = None,
    ) -> (
        None
        | BadKeyIndex
        | VlobUpdateBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
        | RejectedBySequesterService
        | SequesterServiceNotAvailable
        | SequesterInconsistency
    ):
        raise NotImplementedError

    async def read_versions(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        items: list[tuple[VlobID, int]],
    ) -> VlobReadResult | VlobReadAsUserBadOutcome:
        raise NotImplementedError

    async def read_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        vlobs: list[VlobID],
        at: DateTime | None,
    ) -> VlobReadResult | VlobReadAsUserBadOutcome:
        raise NotImplementedError

    async def poll_changes(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        checkpoint: int,
    ) -> tuple[int, list[tuple[VlobID, int]]] | VlobPollChangesAsUserBadOutcome:
        raise NotImplementedError

    async def test_dump_vlobs(
        self, organization_id: OrganizationID
    ) -> dict[VlobID, list[tuple[DeviceID, DateTime, VlobID, bytes]]]:
        raise NotImplementedError

    async def _sequester_service_send_webhook(
        self,
        webhook_url: str,
        service_id: SequesterServiceID,
        organization_id: OrganizationID,
        sequester_blob: bytes,
    ) -> None | SequesterServiceNotAvailable | RejectedBySequesterService:
        # Proceed webhook service before storage (guarantee data are not stored if they are rejected)
        try:
            ret = await self._http_client.post(
                webhook_url,
                params={
                    "organization_id": organization_id.str,
                    "service_id": service_id.hex,
                },
                content=sequester_blob,
            )
            if ret.status_code == 400:
                raw_body = await ret.aread()
                try:
                    body = json.loads(raw_body)
                    if not isinstance(body, dict) or not isinstance(body.get("reason"), str):
                        raise ValueError
                    reason = body["reason"]

                except (json.JSONDecodeError, ValueError):
                    logger.warning(
                        "Invalid rejection reason body returned by webhook",
                        organization_id=organization_id.str,
                        service_id=service_id.hex,
                        body=raw_body,
                    )
                    reason = "File rejected (no reason)"
                return RejectedBySequesterService(
                    service_id=service_id,
                    reason=reason,
                )

            elif not ret.is_success:
                logger.warning(
                    "Invalid HTTP status returned by webhook",
                    organization_id=organization_id.str,
                    service_id=service_id.hex,
                    status=ret.status_code,
                )
                return SequesterServiceNotAvailable(
                    service_id=service_id,
                )

        except OSError as exc:
            logger.warning(
                "Cannot reach webhook server",
                organization_id=organization_id.str,
                service_id=service_id.hex,
                exc_info=exc,
            )
            return SequesterServiceNotAvailable(
                service_id=service_id,
            )

    #
    # API commands
    #

    @api
    async def api_vlob_create(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.vlob_create.Req
    ) -> authenticated_cmds.latest.vlob_create.Rep:
        """
        This API call, when successful, performs the writing of a new vlob version to the database.
        Before adding new entries, extra care should be taken in order to guarantee the consistency in
        the ordering of the different timestamps stored in the database.

        See the `api_vlob_update` docstring for more information about the checks performed and the
        error returned in case those checks failed.
        """
        outcome = await self.create(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            realm_id=req.realm_id,
            vlob_id=req.vlob_id,
            key_index=req.key_index,
            timestamp=req.timestamp,
            blob=req.blob,
            sequester_blob=req.sequester_blob,
        )
        match outcome:
            case None:
                return authenticated_cmds.latest.vlob_create.RepOk()
            case VlobCreateBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.vlob_create.RepAuthorNotAllowed()
            case BadKeyIndex() as error:
                return authenticated_cmds.latest.vlob_create.RepBadKeyIndex(
                    last_realm_certificate_timestamp=error.last_realm_certificate_timestamp,
                )
            case VlobCreateBadOutcome.REALM_NOT_FOUND:
                return authenticated_cmds.latest.vlob_create.RepRealmNotFound()
            case VlobCreateBadOutcome.VLOB_ALREADY_EXISTS:
                return authenticated_cmds.latest.vlob_create.RepVlobAlreadyExists()
            case VlobCreateBadOutcome.ORGANIZATION_NOT_SEQUESTERED:
                return authenticated_cmds.latest.vlob_create.RepOrganizationNotSequestered()
            case TimestampOutOfBallpark() as error:
                return authenticated_cmds.latest.vlob_create.RepTimestampOutOfBallpark(
                    server_timestamp=error.server_timestamp,
                    client_timestamp=error.client_timestamp,
                    ballpark_client_early_offset=error.ballpark_client_early_offset,
                    ballpark_client_late_offset=error.ballpark_client_late_offset,
                )
            case RequireGreaterTimestamp() as error:
                return authenticated_cmds.latest.vlob_create.RepRequireGreaterTimestamp(
                    strictly_greater_than=error.strictly_greater_than
                )
            case SequesterInconsistency() as error:
                return authenticated_cmds.latest.vlob_create.RepSequesterInconsistency(
                    last_common_certificate_timestamp=error.last_common_certificate_timestamp
                )
            case RejectedBySequesterService() as error:
                return authenticated_cmds.latest.vlob_create.RepRejectedBySequesterService(
                    service_id=error.service_id,
                    reason=error.reason,
                )
            case SequesterServiceNotAvailable():
                return authenticated_cmds.latest.vlob_create.RepSequesterServiceUnavailable()
            case VlobCreateBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case VlobCreateBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case VlobCreateBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case VlobCreateBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def api_vlob_update(
        self, client_ctx: AuthenticatedClientContext, req: authenticated_cmds.latest.vlob_update.Req
    ) -> authenticated_cmds.latest.vlob_update.Rep:
        """
        This API call, when successful, performs the writing of a new vlob version to the database.
        Before adding new entries, extra care should be taken in order to guarantee the consistency in
        the ordering of the different timestamps stored in the database.

        See the `api_vlob_update` docstring for more information about the checks performed and the
        error returned in case those checks failed.
        """
        outcome = await self.update(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            vlob_id=req.vlob_id,
            key_index=req.key_index,
            version=req.version,
            timestamp=req.timestamp,
            blob=req.blob,
            sequester_blob=req.sequester_blob,
        )
        match outcome:
            case None:
                return authenticated_cmds.latest.vlob_update.RepOk()
            case VlobUpdateBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.vlob_update.RepAuthorNotAllowed()
            case BadKeyIndex() as error:
                return authenticated_cmds.latest.vlob_update.RepBadKeyIndex(
                    last_realm_certificate_timestamp=error.last_realm_certificate_timestamp,
                )
            case VlobUpdateBadOutcome.BAD_VLOB_VERSION:
                return authenticated_cmds.latest.vlob_update.RepBadVlobVersion()
            case VlobUpdateBadOutcome.VLOB_NOT_FOUND:
                return authenticated_cmds.latest.vlob_update.RepVlobNotFound()
            case VlobUpdateBadOutcome.ORGANIZATION_NOT_SEQUESTERED:
                return authenticated_cmds.latest.vlob_update.RepOrganizationNotSequestered()
            case TimestampOutOfBallpark() as error:
                return authenticated_cmds.latest.vlob_update.RepTimestampOutOfBallpark(
                    server_timestamp=error.server_timestamp,
                    client_timestamp=error.client_timestamp,
                    ballpark_client_early_offset=error.ballpark_client_early_offset,
                    ballpark_client_late_offset=error.ballpark_client_late_offset,
                )
            case RequireGreaterTimestamp() as error:
                return authenticated_cmds.latest.vlob_update.RepRequireGreaterTimestamp(
                    strictly_greater_than=error.strictly_greater_than
                )
            case SequesterInconsistency() as error:
                return authenticated_cmds.latest.vlob_update.RepSequesterInconsistency(
                    last_common_certificate_timestamp=error.last_common_certificate_timestamp
                )
            case RejectedBySequesterService() as error:
                return authenticated_cmds.latest.vlob_update.RepRejectedBySequesterService(
                    service_id=error.service_id,
                    reason=error.reason,
                )
            case SequesterServiceNotAvailable():
                return authenticated_cmds.latest.vlob_update.RepSequesterServiceUnavailable()
            case VlobUpdateBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case VlobUpdateBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case VlobUpdateBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case VlobUpdateBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def api_vlob_read_batch(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.vlob_read_batch.Req,
    ) -> authenticated_cmds.latest.vlob_read_batch.Rep:
        if len(req.vlobs) > VLOB_READ_REQUEST_ITEMS_LIMIT:
            return authenticated_cmds.latest.vlob_read_batch.RepTooManyElements()

        outcome = await self.read_batch(
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            realm_id=req.realm_id,
            vlobs=req.vlobs,
            at=req.at,
        )
        match outcome:
            case VlobReadResult() as result:
                return authenticated_cmds.latest.vlob_read_batch.RepOk(
                    items=result.items,
                    needed_common_certificate_timestamp=result.needed_common_certificate_timestamp,
                    needed_realm_certificate_timestamp=result.needed_realm_certificate_timestamp,
                )
            case VlobReadAsUserBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.vlob_read_batch.RepAuthorNotAllowed()
            case VlobReadAsUserBadOutcome.REALM_NOT_FOUND:
                return authenticated_cmds.latest.vlob_read_batch.RepRealmNotFound()
            case VlobReadAsUserBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case VlobReadAsUserBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case VlobReadAsUserBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case VlobReadAsUserBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def api_vlob_read_versions(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.vlob_read_versions.Req,
    ) -> authenticated_cmds.latest.vlob_read_versions.Rep:
        if len(req.items) > VLOB_READ_REQUEST_ITEMS_LIMIT:
            return authenticated_cmds.latest.vlob_read_versions.RepTooManyElements()

        outcome = await self.read_versions(
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            realm_id=req.realm_id,
            items=req.items,
        )
        match outcome:
            case VlobReadResult() as result:
                return authenticated_cmds.latest.vlob_read_versions.RepOk(
                    items=result.items,
                    needed_common_certificate_timestamp=result.needed_common_certificate_timestamp,
                    needed_realm_certificate_timestamp=result.needed_realm_certificate_timestamp,
                )
            case VlobReadAsUserBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.vlob_read_versions.RepAuthorNotAllowed()
            case VlobReadAsUserBadOutcome.REALM_NOT_FOUND:
                return authenticated_cmds.latest.vlob_read_versions.RepRealmNotFound()
            case VlobReadAsUserBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case VlobReadAsUserBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case VlobReadAsUserBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case VlobReadAsUserBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def api_vlob_poll_changes(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.vlob_poll_changes.Req,
    ) -> authenticated_cmds.latest.vlob_poll_changes.Rep:
        outcome = await self.poll_changes(
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            realm_id=req.realm_id,
            checkpoint=req.last_checkpoint,
        )
        match outcome:
            case (current_checkpoint, changes):
                return authenticated_cmds.latest.vlob_poll_changes.RepOk(
                    current_checkpoint=current_checkpoint, changes=changes
                )
            case VlobPollChangesAsUserBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.vlob_poll_changes.RepAuthorNotAllowed()
            case VlobPollChangesAsUserBadOutcome.REALM_NOT_FOUND:
                return authenticated_cmds.latest.vlob_poll_changes.RepRealmNotFound()
            case VlobPollChangesAsUserBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case VlobPollChangesAsUserBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case VlobPollChangesAsUserBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case VlobPollChangesAsUserBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
