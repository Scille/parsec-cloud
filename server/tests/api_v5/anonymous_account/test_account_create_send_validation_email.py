# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import DateTime, EmailValidationToken, anonymous_account_cmds
from parsec.components.account import CreateEmailValidationTokenBadOutcome
from parsec.components.email import SendEmailBadOutcome
from tests.common import AnonymousAccountRpcClient, Backend, HttpCommonErrorsTester, LetterBox


async def test_anonymous_account_account_create_send_validation_email_ok(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    email_account_letterbox: LetterBox,
) -> None:
    email = "foo@invalid.com"

    # 1st account creation request
    rep = await anonymous_account.account_create_send_validation_email(email=email)
    assert rep == anonymous_account_cmds.latest.account_create_send_validation_email.RepOk()

    assert email_account_letterbox.count() == 1
    (sent_to, _) = await email_account_letterbox.get_next()
    assert sent_to == email

    # 2nd account creation request too soon for a new mail
    rep = await anonymous_account.account_create_send_validation_email(email=email)
    assert rep == anonymous_account_cmds.latest.account_create_send_validation_email.RepOk()

    assert email_account_letterbox.count() == 1


async def test_anonymous_account_account_create_send_validation_email_ok_edge_cases(
    xfail_if_postgresql: None,
    backend: Backend,
) -> None:
    email = "foo@invalid.com"

    # 1st account creation request
    now = DateTime.now()
    rep1 = await backend.account.create_email_validation_token(email, now)
    assert isinstance(rep1, EmailValidationToken)

    # 2nd account creation request too soon for a new mail
    rep2 = await backend.account.create_email_validation_token(email, now.add(microseconds=1))
    assert rep2 is CreateEmailValidationTokenBadOutcome.TOO_SOON_AFTER_PREVIOUS_DEMAND
    # default delay for a new mail during tests is 5 secs

    # 3rd account creation request: a new mail is sent, with a new token

    rep3 = await backend.account.create_email_validation_token(
        email,
        now.add(seconds=backend.config.account_config.account_confirmation_email_resend_delay + 1),
    )
    assert isinstance(rep3, EmailValidationToken)

    assert rep1 != rep3


@pytest.mark.parametrize(
    "bad_outcome",
    (
        SendEmailBadOutcome.BAD_SMTP_CONFIG,
        SendEmailBadOutcome.SERVER_UNAVAILABLE,
    ),
)
async def test_anonymous_account_account_create_send_validation_email_email_server_unavailable(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    bad_outcome: SendEmailBadOutcome,
    monkeypatch: pytest.MonkeyPatch,
):
    async def _mocked_send_email(*args, **kwargs):
        return bad_outcome

    monkeypatch.setattr("parsec.components.account.send_email", _mocked_send_email)
    rep = await anonymous_account.account_create_send_validation_email(email="foo@invalid.com")
    assert (
        rep
        == anonymous_account_cmds.latest.account_create_send_validation_email.RepEmailServerUnavailable()
    )


async def test_anonymous_account_account_create_send_validation_email_email_recipient_refused(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    monkeypatch: pytest.MonkeyPatch,
):
    async def _mocked_send_email(*args, **kwargs):
        return SendEmailBadOutcome.RECIPIENT_REFUSED

    monkeypatch.setattr("parsec.components.account.send_email", _mocked_send_email)
    rep = await anonymous_account.account_create_send_validation_email(email="foo@invalid.com")
    assert (
        rep
        == anonymous_account_cmds.latest.account_create_send_validation_email.RepEmailRecipientRefused()
    )


async def test_anonymous_account_account_create_send_validation_email_invalid_email(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
) -> None:
    rep = await anonymous_account.account_create_send_validation_email(email="not an email")
    assert (
        rep == anonymous_account_cmds.latest.account_create_send_validation_email.RepInvalidEmail()
    )


async def test_anonymous_account_account_create_send_validation_email_http_common_errors(
    xfail_if_postgresql: None,
    anonymous_account: AnonymousAccountRpcClient,
    anonymous_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await anonymous_account.account_create_send_validation_email(email="foo@invalid.com")

    await anonymous_account_http_common_errors_tester(do)
