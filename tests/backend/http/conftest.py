# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
import trio
import ssl
from typing import Optional, Tuple, Union
import json

from tests.common import real_clock_timeout
from tests.backend.common import do_http_request


async def open_stream_to_backend(hostname, port, use_ssl):
    stream = await trio.open_tcp_stream(hostname, port)
    if use_ssl:
        ssl_context = ssl.create_default_context(ssl.Purpose.SERVER_AUTH)
        ssl_context.load_default_certs()
        stream = trio.SSLStream(stream, ssl_context, server_hostname=hostname)
    return stream


@pytest.fixture
def backend_http_send(backend_addr):
    async def _http_send(
        target: Optional[str] = None,
        method: str = "GET",
        req: Optional[bytes] = None,
        headers: Optional[dict] = None,
        body: Optional[bytes] = None,
        sanity_checks: bool = True,
        addr=backend_addr,
    ) -> Tuple[int, dict, bytes]:
        stream = await open_stream_to_backend(addr.hostname, addr.port, addr.use_ssl)

        status, rep_headers, rep_body = await do_http_request(
            stream, target=target, method=method, req=req, headers=headers, body=body
        )

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
def backend_rest_send(running_backend, backend_http_send, backend_addr):
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
            headers.setdefault(
                "Authorization", f"Bearer {running_backend.backend.config.administration_token}"
            )
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
    async with real_clock_timeout():
        rep = await stream.receive_some()
        assert rep == b""
        # From now on, trying to send new request should fail
        with pytest.raises(trio.BrokenResourceError):
            # Peer side closing of TCP socket may take time, so it's possible
            # we can still send for a small amount of time
            while True:
                await stream.send_all(b"GET / HTTP/1.0\r\n\r\n")
