# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from uuid import uuid4
from tests.common import customize_fixtures
from urllib.parse import urlsplit

from parsec.core.types.backend_address import BackendInvitationAddr


@pytest.mark.trio
async def test_send_http_request_invalid_route(running_backend):
    stream = await trio.open_tcp_stream(running_backend.addr.hostname, running_backend.addr.port)
    await stream.send_all(
        b"GET / HTTP/1.1\r\n"
        b"Host: parsec.example.com\r\n"
        b"User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:69.0) Gecko/20100101 Firefox/69.0\r\n"
        b"Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n"
        b"Accept-Language: fr,fr-FR;q=0.8,en-US;q=0.5,en;q=0.3\r\n"
        b"Accept-Encoding: gzip, deflate\r\n"
        b"DNT: 1\r\n"
        b"Connection: keep-alive\r\n"
        b"Upgrade-Insecure-Requests: 1\r\n"
        b"Cache-Control: max-age=0\r\n"
        b"\r\n"
    )
    rep = await stream.receive_some()
    rep = rep.decode("utf-8")
    assert "HTTP/1.1 404" in rep


@pytest.mark.trio
@customize_fixtures(backend_has_email=True)
@pytest.mark.parametrize("no_ssl", [False, True])
async def test_send_http_invite_request_to_redirect(running_backend, no_ssl):
    urlsplitted = urlsplit(running_backend.addr.to_url())
    netloc = urlsplitted.netloc
    splitted_netloc = netloc.split(":")
    hostname = splitted_netloc[0]
    port = splitted_netloc[1]
    backend_invitation_addr = BackendInvitationAddr(
        hostname=hostname,
        port=port,
        use_ssl=not no_ssl,
        organization_id="organization_id",
        invitation_type="invitation_type",
        token=uuid4(),
    )

    stream = await trio.open_tcp_stream(
        backend_invitation_addr.hostname, backend_invitation_addr.port
    )
    req = (
        "GET /api/redirect"
        f"?organization_id={backend_invitation_addr.organization_id}"
        f"&invitation_type={backend_invitation_addr.invitation_type}&"
        f"token={backend_invitation_addr.token}"
        f"&no_ssl={ not backend_invitation_addr.use_ssl} HTTP/1.0\r\n\r\n"
    ).encode("utf-8")

    await stream.send_all(req)
    rep = await stream.receive_some()
    rep = rep.decode("utf-8")
    rep_lines = rep.split("\n")
    assert "HTTP/1.1 302" in rep_lines[0]
    location_header_line = next(line for line in rep_lines if line.startswith("location:"))
    assert (
        f"parsec://example.com:9999?no_ssl={ no_ssl }&organization_id={backend_invitation_addr.organization_id}&invitation_type={backend_invitation_addr.invitation_type}&token={backend_invitation_addr.token}"
        == location_header_line[len("location: ") :].strip()
    )


@pytest.mark.trio
@pytest.mark.parametrize("no_ssl", [False, True])
@customize_fixtures(backend_has_email=True)
async def test_send_http_request_to_redirect(running_backend, no_ssl):
    urlsplitted = urlsplit(running_backend.addr.to_url())
    netloc = urlsplitted.netloc
    splitted_netloc = netloc.split(":")
    hostname = splitted_netloc[0]
    port = splitted_netloc[1]

    stream = await trio.open_tcp_stream(hostname, port)
    token = uuid4()
    req = (
        "GET /api/redirect"
        "?some_args=thisisthefirstarg"
        "&emptyone=&"
        f"token={token}"
        f"&no_ssl={ no_ssl } HTTP/1.0\r\n\r\n"
        "foo=bar#touille"
    ).encode("utf-8")

    await stream.send_all(req)
    rep = await stream.receive_some()
    rep = rep.decode("utf-8")
    rep_lines = rep.split("\n")
    assert "HTTP/1.1 302" in rep_lines[0]
    location_header_line = next(line for line in rep_lines if line.startswith("location:"))
    assert (
        f"parsec://example.com:9999?no_ssl={ no_ssl }&some_args=thisisthefirstarg&token={ token }"
        == location_header_line[len("location: ") :].strip()
    )


@pytest.mark.trio
async def test_static(running_backend):
    # test get favicon.ico
    stream = await trio.open_tcp_stream(running_backend.addr.hostname, running_backend.addr.port)
    await stream.send_all(b"GET /static/favicon.ico HTTP/1.0\r\n\r\n")
    rep = await stream.receive_some()
    rep = rep.decode("utf-8")
    assert "HTTP/1.1 200" in rep
    # same test but inside css folder
    stream = await trio.open_tcp_stream(running_backend.addr.hostname, running_backend.addr.port)
    await stream.send_all(b"GET /static/css/base.css HTTP/1.0\r\n\r\n")
    rep = await stream.receive_some()
    data = await stream.receive_some()
    rep = rep.decode("utf-8")
    data = data.decode("utf-8")
    assert "HTTP/1.1 200" in rep
    data = data.replace("\r", "")
    assert (
        ".main-title {\n    margin: 30px, 10%, 30px;\n}\n\n"
        ".text-secondary {\n    color: #121D43 !important;\n}\n\n"
        ".text-muted {\n    color: #6c757d!important;\n}\n\n"
        ".text-center {\n    text-align: center!important;\n}\n\n"
        ".main-title-spacer {\n    width: 60px;\n    height: 2px;\n    "
        "margin: 20px auto 50px auto;\n}\n\n"
        ".bg-primary {\n    background-color: #006eff !important"
    ) in data
    # same test but resource doesn't exist
    stream = await trio.open_tcp_stream(running_backend.addr.hostname, running_backend.addr.port)
    await stream.send_all(b"GET /static/css/nonexistent1234.css HTTP/1.0\r\n\r\n")
    rep = await stream.receive_some()
    data = await stream.receive_some()
    rep = rep.decode("utf-8")
    data = data.decode("utf-8")
    assert "HTTP/1.1 404" in rep
    assert data == ""
