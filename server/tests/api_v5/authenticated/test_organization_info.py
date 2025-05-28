# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import authenticated_cmds
from parsec.backend import Backend
from tests.common.client import CoolorgRpcClients
from tests.common.data import HttpCommonErrorsTester


async def test_authenticated_organization_info_ok(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    rep = await coolorg.alice.organization_info()
    assert isinstance(rep, authenticated_cmds.latest.organization_info.RepOk)
    assert rep == authenticated_cmds.latest.organization_info.RepOk(
        total_block_bytes=0, total_metadata_bytes=663
    )


async def test_authenticated_organization_info_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.organization_info()

    await authenticated_http_common_errors_tester(do)
