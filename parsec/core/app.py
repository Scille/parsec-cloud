import trio
from marshmallow import fields
from urllib.parse import urlparse

from parsec.core.fs import fs_factory
from parsec.core.fs_api import FSApi
from parsec.core.manifests_manager import ManifestsManager
from parsec.core.blocks_manager import BlocksManager
from parsec.core.backend_connection import BackendConnection
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
                    cmd_func = getattr(self, '_cmd_%s' % req['cmd'].upper())
                except AttributeError:
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
        self.backend_connection = BackendConnection(
            user, self.config['BACKEND_ADDR'])
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
        self.fs.api = FSApi(self.fs)

    async def logout(self):
        await self.fs.teardown()
        await self.backend_connection.teardown()
        self.backend_connection = None
        # await self.fs.manifests_manager.teardown()
        # await self.fs.blocks_manager.teardown()
        self.fs = None
        self.auth_user = None

    async def _cmd_REGISTER(self, req):
        return {'status': 'not_implemented'}

    async def _cmd_IDENTITY_LOGIN(self, req):
        if self.auth_user:
            return {'status': 'already_logged'}
        msg = cmd_LOGIN_Schema().load(req)
        rawkeys = self._get_user(msg['id'], msg['password'])
        if not rawkeys:
            return {'status': 'unknown_user'}
        await self.login(User(msg['id'], *rawkeys))
        return {'status': 'ok'}

    async def _cmd_IDENTITY_LOGOUT(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        await self.logout()
        return {'status': 'ok'}

    async def _cmd_IDENTITY_INFO(self, req):
        return {
            'status': 'ok',
            'loaded': bool(self.auth_user),
            'id': self.auth_user.id if self.auth_user else None
        }

    async def _cmd_GET_AVAILABLE_LOGINS(self, req):
        pass

    async def _cmd_GET_CORE_STATE(self, req):
        return {'status': 'ok'}

    async def _cmd_FILE_CREATE(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs.api._cmd_FILE_CREATE(req)

    async def _cmd_FILE_READ(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs.api._cmd_FILE_READ(req)

    async def _cmd_FILE_WRITE(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs.api._cmd_FILE_WRITE(req)

    async def _cmd_FLUSH(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs.api._cmd_FLUSH(req)

    async def _cmd_SYNCHRONIZE(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs.api._cmd_SYNCHRONIZE(req)

    async def _cmd_STAT(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs.api._cmd_STAT(req)

    async def _cmd_FOLDER_CREATE(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs.api._cmd_FOLDER_CREATE(req)

    async def _cmd_MOVE(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs.api._cmd_MOVE(req)

    async def _cmd_DELETE(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs.api._cmd_DELETE(req)

    async def _cmd_FILE_TRUNCATE(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs.api._cmd_FILE_TRUNCATE(req)
