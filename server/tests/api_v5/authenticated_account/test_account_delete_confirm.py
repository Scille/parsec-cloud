# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import AccountDeletionToken, DateTime, authenticated_account_cmds
from parsec.backend import Backend
from parsec.components.account import AccountDeletionConfirmBadOutcome
from tests.common.client import AuthenticatedAccountRpcClient, RpcTransportError
from tests.common.data import HttpCommonErrorsTester


async def request_account_deletion(
    backend: Backend, account: AuthenticatedAccountRpcClient
) -> tuple[AccountDeletionToken, DateTime]:
    now = DateTime.now()
    outcome = await backend.account.create_email_deletion_token(account.account_email, now)
    match outcome:
        case AccountDeletionToken() as token:
            return (token, now)
        case _:
            raise ValueError(f"Unexpected outcome {outcome}")


async def test_authenticated_account_account_delete_confirm_ok(
    xfail_if_postgresql: None,
    backend: Backend,
    alice_account: AuthenticatedAccountRpcClient,
):
    (deletion_token, _) = await request_account_deletion(backend, alice_account)

    rep = await alice_account.account_delete_confirm(deletion_token=deletion_token)
    assert rep == authenticated_account_cmds.latest.account_delete_confirm.RepOk()

    try:
        rep = await alice_account.ping("foo")
    except RpcTransportError as e:
        assert e.rep.status_code == 403
    else:
        assert False, "Alice should not be able to execute commands"


async def test_authenticated_account_account_delete_confirm_token_too_old(
    xfail_if_postgresql: None,
    backend: Backend,
    alice_account: AuthenticatedAccountRpcClient,
):
    (deletion_token, token_created_at) = await request_account_deletion(backend, alice_account)

    outcome = await backend.account.confirm_account_deletion(
        alice_account.account_email,
        deletion_token,
        token_created_at.add(seconds=backend.config.account_config.deletion_token_duration + 1),
    )
    assert outcome == AccountDeletionConfirmBadOutcome.INVALID_TOKEN

    outcome = await backend.account.confirm_account_deletion(
        alice_account.account_email,
        deletion_token,
        token_created_at.add(seconds=backend.config.account_config.deletion_token_duration),
    )
    assert outcome is None


async def test_authenticated_account_account_delete_confirm_invalid_deletion_token(
    xfail_if_postgresql: None, alice_account: AuthenticatedAccountRpcClient
):
    rep = await alice_account.account_delete_confirm(deletion_token=AccountDeletionToken.new())
    assert rep == authenticated_account_cmds.latest.account_delete_confirm.RepInvalidDeletionToken()


async def test_authenticated_account_account_delete_confirm_http_common_errors(
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
):
    async def do():
        await alice_account.account_delete_confirm(deletion_token=AccountDeletionToken.new())

    await authenticated_account_http_common_errors_tester(do)
