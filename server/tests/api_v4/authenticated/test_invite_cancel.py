# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import authenticated_cmds
from tests.common import CoolorgRpcClients


async def test_ok(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.invite_cancel(coolorg.invited_alice_dev3.token)

    assert rep == authenticated_cmds.v4.invite_cancel.RepOk()
