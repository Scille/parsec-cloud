import trio
from urllib.parse import urlparse
from nacl.signing import SigningKey

from parsec.utils import from_jsonb64, to_jsonb64, CookedSocket, ParsecError


class BackendNotAvailable(ParsecError):
    label = 'backend_not_available'


class BackendConnection:
    def __init__(self, user, addr):
        self.user = user
        self.addr = urlparse(addr)
        self.is_connected = False
        self.socket = None
        self.req_queue = trio.Queue(1)
        self.rep_queue = trio.Queue(1)
        self.___block_foo = {}  # TODO: ...

    async def block_get(self, id):
        # TODO
        return self.___block_foo[id]

    async def block_save(self, buffer):
        # TODO
        from uuid import uuid4
        id = uuid4().hex
        self.___block_foo[id] = buffer
        return id

    async def send(self, req):
        await self.req_queue.put(req)
        rep = await self.rep_queue.get()
        if not rep:
            raise BackendNotAvailable('Try later...')
        return rep

    async def init(self, nursery):
        self.nursery = nursery
        nursery.start_soon(self._backend_connection)

    async def teardown(self):
        pass

    async def ping(self):
        await self.send({'cmd': 'ping', 'ping': ''})

    async def _connect_to_backend(self):
        """
        Connect to the backend and assume handshake
        """
        try:
            rawsock = trio.socket.socket()
            await rawsock.connect((self.addr.hostname, self.addr.port))
            sock = CookedSocket(rawsock)
            # Handshake
            hds1 = await sock.recv()
            assert hds1['handshake'] == 'challenge', hds1
            k = SigningKey(self.user.privkey.encode())
            answer = k.sign(from_jsonb64(hds1['challenge']))
            hds2 = {'handshake': 'answer', 'identity': self.user.id, 'answer': to_jsonb64(answer)}
            await sock.send(hds2)
            hds3 = await sock.recv()
            assert hds3 == {'status': 'ok', 'handshake': 'done'}, hds3
            # TODO: should also register to even to listen here
            return sock
        except (ConnectionRefusedError, Exception):
            # TODO: Fix this ugliness
            return None

    async def _backend_connection(self):
        # First start the connection
        sock = await self._connect_to_backend()
        while True:
            # TODO: handle disconnection
            req = await self.req_queue.get()
            if not sock:
                sock = await self._connect_to_backend()
                if not sock:
                    await self.req_queue.put(None)
                    continue
            try:
                with trio.socket.socket() as sock:
                    await sock.send(req)
                    rep = await sock.recv()
            except Exception:
                sock = await self._connect_to_backend()
                if not sock:
                    await self.req_queue.put(None)
                    continue
                try:
                    with trio.socket.socket() as sock:
                        await sock.send(req)
                        rep = await sock.recv()
                except Exception:
                    await self.req_queue.put(None)
                    continue
            await self.req_queue.put(rep)
