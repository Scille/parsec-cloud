# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from parsec._parsec import AccountToken
from parsec.components.account import BaseAccountComponent, CreateAccountBadOutcome


class PostgreAccountComponent(BaseAccountComponent):
    async def create(self, email: str, secret: AccountToken) -> None | CreateAccountBadOutcome:
        pass

    async def check_signature(self, key: AccountToken):
        pass
