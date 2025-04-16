# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from enum import auto
from uuid import uuid4

from parsec._parsec import anonymous_account_cmds
from parsec.api import api
from parsec.client_context import AnonymousAccountClientContext
from parsec.types import BadOutcomeEnum


class CreateAccountBadOutcome(BadOutcomeEnum):
    ACCOUNT_ALREADY_EXISTS = auto()
    INVALID_EMAIL = auto()


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

    @api
    async def api_account_send_email_validation_token(
        self,
        client_ctx: AnonymousAccountClientContext,
        req: anonymous_account_cmds.latest.account_send_email_validation_token.Req,
    ) -> anonymous_account_cmds.latest.account_send_email_validation_token.Rep:
        outcome = await self.create(req.email)
        match outcome:
            case CreateAccountBadOutcome.INVALID_EMAIL:
                return anonymous_account_cmds.latest.account_send_email_validation_token.RepInvalidEmail()
            case CreateAccountBadOutcome.ACCOUNT_ALREADY_EXISTS:
                # Do nothing
                return anonymous_account_cmds.latest.account_send_email_validation_token.RepOk()
            case None:
                # TODO
                return anonymous_account_cmds.latest.account_send_email_validation_token.RepOk()
