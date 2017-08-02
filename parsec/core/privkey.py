import attr
from effect2 import Effect, TypeDispatcher, do
import hashlib
import websockets

from parsec.core.identity import EIdentityLoad, EIdentityGet
from parsec.exceptions import exception_from_status
from parsec.tools import ejson_dumps, ejson_loads


@attr.s
class EPrivKeyBackendCmd:
    cmd = attr.ib()
    msg = attr.ib(default=attr.Factory(dict))


@attr.s
class EPrivKeyExport:
    password = attr.ib()


@attr.s
class EPrivKeyLoad:
    id = attr.ib()
    password = attr.ib()


@attr.s
class PrivKeyBackendComponent:
    url = attr.ib()

    @do
    def perform_privkey_export(self, intent):
        identity = yield Effect(EIdentityGet())
        id_password_hash = hashlib.sha256((identity.id + ':' + intent.password).encode()).digest()
        cipherkey = identity.private_key.export(intent.password)
        yield Effect(EPrivKeyBackendCmd(
            'privkey_add', {'hash': id_password_hash, 'cipherkey': cipherkey}))

    @do
    def perform_privkey_load(self, intent):
        id_password_hash = hashlib.sha256((intent.id + ':' + intent.password).encode()).digest()
        cipherkey = yield Effect(EPrivKeyBackendCmd('privkey_get', {'hash': id_password_hash}))
        yield Effect(EIdentityLoad(intent.id, cipherkey, intent.password))

    async def perform_privkey_backend_cmd(self, intent):
        msg = {'cmd': intent.cmd, **intent.msg}
        async with websockets.connect(self.url) as ws:
            await ws.send(ejson_dumps(msg))
            raw = await ws.recv()
            if isinstance(raw, bytes):
                raw = raw.decode()
            ret = ejson_loads(raw)
            status = ret['status']
            if status == 'ok':
                return ret
            else:
                raise exception_from_status(status)(ret['label'])

    def get_dispatcher(self):
        return TypeDispatcher({
            EPrivKeyExport: self.perform_privkey_export,
            EPrivKeyLoad: self.perform_privkey_load,
            EPrivKeyBackendCmd: self.perform_privkey_backend_cmd
        })
