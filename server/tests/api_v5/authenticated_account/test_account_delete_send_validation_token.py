# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import AccountDeletionToken, DateTime, EmailAddress, authenticated_account_cmds
from parsec.backend import Backend
from parsec.components.account import AccountCreateAccountDeletionTokenBadOutcome
from parsec.components.email import SendEmailBadOutcome
from tests.common.client import AuthenticatedAccountRpcClient
from tests.common.data import HttpCommonErrorsTester
from tests.common.letter_box import LetterBox


async def test_authenticated_account_account_delete_send_validation_token_ok(
    xfail_if_postgresql: None,
    backend: Backend,
    alice_account: AuthenticatedAccountRpcClient,
    email_account_letterbox: LetterBox,
) -> None:
    rep = await alice_account.account_delete_send_validation_token()
    assert rep == authenticated_account_cmds.latest.account_delete_send_validation_token.RepOk()

    assert email_account_letterbox.count() == 1
    (sent_to, _) = await email_account_letterbox.get_next()
    assert sent_to == alice_account.account_email

    # 2nd deletion request too soon for a new email
    rep = await alice_account.account_delete_send_validation_token()
    assert rep == authenticated_account_cmds.latest.account_delete_send_validation_token.RepOk()

    assert email_account_letterbox.count() == 1


async def test_authenticated_account_account_delete_send_validation_token_ok_edge_cases(
    xfail_if_postgresql: None, backend: Backend, alice_account: AuthenticatedAccountRpcClient
) -> None:
    # 1st account deletion request
    now = DateTime.now()
    rep1 = await backend.account.create_email_deletion_token(alice_account.account_email, now)
    assert isinstance(rep1, AccountDeletionToken)

    # 2nd account deletion request send too soon for a new email + token
    rep2 = await backend.account.create_email_deletion_token(
        alice_account.account_email, now.add(microseconds=1)
    )
    assert rep2 is AccountCreateAccountDeletionTokenBadOutcome.TOO_SOON_AFTER_PREVIOUS_DEMAND

    rep3 = await backend.account.create_email_deletion_token(
        alice_account.account_email,
        now.add(seconds=backend.config.account_confirmation_email_resend_delay + 1),
    )
    assert isinstance(rep3, AccountDeletionToken)

    # A new token should have been generated
    assert rep1 != rep3


async def test_authenticated_account_account_delete_send_validation_token_email_recipient_refused(
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
    monkeypatch: pytest.MonkeyPatch,
):
    async def _mocked_send_email(*args, **kwargs):
        return SendEmailBadOutcome.RECIPIENT_REFUSED

    monkeypatch.setattr("parsec.components.account.send_email", _mocked_send_email)
    alice_account.account_email = EmailAddress("foo@invalid.com")
    rep = await alice_account.account_delete_send_validation_token()
    assert (
        rep
        == authenticated_account_cmds.latest.account_delete_send_validation_token.RepEmailRecipientRefused()
    )


@pytest.mark.parametrize(
    "bad_outcome",
    (
        SendEmailBadOutcome.BAD_SMTP_CONFIG,
        SendEmailBadOutcome.SERVER_UNAVAILABLE,
    ),
)
async def test_authenticated_account_account_delete_send_validation_token_email_server_unavailable(
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
    bad_outcome: SendEmailBadOutcome,
    monkeypatch: pytest.MonkeyPatch,
):
    async def _mocked_send_email(*args, **kwargs):
        return bad_outcome

    monkeypatch.setattr("parsec.components.account.send_email", _mocked_send_email)
    alice_account.account_email = EmailAddress("foo@invalid.com")
    rep = await alice_account.account_delete_send_validation_token()
    assert (
        rep
        == authenticated_account_cmds.latest.account_delete_send_validation_token.RepEmailServerUnavailable()
    )


async def test_authenticated_account_account_delete_send_validation_token_http_common_errors(
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await alice_account.account_delete_send_validation_token()

    await authenticated_account_http_common_errors_tester(do)
