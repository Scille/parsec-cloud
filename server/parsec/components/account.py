# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from enum import auto
from uuid import uuid4

from parsec.types import BadOutcomeEnum


class CreateAccountBadOutcome(BadOutcomeEnum):
    ACCOUNT_ALREADY_EXISTS = auto()


class BaseAccountComponent:
    async def create(self, email: str) -> None | CreateAccountBadOutcome:
        raise NotImplementedError

    async def check_signature(self):
        raise NotImplementedError

    async def test_new_account(self) -> str:
        # generate unique data
        # the email is generated too to avoid concurrence during tests

        email = f"{uuid4().hex[:4]}@invalid.com"

        # create account
        assert await self.create(email) is None

        return email
