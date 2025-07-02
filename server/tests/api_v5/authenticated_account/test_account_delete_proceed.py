# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import DateTime, ValidationCode, authenticated_account_cmds
from parsec.backend import Backend
from parsec.components.account import (
    VALIDATION_CODE_MAX_FAILED_ATTEMPTS,
    VALIDATION_CODE_VALIDITY_DURATION_SECONDS,
    AccountDeleteProceedBadOutcome,
)
from parsec.config import MockedEmailConfig
from tests.common.client import AuthenticatedAccountRpcClient, RpcTransportError
from tests.common.data import HttpCommonErrorsTester


@pytest.fixture
async def alice_validation_code(
    backend: Backend, alice_account: AuthenticatedAccountRpcClient
) -> ValidationCode:
    assert isinstance(backend.config.email_config, MockedEmailConfig)
    assert len(backend.config.email_config.sent_emails) == 0  # Sanity check

    validation_code = await backend.account.delete_send_validation_email(
        DateTime.now(), alice_account.auth_method_id
    )
    assert isinstance(validation_code, ValidationCode)
    backend.config.email_config.sent_emails.clear()

    return validation_code


async def test_authenticated_account_account_delete_proceed_ok(
    xfail_if_postgresql: None,
    backend: Backend,
    alice_account: AuthenticatedAccountRpcClient,
    alice_validation_code: ValidationCode,
):
    rep = await alice_account.account_delete_proceed(validation_code=alice_validation_code)
    assert rep == authenticated_account_cmds.latest.account_delete_proceed.RepOk()

    with pytest.raises(RpcTransportError) as ctx:
        await alice_account.ping("foo")
    assert ctx.value.rep.status_code == 403

    # Since the account is deleted, even if we bypass the authentication layer
    # we shouldn't be able to re-use the validation code!

    outcome = await backend.account.delete_proceed(
        DateTime.now(), alice_account.auth_method_id, alice_validation_code
    )
    assert outcome == AccountDeleteProceedBadOutcome.ACCOUNT_NOT_FOUND


async def test_authenticated_account_account_delete_proceed_invalid_validation_code(
    xfail_if_postgresql: None,
    backend: Backend,
    alice_account: AuthenticatedAccountRpcClient,
    alice_validation_code: ValidationCode,
):
    while True:
        bad_validation_code = ValidationCode.generate()
        if bad_validation_code != alice_validation_code:
            break

    # Multiple bad attempts
    for _ in range(VALIDATION_CODE_MAX_FAILED_ATTEMPTS):
        rep = await alice_account.account_delete_proceed(validation_code=bad_validation_code)
        assert (
            rep
            == authenticated_account_cmds.latest.account_delete_proceed.RepInvalidValidationCode()
        )

    # Further attempts are no longer considered even if the good validation code is provided
    for validation_code in (bad_validation_code, alice_validation_code):
        rep = await alice_account.account_delete_proceed(validation_code=validation_code)
        assert (
            rep
            == authenticated_account_cmds.latest.account_delete_proceed.RepSendValidationEmailRequired()
        )


@pytest.mark.parametrize(
    "kind",
    (
        "validation_code_already_used",
        "validation_code_too_many_attemps",
        "validation_code_too_old",
    ),
)
async def test_authenticated_account_account_delete_proceed_send_validation_email_required(
    xfail_if_postgresql: None,
    kind: str,
    backend: Backend,
    alice_account: AuthenticatedAccountRpcClient,
):
    validation_code = None
    match kind:
        case "validation_code_already_used":
            # Nothing to do: this has already been testbed at the end of
            # `test_authenticated_account_account_delete_proceed_ok`
            return

        case "validation_code_too_many_attemps":
            # Nothing to do: this has already been testbed at the end of
            # `test_authenticated_account_account_delete_proceed_invalid_validation_code`
            return

        case "validation_code_too_old":
            timestamp_too_old = DateTime.now().add(
                seconds=-VALIDATION_CODE_VALIDITY_DURATION_SECONDS
            )
            validation_code = await backend.account.delete_send_validation_email(
                timestamp_too_old, alice_account.auth_method_id
            )

        case unknown:
            assert False, unknown

    assert isinstance(validation_code, ValidationCode)

    rep = await alice_account.account_delete_proceed(validation_code=validation_code)
    assert (
        rep
        == authenticated_account_cmds.latest.account_delete_proceed.RepSendValidationEmailRequired()
    )


async def test_authenticated_account_account_delete_proceed_http_common_errors(
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
):
    async def do():
        await alice_account.account_delete_proceed(validation_code=ValidationCode.generate())

    await authenticated_account_http_common_errors_tester(do)
