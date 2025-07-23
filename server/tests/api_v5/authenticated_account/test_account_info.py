# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import HumanHandle, authenticated_account_cmds
from tests.common import AuthenticatedAccountRpcClient, HttpCommonErrorsTester


async def test_authenticated_account_account_info_ok(
    alice_account: AuthenticatedAccountRpcClient,
) -> None:
    rep = await alice_account.account_info()
    assert rep == authenticated_account_cmds.latest.account_info.RepOk(
        human_handle=HumanHandle(alice_account.account_email, "Alicey McAliceFace")
    )


async def test_authenticated_account_account_info_http_common_errors(
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await alice_account.account_info()

    await authenticated_account_http_common_errors_tester(do)
