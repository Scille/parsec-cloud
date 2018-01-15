import trio
from nacl.public import PrivateKey
from nacl.signing import SigningKey

from parsec.core.fs import fs_factory
from parsec.core.fs_api import FSApi
from parsec.core.devices_manager import DevicesManager, DeviceLoadingError
from parsec.core.manifests_manager import ManifestsManager
from parsec.core.blocks_manager import BlocksManager
from parsec.core.backend_connection import (
    BackendConnection, BackendNotAvailable, backend_send_anonymous_cmd)
from parsec.utils import CookedSocket, ParsecError, to_jsonb64
from parsec.schema import BaseCmdSchema, fields


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


class CoreApp:

    def __init__(self, config):
        self.config = config
        self.backend_addr = config.backend_addr

        self.nursery = None
        self.auth_device = None
        self.auth_privkey = None
        self.fs = None
        self.backend_connection = None
        self.devices_manager = DevicesManager(config.base_settings_path)

        self._fs_api = FSApi()

        self.cmds = {
            'user_invite': self._api_user_invite,
            'user_claim': self._api_user_claim,

            # 'device_declare': self._api_device_declare,
            # 'device_get_configuration_try': self._api_device_get_configuration_try,
            # 'device_accept_configuration_try': self._api_device_accept_configuration_try,
            # 'device_refuse_configuration_try': self._api_device_refuse_configuration_try,

            'register_device': self._api_register_device,
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

    async def handle_client(self, sockstream):
        try:
            sock = CookedSocket(sockstream)
            while True:
                req = await sock.recv()
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
        self.auth_device = device
        self.backend_connection = BackendConnection(
            device, self.config.backend_addr
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

    async def logout(self):
        await self.fs.teardown()
        await self.backend_connection.teardown()
        self.backend_connection = None
        # await self.fs.manifests_manager.teardown()
        # await self.fs.blocks_manager.teardown()
        self.auth_device = None
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

    async def _api_register_device(self, req):
        if not self.auth_device:
            return {'status': 'login_required'}
        return {'status': 'not_implemented'}

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
