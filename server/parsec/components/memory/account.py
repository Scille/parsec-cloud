# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from typing import override

from parsec.components.account import BaseAccountComponent, CreateAccountBadOutcome
from parsec.components.events import EventBus
from parsec.components.memory.datamodel import MemoryAccount, MemoryDatamodel


class MemoryAccountComponent(BaseAccountComponent):
    def __init__(self, data: MemoryDatamodel, event_bus: EventBus) -> None:
        super().__init__()
        self._data = data
        self._event_bus = event_bus

    @override
    async def create(self, email: str) -> None | CreateAccountBadOutcome:
        if email in self._data.accounts:
            return CreateAccountBadOutcome.ACCOUNT_ALREADY_EXISTS
        else:
            self._data.accounts[email] = MemoryAccount(email)

    @override
    async def check_signature(self):
        pass
