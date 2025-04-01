# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from enum import auto

from parsec._parsec import AccountToken
from parsec.types import BadOutcomeEnum


class CreateAccountBadOutcome(BadOutcomeEnum):
    ACCOUNT_ALREADY_EXISTS = auto()


class BaseAccountComponent:
    async def create(self, email: str, secret: AccountToken) -> None | CreateAccountBadOutcome:
        raise NotImplementedError

    async def check_signature(self, key: AccountToken):
        raise NotImplementedError

    async def test_new_account(self) -> tuple[str, AccountToken]:
        # generate unique data
        # the email is generated too to avoid concurrence during tests
        token = AccountToken.new()
        email = f"{token}@invalid.com"

        # create account
        assert await self.create(email, token) is None

        return (email, token)
