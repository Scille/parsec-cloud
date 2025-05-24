# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import override

from pydantic import EmailStr

from parsec._parsec import DateTime, EmailValidationToken, SecretKey
from parsec.components.account import (
    BaseAccountComponent,
    CreateAccountWithPasswordBadOutcome,
    CreateEmailValidationTokenBadOutcome,
    GetPasswordSecretKeyBadOutcome,
    anonymous_account_cmds,
)
from parsec.components.events import EventBus
from parsec.components.memory.datamodel import (
    MemoryAccount,
    MemoryAuthenticationMethod,
    MemoryDatamodel,
)
from parsec.config import BackendConfig

PasswordAlgorithm = (
    anonymous_account_cmds.latest.account_create_with_password_proceed.PasswordAlgorithm
)


class MemoryAccountComponent(BaseAccountComponent):
    def __init__(
        self,
        data: MemoryDatamodel,
        config: BackendConfig,
        event_bus: EventBus,
    ) -> None:
        super().__init__(config)
        self._data = data
        self._event_bus = event_bus

    @override
    async def create_email_validation_token(
        self, email: EmailStr, now: DateTime
    ) -> EmailValidationToken | CreateEmailValidationTokenBadOutcome:
        if email in self._data.accounts:
            return CreateEmailValidationTokenBadOutcome.ACCOUNT_ALREADY_EXISTS
        elif email in self._data.unverified_emails:
            (_, last_email_datetime) = self._data.unverified_emails[email]
            if not self.should_resend_token(now, last_email_datetime):
                return CreateEmailValidationTokenBadOutcome.TOO_SOON_AFTER_PREVIOUS_DEMAND

        token = EmailValidationToken.new()
        self._data.unverified_emails[email] = (token, now)
        return token

    @override
    async def create_account_with_password(
        self,
        token: EmailValidationToken,
        now: DateTime,
        mac_key: SecretKey,
        vault_key_access: bytes,
        human_label: str,
        created_by_user_agent: str,
        created_by_ip: str,
        password_secret_algorithm: PasswordAlgorithm,
    ) -> None | CreateAccountWithPasswordBadOutcome:
        # check token
        # TODO check time

        unverified_email = next(
            (k for (k, v) in self._data.unverified_emails.items() if v[0] == token), None
        )
        if unverified_email is None:  # the provided token is not in unverified emails
            return CreateAccountWithPasswordBadOutcome.INVALID_TOKEN
        email = unverified_email

        # create authentication method
        auth_method = MemoryAuthenticationMethod(
            created_on=now,
            created_by_ip=created_by_ip,
            created_by_user_agent=created_by_user_agent,
            mac_key=mac_key,
            vault_key_access=vault_key_access,
            password_secret_algorithm=password_secret_algorithm,
            enabled=True,
        )

        self._data.accounts[email] = MemoryAccount(
            user_email=email,
            human_label=human_label,
            vaults=[],
            authentication_methods=[auth_method],
        )

        # remove from unverified emails at the end
        self._data.unverified_emails.pop(email)

    @override
    async def check_signature(self):
        pass

    @override
    def get_password_mac_key(
        self, user_email: EmailStr
    ) -> SecretKey | GetPasswordSecretKeyBadOutcome:
        account = self._data.accounts.get(user_email)
        if account is None:
            return GetPasswordSecretKeyBadOutcome.USER_NOT_FOUND
        assert isinstance(account, MemoryAccount)
        secret_key = next((a.mac_key for a in account.authentication_methods if a.enabled), None)
        # TODO, when other auth methods are introduced check for the password one
        if secret_key is None:
            return GetPasswordSecretKeyBadOutcome.UNABLE_TO_GET_SECRET_KEY
        return secret_key

    @override
    def test_get_token_by_email(self, email: str) -> EmailValidationToken | None:
        """Use only in tests, nothing is checked."""
        return self._data.unverified_emails[email][0]
