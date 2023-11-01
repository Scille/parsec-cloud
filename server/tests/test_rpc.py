# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import httpx
import pytest

from parsec._parsec import anonymous_cmds, authenticated_cmds, invited_cmds
from tests.common import CoolorgRpcClients


@pytest.mark.parametrize("family", ("anonymous", "authenticated", "invited"))
async def test_unknown_org(family: str, client: httpx.AsyncClient) -> None:
    response = await client.post(
        f"http://parsec.invalid/{family}/CoolOrg",
        content=anonymous_cmds.latest.ping.Req(ping="hello").dump(),
        headers={
            "Content-Type": "application/msgpack",
            "Api-Version": "4.0",
            "Invitation-Token": "6f56a8579fc4425c82a71f9fc8531b77",
            "Authorization": "PARSEC-SIGN-ED25519",
            "Author": "d2FsZG9Ad2hlcmU=",
            "Signature": "NDI=",
        },
    )
    assert response.status_code == 404


async def test_good_org_authenticated(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.ping(ping="hello")
    assert rep == authenticated_cmds.latest.ping.RepOk(pong="hello")


async def test_good_org_anonymous(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.anonymous.ping(ping="hello")
    assert rep == anonymous_cmds.latest.ping.RepOk(pong="hello")


async def test_good_org_invited(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.invited_zack.ping(ping="hello")
    assert rep == invited_cmds.latest.ping.RepOk(pong="hello")
