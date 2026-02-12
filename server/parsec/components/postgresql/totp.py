# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

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
)
from parsec.components.email import SendEmailBadOutcome
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.totp_create_opaque_key import totp_create_opaque_key
from parsec.components.postgresql.totp_fetch_opaque_key import totp_fetch_opaque_key
from parsec.components.postgresql.totp_reset import totp_reset
from parsec.components.postgresql.totp_setup_confirm import (
    totp_setup_confirm,
    totp_setup_confirm_with_token,
)
from parsec.components.postgresql.totp_setup_get_secret import (
    totp_setup_get_secret,
    totp_setup_get_secret_with_token,
)
from parsec.components.postgresql.utils import no_transaction, transaction
from parsec.components.totp import (
    BaseTOTPComponent,
    TOTPCreateOpaqueKeyBadOutcome,
    TOTPFetchOpaqueKeyBadOutcome,
    TOTPFetchOpaqueKeyThrottled,
    TOTPResetBadOutcome,
    TOTPSetupConfirmBadOutcome,
    TOTPSetupConfirmWithTokenBadOutcome,
    TOTPSetupGetSecretBadOutcome,
    TOTPSetupGetSecretWithTokenBadOutcome,
)
from parsec.config import BackendConfig


class PGTOTPComponent(BaseTOTPComponent):
    def __init__(self, pool: AsyncpgPool, config: BackendConfig) -> None:
        super().__init__(config)
        self.pool = pool

    @override
    @transaction  # Transaction is needed since the TOTP secret is lazily created
    async def setup_get_secret(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> bytes | TOTPSetupGetSecretBadOutcome:
        return await totp_setup_get_secret(conn, organization_id, author)

    @override
    @transaction
    async def setup_confirm(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
        one_time_password: str,
    ) -> None | TOTPSetupConfirmBadOutcome:
        return await totp_setup_confirm(conn, organization_id, author, one_time_password)

    @override
    @transaction  # Transaction is needed since the TOTP secret is lazily created
    async def setup_get_secret_with_token(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        user_id: UserID,
        token: AccessToken,
    ) -> bytes | TOTPSetupGetSecretWithTokenBadOutcome:
        return await totp_setup_get_secret_with_token(conn, organization_id, user_id, token)

    @override
    @transaction
    async def setup_confirm_with_token(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        user_id: UserID,
        token: AccessToken,
        one_time_password: str,
    ) -> None | TOTPSetupConfirmWithTokenBadOutcome:
        return await totp_setup_confirm_with_token(
            conn, organization_id, user_id, token, one_time_password
        )

    @override
    @transaction
    async def create_opaque_key(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> tuple[TOTPOpaqueKeyID, SecretKey] | TOTPCreateOpaqueKeyBadOutcome:
        return await totp_create_opaque_key(conn, organization_id, author)

    @override
    @no_transaction
    async def fetch_opaque_key(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        user_id: UserID,
        opaque_key_id: TOTPOpaqueKeyID,
        one_time_password: str,
    ) -> SecretKey | TOTPFetchOpaqueKeyBadOutcome | TOTPFetchOpaqueKeyThrottled:
        return await totp_fetch_opaque_key(
            conn, now, organization_id, user_id, opaque_key_id, one_time_password
        )

    @override
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
        # Must first do the PostgreSQL transaction...
        outcome = await self._reset_without_sending_email(
            organization_id=organization_id, user_id=user_id, user_email=user_email
        )
        match outcome:
            case (user_id, user_email_addr, totp_reset_token):
                pass
            case error:
                return error

        totp_reset_url = ParsecTOTPResetAddr.build(
            server_addr=self._config.server_addr,
            organization_id=organization_id,
            user_id=user_id,
            token=totp_reset_token,
        )

        # ...and only then send the email, otherwise we would needlessly lock the database for too long
        if send_email:
            send_email_outcome = await self._send_totp_reset_email(
                organization_id=organization_id,
                totp_reset_url=totp_reset_url,
                to_addr=user_email_addr,
            )
        else:
            send_email_outcome = None

        return (user_id, user_email_addr, totp_reset_url, send_email_outcome)

    @transaction
    async def _reset_without_sending_email(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        user_id: UserID | None = None,
        user_email: EmailAddress | None = None,
    ) -> tuple[UserID, EmailAddress, AccessToken] | TOTPResetBadOutcome:
        return await totp_reset(conn, organization_id, user_id=user_id, user_email=user_email)
