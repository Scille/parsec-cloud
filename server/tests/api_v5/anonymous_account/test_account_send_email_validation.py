# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import anonymous_account_cmds
from tests.common import Backend, HttpCommonErrorsTester
from tests.common.client import AccountRpcClient


async def test_anonymous_account_account_send_email_validation_token_ok(
    xfail_if_postgresql: None, account: AccountRpcClient, backend: Backend, with_postgresql: bool
) -> None:
    rep = await account.anonymous_account.account_send_email_validation_token(
        email="alice@invalid.com"
    )
    assert rep == anonymous_account_cmds.latest.account_send_email_validation_token.RepOk()


async def test_anonymous_account_account_send_email_validation_token_send_email_bad_outcome() -> (
    None
):
    # TODO #10216
    pass


async def test_anonymous_account_account_send_email_validation_token_invalid_email(
    xfail_if_postgresql: None, account: AccountRpcClient, backend: Backend
) -> None:
    rep = await account.anonymous_account.account_send_email_validation_token(email="not an email")
    assert (
        rep == anonymous_account_cmds.latest.account_send_email_validation_token.RepInvalidEmail()
    )


async def test_anonymous_account_account_send_email_validation_token_http_common_errors(
    xfail_if_postgresql: None,
    account: AccountRpcClient,
    anonymous_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await account.anonymous_account.account_send_email_validation_token(
            email="alice@invalid.com"
        )

    await anonymous_account_http_common_errors_tester(do)
