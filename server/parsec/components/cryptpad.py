# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from enum import auto

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    VlobID,
    authenticated_cmds,
)
from parsec.api import api
from parsec.ballpark import TimestampOutOfBallpark
from parsec.client_context import AuthenticatedClientContext
from parsec.components.realm import BadKeyIndex
from parsec.config import BackendConfig
from parsec.types import BadOutcomeEnum


class CryptpadRegisterSessionBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    REALM_NOT_FOUND = auto()
    REALM_DELETED = auto()
    AUTHOR_NOT_ALLOWED = auto()
    CRYPTPAD_UNAVAILABLE = auto()


@dataclass(slots=True)
class CryptpadSession:
    author: DeviceID
    timestamp: DateTime
    key_index: int
    encrypted_view_key: bytes
    encrypted_edit_key: bytes | None
    needed_common_certificate_timestamp: DateTime
    needed_realm_certificate_timestamp: DateTime


class BaseCryptpadComponent:
    def __init__(self, config: BackendConfig) -> None:
        self._config = config

    #
    # Public methods
    #

    async def register_session(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        document_id: VlobID,
        key_index: int,
        timestamp: DateTime,
        encrypted_candidate_view_key: bytes,
        encrypted_candidate_edit_key: bytes | None,
    ) -> CryptpadSession | BadKeyIndex | CryptpadRegisterSessionBadOutcome | TimestampOutOfBallpark:
        raise NotImplementedError

    #
    # API commands
    #

    @api
    async def api_cryptpad_register_session(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.cryptpad_register_session.Req,
    ) -> authenticated_cmds.latest.cryptpad_register_session.Rep:

        # Check if cryptpad is available
        if self._config.cryptpad_config is None:
            return authenticated_cmds.latest.cryptpad_register_session.RepCryptpadUnavailable()

        outcome = await self.register_session(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            realm_id=req.realm_id,
            document_id=req.document_id,
            key_index=req.key_index,
            timestamp=req.timestamp,
            encrypted_candidate_view_key=req.encrypted_candidate_view_key,
            encrypted_candidate_edit_key=req.encrypted_candidate_edit_key,
        )

        match outcome:
            case CryptpadSession() as session:
                return authenticated_cmds.latest.cryptpad_register_session.RepOk(
                    author=session.author,
                    timestamp=session.timestamp,
                    key_index=session.key_index,
                    encrypted_edit_key=session.encrypted_edit_key,
                    encrypted_view_key=session.encrypted_view_key,
                    needed_common_certificate_timestamp=session.needed_common_certificate_timestamp,
                    needed_realm_certificate_timestamp=session.needed_realm_certificate_timestamp,
                )
            case BadKeyIndex() as error:
                return authenticated_cmds.latest.cryptpad_register_session.RepBadKeyIndex(
                    last_realm_certificate_timestamp=error.last_realm_certificate_timestamp,
                )
            case TimestampOutOfBallpark() as error:
                return (
                    authenticated_cmds.latest.cryptpad_register_session.RepTimestampOutOfBallpark(
                        server_timestamp=error.server_timestamp,
                        client_timestamp=error.client_timestamp,
                        ballpark_client_early_offset=error.ballpark_client_early_offset,
                        ballpark_client_late_offset=error.ballpark_client_late_offset,
                    )
                )
            case CryptpadRegisterSessionBadOutcome.REALM_NOT_FOUND:
                return authenticated_cmds.latest.cryptpad_register_session.RepRealmNotFound()
            case CryptpadRegisterSessionBadOutcome.REALM_DELETED:
                return authenticated_cmds.latest.cryptpad_register_session.RepRealmDeleted()
            case CryptpadRegisterSessionBadOutcome.AUTHOR_NOT_ALLOWED:
                return authenticated_cmds.latest.cryptpad_register_session.RepAuthorNotAllowed()
            case CryptpadRegisterSessionBadOutcome.CRYPTPAD_UNAVAILABLE:
                return authenticated_cmds.latest.cryptpad_register_session.RepCryptpadUnavailable()
            case CryptpadRegisterSessionBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case CryptpadRegisterSessionBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case CryptpadRegisterSessionBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case CryptpadRegisterSessionBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()
