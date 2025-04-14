# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import override

from pydantic import EmailStr

from parsec._parsec import EmailValidationToken
from parsec.components.account import BaseAccountComponent, CreateEmailValidationTokenBadOutcome
from parsec.components.events import EventBus
from parsec.components.memory.datamodel import (
    MemoryDatamodel,
)


class MemoryAccountComponent(BaseAccountComponent):
    def __init__(self, data: MemoryDatamodel, event_bus: EventBus) -> None:
        super().__init__()
        self._data = data
        self._event_bus = event_bus

    @override
    async def create_email_validation_token(
        self, email: EmailStr
    ) -> EmailValidationToken | CreateEmailValidationTokenBadOutcome:
        if email in self._data.accounts:
            return CreateEmailValidationTokenBadOutcome.ACCOUNT_ALREADY_EXISTS
        else:
            token = EmailValidationToken.new()
            self._data.unverified_emails[email] = token
        return token

    @override
    async def check_signature(self):
        pass
