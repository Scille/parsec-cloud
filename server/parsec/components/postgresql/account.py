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
from parsec.components.postgresql import AsyncpgPool
from parsec.config import BackendConfig

PasswordAlgorithm = (
    anonymous_account_cmds.latest.account_create_with_password_proceed.PasswordAlgorithm
)


class PGAccountComponent(BaseAccountComponent):
    def __init__(self, pool: AsyncpgPool, config: BackendConfig) -> None:
        super().__init__(config=config)
        self.pool = pool

    @override
    async def create_email_validation_token(
        self, email: EmailStr, now: DateTime
    ) -> EmailValidationToken | CreateEmailValidationTokenBadOutcome:
        raise NotImplementedError

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
        raise NotImplementedError

    @override
    def get_password_mac_key(
        self, user_email: EmailStr
    ) -> SecretKey | GetPasswordSecretKeyBadOutcome:
        raise NotImplementedError
