import attr
import json
import asyncio
import blinker
import websockets
from effect2 import TypeDispatcher, do, Effect, AsyncFunc

from parsec.core.identity import EIdentityGet
from parsec.base import ERegisterEvent
from parsec.exceptions import BackendConnectionError, exception_from_status
from parsec.tools import logger, ejson_loads, ejson_dumps, to_jsonb64
from parsec.core.backend_start_api import StartAPIComponent, backend_to_start_api_url


# TODO rename to `EBackendCmd`
@attr.s
class BackendCmd:
    cmd = attr.ib()
    msg = attr.ib(default=attr.Factory(dict))


@attr.s
class EBackendBlockStoreGetURL:
    pass


@attr.s
class EBackendReset:
    pass


@attr.s
class EBackendStatus:
    pass


class BackendConnection:

    def __init__(self, url, watchdog=None, loop=None):
        self._resp_queue = asyncio.Queue(loop=loop)
        self.loop = loop
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
        try:
            self._websocket = await websockets.connect(self.url)
            # Handle handshake
            raw = await self._websocket.recv()
            challenge = ejson_loads(raw)
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
            self._ws_recv_handler_task = asyncio.ensure_future(
                self._ws_recv_handler(), loop=self.loop)
            if self.watchdog_time:
                self._watchdog_task = asyncio.ensure_future(self._watchdog(), loop=self.loop)
        except (ConnectionRefusedError, websockets.exceptions.ConnectionClosed) as exc:
            raise BackendConnectionError('Cannot connect to backend (%s)' % exc)

    async def ping(self):
        assert self._websocket, "Connection to backend not opened"
        try:
            await self._websocket.ping()
        except websockets.exceptions.ConnectionClosed as exc:
            raise BackendConnectionError('Cannot connect to backend (%s)' % exc)

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
        try:
            await self._websocket.send(ejson_dumps(msg))
        except websockets.exceptions.ConnectionClosed as exc:
            raise BackendConnectionError('Cannot connect to backend (%s)' % exc)
        ret = await self._resp_queue.get()
        status = ret['status']
        if status == 'ok':
            return ret
        else:
            raise exception_from_status(status)(ret['label'])


class BackendComponent:

    def __init__(self, url, watchdog=None):
        assert url.startswith('ws://') or url.startswith('wss://')
        self.url = url
        self._start_api_component = StartAPIComponent(backend_to_start_api_url(url))
        self.watchdog = watchdog
        self.connection = None

    async def shutdown(self, app=None):
        await self.perform_backend_reset()

    def performer_with_connection_factory(self, async_performer):
        @do
        def performer_with_connection(intent):
            try:
                if self.connection:
                    # Reuse already opened connection
                    try:
                        return (yield AsyncFunc(async_performer(intent)))
                    except BackendConnectionError:
                        # The connection has been closed, try to reconnect
                        # and rerun the performer
                        yield AsyncFunc(self.connection.close_connection())
                        self.connection = None
                        return (yield performer_with_connection(intent))
                else:
                    # Open new connection and run command
                    identity = yield Effect(EIdentityGet())
                    connection = BackendConnection(self.url, self.watchdog)
                    yield AsyncFunc(connection.open_connection(identity))
                    self.connection = connection
                    return (yield AsyncFunc(async_performer(intent)))
            except BackendConnectionError:
                if self.connection:
                    yield AsyncFunc(self.connection.close_connection())
                    self.connection = None
                raise

        return performer_with_connection

    @do
    def perform_blockstore_get_url(self, intent):
        ret = yield Effect(BackendCmd('blockstore_get_url'))
        url = ret['url']
        if url.startswith('/'):
            if self.url.startswith('ws://'):
                return self.url.replace('ws://', 'http://', 1) + url
            else:  # wss://
                return self.url.replace('wss://', 'https://', 1) + url
        else:
            return url

    async def perform_backend_status(self, intent):
        await self.connection.ping()

    async def perform_backend_cmd(self, intent):
        payload = {'cmd': intent.cmd, **intent.msg}
        return await self.connection.send_cmd(payload)

    async def perform_backend_reset(self, intent=None):
        if self.connection:
            await self.connection.close_connection()
            self.connection = None

    def get_dispatcher(self):
        from parsec.core import (backend_group, backend_message, backend_user_vlob,
                                 backend_vlob, backend_start_api)
        return TypeDispatcher({
            BackendCmd: self.performer_with_connection_factory(self.perform_backend_cmd),

            EBackendReset: self.perform_backend_reset,
            EBackendStatus: self.performer_with_connection_factory(self.perform_backend_status),

            EBackendBlockStoreGetURL: self.perform_blockstore_get_url,
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
            backend_group.EBackendGroupRemoveIdentities: backend_group.perform_group_remove_identities,
            backend_start_api.EBackendCipherKeyAdd: self._start_api_component.perform_cipherkey_add,
            backend_start_api.EBackendCipherKeyGet: self._start_api_component.perform_cipherkey_get,
            backend_start_api.EBackendIdentityRegister: self._start_api_component.perform_identity_register
        })
