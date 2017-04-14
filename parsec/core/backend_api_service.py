import json
import asyncio
import websockets
from blinker import signal

from parsec.backend import (MockedGroupService, InMemoryMessageService, MockedVlobService,
                            MockedNamedVlobService, MockedBlockService)
from parsec.service import BaseService, event


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

    async def start(self):
        self._websocket = await websockets.connect(self._backend_url)
        self._ws_recv_handler_task = asyncio.call_soon(self._ws_recv_handler)

    async def stop(self):
        self._ws_recv_handler_task.cancel()

    async def _ws_recv_handler(self):
        # Given command responses and notifications are all send through the
        # same websocket, separate them here, passing command response thanks
        # to a Queue.
        while True:
            raw = await self._websocket.recv()
            try:
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

    async def _send_cmd(self):
        # To avoid concurrency on message reception, we have to cancel
        self._notification_listener_task.cancel()
        await self._websocket.send(msg)
        resp = await self._resp_queue.get()

    async def _listen_for_notification(self):
        raw = await self._websocket.recv()
        notif = json.loads(raw)

    async def block_create(self, *args, **kwargs):
        await self._init_connection()
        return await self._block_service.create(*args, **kwargs)

    async def block_read(self, *args, **kwargs):
        await self._init_connection()
        return await self._block_service.read(*args, **kwargs)

    async def block_stat(self, *args, **kwargs):
        await self._init_connection()
        return await self._block_service.stat(*args, **kwargs)

    async def message_new(self, *args, **kwargs):
        await self._init_connection()
        return await self._message_service.new(*args, **kwargs)

    async def message_get(self, *args, **kwargs):
        await self._init_connection()
        return await self._message_service.get(*args, **kwargs)

    async def named_vlob_create(self, *args, **kwargs):
        await self._init_connection()
        return await self._named_vlob_service.create(*args, **kwargs)

    async def named_vlob_read(self, *args, **kwargs):
        await self._init_connection()
        return await self._named_vlob_service.read(*args, **kwargs)

    async def named_vlob_update(self, *args, **kwargs):
        await self._init_connection()
        return await self._named_vlob_service.update(*args, **kwargs)

    async def vlob_create(self, *args, **kwargs):
        await self._init_connection()
        return await self._vlob_service.create(*args, **kwargs)

    async def vlob_read(self, *args, **kwargs):
        await self._init_connection()
        return await self._vlob_service.read(*args, **kwargs)

    async def vlob_update(self, *args, **kwargs):
        await self._init_connection()
        return await self._vlob_service.update(*args, **kwargs)


class MockedBackendAPIService(BaseBackendAPIService):

    on_msg_arrived = event('arrived')

    def __init__(self):
        super().__init__()
        self._group_service = MockedGroupService()
        self._message_service = InMemoryMessageService()
        self._named_vlob_service = MockedNamedVlobService()
        self._vlob_service = MockedVlobService()
        self._block_service = MockedBlockService()
        # Events
        self.on_vlob_updated = self._vlob_service.on_updated
        self.on_named_vlob_updated = self._named_vlob_service.on_updated
        self.on_message_arrived = self._message_service.on_arrived

    async def block_create(self, *args, **kwargs):
        return await self._block_service.create(*args, **kwargs)

    async def block_read(self, *args, **kwargs):
        return await self._block_service.read(*args, **kwargs)

    async def block_stat(self, *args, **kwargs):
        return await self._block_service.stat(*args, **kwargs)

    async def group_create(self, *args, **kwargs):
        return await self._group_service.create(*args, **kwargs)

    async def group_read(self, *args, **kwargs):
        return await self._group_service.read(*args, **kwargs)

    async def group_add_identities(self, *args, **kwargs):
        return await self._group_service.add_identities(*args, **kwargs)

    async def group_remove_identities(self, *args, **kwargs):
        return await self._group_service.remove_identities(*args, **kwargs)

    async def message_new(self, *args, **kwargs):
        return await self._message_service.new(*args, **kwargs)

    async def message_get(self, *args, **kwargs):
        return await self._message_service.get(*args, **kwargs)

    async def named_vlob_create(self, *args, **kwargs):
        return await self._named_vlob_service.create(*args, **kwargs)

    async def named_vlob_read(self, *args, **kwargs):
        return await self._named_vlob_service.read(*args, **kwargs)

    async def named_vlob_update(self, *args, **kwargs):
        return await self._named_vlob_service.update(*args, **kwargs)

    async def vlob_create(self, *args, **kwargs):
        return await self._vlob_service.create(*args, **kwargs)

    async def vlob_read(self, *args, **kwargs):
        return await self._vlob_service.read(*args, **kwargs)

    async def vlob_update(self, *args, **kwargs):
        return await self._vlob_service.update(*args, **kwargs)
