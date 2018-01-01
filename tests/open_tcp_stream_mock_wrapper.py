from collections import defaultdict
from contextlib import contextmanager
from unittest.mock import patch
import trio


async def _broken_stream(*args, **kwargs):
    raise trio.BrokenStreamError()


class OpenTCPStreamMockWrapper:
    def __init__(self):
        self.socks = defaultdict(list)
        self._hooks = {}
        self._offlines = set()

    def install_hook(self, addr, hook):
        if not hook:
            self._hooks.pop(addr, None)
        else:
            self._hooks[addr] = hook

    async def __call__(self, host, port, **kwargs):
        addr = 'tcp://%s:%s' % (host, port)
        hook = self._hooks.get(addr)
        if hook:
            if addr in self._offlines:
                raise ConnectionRefusedError('[Errno 111] Connection refused')
            sock = await hook(host, port, **kwargs)
        else:
            sock = self._original_open_tcp_stream(host, port, **kwargs)
        self.socks[addr].append(sock)
        return sock

    def switch_offline(self, addr):
        if addr in self._offlines:
            return
        for sock in self.socks[addr]:
            sock.send_stream.send_all_hook = _broken_stream
            sock.receive_stream.receive_some_hook = _broken_stream
        self._offlines.add(addr)

    def switch_online(self, addr):
        if addr not in self._offlines:
            return
        for sock in self.socks[addr]:
            sock.send_stream.send_all_hook = None
            sock.receive_stream.receive_some_hook = None
        self._offlines.remove(addr)


@contextmanager
def wrap_open_tcp_stream():
    open_tcp_stream_mock_wrapper = OpenTCPStreamMockWrapper()
    with patch('trio.open_tcp_stream', new=open_tcp_stream_mock_wrapper):
        yield


@contextmanager
def offline(addr):
    trio.open_tcp_stream.switch_offline(addr)
    try:
        yield
    finally:
        trio.open_tcp_stream.switch_online(addr)
