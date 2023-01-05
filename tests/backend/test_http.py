# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from urllib.request import HTTPError, Request, urlopen

import pytest
import trio

from parsec import __version__ as parsec_version
from parsec._parsec import InvitationType
from parsec.api.protocol import InvitationToken, OrganizationID
from parsec.backend.asgi import MAX_CONTENT_LENGTH, serve_backend_with_asgi
from parsec.core.types.backend_address import BackendInvitationAddr
from tests.common import customize_fixtures


async def _do_test_redirect(client):
    # No redirection header. shouldn't redirect.
    rep = await client.get("/test")
    assert rep.status == "404 NOT FOUND"

    # Incorrect redirection header with good redirection protocol. shouldn't redirect.
    rep = await client.get("/test", headers={"X-Forwa-P": "https"})  # cspell: ignore Forwa
    assert rep.status == "404 NOT FOUND"

    # Correct header redirection but not same redirection protocol. should redirect.
    rep = await client.get("/test", headers={"X-Forwarded-Proto": "42"})
    # Only non-ssl request should lead to redirection
    assert rep.status == "301 MOVED PERMANENTLY"

    # Make sure header key is case insensitive...
    rep = await client.get("/", headers={"x-forwarded-proto": "https"})
    assert rep.status == "200 OK"

    # ...but header value is not !
    rep = await client.get("/test", headers={"x-forwarded-proto": "HTTPS"})
    # Only non-ssl request should lead to redirection
    assert rep.status == "301 MOVED PERMANENTLY"

    # Correct header and redirection protocol, no redirection.
    rep = await client.get("/test", headers={"X-Forwarded-Proto": "https"})
    assert rep.status == "404 NOT FOUND"

    # Correct header and redirection protocol, no redirection.
    # Root path actually return the index page of parsec so status 200 for this one.
    rep = await client.get("/", headers={"X-Forwarded-Proto": "https"})
    assert rep.status == "200 OK"


@customize_fixtures(backend_forward_proto_enforce_https=("x-forwarded-proto", "https"))
@pytest.mark.trio
async def test_redirect_proxy(backend_asgi_app):
    client = backend_asgi_app.test_client()
    await _do_test_redirect(client)


@customize_fixtures(backend_forward_proto_enforce_https=("x-forwarded-proto", "https"))
@customize_fixtures(backend_over_ssl=True)
@pytest.mark.trio
async def test_forward_proto_enforce_https(backend_asgi_app):
    client = backend_asgi_app.test_client()
    await _do_test_redirect(client)


@pytest.mark.trio
@pytest.mark.parametrize("mode", ("prod", "debug"))
async def test_server_header_in_debug(backend_factory, mode):
    if mode == "debug":
        config = {"debug": True}
        expected_server_header = f"parsec/{parsec_version}"
    else:
        assert mode == "prod"
        config = {"debug": False}
        expected_server_header = f"parsec"

    async with backend_factory(populated=False, config=config) as backend:
        async with trio.open_nursery() as nursery:
            binds = await nursery.start(serve_backend_with_asgi, backend, "127.0.0.1", 0)
            baseurl = binds[0]

            for endpoint, method, expected_status in [
                ("/", "GET", 200),
                ("/", "HEAD", 200),
                ("/dummy", "GET", 404),
                ("/", "POST", 405),
            ]:
                req = Request(url=f"{baseurl}{endpoint}", method=method)
                try:
                    rep = await trio.to_thread.run_sync(urlopen, req)
                except HTTPError as exc:
                    rep = exc
                assert rep.status == expected_status
                assert rep.headers["server"] == expected_server_header

            nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_get_404(backend_asgi_app):
    client = backend_asgi_app.test_client()
    rep = await client.get("/dummy")
    assert rep.status == "404 NOT FOUND"
    assert rep.headers["content-type"] == "text/html; charset=utf-8"
    assert await rep.get_data()


@pytest.mark.trio
async def test_unexpected_exception_get_500(backend_asgi_app, monkeypatch, caplog):
    class MyUnexpectedException(Exception):
        pass

    async def _patched_render_template(*args, **kwargs):
        raise MyUnexpectedException("Unexpected error !")

    monkeypatch.setattr("parsec.backend.asgi.render_template", _patched_render_template)
    client = backend_asgi_app.test_client()
    rep = await client.get("/dummy")
    assert rep.status == "500 INTERNAL SERVER ERROR"

    # ASGI app also report the crash in the log
    caplog.assert_occurred_once("Exception on request GET /dummy")
    caplog.clear()


@pytest.mark.trio
async def test_get_root(backend_asgi_app):
    client = backend_asgi_app.test_client()
    rep = await client.get("/")
    assert rep.status == "200 OK"
    assert rep.headers["content-type"] == "text/html; charset=utf-8"
    assert await rep.get_data()


@pytest.mark.trio
async def test_get_static(backend_asgi_app):
    client = backend_asgi_app.test_client()

    # Get resource
    rep = await client.get("/static/favicon.ico")
    assert rep.status == "200 OK"
    # rep.Oddly enough, Windows considers .ico to be `image/x-icon` while IANA says `image/vnd.microsoft.icon`
    assert rep.headers["content-type"] in ("image/vnd.microsoft.icon", "image/x-icon")
    assert await rep.get_data()

    # Also test resource in a subfolder
    rep = await client.get("/static/base.css")
    assert rep.status == "200 OK"
    assert rep.headers["content-type"] == "text/css; charset=utf-8"
    assert await rep.get_data()

    # Finally test non-existing resource
    rep = await client.get("/static/dummy")
    assert rep.status == "404 NOT FOUND"

    # Prevent from leaving the static directory
    rep = await client.get("/static/../__init__.py")
    assert rep.status == "404 NOT FOUND"


@pytest.mark.trio
async def test_get_redirect(backend_asgi_app, backend_addr):
    client = backend_asgi_app.test_client()

    rep = await client.get("/redirect/foo/bar?a=1&b=2")
    assert rep.status == "302 FOUND"
    assert rep.headers["location"] == f"parsec://{backend_addr.netloc}/foo/bar?a=1&b=2&no_ssl=true"


@pytest.mark.trio
@customize_fixtures(backend_over_ssl=True)
async def test_get_redirect_over_ssl(backend_asgi_app, backend_addr):
    client = backend_asgi_app.test_client()

    rep = await client.get("/redirect/foo/bar?a=1&b=2")
    assert rep.status == "302 FOUND"
    assert rep.headers["location"] == f"parsec://{backend_addr.netloc}/foo/bar?a=1&b=2"


@pytest.mark.trio
async def test_get_redirect_no_ssl_param_overwritten(backend_asgi_app, backend_addr):
    client = backend_asgi_app.test_client()

    rep = await client.get("/redirect/spam?no_ssl=false&a=1&b=2")
    assert rep.status == "302 FOUND"
    assert rep.headers["location"] == f"parsec://{backend_addr.netloc}/spam?a=1&b=2&no_ssl=true"


@pytest.mark.trio
@customize_fixtures(backend_over_ssl=True)
async def test_get_redirect_no_ssl_param_overwritten_with_ssl_enabled(
    backend_asgi_app, backend_addr
):
    client = backend_asgi_app.test_client()

    rep = await client.get(f"/redirect/spam?a=1&b=2&no_ssl=true")
    assert rep.status == "302 FOUND"
    assert rep.headers["location"] == f"parsec://{backend_addr.netloc}/spam?a=1&b=2"


@pytest.mark.trio
async def test_get_redirect_invitation(backend_asgi_app, backend_addr):
    client = backend_asgi_app.test_client()

    invitation_addr = BackendInvitationAddr.build(
        backend_addr=backend_addr,
        organization_id=OrganizationID("Org"),
        invitation_type=InvitationType.USER,
        token=InvitationToken.new(),
    )
    # TODO: should use invitation_addr.to_redirection_url() when available !
    *_, target = invitation_addr.to_url().split("/")
    rep = await client.get(f"/redirect/{target}")
    assert rep.status == "302 FOUND"
    location_addr = BackendInvitationAddr.from_url(rep.headers["location"])
    assert location_addr == invitation_addr


@pytest.mark.trio
@customize_fixtures(backend_over_ssl=True)
async def test_get_redirect_invitation_over_ssl(backend_asgi_app, backend_addr):
    await test_get_redirect_invitation(backend_asgi_app, backend_addr)


@pytest.mark.trio
async def test_content_is_too_big(backend_asgi_app):
    client = backend_asgi_app.test_client()

    max_length_content = b"x" * MAX_CONTENT_LENGTH
    headers = {"Authorization": f"Bearer {backend_asgi_app.backend.config.administration_token}"}

    response = await client.post(
        "/administration/organizations", headers=headers, data=max_length_content + b"y"
    )
    assert response.status == "413 REQUEST ENTITY TOO LARGE"

    # Make sure max length is ok
    response = await client.post(
        "/administration/organizations", headers=headers, data=max_length_content
    )
    assert response.status == "400 BAD REQUEST"
