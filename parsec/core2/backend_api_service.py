import json
import asyncio
import websockets
import blinker
from typing import List

from parsec.service import BaseService, service
from parsec.backend import (
    MockedGroupService, InMemoryMessageService, MockedVlobService, MockedUserVlobService
)
from parsec.tools import logger, to_jsonb64, ejson_dumps, ejson_loads, async_callback
from parsec.exceptions import exception_from_status, IdentityNotLoadedError
from parsec.session import AuthSession


def _patch_service_event_namespace(service, ns):
    """
    By default, service's events uses blinker's global namespace.
    This function patch the given service to use another events namespace.
    """
    for k in dir(service):
        v = getattr(service, k)
        if isinstance(v, blinker.Signal):
            setattr(service, k, ns.signal(v.name))
    service._events = {k: ns.signal(v.name) for k, v in service._events.items()}


class BaseBackendAPIService(BaseService):

    name = 'BackendAPIService'

    async def wait_for_ready(self):
        raise NotImplementedError()

    async def connect_event(self, event: str, sender: str, cb):
        raise NotImplementedError()

    async def group_create(self, name: str):
        raise NotImplementedError()

    async def group_read(self, name: str):
        raise NotImplementedError()

    async def group_add_identities(self, name: str, identities: List[str], admin: bool=False):
        raise NotImplementedError()

    async def group_remove_identities(self, name: str, identities: List[str], admin: bool=False):
        raise NotImplementedError()

    async def message_new(self, recipient: str, body: bytes):
        raise NotImplementedError()

    async def message_get(self, recipient: str, offset: int=0):
        raise NotImplementedError()

    async def user_vlob_read(self, version: int=None):
        raise NotImplementedError()

    async def user_vlob_update(self, version: int, blob: bytes=b''):
        raise NotImplementedError()

    async def vlob_create(self, blob: bytes=b''):
        raise NotImplementedError()

    async def vlob_read(self, id: str, trust_seed: str, version: int=None):
        raise NotImplementedError()

    async def vlob_update(self, id: str, version: int, trust_seed: str, blob: bytes=b''):
        raise NotImplementedError()


class BackendAPIService(BaseBackendAPIService):

    identity = service('IdentityService')

    def __init__(self, backend_url, watchdog=None):
        super().__init__()
        self._resp_queue = asyncio.Queue()
        assert backend_url.startswith('ws://') or backend_url.startswith('wss://')
        self._backend_url = backend_url
        self._websocket = None
        self._ws_recv_handler_task = None
        self._watchdog_task = None
        self._watchdog_time = watchdog
        self._connection_ready_future = asyncio.futures.Future()

    async def wait_for_ready(self):
        await self._connection_ready_future

    async def connect_event(self, event, sender, cb):
        assert event in ('on_vlob_updated', 'on_user_vlob_updated', 'on_message_arrived')
        msg = {'cmd': 'subscribe', 'event': event, 'sender': sender}
        await self._send_cmd(msg)
        blinker.signal(event).connect(cb, sender=sender)

    async def _open_connection(self):
        logger.debug('Connection to backend oppened')
        assert not self._websocket, "Connection to backend already opened"
        self._websocket = await websockets.connect(self._backend_url)
        # Handle handshake
        challenge = ejson_loads(await self._websocket.recv())
        answer = self.identity.private_key.sign(challenge['challenge'].encode())
        await self._websocket.send(ejson_dumps({
            'handshake': 'answer',
            'identity': self.identity.id,
            'answer': to_jsonb64(answer)
        }))
        resp = ejson_loads(await self._websocket.recv())
        if resp['status'] != 'ok':
            await self._close_connection()
            raise exception_from_status(resp['status'])(resp['label'])
        self._ws_recv_handler_task = asyncio.ensure_future(self._ws_recv_handler())
        if self._watchdog_time:
            self._watchdog_task = asyncio.ensure_future(self._watchdog())
        self._connection_ready_future.set_result(None)

    async def _close_connection(self):
        self._connection_ready_future = asyncio.futures.Future()
        assert self._websocket, "Connection to backend not opened"
        for task in (self._ws_recv_handler_task, self._watchdog_task):
            if not task:
                continue
            task.cancel()
            try:
                await task
            except asyncio.CancelledError:
                pass
        await self._websocket.close()
        self._websocket = None
        self._ws_recv_handler_task = None
        self._watchdog_task = None
        logger.debug('Connection to backend closed')

    async def bootstrap(self):
        await super().bootstrap()
        # Must store the callback in the object given the are weak referenced
        # when registered as signal callback
        self._on_identity_loaded_cb = async_callback(lambda x: self._open_connection())
        self._on_identity_unloaded_cb = async_callback(lambda x: self._close_connection())
        self.identity.on_identity_loaded.connect(self._on_identity_loaded_cb)
        self.identity.on_identity_unloaded.connect(self._on_identity_unloaded_cb)

    async def teardown(self):
        self.identity.on_identity_loaded.disconnect(self._on_identity_loaded_cb)
        self.identity.on_identity_unloaded.disconnect(self._on_identity_unloaded_cb)
        if self._websocket:
            await self._close_connection()

    async def _watchdog(self):
        while True:
            await asyncio.sleep(self._watchdog_time)
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
                    blinker.signal(recv['event']).send(recv['sender'])
            except (KeyError, TypeError, json.JSONDecodeError):
                # Dummy ???
                logger.warning('Backend server sent invalid message: %s' % recv)

    async def _send_cmd(self, msg):
        if not self._websocket:
            raise IdentityNotLoadedError('BackendAPIService cannot send command in current state')
        await self._websocket.send(ejson_dumps(msg))
        ret = await self._resp_queue.get()
        status = ret['status']
        if status == 'ok':
            return ret
        else:
            raise exception_from_status(status)(ret['label'])

    async def group_create(self, name):
        msg = {'cmd': 'group_create', 'name': name}
        return await self._send_cmd(msg)

    async def group_read(self, name):
        msg = {'cmd': 'group_read', 'name': name}
        return await self._send_cmd(msg)

    async def group_add_identities(self, name, identities, admin=False):
        msg = {'cmd': 'group_add_identities',
               'name': name,
               'identities': identities,
               'admin': admin}
        return await self._send_cmd(msg)

    async def group_remove_identities(self, name, identities, admin=False):
        msg = {'cmd': 'group_remove_identities',
               'name': name,
               'identities': identities,
               'admin': admin}
        return await self._send_cmd(msg)

    async def message_new(self, recipient, body):
        msg = {'cmd': 'message_new', 'recipient': recipient, 'body': to_jsonb64(body)}
        return await self._send_cmd(msg)

    async def message_get(self, recipient, offset=0):
        msg = {'cmd': 'message_get', 'recipient': recipient, 'offset': offset}
        ret = await self._send_cmd(msg)
        return [msg['body'] for msg in ret['messages']]

    async def user_vlob_read(self, version=None):
        msg = {'cmd': 'user_vlob_read'}
        if version:
            msg['version'] = version
        return await self._send_cmd(msg)

    async def user_vlob_update(self, version, blob=b''):
        msg = {'cmd': 'user_vlob_update', 'version': version, 'blob': to_jsonb64(blob)}
        return await self._send_cmd(msg)

    async def vlob_create(self, blob=b''):
        msg = {'cmd': 'vlob_create', 'blob': to_jsonb64(blob)}
        return await self._send_cmd(msg)

    async def vlob_read(self, id, trust_seed, version=None):
        assert isinstance(id, str)
        msg = {'cmd': 'vlob_read', 'id': id, 'trust_seed': trust_seed}
        response = None
        if version:
            msg['version'] = version
        if not response:
            response = await self._send_cmd(msg)
        return response

    async def vlob_update(self, id, version, trust_seed, blob=b''):
        msg = {
            'cmd': 'vlob_update',
            'id': id,
            'version': version,
            'trust_seed': trust_seed,
            'blob': to_jsonb64(blob)
        }
        return await self._send_cmd(msg)


class MockedBackendAPIService(BaseBackendAPIService):

    identity = service('IdentityService')

    def __init__(self):
        super().__init__()
        self._group_service = MockedGroupService()
        self._message_service = InMemoryMessageService()
        self._user_vlob_service = MockedUserVlobService()
        self._vlob_service = MockedVlobService()
        # Backend services should not share the same event namespace than
        # core ones
        self._backend_event_ns = blinker.Namespace()
        _patch_service_event_namespace(self._group_service, self._backend_event_ns)
        _patch_service_event_namespace(self._message_service, self._backend_event_ns)
        _patch_service_event_namespace(self._user_vlob_service, self._backend_event_ns)
        _patch_service_event_namespace(self._vlob_service, self._backend_event_ns)

    @property
    def _session(self):
        return AuthSession(None, self.identity.id)

    async def wait_for_ready(self):
        pass

    async def connect_event(self, event, sender, cb):
        assert event in ('on_vlob_updated', 'on_user_vlob_updated', 'on_message_arrived')
        self._backend_event_ns.signal(event).connect(cb, sender=sender)

    async def group_create(self, name):
        self.identity.id  # Trigger exception if identity is not loaded
        msg = {'cmd': 'group_create', 'name': name}
        return await self._group_service._cmd_CREATE(self._session, msg)

    async def group_read(self, name):
        self.identity.id  # Trigger exception if identity is not loaded
        msg = {'cmd': 'group_read', 'name': name}
        return await self._group_service._cmd_READ(self._session, msg)

    async def group_add_identities(self, name, identities, admin=False):
        self.identity.id  # Trigger exception if identity is not loaded
        msg = {'cmd': 'group_add_identities',
               'name': name,
               'identities': identities,
               'admin': admin}
        return await self._group_service._cmd_ADD_IDENTITIES(self._session, msg)

    async def group_remove_identities(self, name, identities, admin=False):
        self.identity.id  # Trigger exception if identity is not loaded
        msg = {'cmd': 'group_remove_identities',
               'name': name,
               'identities': identities,
               'admin': admin}
        return await self._group_service._cmd_REMOVE_IDENTITIES(self._session, msg)

    async def message_new(self, recipient, body):
        self.identity.id  # Trigger exception if identity is not loaded
        msg = {'cmd': 'message_new', 'recipient': recipient, 'body': to_jsonb64(body)}
        return await self._message_service._cmd_NEW(self._session, msg)

    async def message_get(self, recipient, offset=0):
        self.identity.id  # Trigger exception if identity is not loaded
        msg = {'cmd': 'message_get', 'recipient': recipient, 'offset': offset}
        ret = await self._message_service._cmd_GET(self._session, msg)
        return [msg['body'] for msg in ret['messages']]

    async def user_vlob_read(self, version=None):
        self.identity.id  # Trigger exception if identity is not loaded
        msg = {'cmd': 'user_vlob_read'}
        if version:
            msg['version'] = version
        return await self._user_vlob_service._cmd_READ(self._session, msg)

    async def user_vlob_update(self, version, blob=b''):
        self.identity.id  # Trigger exception if identity is not loaded
        msg = {
            'cmd': 'user_vlob_update',
            'version': version,
            'blob': to_jsonb64(blob)
        }
        await self._user_vlob_service._cmd_UPDATE(self._session, msg)

    async def vlob_create(self, blob=b''):
        self.identity.id  # Trigger exception if identity is not loaded
        assert isinstance(blob, bytes)
        msg = {'cmd': 'vlob_create', 'blob': to_jsonb64(blob)}
        return await self._vlob_service._cmd_CREATE(self._session, msg)

    async def vlob_read(self, id, trust_seed, version=None):
        self.identity.id  # Trigger exception if identity is not loaded
        msg = {'cmd': 'vlob_read', 'id': id, 'trust_seed': trust_seed}
        if version:
            msg['version'] = version
        return await self._vlob_service._cmd_READ(self._session, msg)

    async def vlob_update(self, id, version, trust_seed, blob=b''):
        self.identity.id  # Trigger exception if identity is not loaded
        msg = {
            'cmd': 'vlob_update',
            'id': id,
            'version': version,
            'trust_seed': trust_seed,
            'blob': to_jsonb64(blob)
        }
        await self._vlob_service._cmd_UPDATE(self._session, msg)
