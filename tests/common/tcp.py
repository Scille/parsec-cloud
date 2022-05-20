# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
import h11
import urllib.error
from typing import Optional, Dict
from contextlib import contextmanager, closing as contextlib_closing
from unittest.mock import patch
import socket
import inspect
from urllib.parse import urlparse
import trio


def addr_to_netloc(addr):
    # Can't do that with Rust, no inheritance
    # assert isinstance(addr, BackendAddr)
    return f"{addr.hostname}:{addr.port}"


class OpenTCPStreamMockWrapper:
    def __init__(self):
        self._socks = []
        self._hooks = {}
        self._offlines = set()

    @contextmanager
    def install_hook(self, addr, hook):
        self.push_hook(addr, hook)
        try:
            yield
        finally:
            self.pop_hook(addr)

    def push_hook(self, addr, hook):
        netloc_suffix = addr_to_netloc(addr)
        assert netloc_suffix not in self._hooks
        self._hooks[netloc_suffix] = hook

    def pop_hook(self, addr):
        netloc_suffix = addr_to_netloc(addr)
        self._hooks.pop(netloc_suffix)

    def get_socks(self, addr):
        netloc_suffix = addr_to_netloc(addr)
        return [sock for netloc, sock in self._socks if netloc.endswith(netloc_suffix)]

    async def __call__(self, host, port, *args, **kwargs):
        netloc = f"{host}:{port}" if port is not None else host
        hook = next(
            (hook for netloc_suffix, hook in self._hooks.items() if netloc.endswith(netloc_suffix)),
            None,
        )
        if hook and not any(netloc.endswith(offline) for offline in self._offlines):
            if inspect.iscoroutinefunction(hook):
                sock = await hook(host, port, *args, **kwargs)
            else:
                sock = hook(host, port, *args, **kwargs)

            if sock:
                self._socks.append((netloc, sock))
                return sock

        raise ConnectionRefusedError(111, "Connection refused")

    async def http_request(
        self,
        url: str,
        data: Optional[bytes] = None,
        headers: Dict[str, str] = {},
        method: Optional[str] = None,
    ) -> bytes:
        """
        The `parsec.core.backend_connection.anonymous._http_request` call originally submit a call to
        `urllib.request.urlopen` to a thread. In order to transparently integrate with the existing
        mock for TCP stream, a simple HTTP client based on trio and h11 is re-implemented here.

        This allows to perform anonymous calls like `pki_enrollment_submit` and `pki_enrollment_info`
        along with the mocked `running_backend` fixture.
        """
        parse_result = urlparse(url, "http")
        netloc = parse_result.netloc
        target = parse_result.path
        host, port = netloc.split(":")
        try:
            sock = await self(host, port)
        except ConnectionRefusedError as exc:
            raise urllib.error.URLError(exc)
        headers = [(key, value) for key, value in headers.items()]
        headers += [("Host", host), ("Content-Length", str(len(data)))]

        connection = h11.Connection(our_role=h11.CLIENT)
        await sock.send_all(
            connection.send(h11.Request(method=method, target=target, headers=headers))
        )
        await sock.send_all(connection.send(h11.Data(data=data)))
        await sock.send_all(connection.send(h11.EndOfMessage()))

        async with sock:
            out_data = b""
            response = None
            while True:
                # Check if an event is already available
                event = connection.next_event()
                if event is h11.NEED_DATA:
                    data = await sock.receive_some(2048)
                    connection.receive_data(data)
                    continue
                if type(event) is h11.EndOfMessage:
                    break
                if type(event) is h11.Response:
                    response = event
                if type(event) is h11.Data:
                    out_data += event.data

        if response is None:
            raise urllib.error.URLError("No response")
        if response.status_code != 200:
            exc = urllib.error.URLError("Error")
            exc.status = response.status_code
            raise exc
        return out_data

    def switch_offline(self, addr):
        netloc_suffix = addr_to_netloc(addr)
        if netloc_suffix in self._offlines:
            return

        for netloc, sock in self._socks:
            if not netloc.endswith(netloc_suffix):
                continue

            async def _broken_stream(*args, **kwargs):
                raise trio.BrokenResourceError()

            _broken_stream.old_send_all_hook = sock.send_stream.send_all_hook
            _broken_stream.old_receive_some_hook = sock.receive_stream.receive_some_hook

            sock.send_stream.send_all_hook = _broken_stream
            sock.receive_stream.receive_some_hook = _broken_stream

            # Unlike for send stream, patching `receive_some_hook` is not enough
            # because it is called by the stream before actually waiting for data
            # (so in case of a long receive call, we could be past the hook call
            # when patching, hence not blocking the next incoming frame)
            sock.receive_stream.put_eof()

        self._offlines.add(netloc_suffix)

    def switch_online(self, addr):
        netloc_suffix = addr_to_netloc(addr)
        if netloc_suffix not in self._offlines:
            return

        # Note we don't try to revert the patched streams because there
        # represent old connections that have been lost
        self._offlines.remove(netloc_suffix)

    @contextmanager
    def offline(self, addr):
        self.switch_offline(addr)
        try:
            yield

        finally:
            self.switch_online(addr)


@contextmanager
def wrap_open_tcp_stream():
    open_tcp_stream_mock_wrapper = OpenTCPStreamMockWrapper()
    with patch("trio.open_tcp_stream", new=open_tcp_stream_mock_wrapper):
        yield


@contextmanager
def offline(addr):
    if not isinstance(trio.open_tcp_stream, OpenTCPStreamMockWrapper):
        raise RuntimeError("`tcp_stream_spy` fixture is missing")
    trio.open_tcp_stream.switch_offline(addr)
    try:
        yield

    finally:
        trio.open_tcp_stream.switch_online(addr)


@pytest.fixture
def tcp_stream_spy(request, monkeypatch):
    if request.node.get_closest_marker("real_tcp"):
        return None
    else:
        open_tcp_stream_mock_wrapper = OpenTCPStreamMockWrapper()
        monkeypatch.setattr(
            "trio.open_tcp_stream", open_tcp_stream_mock_wrapper.mocked_open_tcp_stream
        )
        monkeypatch.setattr("trio.serve_tcp", open_tcp_stream_mock_wrapper.mocked_serve_tcp)
        monkeypatch.setattr(
            "parsec.core.backend_connection.anonymous.http_request",
            open_tcp_stream_mock_wrapper.mocked_http_request,
        )
        return open_tcp_stream_mock_wrapper


@pytest.fixture(scope="session")
def unused_tcp_port():
    """Find an unused localhost TCP port from 1024-65535 and return it."""
    with contextlib_closing(socket.socket()) as sock:
        sock.bind(("127.0.0.1", 0))
        return sock.getsockname()[1]
