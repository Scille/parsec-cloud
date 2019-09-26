# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio

from parsec.api.protocol import packb, unpackb


@pytest.mark.trio
async def test_connection(alice_backend_sock):
    await alice_backend_sock.send(packb({"cmd": "ping", "ping": "42"}))
    rep = await alice_backend_sock.recv()
    assert unpackb(rep) == {"status": "ok", "pong": "42"}


@pytest.mark.trio
async def test_bad_cmd(alice_backend_sock):
    await alice_backend_sock.send(packb({"cmd": "dummy"}))
    rep = await alice_backend_sock.recv()
    assert unpackb(rep) == {"status": "unknown_command", "reason": "Unknown command"}


# @pytest.mark.trio
# async def test_bad_msg_format(alice_backend_sock):
#     await alice_backend_sock.stream.send_all(b"\x00\x00\x00\x04fooo")
#     rep = await alice_backend_sock.recv()
#     assert unpackb(rep) == {"status": "invalid_msg_format", "reason": "Invalid message format"}


@pytest.mark.trio
async def test_try_http_connection(running_backend, mock_clock):
    mock_clock.autojump_threshold = 0
    client_stream = await trio.open_tcp_stream(
        running_backend.addr.hostname, running_backend.addr.port
    )
    request = (
        b"GET /index.html HTTP/1.1\r\n"
        b"Host: parsec.example.com\r\n"
        b"User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:69.0) Gecko/20100101 Firefox/69.0\r\n"
        b"Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n"
        b"Accept-Language: fr,fr-FR;q=0.8,en-US;q=0.5,en;q=0.3\r\n"
        b"Accept-Encoding: gzip, deflate, br\r\n"
        b"Referer: https://parsec.example.com/\r\n"
        b"DNT: 1\r\n"
        b"Connection: keep-alive\r\n"
        b"Upgrade-Insecure-Requests: 1\r\n"
        b"Cache-Control: max-age=0\r\n"
        b"\r\n"
    )
    await client_stream.send_all(request)
    await trio.sleep(1)
    rep = await client_stream.receive_some(1000)
    assert rep == (
        b"HTTP/1.1 426 OK\r\n"
        b"Upgrade: WebSocket\r\n"
        b"Content-Length: 51\r\n"
        b"Connection: Upgrade\r\n"
        b"Content-Type: text/html; charset=UTF-8\r\n"
        b"\r\n"
        b"This service requires use of the WebSocket protocol"
    )
    # Connection has been closed by peer
    with pytest.raises(trio.BrokenResourceError):
        await client_stream.send_all(b"dummy")
