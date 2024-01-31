# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from dataclasses import dataclass
from typing import NoReturn
from uuid import uuid4

import structlog

from parsec._parsec import (
    ApiVersion,
    DeviceID,
    InvitationToken,
    InvitationType,
    OrganizationID,
    UserID,
    VerifyKey,
)

logger = structlog.get_logger()


@dataclass(slots=True)
class AnonymousClientContext:
    client_api_version: ApiVersion
    settled_api_version: ApiVersion
    organization_id: OrganizationID
    organization_internal_id: int
    _logger: structlog.stdlib.BoundLogger | None = None

    @property
    def logger(self) -> structlog.stdlib.BoundLogger:
        if self._logger:
            return self._logger
        self._logger = logger.bind(
            request=uuid4().hex, organization_id=self.organization_id.str, auth="anonymous"
        )
        return self._logger

    def organization_not_found_abort(self) -> NoReturn:
        from parsec.asgi.rpc import CustomHttpStatus, _handshake_abort

        _handshake_abort(
            CustomHttpStatus.OrganizationOrInvitationInvalidOrNotFound,
            api_version=self.settled_api_version,
        )

    def organization_expired_abort(self) -> NoReturn:
        from parsec.asgi.rpc import CustomHttpStatus, _handshake_abort

        _handshake_abort(
            CustomHttpStatus.OrganizationExpired,
            api_version=self.settled_api_version,
        )


@dataclass(slots=True)
class InvitedClientContext:
    client_api_version: ApiVersion
    settled_api_version: ApiVersion
    organization_id: OrganizationID
    token: InvitationToken
    type: InvitationType
    organization_internal_id: int
    invitation_internal_id: int
    _logger: structlog.stdlib.BoundLogger | None = None

    @property
    def logger(self) -> structlog.stdlib.BoundLogger:
        if self._logger:
            return self._logger
        self._logger = logger.bind(
            request=uuid4().hex,
            organization_id=self.organization_id,
            auth="invited",
            token=self.token.hex,
        )
        return self._logger

    def organization_not_found_abort(self) -> NoReturn:
        from parsec.asgi.rpc import CustomHttpStatus, _handshake_abort

        _handshake_abort(
            CustomHttpStatus.OrganizationOrInvitationInvalidOrNotFound,
            api_version=self.settled_api_version,
        )

    def organization_expired_abort(self) -> NoReturn:
        from parsec.asgi.rpc import CustomHttpStatus, _handshake_abort

        _handshake_abort(
            CustomHttpStatus.OrganizationExpired,
            api_version=self.settled_api_version,
        )

    def invitation_invalid_abort(self) -> NoReturn:
        from parsec.asgi.rpc import CustomHttpStatus, _handshake_abort

        _handshake_abort(
            CustomHttpStatus.OrganizationOrInvitationInvalidOrNotFound,
            api_version=self.settled_api_version,
        )


@dataclass(slots=True)
class AuthenticatedClientContext:
    client_api_version: ApiVersion
    settled_api_version: ApiVersion
    organization_id: OrganizationID
    device_id: DeviceID
    device_verify_key: VerifyKey
    organization_internal_id: int
    device_internal_id: int
    _logger: structlog.stdlib.BoundLogger | None = None
    _user_id: UserID | None = None

    @property
    def user_id(self) -> UserID:
        if self._user_id is None:
            self._user_id = self.device_id.user_id
        return self._user_id

    @property
    def logger(self) -> structlog.stdlib.BoundLogger:
        if self._logger:
            return self._logger
        self._logger = logger.bind(
            request=uuid4().hex,
            organization_id=self.organization_id,
            auth="authenticated",
            device_id=self.device_id.str,
        )
        return self._logger

    def organization_not_found_abort(self) -> NoReturn:
        from parsec.asgi.rpc import CustomHttpStatus, _handshake_abort

        _handshake_abort(
            CustomHttpStatus.OrganizationOrInvitationInvalidOrNotFound,
            api_version=self.settled_api_version,
        )

    def organization_expired_abort(self) -> NoReturn:
        from parsec.asgi.rpc import CustomHttpStatus, _handshake_abort

        _handshake_abort(
            CustomHttpStatus.OrganizationExpired,
            api_version=self.settled_api_version,
        )

    def author_not_found_abort(self) -> NoReturn:
        from parsec.asgi.rpc import CustomHttpStatus, _handshake_abort

        _handshake_abort(
            CustomHttpStatus.BadAuthenticationInfo,
            api_version=self.settled_api_version,
        )

    def author_revoked_abort(self) -> NoReturn:
        from parsec.asgi.rpc import CustomHttpStatus, _handshake_abort

        _handshake_abort(
            CustomHttpStatus.UserRevoked,
            api_version=self.settled_api_version,
        )
