import attr
import trio
from marshmallow import fields
from nacl.public import PrivateKey
from urllib.parse import urlparse

from .config import CONFIG
from .local_fs import LocalFS
from parsec.utils import CookedSocket, BaseCmdSchema, ParsecError, User


class cmd_LOGIN_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    password = fields.String(missing=None)


class CoreApp:

    def __init__(self, config):
        self.config = config
        self.server_ready = trio.Event()
        self.backend_addr = config['BACKEND_ADDR']
        addr = self.config['ADDR']
        if addr.startswith('unix://'):
            self.socket_type = trio.socket.AF_UNIX
            self.socket_bind_opts = addr[len('unix://'):]
        elif addr.startswith('tcp://'):
            self.socket_type = trio.socket.AF_INET
            parsed = urlparse(addr)
            self.socket_bind_opts = (parsed.hostname, parsed.port)
        else:
            raise RuntimeError('Invalid ADDR value `%s`' % addr)
        self.nursery = None
        self.auth_user = None
        self.auth_privkey = None
        self.fs = None

    def _get_user(self, userid, password):
        return None

    async def _serve_client(self, client_sock):
        # TODO: handle client not closing there part of the socket...
        print('server sock', client_sock)
        with client_sock:
            sock = CookedSocket(client_sock)
            while True:
                req = await sock.recv()
                if not req:  # Client disconnected
                    print('CLIENT DISCONNECTED')
                    return
                print('REQ %s' % req)
                cmd_func = getattr(self, '_cmd_%s' % req['cmd'].upper())
                try:
                    rep = await cmd_func(req)
                except ParsecError as err:
                    rep = err.to_dict()
                print('REP %s' % rep)
                await sock.send(rep)

    async def _wait_clients(self, nursery):
        with trio.socket.socket(self.socket_type) as listen_sock:
            listen_sock.bind(self.socket_bind_opts)
            listen_sock.listen()
            self.server_ready.set()
            while True:
                server_sock, _ = await listen_sock.accept()
                nursery.start_soon(self._serve_client, server_sock)

    async def run(self):
        try:
            async with trio.open_nursery() as self.nursery:
                self.nursery.start_soon(self._wait_clients, self.nursery)
        finally:
            if self.socket_type == trio.socket.AF_UNIX:
                try:
                    import os
                    os.remove(self.socket_bind_opts)
                except FileNotFoundError:
                    pass

    async def login(self, user):
        self.auth_user = user
        self.fs = LocalFS(self.auth_user, self.backend_addr)
        await self.fs.init(self.nursery)

    async def logout(self):
        await self.fs.teardown()
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
        return await self.fs._cmd_FILE_CREATE(req)

    async def _cmd_FILE_READ(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs._cmd_FILE_READ(req)

    async def _cmd_FILE_WRITE(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs._cmd_FILE_WRITE(req)

    async def _cmd_FILE_SYNC(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs._cmd_FILE_SYNC(req)

    async def _cmd_STAT(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs._cmd_STAT(req)

    async def _cmd_FOLDER_CREATE(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs._cmd_FOLDER_CREATE(req)

    async def _cmd_MOVE(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs._cmd_MOVE(req)

    async def _cmd_DELETE(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs._cmd_DELETE(req)

    async def _cmd_FILE_TRUNCATE(self, req):
        if not self.auth_user:
            return {'status': 'login_required'}
        return await self.fs._cmd_FILE_TRUNCATE(req)
