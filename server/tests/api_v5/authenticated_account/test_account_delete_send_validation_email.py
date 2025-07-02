# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import re

import pytest

from parsec._parsec import DateTime, EmailAddress, ValidationCode, authenticated_account_cmds
from parsec.backend import Backend
from parsec.components.account import (
    VALIDATION_CODE_VALIDITY_DURATION_SECONDS,
    AccountDeleteProceedBadOutcome,
)
from parsec.components.email import SendEmailBadOutcome
from parsec.config import MockedEmailConfig
from tests.common.client import AuthenticatedAccountRpcClient
from tests.common.data import HttpCommonErrorsTester


@pytest.mark.parametrize(
    "kind",
    (
        "no_previous",
        "out_of_cooldown_and_still_valid_previous",
        "too_old_previous",
        "out_of_cooldown_and_too_many_attempts_previous",
    ),
)
async def test_authenticated_account_account_delete_send_validation_email_ok_new(
    xfail_if_postgresql: None,
    kind: str,
    alice_account: AuthenticatedAccountRpcClient,
    backend: Backend,
) -> None:
    assert isinstance(backend.config.email_config, MockedEmailConfig)

    old_validation_code = None
    match kind:
        case "no_previous":
            pass

        case "out_of_cooldown_and_still_valid_previous":
            timestamp_out_of_cooldown = DateTime.now().add(
                seconds=-backend.config.validation_email_cooldown_delay
            )
            outcome = await backend.account.delete_send_validation_email(
                timestamp_out_of_cooldown, alice_account.auth_method_id
            )
            assert isinstance(outcome, ValidationCode)
            assert len(backend.config.email_config.sent_emails) == 1
            backend.config.email_config.sent_emails.clear()

        case "too_old_previous":
            timestamp_too_old = DateTime.now().add(
                seconds=-VALIDATION_CODE_VALIDITY_DURATION_SECONDS
            )
            old_validation_code = await backend.account.delete_send_validation_email(
                timestamp_too_old, alice_account.auth_method_id
            )
            assert isinstance(old_validation_code, ValidationCode)
            assert len(backend.config.email_config.sent_emails) == 1
            backend.config.email_config.sent_emails.clear()

        case "out_of_cooldown_and_too_many_attempts_previous":
            timestamp_out_of_cooldown = DateTime.now().add(
                seconds=-backend.config.validation_email_cooldown_delay
            )
            old_validation_code = await backend.account.delete_send_validation_email(
                timestamp_out_of_cooldown, alice_account.auth_method_id
            )
            assert isinstance(old_validation_code, ValidationCode)
            assert len(backend.config.email_config.sent_emails) == 1
            backend.config.email_config.sent_emails.clear()
            while True:
                bad_validation_code = ValidationCode.generate()
                if bad_validation_code != old_validation_code:
                    break
            while True:
                outcome = await backend.account.delete_proceed(
                    DateTime.now(), alice_account.auth_method_id, bad_validation_code
                )
                match outcome:
                    case AccountDeleteProceedBadOutcome.INVALID_VALIDATION_CODE:
                        continue
                    case AccountDeleteProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED:
                        break
                    case unexpected:
                        assert False, unexpected

        case unknown:
            assert False, unknown

    assert len(backend.config.email_config.sent_emails) == 0

    rep = await alice_account.account_delete_send_validation_email()
    assert rep == authenticated_account_cmds.latest.account_delete_send_validation_email.RepOk()

    assert len(backend.config.email_config.sent_emails) == 1
    assert backend.config.email_config.sent_emails[-1].recipient == alice_account.account_email
    # Extract the validation code from the email
    validation_code_match = re.search(
        r"<pre id=\"code\">([A-Z0-9]+)</pre>", backend.config.email_config.sent_emails[-1].body
    )
    assert validation_code_match is not None
    new_validation_code = ValidationCode(validation_code_match.group(1))

    # Finally ensure the previous validation code is no longer usable...
    if old_validation_code is not None:
        outcome = await backend.account.delete_proceed(
            DateTime.now(), alice_account.auth_method_id, old_validation_code
        )
        assert outcome == AccountDeleteProceedBadOutcome.INVALID_VALIDATION_CODE

    # ...but the new one is !
    outcome = await backend.account.delete_proceed(
        DateTime.now(), alice_account.auth_method_id, new_validation_code
    )
    assert outcome is None


@pytest.mark.parametrize("kind", ("still_valid_previous", "too_many_attempts_previous"))
async def test_authenticated_account_account_delete_send_validation_email_ok_but_too_soon_for_new_mail(
    xfail_if_postgresql: None,
    kind: str,
    alice_account: AuthenticatedAccountRpcClient,
    backend: Backend,
) -> None:
    assert isinstance(backend.config.email_config, MockedEmailConfig)

    # Previous request (that is recent enough)

    validation_code = await backend.account.delete_send_validation_email(
        DateTime.now(), alice_account.auth_method_id
    )
    assert isinstance(validation_code, ValidationCode)
    assert len(backend.config.email_config.sent_emails) == 1
    backend.config.email_config.sent_emails.clear()

    match kind:
        case "still_valid_previous":
            pass

        case "too_many_attempts_previous":
            while True:
                bad_validation_code = ValidationCode.generate()
                if bad_validation_code != validation_code:
                    break
            while True:
                outcome = await backend.account.delete_proceed(
                    DateTime.now(), alice_account.auth_method_id, bad_validation_code
                )
                match outcome:
                    case AccountDeleteProceedBadOutcome.INVALID_VALIDATION_CODE:
                        continue
                    case AccountDeleteProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED:
                        break
                    case unexpected:
                        assert False, unexpected

        case unknown:
            assert False, unknown

    # New attempt that is too soon for a new mail

    rep = await alice_account.account_delete_send_validation_email()
    assert rep == authenticated_account_cmds.latest.account_delete_send_validation_email.RepOk()

    assert len(backend.config.email_config.sent_emails) == 0

    # Finally ensure current validation code hasn't been impacted

    if kind == "still_valid_previous":
        outcome = await backend.account.delete_proceed(
            DateTime.now(), alice_account.auth_method_id, validation_code
        )
        assert outcome is None


@pytest.mark.parametrize(
    "bad_outcome",
    (
        SendEmailBadOutcome.BAD_SMTP_CONFIG,
        SendEmailBadOutcome.SERVER_UNAVAILABLE,
    ),
)
async def test_authenticated_account_account_delete_send_validation_email_email_server_unavailable(
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
    bad_outcome: SendEmailBadOutcome,
    monkeypatch: pytest.MonkeyPatch,
):
    async def _mocked_send_email(*args, **kwargs):
        return bad_outcome

    monkeypatch.setattr("parsec.components.account.send_email", _mocked_send_email)
    alice_account.account_email = EmailAddress("foo@invalid.com")
    rep = await alice_account.account_delete_send_validation_email()
    assert (
        rep
        == authenticated_account_cmds.latest.account_delete_send_validation_email.RepEmailServerUnavailable()
    )


async def test_authenticated_account_account_delete_send_validation_email_email_recipient_refused(
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
    monkeypatch: pytest.MonkeyPatch,
):
    async def _mocked_send_email(*args, **kwargs):
        return SendEmailBadOutcome.RECIPIENT_REFUSED

    monkeypatch.setattr("parsec.components.account.send_email", _mocked_send_email)
    alice_account.account_email = EmailAddress("foo@invalid.com")
    rep = await alice_account.account_delete_send_validation_email()
    assert (
        rep
        == authenticated_account_cmds.latest.account_delete_send_validation_email.RepEmailRecipientRefused()
    )


async def test_authenticated_account_account_delete_send_validation_email_http_common_errors(
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await alice_account.account_delete_send_validation_email()

    await authenticated_account_http_common_errors_tester(do)
