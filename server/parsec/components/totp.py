# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import hashlib
import hmac
import secrets
import struct
from dataclasses import dataclass
from email.message import Message
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
from enum import auto

from jinja2 import Environment

from parsec._parsec import (
    AccessToken,
    DateTime,
    DeviceID,
    EmailAddress,
    OrganizationID,
    ParsecTOTPResetAddr,
    SecretKey,
    TOTPOpaqueKeyID,
    UserID,
    anonymous_cmds,
    authenticated_cmds,
)
from parsec.api import api
from parsec.client_context import AnonymousClientContext, AuthenticatedClientContext
from parsec.components.email import SendEmailBadOutcome, send_email
from parsec.config import BackendConfig
from parsec.types import BadOutcomeEnum


def compute_totp_one_time_password(
    now: DateTime,
    secret: bytes,
    digits: int = 6,
    time_step_seconds: int = 30,
    window_offset: int = 0,
):
    # Implement RFC 6238 (Time-Based One-Time Password Algorithm)
    # Note SHA1, 30-second time step, 6-digit codes are RFC 6238 defaults.
    # see https://datatracker.ietf.org/doc/html/rfc6238

    # TOTP is just HOTP (RFC 4226) with a counter derived from the time
    counter = (now.as_timestamp_seconds() // time_step_seconds) + window_offset

    # Implement RFC 4226 (HMAC-Based One-Time Password Algorithm)
    # see https://datatracker.ietf.org/doc/html/rfc4226#section-5.3

    # Step 1: Generate an HMAC-SHA-1 value
    msg = struct.pack(">Q", counter)
    h = hmac.new(secret, msg, hashlib.sha1).digest()
    # A portion of the HMAC is extracted and displayed to the user as a six- to eight-digit code;

    # Step 2: Generate a 4-byte string (Dynamic Truncation)
    # The last nibble (4 bits) of the result is used as a pointer...
    pointer = h[-1] & 0x0F
    # ...to a 32-bit integer, in the result byte array...
    code = struct.unpack(">I", h[pointer : pointer + 4])[0]
    # ...and masks out the 31st bit.
    code = code & 0x7FFFFFFF

    # Step 3: Compute an HOTP value by truncating to the required number of digits
    code = code % (10**digits)
    return str(code).zfill(digits)


def generate_totp_secret() -> bytes:
    # RFC 4226 recommends 160 bits (i.e. 20 bytes) long secret for TOTP
    # see https://datatracker.ietf.org/doc/html/rfc4226#section-4
    return secrets.token_bytes(20)


def verify_totp(now: DateTime, secret: bytes, one_time_password: str, window: int = 1) -> bool:
    # Window parameter allows checking adjacent time steps for clock skew.
    for window_offset in range(-window, window + 1):
        expected = compute_totp_one_time_password(
            now=now, secret=secret, window_offset=window_offset
        )
        if one_time_password == expected:
            return True
    return False


def generate_totp_reset_email(
    jinja_env: Environment,
    from_addr: EmailAddress,
    to_addr: EmailAddress,
    organization_id: OrganizationID,
    totp_reset_url: str,
    server_url: str,
) -> Message:
    html = jinja_env.get_template("email/totp_reset.html.j2").render(
        organization_id=organization_id.str,
        totp_reset_url=totp_reset_url,
        server_url=server_url,
    )
    text = jinja_env.get_template("email/totp_reset.txt.j2").render(
        organization_id=organization_id.str,
        totp_reset_url=totp_reset_url,
    )

    message = MIMEMultipart("alternative")
    message["Subject"] = f"[Parsec] TOTP reset for {organization_id.str}"
    message["From"] = str(from_addr)
    message["To"] = str(to_addr)

    part1 = MIMEText(text, "plain")
    part2 = MIMEText(html, "html")

    message.attach(part1)
    message.attach(part2)

    return message


class TOTPSetupGetSecretBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    ALREADY_SETUP = auto()


class TOTPSetupConfirmBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()
    INVALID_ONE_TIME_PASSWORD = auto()
    ALREADY_SETUP = auto()


class TOTPSetupGetSecretWithTokenBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    USER_NOT_FOUND = auto()
    USER_REVOKED = auto()
    # Frozen is an annoying corner case that we normally only care for in
    # the auth code.
    # However there is no authentication since this command is anonymous (hence
    # the need to provide a token), so we also have to check for frozen user here.
    USER_FROZEN = auto()
    ALREADY_SETUP = auto()
    BAD_TOKEN = auto()


class TOTPSetupConfirmWithTokenBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    USER_NOT_FOUND = auto()
    USER_REVOKED = auto()
    # Frozen is an annoying corner case that we normally only care for in
    # the auth code.
    # However there is no authentication since this command is anonymous (hence
    # the need to provide a token), so we also have to check for frozen user here.
    USER_FROZEN = auto()
    ALREADY_SETUP = auto()
    BAD_TOKEN = auto()
    INVALID_ONE_TIME_PASSWORD = auto()


class TOTPCreateOpaqueKeyBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    AUTHOR_NOT_FOUND = auto()
    AUTHOR_REVOKED = auto()


class TOTPFetchOpaqueKeyBadOutcome(BadOutcomeEnum):
    ORGANIZATION_NOT_FOUND = auto()
    ORGANIZATION_EXPIRED = auto()
    USER_NOT_FOUND = auto()
    USER_REVOKED = auto()
    # Frozen is an annoying corner case that we normally only care for in
    # the auth code.
    # However there is no authentication since this command is anonymous (hence
    # the need to provide a token), so we also have to check for frozen user here.
    USER_FROZEN = auto()
    NOT_SETUP = auto()
    OPAQUE_KEY_NOT_FOUND = auto()
    INVALID_ONE_TIME_PASSWORD = auto()


class TOTPResetBadOutcome(BadOutcomeEnum):
    # Note we don't care the organization is expired here, this is because this
    # command is used by the administration.
    ORGANIZATION_NOT_FOUND = auto()
    USER_NOT_FOUND = auto()
    USER_REVOKED = auto()
    NO_USER_ID_NOR_EMAIL = auto()
    BOTH_USER_ID_AND_EMAIL = auto()


@dataclass(slots=True)
class TOTPFetchOpaqueKeyThrottled:
    wait_until: DateTime


class BaseTOTPComponent:
    def __init__(self, config: BackendConfig):
        self._config = config

    # Used by `reset` implementations
    async def _send_totp_reset_email(
        self,
        organization_id: OrganizationID,
        totp_reset_url: ParsecTOTPResetAddr,
        to_addr: EmailAddress,
    ) -> None | SendEmailBadOutcome:
        if not self._config.server_addr:
            return SendEmailBadOutcome.BAD_SMTP_CONFIG

        message = generate_totp_reset_email(
            jinja_env=self._config.jinja_env,
            from_addr=self._config.email_config.sender,
            to_addr=to_addr,
            organization_id=organization_id,
            totp_reset_url=totp_reset_url.to_http_redirection_url(),
            server_url=self._config.server_addr.to_http_url(),
        )

        return await send_email(
            email_config=self._config.email_config,
            to_addr=to_addr,
            message=message,
        )

    # Authenticated TOTP setup

    async def setup_get_secret(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> bytes | TOTPSetupGetSecretBadOutcome:
        raise NotImplementedError

    async def setup_confirm(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        one_time_password: str,
    ) -> None | TOTPSetupConfirmBadOutcome:
        raise NotImplementedError

    # Anonymous TOTP setup (with reset token)

    async def setup_get_secret_with_token(
        self,
        organization_id: OrganizationID,
        user_id: UserID,
        token: AccessToken,
    ) -> bytes | TOTPSetupGetSecretWithTokenBadOutcome:
        raise NotImplementedError

    async def setup_confirm_with_token(
        self,
        organization_id: OrganizationID,
        user_id: UserID,
        token: AccessToken,
        one_time_password: str,
    ) -> None | TOTPSetupConfirmWithTokenBadOutcome:
        raise NotImplementedError

    # Opaque key management

    async def create_opaque_key(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> tuple[TOTPOpaqueKeyID, SecretKey] | TOTPCreateOpaqueKeyBadOutcome:
        raise NotImplementedError

    async def fetch_opaque_key(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        user_id: UserID,
        opaque_key_id: TOTPOpaqueKeyID,
        one_time_password: str,
    ) -> SecretKey | TOTPFetchOpaqueKeyBadOutcome | TOTPFetchOpaqueKeyThrottled:
        raise NotImplementedError

    async def reset(
        self,
        organization_id: OrganizationID,
        user_id: UserID | None = None,
        user_email: EmailAddress | None = None,
        send_email: bool = False,
    ) -> (
        tuple[UserID, EmailAddress, ParsecTOTPResetAddr, None | SendEmailBadOutcome]
        | TOTPResetBadOutcome
    ):
        raise NotImplementedError

    # Authenticated API handlers

    @api
    async def authenticated_api_totp_setup_get_secret(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.totp_setup_get_secret.Req,
    ) -> authenticated_cmds.latest.totp_setup_get_secret.Rep:
        outcome = await self.setup_get_secret(
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
        )
        match outcome:
            case bytes() as totp_secret:
                return authenticated_cmds.latest.totp_setup_get_secret.RepOk(
                    totp_secret=totp_secret
                )
            case TOTPSetupGetSecretBadOutcome.ALREADY_SETUP:
                return authenticated_cmds.latest.totp_setup_get_secret.RepAlreadySetup()
            case TOTPSetupGetSecretBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case TOTPSetupGetSecretBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case TOTPSetupGetSecretBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case TOTPSetupGetSecretBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def authenticated_api_totp_setup_confirm(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.totp_setup_confirm.Req,
    ) -> authenticated_cmds.latest.totp_setup_confirm.Rep:
        outcome = await self.setup_confirm(
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
            one_time_password=req.one_time_password,
        )
        match outcome:
            case None:
                return authenticated_cmds.latest.totp_setup_confirm.RepOk()
            case TOTPSetupConfirmBadOutcome.INVALID_ONE_TIME_PASSWORD:
                return authenticated_cmds.latest.totp_setup_confirm.RepInvalidOneTimePassword()
            case TOTPSetupConfirmBadOutcome.ALREADY_SETUP:
                return authenticated_cmds.latest.totp_setup_confirm.RepAlreadySetup()
            case TOTPSetupConfirmBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case TOTPSetupConfirmBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case TOTPSetupConfirmBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case TOTPSetupConfirmBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    @api
    async def authenticated_api_totp_create_opaque_key(
        self,
        client_ctx: AuthenticatedClientContext,
        req: authenticated_cmds.latest.totp_create_opaque_key.Req,
    ) -> authenticated_cmds.latest.totp_create_opaque_key.Rep:
        outcome = await self.create_opaque_key(
            organization_id=client_ctx.organization_id,
            author=client_ctx.device_id,
        )
        match outcome:
            case (opaque_key_id, opaque_key):
                return authenticated_cmds.latest.totp_create_opaque_key.RepOk(
                    opaque_key_id=opaque_key_id,
                    opaque_key=opaque_key,
                )
            case TOTPCreateOpaqueKeyBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case TOTPCreateOpaqueKeyBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
            case TOTPCreateOpaqueKeyBadOutcome.AUTHOR_NOT_FOUND:
                client_ctx.author_not_found_abort()
            case TOTPCreateOpaqueKeyBadOutcome.AUTHOR_REVOKED:
                client_ctx.author_revoked_abort()

    # Anonymous API handlers

    @api
    async def anonymous_api_totp_setup_get_secret(
        self,
        client_ctx: AnonymousClientContext,
        req: anonymous_cmds.latest.totp_setup_get_secret.Req,
    ) -> anonymous_cmds.latest.totp_setup_get_secret.Rep:
        outcome = await self.setup_get_secret_with_token(
            organization_id=client_ctx.organization_id,
            user_id=req.user_id,
            token=req.token,
        )
        match outcome:
            case bytes() as totp_secret:
                return anonymous_cmds.latest.totp_setup_get_secret.RepOk(totp_secret=totp_secret)
            case (
                TOTPSetupGetSecretWithTokenBadOutcome.USER_NOT_FOUND
                | TOTPSetupGetSecretWithTokenBadOutcome.USER_REVOKED
                | TOTPSetupGetSecretWithTokenBadOutcome.USER_FROZEN
                | TOTPSetupGetSecretWithTokenBadOutcome.ALREADY_SETUP
                | TOTPSetupGetSecretWithTokenBadOutcome.BAD_TOKEN
            ):
                return anonymous_cmds.latest.totp_setup_get_secret.RepBadToken()
            case TOTPSetupGetSecretWithTokenBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case TOTPSetupGetSecretWithTokenBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()

    @api
    async def anonymous_api_totp_setup_confirm(
        self,
        client_ctx: AnonymousClientContext,
        req: anonymous_cmds.latest.totp_setup_confirm.Req,
    ) -> anonymous_cmds.latest.totp_setup_confirm.Rep:
        outcome = await self.setup_confirm_with_token(
            organization_id=client_ctx.organization_id,
            user_id=req.user_id,
            token=req.token,
            one_time_password=req.one_time_password,
        )
        match outcome:
            case None:
                return anonymous_cmds.latest.totp_setup_confirm.RepOk()
            case TOTPSetupConfirmWithTokenBadOutcome.INVALID_ONE_TIME_PASSWORD:
                return anonymous_cmds.latest.totp_setup_confirm.RepInvalidOneTimePassword()
            case (
                TOTPSetupConfirmWithTokenBadOutcome.USER_NOT_FOUND
                | TOTPSetupConfirmWithTokenBadOutcome.USER_REVOKED
                | TOTPSetupConfirmWithTokenBadOutcome.USER_FROZEN
                | TOTPSetupConfirmWithTokenBadOutcome.ALREADY_SETUP
                | TOTPSetupConfirmWithTokenBadOutcome.BAD_TOKEN
            ):
                return anonymous_cmds.latest.totp_setup_confirm.RepBadToken()
            case TOTPSetupConfirmWithTokenBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case TOTPSetupConfirmWithTokenBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()

    @api
    async def api_totp_fetch_opaque_key(
        self,
        client_ctx: AnonymousClientContext,
        req: anonymous_cmds.latest.totp_fetch_opaque_key.Req,
    ) -> anonymous_cmds.latest.totp_fetch_opaque_key.Rep:
        outcome = await self.fetch_opaque_key(
            now=DateTime.now(),
            organization_id=client_ctx.organization_id,
            user_id=req.user_id,
            opaque_key_id=req.opaque_key_id,
            one_time_password=req.one_time_password,
        )
        match outcome:
            case SecretKey() as opaque_key:
                return anonymous_cmds.latest.totp_fetch_opaque_key.RepOk(
                    opaque_key=opaque_key,
                )
            case (
                TOTPFetchOpaqueKeyBadOutcome.USER_NOT_FOUND
                | TOTPFetchOpaqueKeyBadOutcome.USER_REVOKED
                | TOTPFetchOpaqueKeyBadOutcome.USER_FROZEN
                | TOTPFetchOpaqueKeyBadOutcome.NOT_SETUP
                | TOTPFetchOpaqueKeyBadOutcome.OPAQUE_KEY_NOT_FOUND
                | TOTPFetchOpaqueKeyBadOutcome.INVALID_ONE_TIME_PASSWORD
            ):
                return anonymous_cmds.latest.totp_fetch_opaque_key.RepInvalidOneTimePassword()
            case TOTPFetchOpaqueKeyThrottled() as throttled:
                return anonymous_cmds.latest.totp_fetch_opaque_key.RepThrottled(
                    wait_until=throttled.wait_until,
                )
            case TOTPFetchOpaqueKeyBadOutcome.ORGANIZATION_NOT_FOUND:
                client_ctx.organization_not_found_abort()
            case TOTPFetchOpaqueKeyBadOutcome.ORGANIZATION_EXPIRED:
                client_ctx.organization_expired_abort()
