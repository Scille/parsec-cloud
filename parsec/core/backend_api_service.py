import json
import asyncio
import websockets
from blinker import signal

from parsec.backend import (
    MockedGroupService, InMemoryMessageService, MetaBlockService, MockedBlockService,
    MockedVlobService, MockedNamedVlobService, VlobNotFound, VlobBadVersionError
)
from parsec.backend.vlob_service import VlobError
from parsec.service import BaseService, event
from parsec.tools import logger


class BaseBackendAPIService(BaseService):

    name = 'BackendAPIService'


class BackendAPIService(BaseBackendAPIService):

    on_vlob_updated = signal('on_vlob_updated')
    on_named_vlob_updated = signal('on_named_vlob_updated')
    on_message_arrived = signal('on_message_arrived')

    def __init__(self, backend_url):
        super().__init__()
        self._resp_queue = asyncio.Queue()
        assert backend_url.startswith('ws://') or backend_url.startswith('wss://')
        self._backend_url = backend_url
        self._websocket = None
        self._ws_recv_handler_task = None

    async def bootstrap(self):
        assert not self._websocket, "Service already bootstraped"
        self._websocket = await websockets.connect(self._backend_url)
        self._ws_recv_handler_task = asyncio.ensure_future(self._ws_recv_handler())

    async def teardown(self):
        assert self._websocket, "Service hasn't been bootstraped"
        self._ws_recv_handler_task.cancel()
        try:
            await self._ws_recv_handler_task
        except asyncio.CancelledError:
            pass

    async def _ws_recv_handler(self):
        # Given command responses and notifications are all send through the
        # same websocket, separate them here, passing command response thanks
        # to a Queue.
        while True:
            raw = await self._websocket.recv()
            try:
                if isinstance(raw, bytes):
                    raw = raw.decode()
                recv = json.loads(raw)
                if 'status' in recv:
                    # Message response
                    self._resp_queue.put_nowait(recv)
                else:
                    # Event
                    signal(recv['event']).send(recv['sender'])
            except (KeyError, TypeError, json.JSONDecodeError):
                # Dummy ???
                logger.warning('Backend server send invalid message: %s' % recv)

    async def _send_cmd(self, msg):
        await self._websocket.send(json.dumps(msg))
        ret = await self._resp_queue.get()
        status = ret['status']
        if status == 'ok':
            return ret
        elif status == 'not_found':
            raise VlobNotFound(ret['label'])
        elif status == 'bad_version':
            raise VlobBadVersionError(ret['label'])
        else:
            raise VlobError(ret['label'])

    async def _listen_for_notification(self):
        raw = await self._websocket.recv()
        notif = json.loads(raw)

    async def block_create(self, content):
        msg = {'cmd': 'block_create', 'content': content}
        ret = await self._send_cmd(msg)
        return ret['id']

    async def block_read(self, id):
        msg = {'cmd': 'block_read', 'id': id}
        return await self._send_cmd(msg)

    async def block_stat(self, id):
        msg = {'cmd': 'block_stat', 'id': id}
        return await self._send_cmd(msg)

    async def group_create(self, name):
        msg = {'cmd': 'group_create', 'name': name}
        return await self._send_cmd(msg)

    async def group_read(self, name):
        msg = {'cmd': 'group_read', 'name': name}
        return await self._send_cmd(msg)

    async def group_add_identities(self, name, identities, admin=False):
        msg = {'cmd': 'group_add_identities', 'name': name, 'identities': identities, 'admin': admin}
        return await self._send_cmd(msg)

    async def group_remove_identities(self, name, identities, admin=False):
        msg = {'cmd': 'group_remove_identities', 'name': name, 'identities': identities, 'admin': admin}
        return await self._send_cmd(msg)

    async def message_new(self, recipient, body):
        msg = {'cmd': 'message_new', 'recipient': recipient, 'body': body}
        return await self._send_cmd(msg)

    async def message_get(self, recipient, offset=0):
        msg = {'cmd': 'message_get', 'recipient': recipient, 'offset': offset}
        return await self._send_cmd(msg)

    async def named_vlob_create(self, id, blob=''):
        assert isinstance(blob, str)
        msg = {'cmd': 'named_vlob_create', 'id': id, 'blob': blob}
        return await self._send_cmd(msg)

    async def named_vlob_read(self, id, trust_seed):
        assert isinstance(id, str)
        msg = {'cmd': 'named_vlob_read', 'id': id, 'trust_seed': trust_seed}
        return await self._send_cmd(msg)

    async def named_vlob_update(self, id, version, trust_seed, blob=''):
        msg = {
            'cmd': 'named_vlob_update',
            'id': id,
            'version': version,
            'trust_seed': trust_seed,
            'blob': blob
        }
        return await self._send_cmd(msg)

    async def vlob_create(self, blob=''):
        assert isinstance(blob, str)
        msg = {'cmd': 'vlob_create', 'blob': blob}
        return await self._send_cmd(msg)

    async def vlob_read(self, id, trust_seed):
        assert isinstance(id, str)
        msg = {'cmd': 'vlob_read', 'id': id, 'trust_seed': trust_seed}
        return await self._send_cmd(msg)

    async def vlob_update(self, id, version, trust_seed, blob=''):
        msg = {
            'cmd': 'vlob_update',
            'id': id,
            'version': version,
            'trust_seed': trust_seed,
            'blob': blob
        }
        return await self._send_cmd(msg)


class MockedBackendAPIService(BaseBackendAPIService):

    on_msg_arrived = event('arrived')

    def __init__(self):
        super().__init__()
        self._group_service = MockedGroupService()
        self._message_service = InMemoryMessageService()
        self._named_vlob_service = MockedNamedVlobService()
        self._vlob_service = MockedVlobService()
        self._block_service = MetaBlockService(backends=[MockedBlockService, MockedBlockService])
        # Events
        self.on_vlob_updated = self._vlob_service.on_updated
        self.on_named_vlob_updated = self._named_vlob_service.on_updated
        self.on_message_arrived = self._message_service.on_arrived

    async def block_create(self, content):
        msg = {'cmd': 'block_create', 'content': content}
        ret = await self._block_service._cmd_CREATE(None, msg)
        return ret['id']

    async def block_read(self, id):
        msg = {'cmd': 'block_read', 'id': id}
        return await self._block_service._cmd_READ(None, msg)

    async def block_stat(self, id):
        msg = {'cmd': 'block_stat', 'id': id}
        return await self._block_service._cmd_STAT(None, msg)

    async def group_create(self, name):
        msg = {'cmd': 'group_create', 'name': name}
        return await self._group_service._cmd_CREATE(None, msg)

    async def group_read(self, name):
        msg = {'cmd': 'group_read', 'name': name}
        return await self._group_service._cmd_READ(None, msg)

    async def group_add_identities(self, name, identities, admin=False):
        msg = {'cmd': 'group_add_identities', 'name': name, 'identities': identities, 'admin': admin}
        return await self._group_service._cmd_ADD_IDENTITIES(None, msg)

    async def group_remove_identities(self, name, identities, admin=False):
        msg = {'cmd': 'group_remove_identities', 'name': name, 'identities': identities, 'admin': admin}
        return await self._group_service._cmd_REMOVE_IDENTITIES(None, msg)

    async def message_new(self, recipient, body):
        msg = {'cmd': 'message_new', 'recipient': recipient, 'body': body}
        return await self._message_service._cmd_NEW(None, msg)

    async def message_get(self, recipient, offset=0):
        msg = {'cmd': 'message_get', 'recipient': recipient, 'offset': offset}
        return await self._message_service._cmd_GET(None, msg)

    async def named_vlob_create(self, id, blob=''):
        assert isinstance(blob, str)
        msg = {'cmd': 'named_vlob_create', 'id': id, 'blob': blob}
        return await self._named_vlob_service._cmd_CREATE(None, msg)

    async def named_vlob_read(self, id, trust_seed):
        msg = {'cmd': 'named_vlob_read', 'id': id, 'trust_seed': trust_seed}
        return await self._named_vlob_service._cmd_READ(None, msg)

    async def named_vlob_update(self, id, version, trust_seed, blob=''):
        msg = {
            'cmd': 'named_vlob_update',
            'id': id,
            'version': version,
            'trust_seed': trust_seed,
            'blob': blob
        }
        await self._named_vlob_service._cmd_UPDATE(None, msg)

    async def vlob_create(self, blob=''):
        assert isinstance(blob, str)
        msg = {'cmd': 'vlob_create', 'blob': blob}
        return await self._vlob_service._cmd_CREATE(None, msg)

    async def vlob_read(self, id, trust_seed):
        msg = {'cmd': 'vlob_read', 'id': id, 'trust_seed': trust_seed}
        return await self._vlob_service._cmd_READ(None, msg)

    async def vlob_update(self, id, version, trust_seed, blob=''):
        msg = {
            'cmd': 'vlob_update',
            'id': id,
            'version': version,
            'trust_seed': trust_seed,
            'blob': blob
        }
        await self._vlob_service._cmd_UPDATE(None, msg)
