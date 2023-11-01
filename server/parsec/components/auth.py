# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from dataclasses import dataclass
from enum import Enum

from parsec._parsec import (
    CryptoError,
    DateTime,
    DeviceID,
    InvitationToken,
    InvitationType,
    OrganizationID,
    VerifyKey,
)
from parsec.config import BackendConfig

CACHE_TIME = 60  # seconds


AuthAnonymousAuthBadOutcome = Enum(
    "AuthAnonymousAuthBadOutcome",
    (
        "ORGANIZATION_EXPIRED",
        "ORGANIZATION_NOT_FOUND",
    ),
)
AuthInvitedAuthBadOutcome = Enum(
    "AuthInvitedAuthBadOutcome",
    (
        "ORGANIZATION_EXPIRED",
        "ORGANIZATION_NOT_FOUND",
        "INVITATION_NOT_FOUND",
        "INVITATION_ALREADY_USED",
    ),
)
AuthAuthenticatedAuthBadOutcome = Enum(
    "AuthAuthenticatedAuthBadOutcome",
    (
        "ORGANIZATION_EXPIRED",
        "ORGANIZATION_NOT_FOUND",
        "USER_REVOKED",
        "DEVICE_NOT_FOUND",
        "INVALID_SIGNATURE",
    ),
)


@dataclass
class AnonymousAuthInfo:
    organization_id: OrganizationID
    organization_internal_id: int


@dataclass
class InvitedAuthInfo:
    organization_id: OrganizationID
    token: InvitationToken
    type: InvitationType
    organization_internal_id: int
    invitation_internal_id: int


@dataclass
class AuthenticatedAuthInfo:
    organization_id: OrganizationID
    device_id: DeviceID
    device_verify_key: VerifyKey
    organization_internal_id: int
    device_internal_id: int


class BaseAuthComponent:
    def __init__(self, config: BackendConfig):
        self._config = config
        self._device_cache: dict[
            tuple[OrganizationID, DeviceID],
            tuple[DateTime, AuthenticatedAuthInfo | AuthAuthenticatedAuthBadOutcome],
        ] = {}

    #
    # Public methods
    #

    async def anonymous_auth(
        self, now: DateTime, organization_id: OrganizationID, spontaneous_bootstrap: bool
    ) -> AnonymousAuthInfo | AuthAnonymousAuthBadOutcome:
        raise NotImplementedError

    async def invited_auth(
        self, now: DateTime, organization_id: OrganizationID, token: InvitationToken
    ) -> InvitedAuthInfo | AuthInvitedAuthBadOutcome:
        raise NotImplementedError

    async def authenticated_auth(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        device_id: DeviceID,
        signature: bytes,
        body: bytes,
    ) -> AuthenticatedAuthInfo | AuthAuthenticatedAuthBadOutcome:
        try:
            cache_timestamp, auth_or_bad_outcome = self._device_cache[(organization_id, device_id)]
            if now - cache_timestamp > CACHE_TIME:
                del self._device_cache[(organization_id, device_id)]
                raise KeyError

            match auth_or_bad_outcome:
                case AuthenticatedAuthInfo() as client_ctx:
                    pass
                case bad_outcome:
                    return bad_outcome

        except KeyError:
            outcome = await self._get_authenticated_info(organization_id, device_id)
            match outcome:
                case AuthenticatedAuthInfo() as client_ctx:
                    self._device_cache[(organization_id, device_id)] = (now, client_ctx)

                case AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND:
                    # Cannot store cache as the organization might be created at anytime !
                    return AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND

                case bad_outcome:
                    self._device_cache[(organization_id, device_id)] = (now, bad_outcome)
                    return bad_outcome

        try:
            client_ctx.device_verify_key.verify_with_signature(signature=signature, message=body)
        except CryptoError:
            return AuthAuthenticatedAuthBadOutcome.INVALID_SIGNATURE

        return client_ctx

    async def _get_authenticated_info(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> AuthenticatedAuthInfo | AuthAuthenticatedAuthBadOutcome:
        raise NotImplementedError
