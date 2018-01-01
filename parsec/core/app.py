import trio
from marshmallow import fields
from urllib.parse import urlparse

from parsec.core.fs import fs_factory
from parsec.core.fs_api import FSApi
from parsec.core.manifests_manager import ManifestsManager
from parsec.core.blocks_manager import BlocksManager
from parsec.core.backend_connection import BackendConnection, BackendNotAvailable
from parsec.utils import CookedSocket, BaseCmdSchema, ParsecError, User


class cmd_LOGIN_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    password = fields.String(missing=None)


class CoreApp:

    def __init__(self, config):
        self.config = config
        self.backend_addr = config['BACKEND_ADDR']

        self.nursery = None
        self.auth_user = None
        self.auth_privkey = None
        self.fs = None
        self.backend_connection = None

        self._fs_api = FSApi()

        self.cmds = {
            'register': self._api_register,
            'identity_login': self._api_identity_login,
            'identity_logout': self._api_identity_logout,
            'identity_info': self._api_identity_info,
            'get_available_logins': self._api_get_available_logins,
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

    def _get_user(self, userid, password):
        return None

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

    async def login(self, user):
        self.auth_user = user
        self.backend_connection = BackendConnection(user, self.config['BACKEND_ADDR'])
        await self.backend_connection.init(self.nursery)
        self.fs = await fs_factory(user, self.config, self.backend_connection)
        # local_storage = LocalStorage()
        # backend_storage = BackendStorage()
        # manifests_manager = ManifestsManager(self.auth_user, local_storage, backend_storage)
        # blocks_manager = BlocksManager(self.auth_user, local_storage, backend_storage)
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
        self.auth_user = None
        self.fs = None
        self._fs_api.fs = None

    async def _api_register(self, req):
        return {'status': 'not_implemented'}

    async def _api_identity_login(self, req):
        if self.auth_user:
            return {'status': 'already_logged'}
        msg = cmd_LOGIN_Schema().load(req)
        rawkeys = self._get_user(msg['id'], msg['password'])
        if not rawkeys:
            return {'status': 'unknown_user'}
        await self.login(User(msg['id'], *rawkeys))
        return {'status': 'ok'}

    async def _api_identity_logout(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        await self.logout()
        return {'status': 'ok'}

    async def _api_identity_info(self, req):
        return {
            'status': 'ok',
            'loaded': bool(self.auth_user),
            'id': self.auth_user.id if self.auth_user else None
        }

    async def _api_get_available_logins(self, req):
        pass

    async def _api_get_core_state(self, req):
        status = {'status': 'ok', 'login': None, 'backend_online': False}
        if self.auth_user:
            status['login'] = self.auth_user.id
            try:
                await self.backend_connection.ping()
                status['backend_online'] = True
            except BackendNotAvailable:
                status['backend_online'] = False
        return status
