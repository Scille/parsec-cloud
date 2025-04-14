# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from enum import auto
from uuid import uuid4

from pydantic import EmailStr, TypeAdapter

from parsec._parsec import EmailValidationToken, anonymous_account_cmds
from parsec.api import api
from parsec.client_context import AnonymousAccountClientContext
from parsec.types import BadOutcomeEnum

EmailAdapter = TypeAdapter(EmailStr)


class CreateEmailValidationTokenBadOutcome(BadOutcomeEnum):
    ACCOUNT_ALREADY_EXISTS = auto()


class BaseAccountComponent:
    async def create_email_validation_token(
        self, email: EmailStr
    ) -> EmailValidationToken | CreateEmailValidationTokenBadOutcome:
        raise NotImplementedError

    async def check_signature(self):
        raise NotImplementedError

    async def test_new_account(self) -> str:
        # generate unique data
        # the email is generated too to avoid concurrence during tests

        email = f"{uuid4().hex[:4]}@invalid.com"

        # create account
        await self.create_email_validation_token(email)

        return email

    @api
    async def api_account_send_email_validation_token(
        self,
        client_ctx: AnonymousAccountClientContext,
        req: anonymous_account_cmds.latest.account_send_email_validation_token.Req,
    ) -> anonymous_account_cmds.latest.account_send_email_validation_token.Rep:
        try:
            # TODO use specific email type #10211
            email_parsed = EmailAdapter.validate_python(req.email)
        except ValueError:
            return (
                anonymous_account_cmds.latest.account_send_email_validation_token.RepInvalidEmail()
            )

        outcome = await self.create_email_validation_token(email_parsed)
        match outcome:
            case EmailValidationToken():  # as token:
                # TODO  #10216 send email with token
                return anonymous_account_cmds.latest.account_send_email_validation_token.RepOk()
            case CreateEmailValidationTokenBadOutcome.ACCOUNT_ALREADY_EXISTS:
                # Respond OK without sending token to prevent creating
                return anonymous_account_cmds.latest.account_send_email_validation_token.RepOk()
