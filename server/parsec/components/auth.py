# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from base64 import urlsafe_b64decode, urlsafe_b64encode
from dataclasses import dataclass
from enum import auto

from parsec._parsec import (
    AccountAuthMethodID,
    CryptoError,
    DateTime,
    DeviceID,
    EmailAddress,
    InvitationToken,
    InvitationType,
    OrganizationID,
    SecretKey,
    SigningKey,
    UserID,
    VerifyKey,
)
from parsec.ballpark import timestamps_in_the_ballpark
from parsec.components.events import EventBus
from parsec.config import AllowedClientAgent, BackendConfig
from parsec.events import (
    Event,
    EventOrganizationTosUpdated,
    EventUserRevokedOrFrozen,
    EventUserUnfrozen,
)
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
    USER_MUST_ACCEPT_TOS = auto()
    DEVICE_NOT_FOUND = auto()
    INVALID_TOKEN = auto()
    TOKEN_OUT_OF_BALLPARK = auto()


class AuthAuthenticatedAccountAuthBadOutcome(BadOutcomeEnum):
    ACCOUNT_NOT_FOUND = auto()
    INVALID_TOKEN = auto()
    TOKEN_OUT_OF_BALLPARK = auto()


@dataclass
class AnonymousAuthInfo:
    organization_id: OrganizationID
    organization_internal_id: int
    organization_allowed_client_agent: AllowedClientAgent


@dataclass
class AuthenticatedAccountAuthInfo:
    account_email: EmailAddress
    auth_method_id: AccountAuthMethodID
    account_internal_id: int
    auth_method_internal_id: int


@dataclass
class InvitedAuthInfo:
    organization_id: OrganizationID
    token: InvitationToken
    type: InvitationType
    organization_internal_id: int
    invitation_internal_id: int
    organization_allowed_client_agent: AllowedClientAgent


@dataclass
class AuthenticatedAuthInfo:
    organization_id: OrganizationID
    user_id: UserID
    device_id: DeviceID
    device_verify_key: VerifyKey
    organization_internal_id: int
    device_internal_id: int
    organization_allowed_client_agent: AllowedClientAgent


@dataclass
class AuthenticatedToken:
    """
    Token format: `PARSEC-SIGN-ED25519.<device_id_hex>.<timestamp>.<b64_signature>`
    with:
        <device_id_hex> = hex(<device_id>)
        <timestamp> = str(<seconds since UNIX epoch>)
        <b64_signature> = base64(ed25519(`PARSEC-SIGN-ED25519.<device_id_hex>.<timestamp>`))
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
        except ValueError as e:
            raise ValueError("Invalid token format") from e

        return cls(
            device_id=device_id,
            timestamp=timestamp,
            header_and_payload=header_and_payload,
            signature=signature,
        )

    def verify(self, verify_key: VerifyKey) -> bool:
        try:
            verify_key.verify_with_signature(
                signature=self.signature, message=self.header_and_payload
            )
            return True
        except CryptoError:
            return False


@dataclass
class AccountAuthenticationToken:
    """
    Token format: `PARSEC-MAC-BLAKE2B.<auth_method_id_hex>.<timestamp>.<b64_signature>`
    with:
        <auth_method_id_hex> = hex(<auth_method.id>)
        <timestamp> = str(<seconds since UNIX epoch>)
        <b64_signature> = base64(blake2b_mac_512bits(
            data=`PARSEC-MAC-BLAKE2B.<auth_method_id_hex>.<timestamp>`,
            key=`auth_method.mac_key`
        ))
    base64() is the URL-safe variant (https://tools.ietf.org/html/rfc4648#section-5).
    """

    auth_method_id: AccountAuthMethodID
    timestamp: DateTime
    header_and_payload: bytes
    signature: bytes

    HEADER = b"PARSEC-MAC-BLAKE2B"

    # Only used for tests, but coherent to have it here
    @staticmethod
    def generate_raw(
        timestamp: DateTime,
        auth_method_id: AccountAuthMethodID,
        auth_method_mac_key: SecretKey,
    ) -> bytes:
        raw_auth_method_id = auth_method_id.hex.encode("ascii")
        raw_timestamp_us = str(timestamp.as_timestamp_seconds()).encode("ascii")
        header_and_payload = b"%s.%s.%s" % (
            AccountAuthenticationToken.HEADER,
            raw_auth_method_id,
            raw_timestamp_us,
        )
        signature = auth_method_mac_key.mac_512(header_and_payload)
        raw_signature = urlsafe_b64encode(signature)
        return b"%s.%s" % (header_and_payload, raw_signature)

    @classmethod
    def from_raw(cls, raw: bytes) -> "AccountAuthenticationToken":
        try:
            header_and_payload, raw_signature = raw.rsplit(b".", 1)
            header, raw_auth_method_id, raw_timestamp_us = header_and_payload.split(b".")
            if header != cls.HEADER:
                raise ValueError
            signature = urlsafe_b64decode(raw_signature)
            auth_method_id = AccountAuthMethodID.from_hex(raw_auth_method_id.decode("ascii"))
            timestamp = DateTime.from_timestamp_seconds(int(raw_timestamp_us))
        except ValueError as e:
            raise ValueError("Invalid token format") from e

        return cls(
            auth_method_id=auth_method_id,
            timestamp=timestamp,
            header_and_payload=header_and_payload,
            signature=signature,
        )

    def verify(self, auth_method_mac_key: SecretKey) -> bool:
        expected_signature = auth_method_mac_key.mac_512(self.header_and_payload)
        return self.signature == expected_signature


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
            # If TOS has changed they must be re-accepted by the users, so we clear the
            # cache given that the TOS acceptance is checked on cache miss.
            case EventOrganizationTosUpdated():
                self._device_cache = {
                    (org_id, device_id): v
                    for ((org_id, device_id), v) in self._device_cache.items()
                    if org_id != event.organization_id
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
        tos_acceptance_required: bool = True,
    ) -> AuthenticatedAuthInfo | AuthAuthenticatedAuthBadOutcome:
        try:
            # The cache is only available if the authentication already succeeded,
            # and if no revocation or freezing occurred since then.
            auth_info = self._device_cache[(organization_id, token.device_id)]

        except KeyError:
            outcome = await self._get_authenticated_info(
                organization_id, token.device_id, tos_acceptance_required=tos_acceptance_required
            )
            match outcome:
                case AuthenticatedAuthInfo() as auth_info:
                    # Terms Of Services acceptance is only checked on cache miss,
                    # so we shouldn't updated the cache if we got our auth info
                    # without having checked the TOS acceptance !
                    if tos_acceptance_required:
                        self._device_cache[(organization_id, token.device_id)] = auth_info

                case AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND:
                    # Cannot store cache as the organization might be created at anytime !
                    return AuthAuthenticatedAuthBadOutcome.ORGANIZATION_NOT_FOUND

                case bad_outcome:
                    return bad_outcome

        if not token.verify(auth_info.device_verify_key):
            return AuthAuthenticatedAuthBadOutcome.INVALID_TOKEN

        if timestamps_in_the_ballpark(token.timestamp, now) is not None:
            return AuthAuthenticatedAuthBadOutcome.TOKEN_OUT_OF_BALLPARK

        return auth_info

    async def _get_authenticated_info(
        self, organization_id: OrganizationID, device_id: DeviceID, tos_acceptance_required: bool
    ) -> AuthenticatedAuthInfo | AuthAuthenticatedAuthBadOutcome:
        raise NotImplementedError

    async def authenticated_account_auth(
        self,
        now: DateTime,
        token: AccountAuthenticationToken,
    ) -> AuthenticatedAccountAuthInfo | AuthAuthenticatedAccountAuthBadOutcome:
        raise NotImplementedError
