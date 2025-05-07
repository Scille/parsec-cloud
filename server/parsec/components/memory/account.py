# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import override

from pydantic import EmailStr

from parsec._parsec import DateTime, EmailValidationToken
from parsec.components.account import BaseAccountComponent, CreateEmailValidationTokenBadOutcome
from parsec.components.events import EventBus
from parsec.components.memory.datamodel import (
    MemoryDatamodel,
)
from parsec.config import BackendConfig


class MemoryAccountComponent(BaseAccountComponent):
    def __init__(
        self,
        data: MemoryDatamodel,
        config: BackendConfig,
        event_bus: EventBus,
    ) -> None:
        super().__init__(config)
        self._data = data
        self._event_bus = event_bus

    @override
    async def create_email_validation_token(
        self, email: EmailStr, now: DateTime
    ) -> EmailValidationToken | CreateEmailValidationTokenBadOutcome:
        if email in self._data.accounts:
            return CreateEmailValidationTokenBadOutcome.ACCOUNT_ALREADY_EXISTS
        elif email in self._data.unverified_emails:
            (_, last_email_datetime) = self._data.unverified_emails[email]
            if not self.should_resend_token(now, last_email_datetime):
                return CreateEmailValidationTokenBadOutcome.TOO_SOON_AFTER_PREVIOUS_DEMAND

        token = EmailValidationToken.new()
        self._data.unverified_emails[email] = (token, now)
        return token

    @override
    async def check_signature(self):
        pass
