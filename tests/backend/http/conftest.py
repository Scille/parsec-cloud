# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
import trio
import ssl
from typing import Optional, Tuple, Union
import json


async def open_stream_to_backend(backend_addr):
    stream = await trio.open_tcp_stream(backend_addr.hostname, backend_addr.port)
    if backend_addr.use_ssl:
        ssl_context = ssl.create_default_context(ssl.Purpose.SERVER_AUTH)
        ssl_context.load_default_certs()
        stream = trio.SSLStream(stream, ssl_context, server_hostname=backend_addr.hostname)
    return stream


def craft_http_request(
    target: str, method: str, headers: dict, body: Optional[bytes], protocol: str = "1.0"
) -> bytes:
    if body is None:
        body = b""
    else:
        assert isinstance(body, bytes)
        headers = {**headers, "content-length": len(body)}

    # Use HTTP 1.0 by default given 1.1 requires Host header
    req = f"{method} {target} HTTP/{protocol}\r\n"
    req += "\r\n".join(f"{key}: {value}" for key, value in headers.items())
    while not req.endswith("\r\n\r\n"):
        req += "\r\n"

    return req.encode("ascii") + body


def parse_http_response(raw: bytes):
    head, _ = raw.split(b"\r\n\r\n", 1)  # Ignore the body part
    status, *headers = head.split(b"\r\n")
    protocole, status_code, status_label = status.split(b" ", 2)
    assert protocole == b"HTTP/1.1"
    cooked_status = (int(status_code.decode("ascii")), status_label.decode("ascii"))
    cooked_headers = {}
    for header in headers:
        key, value = header.split(b": ")
        cooked_headers[key.decode("ascii").lower()] = value.decode("ascii")
    return cooked_status, cooked_headers


@pytest.fixture
def backend_http_send(running_backend, backend_addr):
    async def _http_send(
        target: Optional[str] = None,
        method: str = "GET",
        req: Optional[bytes] = None,
        headers: Optional[dict] = None,
        body: Optional[bytes] = None,
        sanity_checks: bool = True,
        addr=backend_addr,
    ) -> Tuple[int, dict, bytes]:
        if req is None:
            assert target is not None
            req = craft_http_request(target, method, headers or {}, body)
        else:
            assert target is None
            assert headers is None
            assert body is None

        stream = await open_stream_to_backend(addr)
        await stream.send_all(req)

        # In theory there is no guarantee `stream.receive_some()` outputs
        # an entire HTTP request (it typically depends on the TCP stack and
        # the network). However given we mock the TCP stack in the tests
        # (see `OpenTCPStreamMockWrapper` class), we have the guarantee
        # a buffer send through `stream.send_data()` on the backend side
        # won't be splitted into multiple `stream.receive_some()` on the
        # client side (and vice-versa). However it's still possible for
        # multiple `stream.send_data()` to be merged and outputed on a
        # single `stream.receive_some()`.
        # Of course this is totally dependant of the backend's implementation
        # so things may change in the future ;-)
        rep = await stream.receive_some()
        status, rep_headers = parse_http_response(rep)
        rep_body = rep.split(b"\r\n\r\n", 1)[1]
        content_size = int(rep_headers.get("content-length", "0"))
        if content_size:
            while len(rep_body) < content_size:
                rep_body += await stream.receive_some()
            # No need to check for another request beeing put after the
            # body in the buffer given we don't use keep alive
            assert len(rep_body) == content_size
        else:
            assert rep_body == b""
        if sanity_checks:
            # Cheap checks on always present headers
            assert "date" in rep_headers
            assert rep_headers["server"] == "parsec"
            # We never support any sort of keep-alive
            assert rep_headers["connection"] == "close"
            await assert_stream_closed_on_peer_side(stream)
        return status, rep_headers, rep_body

    return _http_send


@pytest.fixture
def backend_rest_send(backend, backend_http_send, backend_addr):
    async def _rest_send(
        target: str,
        method: str = "GET",
        headers: Optional[dict] = None,
        body: Union[list, dict, None] = None,
        sanity_checks: bool = True,
        with_administration_token: bool = True,
        addr=backend_addr,
    ) -> Tuple[int, dict, Union[list, dict]]:
        headers = headers or {}
        if with_administration_token:
            headers.setdefault("Authorization", f"Bearer {backend.config.administration_token}")
        if body is not None:
            body = json.dumps(body).encode("utf-8")
            headers.setdefault("content-type", "application/json;charset=utf-8")

        status, rep_headers, rep_body = await backend_http_send(
            target=target,
            method=method,
            headers=headers,
            body=body,
            sanity_checks=sanity_checks,
            addr=addr,
        )

        if rep_headers.get("content-type", "") == "application/json;charset=utf-8":
            rep_body = json.loads(rep_body.decode("utf-8"))

        return status, rep_headers, rep_body

    return _rest_send


async def assert_stream_closed_on_peer_side(stream):
    # Peer should send EOF and close connection
    rep = await stream.receive_some()
    assert rep == b""
    # From now on, trying to send new request should fail
    with pytest.raises(trio.BrokenResourceError):
        await stream.send_all(b"GET / HTTP/1.0\r\n\r\n")
