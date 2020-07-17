# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio


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
async def test_send_http_request_to_redirect(running_backend):
    stream = await trio.open_tcp_stream(running_backend.addr.hostname, running_backend.addr.port)
    await stream.send_all(
        b"GET /api/redirect?organization_id=thisistheorganizationid123456789&invitation_type=invitation_type&token=S3CRET&no_ssl=true HTTP/1.0\r\n\r\n"
    )
    rep = await stream.receive_some()
    rep = rep.decode("utf-8")
    assert str(running_backend.addr) in rep
    assert "HTTP/1.1 302" in rep


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
        ".main-title {\n    margin: 30px, 10%, 30px;\n}\n\n.text-secondary {\n    color: #121D43 !important;\n}\n\n.text-muted {\n    color: #6c757d!important;\n}\n\n.text-center {\n    text-align: center!important;\n}\n\n.main-title-spacer {\n    width: 60px;\n    height: 2px;\n    margin: 20px auto 50px auto;\n}\n\n.bg-primary {\n    background-color: #006eff !important"
        in data
    )
    # same test but resource doesn't exist
    stream = await trio.open_tcp_stream(running_backend.addr.hostname, running_backend.addr.port)
    await stream.send_all(b"GET /static/css/nonexistent1234.css HTTP/1.0\r\n\r\n")
    rep = await stream.receive_some()
    data = await stream.receive_some()
    rep = rep.decode("utf-8")
    data = data.decode("utf-8")
    assert "HTTP/1.1 404" in rep
    assert data == ""
