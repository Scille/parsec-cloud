import attr
import json
from marshmallow import fields
from effect2 import TypeDispatcher, Effect, do

from parsec.exceptions import PrivKeyHashCollision, PrivKeyNotFound
from parsec.tools import UnknownCheckedSchema, ejson_dumps, ejson_loads
from parsec.exceptions import ParsecError


@attr.s
class EPrivKeyGet:
    hash = attr.ib()


@attr.s
class EPrivKeyAdd:
    hash = attr.ib()
    cipherkey = attr.ib()


class cmd_PRIVKEY_GET_Schema(UnknownCheckedSchema):
    hash = fields.String(required=True)


class cmd_PRIVKEY_ADD_Schema(UnknownCheckedSchema):
    hash = fields.String(required=True)
    cipherkey = fields.Base64Bytes(required=True)


@do
def api_privkey_get(msg):
    msg = cmd_PRIVKEY_GET_Schema().load(msg)
    cipherkey = yield Effect(EPrivKeyGet(**msg))
    return {'status': 'ok', 'hash': msg['hash'], 'cipherkey': cipherkey}


@do
def api_privkey_add(msg):
    msg = cmd_PRIVKEY_ADD_Schema().load(msg)
    yield Effect(EPrivKeyAdd(**msg))
    return {'status': 'ok'}


@attr.s
class MockedPrivKeyComponent:
    _keys = attr.ib(default=attr.Factory(dict))

    @do
    def perform_privkey_add(self, intent):
        # TODO: should check for authorization token to avoid impersonation
        assert isinstance(intent.cipherkey, (bytes, bytearray))
        if intent.hash in self._keys:
            raise PrivKeyHashCollision('Hash collision, change your password and retry.')
        else:
            self._keys[intent.hash] = intent.cipherkey

    @do
    def perform_privkey_get(self, intent):
        try:
            return self._keys[intent.hash]
        except KeyError:
            raise PrivKeyNotFound('No entry with this hash')

    def get_dispatcher(self):
        return TypeDispatcher({
            EPrivKeyGet: self.perform_privkey_get,
            EPrivKeyAdd: self.perform_privkey_add
        })


@do
def execute_raw_cmd(raw_cmd: str):
    try:
        params = ejson_loads(raw_cmd)
    except json.decoder.JSONDecodeError:
        ret = {'status': 'bad_msg', 'label': 'Message is not a valid JSON.'}
    else:
        cmd_type = params.pop('cmd', None)
        if not isinstance(cmd_type, str):
            ret = {'status': 'bad_msg', 'label': '`cmd` string field is mandatory.'}
        else:
            ret = yield execute_cmd(cmd_type, params)
    return ejson_dumps(ret).encode('utf-8')


@do
def execute_cmd(cmd, params):
    try:
        resp = yield API_CMDS_ROUTER[cmd](params)
    except KeyError:
        resp = {'status': 'bad_msg', 'label': 'Unknown command `%s`' % cmd}
    except ParsecError as exc:
        resp = exc.to_dict()
    return resp


API_CMDS_ROUTER = {
    'privkey_add': api_privkey_add,
    'privkey_get': api_privkey_get
}
