# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from asyncio import sleep

import pytest

from parsec._parsec import anonymous_account_cmds
from parsec.components.email import SendEmailBadOutcome
from tests.common import Backend, HttpCommonErrorsTester
from tests.common.client import AccountRpcClient
from tests.common.letter_box import LetterBox


async def test_anonymous_account_account_create_send_validation_email_ok(
    xfail_if_postgresql: None,
    account: AccountRpcClient,
    backend: Backend,
    email_account_letterbox: LetterBox,
) -> None:
    alice_s_mail = "alice@invalid.com"
    # 1st account creation request
    rep = await account.anonymous_account.account_create_send_validation_email(email=alice_s_mail)
    assert rep == anonymous_account_cmds.latest.account_create_send_validation_email.RepOk()

    assert email_account_letterbox.count() == 1
    (sent_to, first_email) = await email_account_letterbox.get_next()
    assert sent_to == alice_s_mail
    # 2nd account creation request too soon for a new mail
    rep = await account.anonymous_account.account_create_send_validation_email(email=alice_s_mail)
    assert rep == anonymous_account_cmds.latest.account_create_send_validation_email.RepOk()

    assert email_account_letterbox.count() == 1

    await sleep(20)  # default delay for a new mail during tests is 10 secs
    # 3rd account creation request: a new mail is sent, with a new token
    rep = await account.anonymous_account.account_create_send_validation_email(email=alice_s_mail)
    assert rep == anonymous_account_cmds.latest.account_create_send_validation_email.RepOk()

    assert email_account_letterbox.count() == 2
    (sent_to, second_email) = await email_account_letterbox.get_next()
    assert sent_to == alice_s_mail

    assert first_email != second_email


@pytest.mark.parametrize(
    "bad_outcome",
    (
        SendEmailBadOutcome.BAD_SMTP_CONFIG,
        SendEmailBadOutcome.SERVER_UNAVAILABLE,
    ),
)
async def test_anonymous_account_account_create_send_validation_email_email_server_unavailable(
    xfail_if_postgresql: None,
    account: AccountRpcClient,
    backend: Backend,
    bad_outcome: SendEmailBadOutcome,
    monkeypatch,
):
    async def _mocked_send_email(*args, **kwargs):
        return bad_outcome

    monkeypatch.setattr("parsec.components.account.send_email", _mocked_send_email)
    rep = await account.anonymous_account.account_create_send_validation_email(
        email="alice@invalid.com"
    )
    assert (
        rep
        == anonymous_account_cmds.latest.account_create_send_validation_email.RepEmailServerUnavailable()
    )


async def test_anonymous_account_account_create_send_validation_email_email_recipient_refused(
    xfail_if_postgresql: None,
    account: AccountRpcClient,
    backend: Backend,
    monkeypatch,
):
    async def _mocked_send_email(*args, **kwargs):
        return SendEmailBadOutcome.RECIPIENT_REFUSED

    monkeypatch.setattr("parsec.components.account.send_email", _mocked_send_email)
    rep = await account.anonymous_account.account_create_send_validation_email(
        email="alice@invalid.com"
    )
    assert (
        rep
        == anonymous_account_cmds.latest.account_create_send_validation_email.RepEmailRecipientRefused()
    )


async def test_anonymous_account_account_create_send_validation_email_invalid_email(
    xfail_if_postgresql: None, account: AccountRpcClient, backend: Backend
) -> None:
    rep = await account.anonymous_account.account_create_send_validation_email(email="not an email")
    assert (
        rep == anonymous_account_cmds.latest.account_create_send_validation_email.RepInvalidEmail()
    )


async def test_anonymous_account_account_create_send_validation_email_http_common_errors(
    xfail_if_postgresql: None,
    account: AccountRpcClient,
    anonymous_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await account.anonymous_account.account_create_send_validation_email(
            email="alice@invalid.com"
        )

    await anonymous_account_http_common_errors_tester(do)
