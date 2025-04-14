# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from pydantic import EmailStr

from parsec._parsec import EmailValidationToken
from parsec.components.account import BaseAccountComponent, CreateEmailValidationTokenBadOutcome
from parsec.components.postgresql import AsyncpgPool


class PGAccountComponent(BaseAccountComponent):
    def __init__(self, pool: AsyncpgPool) -> None:
        super().__init__()
        self.pool = pool

    async def create_email_validation_token(
        self, email: EmailStr
    ) -> EmailValidationToken | CreateEmailValidationTokenBadOutcome:
        raise NotImplementedError

    async def check_signature(self):
        pass
