# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import httpx
import pytest

from parsec._parsec import (
    InvitationToken,
    InvitationType,
    OrganizationID,
    ParsecAddr,
    ParsecInvitationAddr,
)
from tests.common import Backend


async def test_get_redirect(client: httpx.AsyncClient, backend: Backend):
    backend.config.server_addr = ParsecAddr(hostname="parsec.invalid", port=None, use_ssl=False)

    rep = await client.get("http://parsec.invalid/redirect/foo/bar?a=1&b=2")
    assert rep.status_code == 302
    assert rep.headers["location"] == "parsec://parsec.invalid/foo/bar?a=1&b=2&no_ssl=true"


async def test_get_redirect_over_ssl(client: httpx.AsyncClient, backend: Backend):
    backend.config.server_addr = ParsecAddr(hostname="parsec.invalid", port=None, use_ssl=True)

    rep = await client.get("https://parsec.invalid/redirect/foo/bar?a=1&b=2")
    assert rep.status_code == 302
    assert rep.headers["location"] == "parsec://parsec.invalid/foo/bar?a=1&b=2"


async def test_get_redirect_no_ssl_param_overwritten(client: httpx.AsyncClient, backend: Backend):
    backend.config.server_addr = ParsecAddr(hostname="parsec.invalid", port=None, use_ssl=False)

    rep = await client.get("http://parsec.invalid/redirect/spam?no_ssl=false&a=1&b=2")
    assert rep.status_code == 302
    assert rep.headers["location"] == "parsec://parsec.invalid/spam?a=1&b=2&no_ssl=true"


async def test_get_redirect_no_ssl_param_overwritten_with_ssl_enabled(
    client: httpx.AsyncClient, backend: Backend
):
    backend.config.server_addr = ParsecAddr(hostname="parsec.invalid", port=None, use_ssl=True)

    rep = await client.get("https://parsec.invalid/redirect/spam?a=1&b=2&no_ssl=true")
    assert rep.status_code == 302
    assert rep.headers["location"] == "parsec://parsec.invalid/spam?a=1&b=2"


@pytest.mark.parametrize("use_ssl", (False, True))
async def test_get_redirect_invitation(use_ssl: bool, client: httpx.AsyncClient, backend: Backend):
    backend.config.server_addr = ParsecAddr(hostname="parsec.invalid", port=None, use_ssl=use_ssl)

    invitation_addr = ParsecInvitationAddr.build(
        server_addr=backend.config.server_addr,
        organization_id=OrganizationID("Org"),
        invitation_type=InvitationType.USER,
        token=InvitationToken.new(),
    )
    # TODO: should use invitation_addr.to_redirection_url() when available !
    *_, target = invitation_addr.to_url().split("/")
    rep = await client.get(f"http{'s' if use_ssl else ''}://parsec.invalid/redirect/{target}")
    assert rep.status_code == 302
    location_addr = ParsecInvitationAddr.from_url(rep.headers["location"])
    assert location_addr == invitation_addr
