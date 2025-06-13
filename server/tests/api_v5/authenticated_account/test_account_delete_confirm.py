# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import DateTime, ValidationCode, authenticated_account_cmds
from parsec.backend import Backend
from parsec.components.account import (
    VALIDATION_CODE_MAX_FAILED_ATTEMPTS,
    VALIDATION_CODE_VALIDITY_DURATION_SECONDS,
    AccountDeletionConfirmBadOutcome,
)
from tests.common.client import AuthenticatedAccountRpcClient, RpcTransportError
from tests.common.data import HttpCommonErrorsTester


async def request_account_deletion(
    backend: Backend, account: AuthenticatedAccountRpcClient, now: DateTime
) -> ValidationCode:
    outcome = await backend.account.create_email_deletion_code(account.account_email, now)
    match outcome:
        case ValidationCode() as code:
            return code
        case _:
            raise ValueError(f"Unexpected outcome {outcome}")


async def test_authenticated_account_account_delete_confirm_ok(
    xfail_if_postgresql: None,
    backend: Backend,
    alice_account: AuthenticatedAccountRpcClient,
):
    now = DateTime.now()
    deletion_code = await request_account_deletion(backend, alice_account, now)

    rep = await alice_account.account_delete_confirm(deletion_code=deletion_code)
    assert rep == authenticated_account_cmds.latest.account_delete_confirm.RepOk()

    try:
        rep = await alice_account.ping("foo")
    except RpcTransportError as e:
        assert e.rep.status_code == 403
    else:
        assert False, "Alice should not be able to execute commands"


async def test_authenticated_account_account_delete_confirm_code_expired(
    xfail_if_postgresql: None,
    backend: Backend,
    alice_account: AuthenticatedAccountRpcClient,
):
    now = DateTime.now()
    deletion_code = await request_account_deletion(backend, alice_account, now)
    alice_client_context = alice_account.to_context()
    outcome = await backend.account.confirm_account_deletion(
        alice_client_context,
        deletion_code,
        now.add(seconds=VALIDATION_CODE_VALIDITY_DURATION_SECONDS + 1),
    )
    assert outcome == AccountDeletionConfirmBadOutcome.NEW_CODE_NEEDED

    outcome = await backend.account.confirm_account_deletion(
        alice_client_context,
        deletion_code,
        now.add(seconds=VALIDATION_CODE_VALIDITY_DURATION_SECONDS),
    )
    assert outcome is None


async def test_authenticated_account_account_delete_confirm_new_code_needed(
    xfail_if_postgresql: None, backend: Backend, alice_account: AuthenticatedAccountRpcClient
):
    now = DateTime.now()
    deletion_code = await request_account_deletion(backend, alice_account, now)

    other_code = ValidationCode.generate()
    assert deletion_code != other_code
    alice_client_context = alice_account.to_context()

    for tries in range(VALIDATION_CODE_MAX_FAILED_ATTEMPTS):
        print(f"Tries #{tries} account deletion confirmation")
        rep = await backend.account.confirm_account_deletion(alice_client_context, other_code, now)
        assert rep == AccountDeletionConfirmBadOutcome.INVALID_CODE

    rep = await backend.account.confirm_account_deletion(alice_client_context, deletion_code, now)
    assert rep == AccountDeletionConfirmBadOutcome.NEW_CODE_NEEDED


async def test_authenticated_account_account_delete_confirm_invalid_deletion_code(
    xfail_if_postgresql: None, backend: Backend, alice_account: AuthenticatedAccountRpcClient
):
    now = DateTime.now()
    deletion_code = await request_account_deletion(backend, alice_account, now)
    other_code = ValidationCode.generate()
    assert other_code != deletion_code
    rep = await alice_account.account_delete_confirm(deletion_code=other_code)
    assert rep == authenticated_account_cmds.latest.account_delete_confirm.RepInvalidDeletionCode()


async def test_authenticated_account_account_delete_confirm_http_common_errors(
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
):
    async def do():
        await alice_account.account_delete_confirm(deletion_code=ValidationCode.generate())

    await authenticated_account_http_common_errors_tester(do)
