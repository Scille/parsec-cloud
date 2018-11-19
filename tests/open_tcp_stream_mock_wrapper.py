from collections import defaultdict
from contextlib import contextmanager
from unittest.mock import patch
import inspect
import trio


class OpenTCPStreamMockWrapper:
    def __init__(self):
        self.socks = defaultdict(list)
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
        assert addr not in self._hooks
        self._hooks[addr] = hook

    def pop_hook(self, addr):
        self._hooks.pop(addr)

    async def __call__(self, host, port, **kwargs):
        addr = "ws://%s:%s" % (host, port)
        hook = self._hooks.get(addr)
        if hook and addr not in self._offlines:
            if inspect.iscoroutinefunction(hook):
                sock = await hook(host, port, **kwargs)
            else:
                sock = hook(host, port, **kwargs)
        else:
            raise ConnectionRefusedError(111, "Connection refused")

        self.socks[addr].append(sock)
        return sock

    def switch_offline(self, addr):
        if addr in self._offlines:
            return

        for sock in self.socks[addr]:

            async def _broken_stream(*args, **kwargs):
                raise trio.BrokenStreamError()

            _broken_stream.old_send_all_hook = sock.send_stream.send_all_hook
            _broken_stream.old_receive_some_hook = sock.receive_stream.receive_some_hook

            sock.send_stream.send_all_hook = _broken_stream
            sock.receive_stream.receive_some_hook = _broken_stream

        self._offlines.add(addr)

    def switch_online(self, addr):
        if addr not in self._offlines:
            return

        for sock in self.socks[addr]:
            sock.send_stream.send_all_hook = sock.send_stream.send_all_hook.old_send_all_hook
            sock.receive_stream.receive_some_hook = (
                sock.receive_stream.receive_some_hook.old_receive_some_hook
            )
        self._offlines.remove(addr)

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
