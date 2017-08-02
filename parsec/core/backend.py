import attr
import json
import asyncio
import blinker
import websockets
from effect2 import TypeDispatcher, do, Effect

from parsec.exceptions import BackendConnectionError, exception_from_status
from parsec.tools import logger, ejson_loads, ejson_dumps, to_jsonb64


@attr.s
class BackendCmd:
    cmd = attr.ib()
    msg = attr.ib(default=attr.Factory(dict))


@attr.s
class EBackendCloseConnection:
    pass


@attr.s
class EBackendStatus:
    pass


class BackendConnection:

    def __init__(self, url, watchdog=None):
        self._resp_queue = asyncio.Queue()
        assert url.startswith('ws://') or url.startswith('wss://')
        self.url = url
        self._websocket = None
        self._ws_recv_handler_task = None
        self._watchdog_task = None
        self.watchdog_time = watchdog
        self._signal_ns = blinker.Namespace()

    async def connect_event(self, event, sender, cb):
        msg = {'cmd': 'subscribe', 'event': event, 'sender': sender}
        await self.send_cmd(msg)
        self._signal_ns.signal(event).connect(cb, sender=sender)

    async def open_connection(self, identity):
        logger.debug('Connection to backend opened')
        assert not self._websocket, "Connection to backend already opened"
        self._websocket = await websockets.connect(self.url)
        # Handle handshake
        challenge = ejson_loads(await self._websocket.recv())
        answer = identity.private_key.sign(challenge['challenge'].encode())
        await self._websocket.send(ejson_dumps({
            'handshake': 'answer',
            'identity': identity.id,
            'answer': to_jsonb64(answer)
        }))
        resp = ejson_loads(await self._websocket.recv())
        if resp['status'] != 'ok':
            await self.close_connection()
            raise exception_from_status(resp['status'])(resp['label'])
        self._ws_recv_handler_task = asyncio.ensure_future(self._ws_recv_handler())
        if self.watchdog_time:
            self._watchdog_task = asyncio.ensure_future(self._watchdog())

    async def ping(self):
        assert self._websocket, "Connection to backend not opened"
        await self._websocket.ping()

    async def close_connection(self):
        assert self._websocket, "Connection to backend not opened"
        for task in (self._ws_recv_handler_task, self._watchdog_task):
            if not task:
                continue
            task.cancel()
            try:
                await task
            except (asyncio.CancelledError, websockets.exceptions.ConnectionClosed):
                pass
        try:
            await self._websocket.close()
        except websockets.exceptions.ConnectionClosed:
            pass
        self._websocket = None
        self._ws_recv_handler_task = None
        self._watchdog_task = None
        logger.debug('Connection to backend closed')

    async def _watchdog(self):
        while True:
            await asyncio.sleep(self.watchdog_time)
            logger.debug('Watchdog ping to backend.')
            await self._websocket.ping()

    async def _ws_recv_handler(self):
        # Given command responses and notifications are all send through the
        # same websocket, separate them here, passing command response thanks
        # to a Queue.
        while True:
            raw = await self._websocket.recv()
            try:
                if isinstance(raw, bytes):
                    raw = raw.decode()
                recv = ejson_loads(raw)
                if 'status' in recv:
                    # Message response
                    self._resp_queue.put_nowait(recv)
                else:
                    # Event
                    self._signal_ns.signal(recv['event']).send(recv['sender'])
            except (KeyError, TypeError, json.JSONDecodeError):
                # Dummy ???
                logger.warning('Backend server sent invalid message: %s' % raw)

    async def send_cmd(self, msg):
        if not self._websocket:
            raise BackendConnectionError('BackendAPIService cannot send command in current state')
        await self._websocket.send(ejson_dumps(msg))
        ret = await self._resp_queue.get()
        status = ret['status']
        if status == 'ok':
            return ret
        else:
            raise exception_from_status(status)(ret['label'])


class BackendComponent:

    def __init__(self, url, watchdog=None):
        self.app = None
        self.url = url
        self.watchdog = watchdog
        self.connection = None
        blinker.signal('app_start').connect(self.on_app_start)
        blinker.signal('app_stop').connect(self.on_app_stop)
        blinker.signal('identity_loaded').connect(self.on_identity_loaded)
        blinker.signal('identity_unloaded').connect(self.on_identity_unloaded)

    @do
    def on_app_start(self, app):
        self.app = app

    @do
    def on_app_stop(self, app):
        yield Effect(EBackendCloseConnection())

    def on_identity_loaded(self, identity):
        self.identity = identity

    def on_identity_unloaded(self, _):
        self.identity = None

    async def open_connection(self):
        if not self.connection:
            if not self.identity:
                raise BackendConnectionError('Identity must be loaded to connect to backend')
            connection = BackendConnection(self.url, self.watchdog)
            try:
                await connection.open_connection(self.identity)
            except ConnectionRefusedError as exc:
                raise BackendConnectionError('Cannot connect to backend (%s)' % exc)
            self.connection = connection

    async def close_connection(self):
        if self.connection:
            await self.connection.close_connection()
            self.connection = None

    async def perform_backend_close_connection(self, intent):
        await self.close_connection()

    async def perform_backend_status(self, intent):
        await self.open_connection()
        try:
            await self.connection.ping()
        except websockets.exceptions.ConnectionClosed as exc:
            await self.close_connection()
            raise BackendConnectionError('Cannot connect to the backend (%s)' % exc)

    async def perform_backend_cmd(self, intent):
        await self.open_connection()
        payload = {'cmd': intent.cmd, **intent.msg}
        try:
            return await self.connection.send_cmd(payload)
        except BackendConnectionError:
            await self.close_connection()
            raise

    def get_dispatcher(self):
        from parsec.core import backend_group, backend_message, backend_user_vlob, backend_vlob
        return TypeDispatcher({
            BackendCmd: self.perform_backend_cmd,
            EBackendStatus: self.perform_backend_status,
            EBackendCloseConnection: self.perform_backend_close_connection,
            backend_vlob.EBackendVlobCreate: backend_vlob.perform_vlob_create,
            backend_vlob.EBackendVlobUpdate: backend_vlob.perform_vlob_update,
            backend_vlob.EBackendVlobRead: backend_vlob.perform_vlob_read,
            backend_user_vlob.EBackendUserVlobUpdate: backend_user_vlob.perform_user_vlob_update,
            backend_user_vlob.EBackendUserVlobRead: backend_user_vlob.perform_user_vlob_read,
            backend_message.EBackendMessageGet: backend_message.perform_message_get,
            backend_message.EBackendMessageNew: backend_message.perform_message_new,
            backend_group.EBackendGroupRead: backend_group.perform_group_read,
            backend_group.EBackendGroupCreate: backend_group.perform_group_create,
            backend_group.EBackendGroupAddIdentities: backend_group.perform_group_add_identities,
            backend_group.EBackendGroupRemoveIdentities: backend_group.perform_group_remove_identities
        })
