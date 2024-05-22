# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from base64 import urlsafe_b64decode, urlsafe_b64encode
from dataclasses import dataclass
from enum import auto

from parsec._parsec import (
    CryptoError,
    DateTime,
    DeviceID,
    InvitationToken,
    InvitationType,
    OrganizationID,
    SigningKey,
    UserID,
    VerifyKey,
)
from parsec.ballpark import timestamps_in_the_ballpark
from parsec.components.events import EventBus
from parsec.config import BackendConfig
from parsec.events import Event, EventUserRevokedOrFrozen, EventUserUnfrozen
from parsec.types import BadOutcomeEnum


class AuthAnonymousAuthBadOutcome(BadOutcomeEnum):
    ORGANIZATION_EXPIRED = auto()
    ORGANIZATION_NOT_FOUND = auto()


class AuthInvitedAuthBadOutcome(BadOutcomeEnum):
    ORGANIZATION_EXPIRED = auto()
    ORGANIZATION_NOT_FOUND = auto()
    INVITATION_NOT_FOUND = auto()
    INVITATION_ALREADY_USED = auto()


class AuthAuthenticatedAuthBadOutcome(BadOutcomeEnum):
    ORGANIZATION_EXPIRED = auto()
    ORGANIZATION_NOT_FOUND = auto()
    USER_REVOKED = auto()
    USER_FROZEN = auto()
    DEVICE_NOT_FOUND = auto()
    INVALID_SIGNATURE = auto()
    TOKEN_TOO_OLD = auto()


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
    user_id: UserID
    device_id: DeviceID
    device_verify_key: VerifyKey
    organization_internal_id: int
    device_internal_id: int


@dataclass
class AuthenticatedToken:
    """
    token format: `PARSEC-SIGN-ED25519.<b64_device_id>.<timestamp>.<b64_signature>`
    with:
        <b64_device_id> = base64(<device_id>)
        <timestamp> = str(<seconds since UNIX epoch>)
        <b64_signature> = base64(ed25519(`PARSEC-SIGN-ED25519.<b64_device_id>.<timestamp>`))
        base64() is the URL-safe variant (https://tools.ietf.org/html/rfc4648#section-5).
    """

    device_id: DeviceID
    timestamp: DateTime
    header_and_payload: bytes
    signature: bytes

    HEADER: bytes = b"PARSEC-SIGN-ED25519"

    # Only used for tests, but coherent to have it here
    @staticmethod
    def generate_raw(device_id: DeviceID, timestamp: DateTime, key: SigningKey) -> bytes:
        raw_device_id = device_id.hex.encode("ascii")
        raw_timestamp_us = str(timestamp.as_timestamp_seconds()).encode("ascii")
        header_and_payload = b"%s.%s.%s" % (
            AuthenticatedToken.HEADER,
            raw_device_id,
            raw_timestamp_us,
        )
        signature = key.sign_only_signature(header_and_payload)
        raw_signature = urlsafe_b64encode(signature)
        return b"%s.%s" % (header_and_payload, raw_signature)

    @classmethod
    def from_raw(cls, raw: bytes) -> "AuthenticatedToken":
        try:
            header_and_payload, raw_signature = raw.rsplit(b".", 1)
            header, raw_device_id, raw_timestamp_us = header_and_payload.split(b".")
            if header != cls.HEADER:
                raise ValueError
            signature = urlsafe_b64decode(raw_signature)
            device_id = DeviceID.from_hex(raw_device_id.decode("ascii"))
            timestamp = DateTime.from_timestamp_seconds(int(raw_timestamp_us))
        except ValueError:
            raise ValueError("Invalid token format")

        return cls(
            device_id=device_id,
            timestamp=timestamp,
            header_and_payload=header_and_payload,
            signature=signature,
        )

    def verify_signature(self, verify_key: VerifyKey) -> bool:
        try:
            verify_key.verify_with_signature(
                signature=self.signature, message=self.header_and_payload
            )
            return True
        except CryptoError:
            return False


class BaseAuthComponent:
    def __init__(self, event_bus: EventBus, config: BackendConfig):
        self._config = config
        self._device_cache: dict[tuple[OrganizationID, DeviceID], AuthenticatedAuthInfo] = {}
        event_bus.connect(self._on_event)

    def _on_event(self, event: Event) -> None:
        match event:
            # Revocation and freezing/unfreezing affect the authentication process,
            # so we clear the cache when such events occur.
            case EventUserUnfrozen() | EventUserRevokedOrFrozen():
                self._device_cache = {
                    (org_id, device_id): v
                    for ((org_id, device_id), v) in self._device_cache.items()
                    if org_id != event.organization_id or v.user_id != event.user_id
                }
            case _:
                pass

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
        token: AuthenticatedToken,
    ) -> AuthenticatedAuthInfo | AuthAuthenticatedAuthBadOutcome:
        try:
            # The cache is only available if the authentication already succeeded,
            # and if no revocation or freezing occurred since then.
            auth_info = self._device_cache[(organization_id, token.device_id)]

        except KeyError:
            outcome = await self._get_authenticated_info(organization_id, token.device_id)
            match outcome:
                case AuthenticatedAuthInfo() as auth_info:
                    self._device_cache[(organization_id, token.device_id)] = auth_info

                case AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND:
                    # Cannot store cache as the organization might be created at anytime !
                    return AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND

                case bad_outcome:
                    return bad_outcome

        if not token.verify_signature(auth_info.device_verify_key):
            return AuthAuthenticatedAuthBadOutcome.INVALID_SIGNATURE

        if timestamps_in_the_ballpark(token.timestamp, now) is not None:
            return AuthAuthenticatedAuthBadOutcome.TOKEN_TOO_OLD

        return auth_info

    async def _get_authenticated_info(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> AuthenticatedAuthInfo | AuthAuthenticatedAuthBadOutcome:
        raise NotImplementedError
