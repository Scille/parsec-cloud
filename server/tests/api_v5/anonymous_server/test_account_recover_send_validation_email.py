# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailAddress,
    SecretKey,
    ValidationCode,
    anonymous_server_cmds,
)
from parsec.components.account import (
    VALIDATION_CODE_VALIDITY_DURATION_SECONDS,
    AccountRecoverProceedBadOutcome,
)
from parsec.components.email import SendEmailBadOutcome
from parsec.config import MockedEmailConfig
from tests.common import (
    AnonymousServerRpcClient,
    AuthenticatedAccountRpcClient,
    Backend,
    HttpCommonErrorsTester,
    extract_validation_code_from_email,
)


@pytest.mark.parametrize(
    "kind",
    (
        "no_previous",
        "out_of_cooldown_and_still_valid_previous",
        "too_old_previous",
        "out_of_cooldown_and_too_many_attempts_previous",
    ),
)
async def test_anonymous_server_account_recover_send_validation_email_ok(
    kind: str,
    anonymous_server: AnonymousServerRpcClient,
    alice_account: AuthenticatedAccountRpcClient,
    backend: Backend,
) -> None:
    assert isinstance(backend.config.email_config, MockedEmailConfig)

    email = alice_account.account_email

    old_validation_code = None
    match kind:
        case "no_previous":
            pass

        case "out_of_cooldown_and_still_valid_previous":
            timestamp_out_of_cooldown = DateTime.now().add(
                seconds=-backend.config.email_rate_limit_cooldown_delay
            )
            outcome = await backend.account.recover_send_validation_email(
                timestamp_out_of_cooldown, email
            )
            assert isinstance(outcome, ValidationCode)
            assert len(backend.config.email_config.sent_emails) == 1
            backend.config.email_config.sent_emails.clear()

        case "too_old_previous":
            timestamp_too_old = DateTime.now().add(
                seconds=-VALIDATION_CODE_VALIDITY_DURATION_SECONDS
            )
            old_validation_code = await backend.account.recover_send_validation_email(
                timestamp_too_old, email
            )
            assert isinstance(old_validation_code, ValidationCode)
            assert len(backend.config.email_config.sent_emails) == 1
            backend.config.email_config.sent_emails.clear()

        case "out_of_cooldown_and_too_many_attempts_previous":
            timestamp_out_of_cooldown = DateTime.now().add(
                seconds=-backend.config.email_rate_limit_cooldown_delay
            )
            old_validation_code = await backend.account.recover_send_validation_email(
                timestamp_out_of_cooldown, email
            )
            assert isinstance(old_validation_code, ValidationCode)
            assert len(backend.config.email_config.sent_emails) == 1
            backend.config.email_config.sent_emails.clear()
            while True:
                bad_validation_code = ValidationCode.generate()
                if bad_validation_code != old_validation_code:
                    break
            while True:
                outcome = await backend.account.recover_proceed(
                    now=DateTime.now(),
                    validation_code=bad_validation_code,
                    email=alice_account.account_email,
                    created_by_user_agent="",
                    created_by_ip="",
                    new_vault_key_access=b"<dummy>",
                    new_auth_method_id=AccountAuthMethodID.new(),
                    new_auth_method_mac_key=SecretKey.generate(),
                    new_auth_method_password_algorithm=None,
                )
                match outcome:
                    case AccountRecoverProceedBadOutcome.INVALID_VALIDATION_CODE:
                        continue
                    case AccountRecoverProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED:
                        break
                    case unexpected:
                        assert False, unexpected

        case unknown:
            assert False, unknown

    assert len(backend.config.email_config.sent_emails) == 0

    rep = await anonymous_server.account_recover_send_validation_email(email=email)
    assert rep == anonymous_server_cmds.latest.account_recover_send_validation_email.RepOk()

    assert len(backend.config.email_config.sent_emails) == 1
    assert backend.config.email_config.sent_emails[-1].recipient == email
    new_validation_code = extract_validation_code_from_email(
        backend.config.email_config.sent_emails[-1].body
    )

    # Finally ensure the previous validation code is no longer usable...
    if old_validation_code is not None:
        outcome = await backend.account.recover_proceed(
            now=DateTime.now(),
            validation_code=old_validation_code,
            email=alice_account.account_email,
            created_by_user_agent="",
            created_by_ip="",
            new_vault_key_access=b"<dummy>",
            new_auth_method_id=AccountAuthMethodID.new(),
            new_auth_method_mac_key=SecretKey.generate(),
            new_auth_method_password_algorithm=None,
        )
        assert outcome == AccountRecoverProceedBadOutcome.INVALID_VALIDATION_CODE

    # ...but the new one is !
    outcome = await backend.account.recover_proceed(
        now=DateTime.now(),
        validation_code=new_validation_code,
        email=alice_account.account_email,
        created_by_user_agent="",
        created_by_ip="",
        new_vault_key_access=b"<dummy>",
        new_auth_method_id=AccountAuthMethodID.new(),
        new_auth_method_mac_key=SecretKey.generate(),
        new_auth_method_password_algorithm=None,
    )
    assert outcome is None


@pytest.mark.parametrize("kind", ("still_valid_previous", "too_many_attempts_previous"))
async def test_anonymous_server_account_recover_send_validation_email_email_sending_rate_limited(
    kind: str,
    anonymous_server: AnonymousServerRpcClient,
    alice_account: AuthenticatedAccountRpcClient,
    backend: Backend,
) -> None:
    assert isinstance(backend.config.email_config, MockedEmailConfig)

    # Previous request (that is recent enough)

    # Note we cannot use `backend.account.create_send_validation_email()` here
    # since the rate limit is done in a upper layer.
    rep = await anonymous_server.account_recover_send_validation_email(alice_account.account_email)
    assert rep == anonymous_server_cmds.latest.account_recover_send_validation_email.RepOk()
    assert len(backend.config.email_config.sent_emails) == 1
    validation_code = extract_validation_code_from_email(
        backend.config.email_config.sent_emails[-1].body
    )
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
                outcome = await backend.account.recover_proceed(
                    now=DateTime.now(),
                    validation_code=bad_validation_code,
                    email=alice_account.account_email,
                    created_by_user_agent="",
                    created_by_ip="",
                    new_vault_key_access=b"<dummy>",
                    new_auth_method_id=AccountAuthMethodID.new(),
                    new_auth_method_mac_key=SecretKey.generate(),
                    new_auth_method_password_algorithm=None,
                )
                match outcome:
                    case AccountRecoverProceedBadOutcome.INVALID_VALIDATION_CODE:
                        continue
                    case AccountRecoverProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED:
                        break
                    case unexpected:
                        assert False, unexpected

        case unknown:
            assert False, unknown

    # New attempt that is too soon for a new mail

    rep = await anonymous_server.account_recover_send_validation_email(
        email=alice_account.account_email
    )
    assert isinstance(
        rep,
        anonymous_server_cmds.latest.account_recover_send_validation_email.RepEmailSendingRateLimited,
    )

    assert len(backend.config.email_config.sent_emails) == 0


@pytest.mark.parametrize(
    "bad_outcome",
    (
        SendEmailBadOutcome.BAD_SMTP_CONFIG,
        SendEmailBadOutcome.SERVER_UNAVAILABLE,
    ),
)
async def test_anonymous_server_account_recover_send_validation_email_email_server_unavailable(
    bad_outcome: SendEmailBadOutcome,
    anonymous_server: AnonymousServerRpcClient,
    alice_account: AuthenticatedAccountRpcClient,
    monkeypatch: pytest.MonkeyPatch,
):
    async def _mocked_send_email(*args, **kwargs):
        return bad_outcome

    monkeypatch.setattr("parsec.components.account.send_email", _mocked_send_email)
    rep = await anonymous_server.account_recover_send_validation_email(
        email=alice_account.account_email
    )
    assert (
        rep
        == anonymous_server_cmds.latest.account_recover_send_validation_email.RepEmailServerUnavailable()
    )


async def test_anonymous_server_account_recover_send_validation_email_email_recipient_refused(
    anonymous_server: AnonymousServerRpcClient,
    monkeypatch: pytest.MonkeyPatch,
    alice_account: AuthenticatedAccountRpcClient,
):
    async def _mocked_send_email(*args, **kwargs):
        return SendEmailBadOutcome.RECIPIENT_REFUSED

    monkeypatch.setattr("parsec.components.account.send_email", _mocked_send_email)
    rep = await anonymous_server.account_recover_send_validation_email(
        email=alice_account.account_email
    )
    assert (
        rep
        == anonymous_server_cmds.latest.account_recover_send_validation_email.RepEmailRecipientRefused()
    )


async def test_anonymous_server_account_recover_send_validation_email_http_common_errors(
    anonymous_server: AnonymousServerRpcClient,
    anonymous_server_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await anonymous_server.account_recover_send_validation_email(
            email=EmailAddress("foo@invalid.com")
        )

    await anonymous_server_http_common_errors_tester(do)
