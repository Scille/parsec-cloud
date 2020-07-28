# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
import ssl
from uuid import uuid4
from tests.common import customize_fixtures

from parsec.api.protocol import OrganizationID, InvitationType
from parsec.core.types.backend_address import BackendInvitationAddr


@pytest.fixture
def backend_http_send(running_backend, backend_addr):
    async def _http_send(target):
        stream = await trio.open_tcp_stream(backend_addr.hostname, backend_addr.port)
        if backend_addr.use_ssl:
            ssl_context = ssl.create_default_context(ssl.Purpose.SERVER_AUTH)
            ssl_context.load_default_certs()
            stream = trio.SSLStream(stream, ssl_context, server_hostname=backend_addr.hostname)

        # Use HTTP 1.0 given 1.1 requires Host header
        req = f"GET {target} HTTP/1.0\r\n\r\n"
        await stream.send_all(req.encode("utf8"))
        rep = await stream.receive_some()
        await stream.aclose()
        return rep.decode("utf8")

    return _http_send


def _get_header(rep, header_name):
    header_line = next(line for line in rep.split("\r\n") if line.startswith(f"{header_name}:"))
    return header_line.split(": ", 1)[1]


@pytest.mark.trio
async def test_get_404(backend_http_send):
    rep = await backend_http_send("/dummy")
    assert rep.startswith("HTTP/1.1 404 \r\n")


@pytest.mark.trio
async def test_get_root(backend_http_send):
    rep = await backend_http_send("/")
    assert rep.startswith("HTTP/1.1 200 \r\n")


@pytest.mark.trio
async def test_get_static(backend_http_send):
    # Get resource
    rep = await backend_http_send("/static/favicon.ico")
    assert rep.startswith("HTTP/1.1 200 \r\n")

    # Also test resource in a subfolder
    rep = await backend_http_send("/static/css/base.css")
    assert rep.startswith("HTTP/1.1 200 \r\n")

    # Finally test non-existing resource
    rep = await backend_http_send("/static/dummy")
    assert rep.startswith("HTTP/1.1 404 \r\n")


@pytest.mark.trio
async def test_get_api_redirect_not_available(backend_http_send):
    rep = await backend_http_send(f"/api/redirect/foo/bar?a=1&b=2")
    assert rep.startswith("HTTP/1.1 501 \r\n")


@pytest.mark.trio
@customize_fixtures(backend_has_email=True)
async def test_get_api_redirect(backend_http_send, backend_addr):
    rep = await backend_http_send(f"/api/redirect/foo/bar?a=1&b=2")
    assert rep.startswith("HTTP/1.1 302 \r\n")
    assert _get_header(rep, "location") == f"parsec://example.com:9999/foo/bar?a=1&b=2&no_ssl=true"


@pytest.mark.trio
@customize_fixtures(backend_over_ssl=True, backend_has_email=True)
async def test_get_api_redirect_over_ssl(backend_http_send, backend_addr):
    rep = await backend_http_send(f"/api/redirect/foo/bar?a=1&b=2")
    assert rep.startswith("HTTP/1.1 302 \r\n")
    assert _get_header(rep, "location") == f"parsec://example.com:9999/foo/bar?a=1&b=2"


@pytest.mark.trio
@customize_fixtures(backend_has_email=True)
async def test_get_api_redirect_no_ssl_param_overwritten(backend_http_send, backend_addr):
    rep = await backend_http_send(f"/api/redirect/spam?no_ssl=false&a=1&b=2")
    assert rep.startswith("HTTP/1.1 302 \r\n")
    assert _get_header(rep, "location") == f"parsec://example.com:9999/spam?a=1&b=2&no_ssl=true"


@pytest.mark.trio
@customize_fixtures(backend_over_ssl=True, backend_has_email=True)
async def test_get_api_redirect_no_ssl_param_overwritten_with_ssl_enabled(
    backend_http_send, backend_addr
):
    rep = await backend_http_send(f"/api/redirect/spam?a=1&b=2&no_ssl=true")
    assert rep.startswith("HTTP/1.1 302 \r\n")
    assert _get_header(rep, "location") == f"parsec://example.com:9999/spam?a=1&b=2"


@pytest.mark.trio
@customize_fixtures(backend_has_email=True)
async def test_get_api_redirect_invitation(backend_http_send, backend_addr):
    invitation_addr = BackendInvitationAddr.build(
        backend_addr=backend_addr,
        organization_id=OrganizationID("Org"),
        invitation_type=InvitationType.USER,
        token=uuid4(),
    )
    # TODO: should use invitation_addr.to_redirection_url() when available !
    *_, target = invitation_addr.to_url().split("/")
    rep = await backend_http_send(f"/api/redirect/{target}")
    assert rep.startswith("HTTP/1.1 302 \r\n")
    location = _get_header(rep, "location")
    location_addr = BackendInvitationAddr.from_url(location)
    assert location_addr == invitation_addr


@pytest.mark.trio
@customize_fixtures(backend_over_ssl=True, backend_has_email=True)
async def test_get_api_redirect_invitation_over_ssl(backend_http_send, backend_addr):
    await test_get_api_redirect_invitation(backend_http_send, backend_addr)
