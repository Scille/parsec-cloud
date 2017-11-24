import trio
import json
from urllib.parse import urlparse
from nacl.signing import SigningKey

from parsec.utils import from_jsonb64, to_jsonb64, CookedSocket, ParsecError


# Keep things simple, so far we make the following assumptions:
# - backend is always started and ready before we try to connect to it
# - handshake is going to succeed (our user is present in the backend)
# - blockstore is handled by the backend
# - disconnection cannot occurs
# TODO: fix all those points !


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
        self._nursery = None

    async def block_get(self, id):
        # Only support block service embedded in backend so far
        rep = await self.send({'cmd': 'blockstore_post', 'id': id})
        # TODO: handle errors
        return from_jsonb64(rep['block'])

    async def block_save(self, block):
        # Only support block service embedded in backend so far
        rep = await self.send({'cmd': 'blockstore_post', 'block': to_jsonb64(block)})
        # TODO: handle errors
        return rep['id']

    async def send(self, req):
        await self.req_queue.put(req)
        rep = await self.rep_queue.get()
        if not rep:
            raise BackendNotAvailable('Try later...')
        return rep

    async def init(self, nursery):
        nursery.start_soon(self.start)

    async def start(self):
        async with trio.open_nursery() as self._nursery:
            self._nursery.start_soon(self._backend_connection)

    async def teardown(self):
        self._nursery.cancel_scope.cancel()

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
            k = SigningKey(self.user.signkey.encode())
            answer = k.sign(from_jsonb64(hds1['challenge']))
            hds2 = {'handshake': 'answer', 'identity': self.user.id, 'answer': to_jsonb64(answer)}
            await sock.send(hds2)
            hds3 = await sock.recv()
            assert hds3 == {'status': 'ok', 'handshake': 'done'}, hds3
            # TODO: should also register to even to listen here
            return rawsock, sock
        except (ConnectionRefusedError, Exception):
            # TODO: Fix this ugliness
            return None, None

    async def _backend_connection(self):
        # First start the connection
        rawsock, sock = await self._connect_to_backend()
        if not rawsock:
            # Tough shit... :'-(
            return
        with rawsock:
            while True:
                req = await self.req_queue.get()
                print('=======>', req)
                await sock.send(req)
                rep = await sock.recv()
                print('<=======', rep)
                await self.rep_queue.put(rep)
            # TODO: handle disconnection
            # req = await self.req_queue.get()
            # import pdb; pdb.set_trace()
            # if not sock:
            #     sock = await self._connect_to_backend()
            #     if not sock:
            #         await self.req_queue.put(None)
            #         continue
            # try:
            #     with trio.socket.socket() as sock:
            #         await sock.send(req)
            #         rep = await sock.recv()
            # except Exception:
            #     sock = await self._connect_to_backend()
            #     if not sock:
            #         await self.req_queue.put(None)
            #         continue
            #     try:
            #         with trio.socket.socket() as sock:
            #             await sock.send(req)
            #             rep = await sock.recv()
            #     except Exception:
            #         await self.req_queue.put(None)
            #         continue
            # await self.req_queue.put(rep)
