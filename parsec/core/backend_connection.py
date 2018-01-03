import trio
from urllib.parse import urlparse

from parsec.utils import CookedSocket
from parsec.handshake import ClientHandshake, AnonymousClientHandshake


class BackendError(Exception):
    pass


class BackendNotAvailable(BackendError):
    pass


class BackendConnection:
    def __init__(self, user, addr):
        self.handshake_id = user.id
        self.handshake_signkey = user.signkey
        self.addr = urlparse(addr)
        self._lock = trio.Lock()
        self._sock = None

    async def _init_connection(self):
        if self._sock:
            await self._sock.aclose()
        sockstream = await trio.open_tcp_stream(self.addr.hostname, self.addr.port)
        try:
            sock = CookedSocket(sockstream)
            if self.handshake_id == 'anonymous':
                ch = AnonymousClientHandshake()
            else:
                ch = ClientHandshake(self.handshake_id, self.handshake_signkey)
            challenge_req = await sock.recv()
            answer_req = ch.process_challenge_req(challenge_req)
            await sock.send(answer_req)
            result_req = await sock.recv()
            ch.process_result_req(result_req)
        except Exception as exc:
            await sockstream.aclose()
            raise exc

        self._sock = sock

    async def _naive_send(self, req):
        if not self._sock:
            raise BackendNotAvailable()
        await self._sock.send(req)
        return await self._sock.recv()

    async def send(self, req):
        async with self._lock:
            # Try to use the already connected socket
            try:
                return await self._naive_send(req)
            except (BackendNotAvailable, trio.BrokenStreamError, trio.ClosedStreamError):
                try:
                    # If it failed, reopen the socket and retry the request
                    await self._init_connection()
                    return await self._naive_send(req)
                except (OSError, trio.BrokenStreamError) as e:
                    # Failed again, it seems we are offline
                    raise BackendNotAvailable(str(e))

    async def init(self, nursery):
        # Try to open connection with the backend to save time for first
        # request
        try:
            async with self._lock:
                await self._init_connection()
        except (OSError, trio.BrokenStreamError):
            pass

    async def teardown(self):
        if self._sock:
            await self._sock.aclose()

    async def ping(self):
        await self.send({'cmd': 'ping', 'ping': ''})


class AnonymousBackendConnection(BackendConnection):
    def __init__(self, addr):
        self.handshake_id = 'anonymous'
        self.handshake_signkey = None
        self.addr = urlparse(addr)
        self._lock = trio.Lock()
        self._sock = None


async def backend_send_anonymous_cmd(addr, cmd):
    conn = AnonymousBackendConnection(addr)
    try:
        return await conn.send(cmd)
    finally:
        await conn.teardown()
