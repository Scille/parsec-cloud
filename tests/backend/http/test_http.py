# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
import trio
import h11

from parsec import __version__ as parsec_version
from parsec.api.protocol import OrganizationID, InvitationToken, InvitationType
from parsec.core.types.backend_address import BackendAddr, BackendInvitationAddr
from parsec.backend.app import MAX_INITIAL_HTTP_REQUEST_SIZE

from tests.common import customize_fixtures
from tests.backend.http.conftest import open_stream_to_backend
from tests.backend.common import parse_http_response


async def _do_test_redirect(backend_http_send):
    # No redirection header. shouln't redirect.
    req = b"GET /test HTTP/1.0\r\n\r\n"
    status, headers, test = await backend_http_send(req=req)
    assert status == (404, "Not Found")

    # Incorrect redirection header with good redirection protocol. shouln't redirect.
    req = b"GET /test HTTP/1.0\r\nX-Forwa-P:https\r\n\r\n"
    status, headers, test = await backend_http_send(req=req)
    assert status == (404, "Not Found")

    # Correct header redirection but not same redirection protocol. should redirect.
    req = b"GET / HTTP/1.0\r\nX-Forwarded-Proto:42\r\n\r\n"
    status, headers, test = await backend_http_send(req=req)
    # Only non-ssl request should lead to redirection
    assert status == (301, "Moved Permanently")

    # Make sure header key is case insensitive...
    req = b"GET / HTTP/1.0\r\nx-forwarded-proto:https\r\n\r\n"
    status, headers, test = await backend_http_send(req=req)
    assert status == (200, "OK")

    # ...but header value is not !
    req = b"GET / HTTP/1.0\r\nx-forwarded-proto:HTTPS\r\n\r\n"
    status, headers, test = await backend_http_send(req=req)
    # Only non-ssl request should lead to redirection
    assert status == (301, "Moved Permanently")

    # Correct header and redirection protocol, no redirection.
    req = b"GET /test HTTP/1.0\r\nX-Forwarded-Proto:https\r\n\r\n"
    status, headers, test = await backend_http_send(req=req)
    assert status == (404, "Not Found")

    # Correct header and redirection protocol, no redirection.
    # Root path actually return the index page of parsec so status 200 for this one.
    req = b"GET / HTTP/1.0\r\nX-Forwarded-Proto:https\r\n\r\n"
    status, headers, test = await backend_http_send(req=req)
    assert status == (200, "OK")


@customize_fixtures(backend_forward_proto_enforce_https=(b"x-forwarded-proto", b"https"))
@pytest.mark.trio
async def test_redirect_proxy(backend_http_send, running_backend):
    await _do_test_redirect(backend_http_send)


@customize_fixtures(backend_forward_proto_enforce_https=(b"x-forwarded-proto", b"https"))
@customize_fixtures(backend_over_ssl=True)
@pytest.mark.trio
async def test_forward_proto_enforce_https(backend_http_send, running_backend):
    await _do_test_redirect(backend_http_send)


@pytest.mark.trio
async def test_invalid_request_line(backend_http_send, running_backend):
    for req in [
        b"\x00",  # Early check should detect this has no chance of being an HTTP request
        b"\r\n\r\n",  # Missing everything :/
        b"HTTP/1.0\r\n\r\n",  # Missing method and target
        "GET /开始 HTTP/1.0\r\n\r\n".encode("utf8"),  # UTF-8 is not ISO-8859-1 !
        b"GET /\xf1 HTTP/1.0\r\n\r\n",  # Target part must be ISO-8859-1
        b"G\xf1T / HTTP/1.0\r\n\r\n",  # Method must be ISO-8859-1
        b"GET / HTTP/42.0\r\n\r\n",  # Only supported in Cyberpunk 2077
    ]:
        with trio.fail_after(1):
            status, _, _ = await backend_http_send(req=req)
            assert status == (400, "Bad Request")


@pytest.mark.trio
async def test_upgrade_impossible_from_http_1_0(backend_http_send, running_backend):
    # Upgrade header has been introduced in HTTP 1.1
    req_with_host = (
        b"GET /ws HTTP/1.0\r\n"
        b"host: parsec.example.com\r\n"
        b"connection: upgrade\r\n"
        b"upgrade: websocket\r\n"
        b"Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n"
        b"Sec-WebSocket-Version: 13\r\n"
        b"\r\n"
    )
    # Test with and without host header given it is mandatory from HTTP 1.1
    req_without_host = (
        b"GET /ws HTTP/1.0\r\n"
        b"connection: upgrade\r\n"
        b"upgrade: websocket\r\n"
        b"Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n"
        b"Sec-WebSocket-Version: 13\r\n"
        b"\r\n"
    )

    for req in (req_with_host, req_without_host):
        status, _, _ = await backend_http_send(req=req)
        assert status == (404, "Not Found")  # Should have been 101 if upgrade had succeeded


@pytest.mark.trio
async def test_sender_use_100_continue(running_backend, backend_addr):
    req = b"GET / HTTP/1.1\r\n" b"host: parsec.example.com\r\n" b"Expect: 100-continue\r\n" b"\r\n"

    stream = await open_stream_to_backend(
        hostname=backend_addr.hostname, port=backend_addr.port, use_ssl=backend_addr.use_ssl
    )
    await stream.send_all(req)

    rep_buf = b""

    async def _receive_rep():
        nonlocal rep_buf
        while True:
            try:
                rep, rep_buf = rep_buf.split(b"\r\n\r\n", 1)
                return parse_http_response(rep + b"\r\n\r\n")
            except ValueError:
                sub = await stream.receive_some()
                if not sub:
                    rep = rep_buf
                    rep_buf = b""
                    return parse_http_response(rep + b"\r\n\r\n")
                rep_buf += sub

    # First we should have the continue...
    status, headers = await _receive_rep()
    assert status == (100, "")
    # ...then the actual response
    status, headers = await _receive_rep()
    assert status == (200, "OK")


@pytest.mark.trio
async def test_request_is_too_big(backend_http_send, running_backend):
    real_max_initial_http_request_size = MAX_INITIAL_HTTP_REQUEST_SIZE
    max_size_req = b"GET /dummy HTTP/1.0\r\n"
    # Total request is request line + headers + final "\r\n"
    fillup_size = real_max_initial_http_request_size - len(max_size_req) - 2
    # Divide size between multiple headers
    per_header_size = 500
    headers_count = fillup_size // per_header_size
    for i in range(headers_count - 1):
        key = f"header-{i:0>4}: ".encode()
        val = b"x" * (per_header_size - len(key) - 2)
        max_size_req += key + val + b"\r\n"
    else:
        # Last header
        last_header_size = fillup_size % per_header_size + per_header_size
        key = f"header-{i+1:0>4}: ".encode()
        val = b"x" * (last_header_size - len(key) - 2)
        # Don't add the final \r\n, see below
        max_size_req += key + val
    # Too big contains one more byte
    too_big_req = max_size_req + b"y\r\n\r\n"
    max_size_req += b"\r\n\r\n"
    assert len(max_size_req) == real_max_initial_http_request_size
    assert len(too_big_req) == real_max_initial_http_request_size + 1

    # The server send the response then close the connection rigth away, this is
    # not really elegant given the client most likely only receive the closed
    # connection information
    with pytest.raises(trio.BrokenResourceError):
        status, _, _ = await backend_http_send(req=too_big_req)
        assert status == (431, "Request Header Fields Too Large")

    # Make sure max size is ok
    status, _, _ = await backend_http_send(req=max_size_req)
    assert status == (404, "Not Found")


@pytest.mark.trio
async def test_upgrade_to_websocket(backend_http_send, running_backend):
    req = (
        b"GET /ws HTTP/1.1\r\n"
        b"connection: upgrade\r\n"
        b"upgrade: websocket\r\n"
        b"host: parsec.example.com\r\n"
        b"Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n"
        b"Sec-WebSocket-Version: 13\r\n"
        b"\r\n"
    )
    status, _, _ = await backend_http_send(req=req, sanity_checks=False)
    assert status == (101, "")


@pytest.mark.trio
async def test_upgrade_to_websocket_bad_target(backend_http_send, running_backend):
    # Only /ws target is allowed for upgrade
    req = (
        b"GET /dummy HTTP/1.1\r\n"
        b"connection: upgrade\r\n"
        b"upgrade: websocket\r\n"
        b"host: parsec.example.com\r\n"
        b"Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n"
        b"Sec-WebSocket-Version: 13\r\n"
        b"\r\n"
    )
    status, _, _ = await backend_http_send(req=req)
    assert status == (404, "Not Found")


@pytest.mark.trio
async def test_unknown_upgrade(backend_http_send, running_backend):
    req_unknown_upgrade_type = (
        b"GET /ws HTTP/1.1\r\n"
        b"connection: upgrade\r\n"
        b"upgrade: dummy\r\n"
        b"host: parsec.example.com\r\n"
        b"\r\n"
    )
    req_missing_upgrade_type = (
        b"GET /ws HTTP/1.1\r\n" b"connection: upgrade\r\n" b"host: parsec.example.com\r\n" b"\r\n"
    )
    req_bad_connection_type = (
        b"GET /ws HTTP/1.1\r\n"
        b"connection: dummy\r\n"
        b"upgrade: websocket\r\n"
        b"host: parsec.example.com\r\n"
        b"\r\n"
    )

    for req in (req_unknown_upgrade_type, req_missing_upgrade_type, req_bad_connection_type):
        status, _, _ = await backend_http_send(req=req)
        # Unknown upgrade type is just ignored, so we do the http request
        assert status == (404, "Not Found")


@pytest.mark.trio
async def test_bad_method(backend_http_send, running_backend):
    req = b"SPAM / HTTP/1.0\r\n\r\n"
    status, _, _ = await backend_http_send(req=req)
    assert status == (405, "Method Not Allowed")


@pytest.mark.trio
async def test_try_keep_alive(backend_http_send, running_backend):
    # Typical request send from a web browser
    req = (
        b"GET / HTTP/1.1\r\n"
        b"Host: parsec.example.com\r\n"
        b"User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:80.0) Gecko/20100101 Firefox/80.0\r\n"
        b"Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8\r\n"
        b"Accept-Language: fr,fr-FR;q=0.8,en-US;q=0.5,en;q=0.3\r\n"
        b"Accept-Encoding: gzip, deflate\r\n"
        b"DNT: 1\r\n"
        b"Connection: keep-alive\r\n"
        b"Upgrade-Insecure-Requests: 1\r\n"
        b"Cache-Control: max-age=0\r\n"
        b"\r\n"
    )

    status, headers, _ = await backend_http_send(req=req)
    assert status == (200, "OK")
    # Parsec doesn't support keep-alive
    assert headers["connection"] == "close"


@pytest.mark.trio
async def test_server_header_in_debug(backend_factory, server_factory, backend_http_send):
    config = {"debug": True}
    expected_server_label = f"parsec/{parsec_version} {h11.PRODUCT_ID}"

    async with backend_factory(populated=False, config=config) as backend:
        async with server_factory(backend.handle_client) as server:
            addr = BackendAddr(hostname="127.0.0.1", port=server.port, use_ssl=False)
            req = b"GET / HTTP/1.0\r\n\r\n"
            status, headers, _ = await backend_http_send(req=req, addr=addr, sanity_checks=False)
            assert status == (200, "OK")
            assert headers["server"] == expected_server_label


@pytest.mark.trio
async def test_get_404(backend_http_send, running_backend):
    status, headers, body = await backend_http_send("/dummy")
    assert status == (404, "Not Found")
    assert headers["content-type"] == "text/html;charset=utf-8"
    assert body


@pytest.mark.trio
async def test_unexpected_exception_get_500(
    backend, server_factory, backend_http_send, monkeypatch
):
    class MyUnexpectedException(Exception):
        pass

    async def _patched_http_404(*args, **kwargs):
        raise MyUnexpectedException("Unexpected error !")

    monkeypatch.setattr("parsec.backend.http.HTTPComponent._http_404", _patched_http_404)

    async def _run_server_until_unexpected_exception(*, task_status=trio.TASK_STATUS_IGNORED):
        try:
            async with server_factory(backend.handle_client) as server:
                task_status.started(server.port)
                await trio.sleep_forever()
        except MyUnexpectedException:
            pass

    async with trio.open_nursery() as nursery:
        backend_port = await nursery.start(_run_server_until_unexpected_exception)
        addr = BackendAddr(hostname="127.0.0.1", port=backend_port, use_ssl=False)

        status, *_ = await backend_http_send("/dummy", addr=addr)

        assert status == (500, "Internal Server Error")


@pytest.mark.trio
async def test_get_root(backend_http_send, running_backend):
    status, headers, body = await backend_http_send("/")
    assert status == (200, "OK")
    assert headers["content-type"] == "text/html;charset=utf-8"
    assert body


@pytest.mark.trio
async def test_get_static(backend_http_send, running_backend):
    # Get resource
    status, headers, body = await backend_http_send("/static/favicon.ico")
    assert status == (200, "OK")
    # Oddly enough, Windows considers .ico to be `image/x-icon` while IANA says `image/vnd.microsoft.icon`
    assert headers["content-type"] in ("image/vnd.microsoft.icon", "image/x-icon")
    assert body

    # Also test resource in a subfolder
    status, headers, body = await backend_http_send("/static/base.css")
    assert status == (200, "OK")
    assert headers["content-type"] == "text/css"
    assert body

    # __init__.py is present but shouldn't be visible
    status, headers, body = await backend_http_send("/static/__init__.py")
    assert status == (404, "Not Found")
    assert not body

    # Finally test non-existing resource
    status, headers, body = await backend_http_send("/static/dummy")
    assert status == (404, "Not Found")
    assert not body

    # Prevent from leaving the static directory
    status, headers, body = await backend_http_send("/static/../__init__.py")
    assert status == (404, "Not Found")
    assert not body


@pytest.mark.trio
@pytest.mark.parametrize("kind", ["ascii", "unicode"])
async def test_get_redirect(backend_http_send, running_backend, kind):
    if kind == "ascii":
        target = "foo/bar?a=1&b=2"
    else:
        assert kind == "unicode"
        target = "%E4%BD%A0%E5%A5%BD/%E8%82%AF%E8%AF%BA%E6%AF%94?%E4%B8%8A%E5%B0%86=%E8%A1%8C"

    status, headers, body = await backend_http_send(f"/redirect/{target}")
    assert status == (302, "Found")
    assert headers["location"] == f"parsec://127.0.0.1:{running_backend.port}/{target}&no_ssl=true"
    assert not body


@pytest.mark.trio
@customize_fixtures(backend_over_ssl=True)
async def test_get_redirect_over_ssl(backend_http_send, running_backend):
    status, headers, body = await backend_http_send("/redirect/foo/bar?a=1&b=2")
    assert status == (302, "Found")
    assert headers["location"] == f"parsec://127.0.0.1:{running_backend.port}/foo/bar?a=1&b=2"
    assert not body


@pytest.mark.trio
async def test_get_redirect_no_ssl_param_overwritten(backend_http_send, running_backend):
    status, headers, body = await backend_http_send("/redirect/spam?no_ssl=false&a=1&b=2")
    assert status == (302, "Found")
    assert (
        headers["location"] == f"parsec://127.0.0.1:{running_backend.port}/spam?a=1&b=2&no_ssl=true"
    )
    assert not body


@pytest.mark.trio
@customize_fixtures(backend_over_ssl=True)
async def test_get_redirect_no_ssl_param_overwritten_with_ssl_enabled(
    backend_http_send, running_backend
):
    status, headers, body = await backend_http_send(f"/redirect/spam?a=1&b=2&no_ssl=true")
    assert status == (302, "Found")
    assert headers["location"] == f"parsec://127.0.0.1:{running_backend.port}/spam?a=1&b=2"
    assert not body


@pytest.mark.trio
async def test_get_redirect_invitation(backend_http_send, running_backend, backend_addr):
    invitation_addr = BackendInvitationAddr.build(
        backend_addr=backend_addr,
        organization_id=OrganizationID("Org"),
        invitation_type=InvitationType.USER,
        token=InvitationToken.new(),
    )
    # TODO: should use invitation_addr.to_redirection_url() when available !
    *_, target = invitation_addr.to_url().split("/")
    status, headers, body = await backend_http_send(f"/redirect/{target}")
    assert status == (302, "Found")
    location_addr = BackendInvitationAddr.from_url(headers["location"])
    assert location_addr == invitation_addr


@pytest.mark.trio
@customize_fixtures(backend_over_ssl=True)
async def test_get_redirect_invitation_over_ssl(backend_http_send, running_backend, backend_addr):
    await test_get_redirect_invitation(backend_http_send, running_backend, backend_addr)
