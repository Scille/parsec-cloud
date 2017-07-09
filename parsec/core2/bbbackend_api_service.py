import attr
import asyncio
import json
import websockets
from effect import Effect, TypeDispatcher, sync_perform
from effect.do import do
from aioeffect import perform as asyncio_perform, performer as asyncio_performer

from parsec.service import BaseService, service
from parsec.core2.identity_service import Event, IdentityGet
from parsec.tools import to_jsonb64, from_jsonb64, ejson_dumps, ejson_loads, async_callback, logger
from parsec.exceptions import exception_from_status


@attr.s
class BackendCmd:
    cmd = attr.ib()
    msg = attr.ib(default=dict)


@attr.s
class UserVlobAtom:
    version = attr.ib()
    blob = attr.ib()


@attr.s
class UserVlobRead:
    version = attr.ib()


@attr.s
class UserVlobUpdate:
    version = attr.ib()
    blob = attr.ib()


@do
def perform_user_vlob_read(version=None):
    msg = {'version': version} if version else {}
    ret = yield Effect(BackendCmd('user_vlob_read', msg))
    status = ret['status']
    if status == 'ok':
        return UserVlobAtom(ret['version'], from_jsonb64(ret['blob']))
    else:
        raise exception_from_status(status)(ret['label'])


@do
def perform_user_vlob_update(version, blob):
    msg = {
        'version': version,
        'blob': to_jsonb64(blob)
    }
    ret = yield Effect(BackendCmd('user_vlob_update', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])


@attr.s
class VlobAtom:
    id = attr.ib()
    version = attr.ib()
    blob = attr.ib()


@attr.s
class VlobAccess:
    id = attr.ib()
    read_trust_seed = attr.ib()
    write_trust_seed = attr.ib()


@attr.s
class VlobRead:
    id = attr.ib()
    trust_seed = attr.ib()
    version = attr.ib(default=None)


@attr.s
class VlobCreate:
    blob = attr.ib()


@attr.s
class VlobUpdate:
    id = attr.ib()
    trust_seed = attr.ib()
    version = attr.ib()
    blob = attr.ib()


@do
def perform_vlob_read(id, trust_seed, version=None):
    msg = {
        'id': id,
        'trust_seed': trust_seed
    }
    if version is not None:
        msg['version'] = version
    ret = yield Effect(BackendCmd('vlob_read', msg))
    status = ret['status']
    if status == 'ok':
        return VlobAtom(id, ret['version'], from_jsonb64(ret['blob']))
    else:
        raise exception_from_status(status)(ret['label'])


@do
def perform_vlob_create(blob=b''):
    ret = yield Effect(BackendCmd('vlob_create', {'blob': to_jsonb64(blob)}))
    status = ret['status']
    if status == 'ok':
        return VlobAccess(
            id=ret['id'],
            read_trust_seed=ret['read_trust_seed'],
            write_trust_seed=ret['write_trust_seed']
        )
    else:
        raise exception_from_status(status)(ret['label'])


@do
def perform_vlob_update(id, version, trust_seed, blob):
    msg = {
        'id': id,
        'version': version,
        'trust_seed': trust_seed,
        'blob': to_jsonb64(blob)
    }
    ret = yield Effect(BackendCmd('vlob_update', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])


# TODO

# @do
# def connect_event(event, sender, cb):
#     msg = {'event': event, 'sender': sender}
#     ret = yield Effect(BackendCmd('subscribe', msg))
#     status = ret['status']
#     if status != 'ok':
#         raise exception_from_status(status)(ret['label'])


# @attr.s
# class Group:
#     name = attr.ib()
#     admins = attr.ib(default=attr.Factory(list))
#     users = attr.ib(default=attr.Factory(list))


# @do
# def group_create(name):
#     ret = yield Effect(BackendCmd('group_create', {'name': name}))
#     status = ret['status']
#     if status != 'ok':
#         raise exception_from_status(status)(ret['label'])


# @do
# def group_read(name):
#     ret = yield Effect(BackendCmd('group_create', {'name': name}))
#     status = ret['status']
#     if status != 'ok':
#         raise exception_from_status(status)(ret['label'])
#     return Group(name, ret['admins'], ret['users'])


# @do
# def group_add_identities(name, identities, admin=False):
#     msg = {'name': name, 'identities': identities, 'admin': admin}
#     ret = yield Effect(BackendCmd('group_add_identities', msg))
#     status = ret['status']
#     if status != 'ok':
#         raise exception_from_status(status)(ret['label'])


# @do
# def group_remove_identities(name, identities, admin=False):
#     msg = {'name': name, 'identities': identities, 'admin': admin}
#     ret = yield Effect(BackendCmd('group_remove_identities', msg))
#     status = ret['status']
#     if status != 'ok':
#         raise exception_from_status(status)(ret['label'])


# @attr.s
# class Message:
#     count = attr.ib()
#     body = attr.ib()


# @do
# def message_new(recipient, body):
#     msg = {'recipient': recipient, 'body': to_jsonb64(body)}
#     ret = yield Effect(BackendCmd('message_new', msg))
#     status = ret['status']
#     if status != 'ok':
#         raise exception_from_status(status)(ret['label'])


# @do
# def message_get(recipient, offset=0):
#     self.identity.id  # Trigger exception if identity is not loaded
#     msg = {'recipient': recipient, 'offset': offset}
#     ret = yield Effect(BackendCmd('message_get', msg))
#     status = ret['status']
#     if status != 'ok':
#         raise exception_from_status(status)(ret['label'])
#     return [Message(msg['count'], from_jsonb64(msg['body'])) for msg in ret['messages']]



class BackendAPIService(BaseService):

    name = 'BackendAPIService'

    identity = service('IdentityService')

    def __init__(self, backend_url, watchdog=None, dispatcher=None):
        super().__init__(dispatcher=dispatcher)
        self._resp_queue = asyncio.Queue()
        assert backend_url.startswith('ws://') or backend_url.startswith('wss://')
        self._backend_url = backend_url
        self._websocket = None
        self._ws_recv_handler_task = None
        self._watchdog_task = None
        self._watchdog_time = watchdog
        self._connection_ready_future = asyncio.Future()

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
        identity = await self.identity.get()
        answer = identity.private_key.sign(challenge['challenge'].encode())
        await self._websocket.send(ejson_dumps({
            'handshake': 'answer',
            'identity': identity.id,
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
            raise Identit

    async def vlob_read(self, id, trust_seed, version=None):
        return await asyncio_perform(self.dispatcher, perform_vlob_read(id, trust_seed, version))

    async def vlob_create(self, blob=b''):
        return await asyncio_perform(self.dispatcher, perform_vlob_create(blob))

    async def vlob_update(self, id, version, trust_seed, blob):
        return await asyncio_perform(self.dispatcher, perform_vlob_update(id, version, trust_seed, blob))

    async def user_vlob_read(self, version=None):
        return await asyncio_perform(self.dispatcher, perform_user_vlob_read(version))

    async def user_vlob_update(self, version, blob=b''):
        return await asyncio_perform(self.dispatcher, perform_user_vlob_update(version, blob))

    # async def connect_event(self, event, sender, cb):
    #     return await asyncio_perform(self._dispatcher, connect_event(event, sender, cb))

    # async def group_create(self, name):
    #     return await asyncio_perform(self._dispatcher, group_create(name))

    # async def group_read(self, name):
    #     return await asyncio_perform(self._dispatcher, group_read(name))

    # async def group_add_identities(self, name, identities, admin=False):
    #     return await asyncio_perform(
    #         self._dispatcher, group_add_identities(name, identities, admin))

    # async def group_remove_identities(self, name, identities, admin=False):
    #     return await asyncio_perform(
    #         self._dispatcher, group_remove_identities(name, identities, admin))

    # async def message_new(self, recipient, body):
    #     return await asyncio_perform(self._dispatcher, message_new(recipient, body))

    # async def message_get(self, recipient, offset=0):
    #     return await asyncio_perform(self._dispatcher, message_get(recipient, offset=0))


async def backend_api_service_factory(backend_url, watchdog=None):
    svc = BackendAPIService(backend_url, watchdog=watchdog)

    @asyncio_performer
    async def perform_backend_cmd(dispatcher, backend_cmd):
        payload = json_dumps({**backend_cmd.msg, 'cmd': backend_cmd.cmd})
        await svc._websocket.send(payload)
        return await svc._resp_queue.get()

    dispatcher = TypeDispatcher({
        BackendCmd: perform_backend_cmd,
        UserVlobUpdate: perform_user_vlob_update,
        UserVlobRead: perform_user_vlob_read,
        VlobRead: perform_vlob_read,
        VlobUpdate: perform_vlob_update,
        VlobCreate: perform_vlob_create,
    })
    svc.dispatcher = dispatcher
    return svc
