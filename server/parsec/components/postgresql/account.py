# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from parsec.components.account import BaseAccountComponent, CreateAccountBadOutcome
from parsec.components.postgresql import AsyncpgPool


class PGAccountComponent(BaseAccountComponent):
    def __init__(self, pool: AsyncpgPool) -> None:
        super().__init__()
        self.pool = pool

    async def create(self, email: str) -> None | CreateAccountBadOutcome:
        pass

    async def check_signature(self):
        pass
