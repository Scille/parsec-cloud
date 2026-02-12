# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import assert_never, override

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
from parsec.components.memory.datamodel import (
    MemoryDatamodel,
    MemoryTOTPThrottle,
)
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
    generate_totp_secret,
    verify_totp,
)
from parsec.config import BackendConfig


class MemoryTOTPComponent(BaseTOTPComponent):
    def __init__(self, data: MemoryDatamodel, config: BackendConfig) -> None:
        super().__init__(config)
        self._data = data

    @override
    async def setup_get_secret(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> bytes | TOTPSetupGetSecretBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return TOTPSetupGetSecretBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return TOTPSetupGetSecretBadOutcome.ORGANIZATION_EXPIRED

        try:
            device = org.devices[author]
        except KeyError:
            return TOTPSetupGetSecretBadOutcome.AUTHOR_NOT_FOUND

        try:
            user = org.users[device.cooked.user_id]
        except KeyError:
            return TOTPSetupGetSecretBadOutcome.AUTHOR_NOT_FOUND

        if user.is_revoked:
            return TOTPSetupGetSecretBadOutcome.AUTHOR_REVOKED

        if user.totp_setup_completed:
            return TOTPSetupGetSecretBadOutcome.ALREADY_SETUP

        if user.totp_secret is None:
            user.totp_secret = generate_totp_secret()

        return user.totp_secret

    @override
    async def setup_confirm(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        one_time_password: str,
    ) -> None | TOTPSetupConfirmBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return TOTPSetupConfirmBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return TOTPSetupConfirmBadOutcome.ORGANIZATION_EXPIRED

        try:
            device = org.devices[author]
        except KeyError:
            return TOTPSetupConfirmBadOutcome.AUTHOR_NOT_FOUND

        try:
            user = org.users[device.cooked.user_id]
        except KeyError:
            return TOTPSetupConfirmBadOutcome.AUTHOR_NOT_FOUND
        if user.is_revoked:
            return TOTPSetupConfirmBadOutcome.AUTHOR_REVOKED

        if user.totp_setup_completed:
            return TOTPSetupConfirmBadOutcome.ALREADY_SETUP

        if user.totp_secret is None:
            return TOTPSetupConfirmBadOutcome.INVALID_ONE_TIME_PASSWORD

        if not verify_totp(
            now=DateTime.now(), secret=user.totp_secret, one_time_password=one_time_password
        ):
            return TOTPSetupConfirmBadOutcome.INVALID_ONE_TIME_PASSWORD

        user.totp_setup_completed = True
        user.totp_reset_token = None

    @override
    async def setup_get_secret_with_token(
        self,
        organization_id: OrganizationID,
        user_id: UserID,
        token: AccessToken,
    ) -> bytes | TOTPSetupGetSecretWithTokenBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return TOTPSetupGetSecretWithTokenBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return TOTPSetupGetSecretWithTokenBadOutcome.ORGANIZATION_EXPIRED

        try:
            user = org.users[user_id]
        except KeyError:
            return TOTPSetupGetSecretWithTokenBadOutcome.USER_NOT_FOUND

        if user.is_revoked:
            return TOTPSetupGetSecretWithTokenBadOutcome.USER_REVOKED

        if user.is_frozen:
            return TOTPSetupGetSecretWithTokenBadOutcome.USER_FROZEN

        if user.totp_reset_token != token:
            return TOTPSetupGetSecretWithTokenBadOutcome.BAD_TOKEN

        if user.totp_setup_completed:
            return TOTPSetupGetSecretWithTokenBadOutcome.ALREADY_SETUP

        if user.totp_secret is None:
            user.totp_secret = generate_totp_secret()

        return user.totp_secret

    @override
    async def setup_confirm_with_token(
        self,
        organization_id: OrganizationID,
        user_id: UserID,
        token: AccessToken,
        one_time_password: str,
    ) -> None | TOTPSetupConfirmWithTokenBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return TOTPSetupConfirmWithTokenBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return TOTPSetupConfirmWithTokenBadOutcome.ORGANIZATION_EXPIRED

        try:
            user = org.users[user_id]
        except KeyError:
            return TOTPSetupConfirmWithTokenBadOutcome.USER_NOT_FOUND

        if user.is_revoked:
            return TOTPSetupConfirmWithTokenBadOutcome.USER_REVOKED

        if user.is_frozen:
            return TOTPSetupConfirmWithTokenBadOutcome.USER_FROZEN

        if user.totp_setup_completed:
            return TOTPSetupConfirmWithTokenBadOutcome.ALREADY_SETUP

        if user.totp_reset_token != token:
            return TOTPSetupConfirmWithTokenBadOutcome.BAD_TOKEN

        if user.totp_secret is None:
            return TOTPSetupConfirmWithTokenBadOutcome.INVALID_ONE_TIME_PASSWORD

        if not verify_totp(
            now=DateTime.now(), secret=user.totp_secret, one_time_password=one_time_password
        ):
            return TOTPSetupConfirmWithTokenBadOutcome.INVALID_ONE_TIME_PASSWORD

        user.totp_setup_completed = True
        user.totp_reset_token = None

    @override
    async def create_opaque_key(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> tuple[TOTPOpaqueKeyID, SecretKey] | TOTPCreateOpaqueKeyBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return TOTPCreateOpaqueKeyBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return TOTPCreateOpaqueKeyBadOutcome.ORGANIZATION_EXPIRED

        try:
            device = org.devices[author]
        except KeyError:
            return TOTPCreateOpaqueKeyBadOutcome.AUTHOR_NOT_FOUND

        try:
            user = org.users[device.cooked.user_id]
        except KeyError:
            return TOTPCreateOpaqueKeyBadOutcome.AUTHOR_NOT_FOUND
        if user.is_revoked:
            return TOTPCreateOpaqueKeyBadOutcome.AUTHOR_REVOKED

        opaque_key_id = TOTPOpaqueKeyID.new()
        opaque_key = SecretKey.generate()

        user.totp_opaque_keys[opaque_key_id] = opaque_key, MemoryTOTPThrottle()

        return (opaque_key_id, opaque_key)

    @override
    async def fetch_opaque_key(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        user_id: UserID,
        opaque_key_id: TOTPOpaqueKeyID,
        one_time_password: str,
    ) -> SecretKey | TOTPFetchOpaqueKeyBadOutcome | TOTPFetchOpaqueKeyThrottled:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return TOTPFetchOpaqueKeyBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return TOTPFetchOpaqueKeyBadOutcome.ORGANIZATION_EXPIRED

        try:
            user = org.users[user_id]
        except KeyError:
            return TOTPFetchOpaqueKeyBadOutcome.USER_NOT_FOUND

        if user.is_revoked:
            return TOTPFetchOpaqueKeyBadOutcome.USER_REVOKED

        if user.is_frozen:
            return TOTPFetchOpaqueKeyBadOutcome.USER_FROZEN

        if not user.totp_setup_completed:
            return TOTPFetchOpaqueKeyBadOutcome.NOT_SETUP

        assert user.totp_secret is not None

        try:
            opaque_key, throttle = user.totp_opaque_keys[opaque_key_id]
        except KeyError:
            return TOTPFetchOpaqueKeyBadOutcome.OPAQUE_KEY_NOT_FOUND

        match throttle.wait_until:
            case None:
                pass  # No throttle needed
            case wait_until if now > wait_until:
                pass  # Throttle needed, but we are already passed it
            case wait_until:
                return TOTPFetchOpaqueKeyThrottled(wait_until=wait_until)

        if not verify_totp(now=now, secret=user.totp_secret, one_time_password=one_time_password):
            throttle.register_failed_attempt(now)
            return TOTPFetchOpaqueKeyBadOutcome.INVALID_ONE_TIME_PASSWORD

        throttle.reset()

        return opaque_key

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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return TOTPResetBadOutcome.ORGANIZATION_NOT_FOUND

        match (user_id, user_email):
            case (None, None):
                return TOTPResetBadOutcome.NO_USER_ID_NOR_EMAIL
            case (UserID() as user_id, None):
                try:
                    user = org.users[user_id]
                except KeyError:
                    return TOTPResetBadOutcome.USER_NOT_FOUND
            case (None, EmailAddress() as user_email):
                # Multiple user can have the same email, but at most one of them is active
                for user in org.active_users():
                    if user.cooked.human_handle.email == user_email:
                        user_id = user.cooked.user_id
                        break
                else:
                    return TOTPResetBadOutcome.USER_NOT_FOUND
            case (UserID(), EmailAddress()):
                return TOTPResetBadOutcome.BOTH_USER_ID_AND_EMAIL
            case never:  # pyright: ignore [reportUnnecessaryComparison]
                assert_never(never)

        if user.is_revoked:
            return TOTPResetBadOutcome.USER_REVOKED

        user.totp_setup_completed = False
        user.totp_secret = None
        user.totp_reset_token = AccessToken.new()
        for _, throttle in user.totp_opaque_keys.values():
            throttle.reset()

        totp_reset_url = ParsecTOTPResetAddr.build(
            server_addr=self._config.server_addr,
            organization_id=organization_id,
            user_id=user_id,
            token=user.totp_reset_token,
        )

        if send_email:
            send_email_outcome = await self._send_totp_reset_email(
                organization_id=organization_id,
                totp_reset_url=totp_reset_url,
                to_addr=user.cooked.human_handle.email,
            )
        else:
            send_email_outcome = None

        return (user_id, user.cooked.human_handle.email, totp_reset_url, send_email_outcome)
