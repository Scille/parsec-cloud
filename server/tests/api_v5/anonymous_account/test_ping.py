# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import anonymous_account_cmds
from tests.common import Backend, HttpCommonErrorsTester
from tests.common.client import AccountRpcClient


async def test_anonymous_account_ping_ok(
    account: AccountRpcClient, backend: Backend, with_postgresql: bool
) -> None:
    if with_postgresql:
        pytest.xfail("TODO: postgre not implemented yet")
    rep = await account.anonymous_account.ping(ping="hello")
    assert rep == anonymous_account_cmds.latest.ping.RepOk(pong="hello")


async def test_anonymous_account_ping_http_common_errors(
    account: AccountRpcClient,
    anonymous_account_http_common_errors_tester: HttpCommonErrorsTester,
    with_postgresql: bool,
) -> None:
    if with_postgresql:
        pytest.xfail("TODO: postgre not implemented yet")

    async def do():
        await account.anonymous_account.ping(ping="hello")

    await anonymous_account_http_common_errors_tester(do)
