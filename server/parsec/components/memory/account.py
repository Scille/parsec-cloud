# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from typing import override

from parsec.components.account import BaseAccountComponent, CreateAccountBadOutcome
from parsec.components.events import EventBus
from parsec.components.memory.datamodel import (
    MemoryAccount,
    MemoryDatamodel,
    UserAccountDataValidationModel,
)


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
            try:
                user_email = UserAccountDataValidationModel(user_email=email)
                self._data.accounts[email] = MemoryAccount(user_email=user_email.user_email)
            except ValueError:
                return CreateAccountBadOutcome.INVALID_EMAIL

    @override
    async def check_signature(self):
        pass
