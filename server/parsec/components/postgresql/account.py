# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from pydantic import EmailStr

from parsec._parsec import DateTime, EmailValidationToken
from parsec.components.account import BaseAccountComponent, CreateEmailValidationTokenBadOutcome
from parsec.components.postgresql import AsyncpgPool
from parsec.config import BackendConfig


class PGAccountComponent(BaseAccountComponent):
    def __init__(self, pool: AsyncpgPool, config: BackendConfig) -> None:
        super().__init__(config=config)
        self.pool = pool

    async def create_email_validation_token(
        self, email: EmailStr, now: DateTime
    ) -> EmailValidationToken | CreateEmailValidationTokenBadOutcome:
        raise NotImplementedError

    async def check_signature(self):
        pass
