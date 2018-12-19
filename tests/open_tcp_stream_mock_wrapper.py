from collections import defaultdict
from contextlib import contextmanager
from unittest.mock import patch
import inspect
import trio
from urllib.parse import urlparse


def addr_to_netloc(addr):
    return urlparse(addr).netloc


class OpenTCPStreamMockWrapper:
    def __init__(self):
        self._socks = defaultdict(list)
        self._hooks = {}
        self._offlines = set()

    @contextmanager
    def install_hook(self, addr, hook):
        netloc = addr_to_netloc(addr)
        self.push_hook(netloc, hook)
        try:
            yield
        finally:
            self.pop_hook(netloc)

    def push_hook(self, addr, hook):
        netloc = addr_to_netloc(addr)
        assert netloc not in self._hooks
        self._hooks[netloc] = hook

    def pop_hook(self, addr):
        netloc = addr_to_netloc(addr)
        self._hooks.pop(netloc)

    def get_socks(self, addr):
        netloc = addr_to_netloc(addr)
        return self._socks[netloc]

    async def __call__(self, host, port, **kwargs):
        netloc = f"{host}:{port}" if port is not None else host
        hook = self._hooks.get(netloc)
        if hook and netloc not in self._offlines:
            if inspect.iscoroutinefunction(hook):
                sock = await hook(host, port, **kwargs)
            else:
                sock = hook(host, port, **kwargs)
        else:
            raise ConnectionRefusedError(111, "Connection refused")

        self._socks[netloc].append(sock)
        return sock

    def switch_offline(self, addr):
        netloc = addr_to_netloc(addr)
        if netloc in self._offlines:
            return

        for sock in self._socks[netloc]:

            async def _broken_stream(*args, **kwargs):
                raise trio.BrokenStreamError()

            _broken_stream.old_send_all_hook = sock.send_stream.send_all_hook
            _broken_stream.old_receive_some_hook = sock.receive_stream.receive_some_hook

            sock.send_stream.send_all_hook = _broken_stream
            sock.receive_stream.receive_some_hook = _broken_stream

        self._offlines.add(netloc)

    def switch_online(self, addr):
        netloc = addr_to_netloc(addr)
        if netloc not in self._offlines:
            return

        for sock in self._socks[netloc]:
            sock.send_stream.send_all_hook = sock.send_stream.send_all_hook.old_send_all_hook
            sock.receive_stream.receive_some_hook = (
                sock.receive_stream.receive_some_hook.old_receive_some_hook
            )
        self._offlines.remove(netloc)

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
