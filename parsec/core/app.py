import trio
import blinker
from nacl.public import PrivateKey, PublicKey, SealedBox
from nacl.signing import SigningKey
from json import JSONDecodeError

from parsec.core.fs import fs_factory
from parsec.core.fs_api import FSApi
from parsec.core.devices_manager import DevicesManager, DeviceLoadingError
from parsec.core.manifests_manager import ManifestsManager
from parsec.core.blocks_manager import BlocksManager
from parsec.core.backend_connection import (
    BackendConnection, BackendNotAvailable, backend_send_anonymous_cmd)
from parsec.utils import CookedSocket, ParsecError, to_jsonb64, from_jsonb64
from parsec.schema import BaseCmdSchema, fields, validate


class cmd_LOGIN_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    password = fields.String(missing=None)


class cmd_USER_INVITE_Schema(BaseCmdSchema):
    user_id = fields.String(required=True)


# TODO: change id to user_id/device_name
class cmd_USER_CLAIM_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    invitation_token = fields.String(required=True)
    password = fields.String(required=True)


class cmd_EVENT_LISTEN_Schema(BaseCmdSchema):
    wait = fields.Boolean(missing=True)


class cmd_EVENT_SUBSCRIBE_Schema(BaseCmdSchema):
    event = fields.String(required=True)
    subject = fields.String(missing=None)


class cmd_DEVICE_CONFIGURE_Schema(BaseCmdSchema):
    # TODO: add regex validation
    device_id = fields.String(required=True)
    password = fields.String(required=True)
    configure_device_token = fields.String(required=True)


class cmd_DEVICE_ACCEPT_CONFIGURATION_TRY_Schema(BaseCmdSchema):
    configuration_try_id = fields.String(required=True)


class CoreApp:

    def __init__(self, config):
        self.signal_ns = blinker.Namespace()
        self.config = config
        self.backend_addr = config.backend_addr

        self._config_try_pendings = {}

        self.nursery = None
        # TODO: create a context object to store/manipulate auth_* data
        self.auth_device = None
        self.auth_privkey = None
        self.auth_subscribed_events = None
        self.auth_events = None
        self.fs = None
        self.backend_connection = None
        self.devices_manager = DevicesManager(config.base_settings_path)

        self._fs_api = FSApi()

        self.cmds = {
            'event_subscribe': self._api_event_subscribe,
            'event_unsubscribe': self._api_event_unsubscribe,
            'event_listen': self._api_event_listen,
            'event_list_subscribed': self._api_event_list_subscribed,

            'user_invite': self._api_user_invite,
            'user_claim': self._api_user_claim,

            'device_declare': self._api_device_declare,
            'device_configure': self._api_device_configure,
            'device_get_configuration_try': self._api_device_get_configuration_try,
            'device_accept_configuration_try': self._api_device_accept_configuration_try,

            'login': self._api_login,
            'logout': self._api_logout,
            'info': self._api_info,
            'list_available_logins': self._api_list_available_logins,
            'get_core_state': self._api_get_core_state,

            'file_create': self._fs_api.file_create,
            'file_read': self._fs_api.file_read,
            'file_write': self._fs_api.file_write,
            'flush': self._fs_api.flush,
            'synchronize': self._fs_api.synchronize,
            'stat': self._fs_api.stat,
            'folder_create': self._fs_api.folder_create,
            'move': self._fs_api.move,
            'delete': self._fs_api.delete,
            'file_truncate': self._fs_api.file_truncate,
        }

    async def init(self, nursery):
        self.nursery = nursery

    async def shutdown(self):
        if self.auth_device:
            await self.logout()

    async def handle_client(self, sockstream):
        try:
            sock = CookedSocket(sockstream)
            while True:
                try:
                    req = await sock.recv()
                except JSONDecodeError:
                    rep = {'status': 'invalid_msg_format'}
                    await sock.send(rep)
                    continue
                if not req:  # Client disconnected
                    print('CLIENT DISCONNECTED')
                    return
                print('REQ %s' % req)
                try:
                    cmd_func = self.cmds[req['cmd']]
                except KeyError:
                    rep = {'status': 'unknown_command'}
                else:
                    try:
                        rep = await cmd_func(req)
                    except ParsecError as err:
                        rep = err.to_dict()
                print('REP %s' % rep)
                await sock.send(rep)
        except trio.BrokenStreamError:
            # Client has closed connection
            pass

    async def login(self, device):
        self.auth_subscribed_events = {}
        self.auth_events = trio.Queue(100)
        self.backend_connection = BackendConnection(
            device, self.config.backend_addr, self.signal_ns
        )
        await self.backend_connection.init(self.nursery)
        self.fs = await fs_factory(device, self.config, self.backend_connection)
        # local_storage = LocalStorage()
        # backend_storage = BackendStorage()
        # manifests_manager = ManifestsManager(self.auth_device, local_storage, backend_storage)
        # blocks_manager = BlocksManager(self.auth_device, local_storage, backend_storage)
        # # await manifests_manager.init()
        # # await blocks_manager.init()
        # self.fs = FS(manifests_manager, blocks_manager)
        await self.fs.init()
        self._fs_api.fs = self.fs
        # Keep this last to guarantee login was ok if it is set
        self.auth_device = device

    async def logout(self):
        await self.fs.teardown()
        await self.backend_connection.teardown()
        self.backend_connection = None
        # await self.fs.manifests_manager.teardown()
        # await self.fs.blocks_manager.teardown()
        self.auth_device = None
        self.auth_subscribed_events = None
        self.auth_events = None
        self.fs = None
        self._fs_api.fs = None

    async def _api_user_invite(self, req):
        if not self.auth_device:
            return {'status': 'login_required'}
        msg = cmd_USER_INVITE_Schema().load_or_abort(req)
        try:
            rep = await self.backend_connection.send(
                {'cmd': 'user_invite', 'user_id': msg['user_id']})
        except BackendNotAvailable:
            return {'status': 'backend_not_availabled'}
        return rep

    async def _api_user_claim(self, req):
        if self.auth_device:
            return {'status': 'already_logged'}
        msg = cmd_USER_CLAIM_Schema().load_or_abort(req)
        user_privkey = PrivateKey.generate()
        device_signkey = SigningKey.generate()
        user_id, device_name = msg['id'].split('@')
        try:
            rep = await backend_send_anonymous_cmd(self.backend_addr, {
                'cmd': 'user_claim',
                'user_id': user_id,
                'device_name': device_name,
                'invitation_token': msg['invitation_token'],
                'broadcast_key': to_jsonb64(user_privkey.public_key.encode()),
                'device_verify_key': to_jsonb64(device_signkey.verify_key.encode()),
            })
        except BackendNotAvailable:
            return {'status': 'backend_not_availabled'}
        self.devices_manager.register_new_device(
            msg['id'], user_privkey.encode(), device_signkey.encode(), msg['password'])
        return rep

    async def _backend_passthrough(self, req):
        try:
            rep = await self.backend_connection.send(req)
        except BackendNotAvailable:
            return {'status': 'backend_not_availabled'}
        return rep

    async def _api_device_declare(self, req):
        if not self.auth_device:
            return {'status': 'login_required'}
        return await self._backend_passthrough(req)

    async def _api_device_configure(self, req):
        msg = cmd_DEVICE_CONFIGURE_Schema().load_or_abort(req)

        user_id, device_name = msg['device_id'].split('@')
        user_privkey_cypherkey_privkey = PrivateKey.generate()
        device_signkey = SigningKey.generate()

        try:
            rep = await backend_send_anonymous_cmd(self.backend_addr, {
                'cmd': 'device_configure',
                'user_id': user_id,
                'device_name': device_name,
                'configure_device_token': msg['configure_device_token'],
                'device_verify_key': to_jsonb64(device_signkey.verify_key.encode()),
                'user_privkey_cypherkey': to_jsonb64(user_privkey_cypherkey_privkey.public_key.encode()),
            })
        except BackendNotAvailable:
            return {'status': 'backend_not_availabled'}
        if rep['status'] != 'ok':
            return rep

        cyphered = from_jsonb64(rep['cyphered_user_privkey'])
        box = SealedBox(user_privkey_cypherkey_privkey)
        user_privkey_raw = box.decrypt(cyphered)
        user_privkey = PrivateKey(user_privkey_raw)

        self.devices_manager.register_new_device(
            msg['device_id'], user_privkey.encode(), device_signkey.encode(), msg['password'])

        return {'status': 'ok'}

    async def _api_device_get_configuration_try(self, req):
        if not self.auth_device:
            return {'status': 'login_required'}
        return await self._backend_passthrough(req)

    async def _api_device_accept_configuration_try(self, req):
        if not self.auth_device:
            return {'status': 'login_required'}

        msg = cmd_DEVICE_ACCEPT_CONFIGURATION_TRY_Schema().load_or_abort(req)

        conf_try = self._config_try_pendings.get(msg['configuration_try_id'])
        if not conf_try:
            return {'status': 'unknown_configuration_try_id'}

        user_privkey_cypherkey_raw = from_jsonb64(conf_try['user_privkey_cypherkey'])
        box = SealedBox(PublicKey(user_privkey_cypherkey_raw))
        cyphered_user_privkey = box.encrypt(self.auth_device.user_privkey.encode())

        try:
            rep = await self.backend_connection.send({
                'cmd': 'device_accept_configuration_try',
                'configuration_try_id': msg['configuration_try_id'],
                'cyphered_user_privkey': to_jsonb64(cyphered_user_privkey),
            })
        except BackendNotAvailable:
            return {'status': 'backend_not_availabled'}
        if rep != 'ok':
            return rep
        return {'status': 'ok'}

    async def _api_login(self, req):
        if self.auth_device:
            return {'status': 'already_logged'}
        msg = cmd_LOGIN_Schema().load_or_abort(req)
        try:
            device = self.devices_manager.load_device(msg['id'], msg['password'])
        except DeviceLoadingError:
            return {'status': 'unknown_user'}
        await self.login(device)
        return {'status': 'ok'}

    async def _api_logout(self, req):
        if not self.auth_device:
            return {'status': 'login_required'}
        await self.logout()
        return {'status': 'ok'}

    async def _api_info(self, req):
        return {
            'status': 'ok',
            # TODO: replace by `logged_in`
            'loaded': bool(self.auth_device),
            # TODO: replace by `device_id` ?
            'id': self.auth_device.id if self.auth_device else None
        }

    async def _api_list_available_logins(self, req):
        devices = self.devices_manager.list_available_devices()
        return {
            'status': 'ok',
            'devices': devices
        }

    async def _api_get_core_state(self, req):
        status = {'status': 'ok', 'login': None, 'backend_online': False}
        if self.auth_device:
            status['login'] = self.auth_device.id
            try:
                await self.backend_connection.ping()
                status['backend_online'] = True
            except BackendNotAvailable:
                status['backend_online'] = False
        return status

    async def _api_event_subscribe(self, req):
        if not self.auth_device:
            return {'status': 'login_required'}

        msg = cmd_EVENT_SUBSCRIBE_Schema().load_or_abort(req)
        event = msg['event']
        subject = msg['subject']

        def _handle_event(sender):
            try:
                self.auth_events.put_nowait((event, sender))
            except trio.WouldBlock:
                print('WARNING: event queue is full')

        self.auth_subscribed_events[event, subject] = _handle_event
        if event == 'device_try_claim_submitted':
            await self.backend_connection.subscribe_event(event, subject)
        if subject:
            self.signal_ns.signal(event).connect(
                _handle_event, sender=subject, weak=True)
        else:
            self.signal_ns.signal(event).connect(_handle_event, weak=True)
        return {'status': 'ok'}

    async def _api_event_unsubscribe(self, req):
        if not self.auth_device:
            return {'status': 'login_required'}

        msg = cmd_EVENT_SUBSCRIBE_Schema().load_or_abort(req)
        try:
            del self.auth_subscribed_events[msg['event'], msg['subject']]
        except KeyError:
            return {
                'status': 'not_subscribed',
                'reason': 'Not subscribed to this event/subject couple'
            }
        return {'status': 'ok'}

    async def _api_event_listen(self, req):
        if not self.auth_device:
            return {'status': 'login_required'}

        msg = cmd_EVENT_LISTEN_Schema().load_or_abort(req)
        if msg['wait']:
            event, subject = await self.auth_events.get()
        else:
            try:
                event, subject = await self.auth_events.get_nowait()
            except trio.WouldBlock:
                return {'status': 'ok'}

        # TODO: make more generic
        if event == 'device_try_claim_submitted':
            rep = await self.backend_connection.send({
                'cmd': 'device_get_configuration_try',
                'configuration_try_id': subject
            })
            assert rep['status'] == 'ok'
            self._config_try_pendings[subject] = rep
            return {
                'status': 'ok',
                'event': event,
                'device_name': rep['device_name'],
                'configuration_try_id': subject,
            }
        else:
            return {'status': 'ok', 'event': event, 'subject': subject}

    async def _api_event_list_subscribed(self, req):
        if not self.auth_device:
            return {'status': 'login_required'}

        BaseCmdSchema().load_or_abort(req)  # empty msg expected
        return {
            'status': 'ok',
            'subscribed': list(self.auth_subscribed_events.keys())
        }
