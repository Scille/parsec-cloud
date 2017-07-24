import attr
from marshmallow import fields
from effect2 import TypeDispatcher, Effect, do

from parsec.exceptions import PubKeyError, PubKeyNotFound
from parsec.crypto import load_public_key
from parsec.tools import UnknownCheckedSchema


@attr.s
class EPubkeyGet:
    id = attr.ib()
    raw = attr.ib(default=False)


@attr.s
class EPubkeyAdd:
    id = attr.ib()
    key = attr.ib()


class cmd_PUBKEY_GET_Schema(UnknownCheckedSchema):
    id = fields.String(required=True)


@do
def api_pubkey_get(msg):
    msg = cmd_PUBKEY_GET_Schema().load(msg)
    key = yield Effect(EPubkeyGet(**msg, raw=True))
    return {'status': 'ok', 'id': msg['id'], 'key': key}


@attr.s
class MockedPubkeyComponent:
    _keys = attr.ib(default=attr.Factory(dict))

    @do
    def perform_pubkey_add(self, intent):
        assert isinstance(intent.key, (bytes, bytearray))
        if intent.id in self._keys:
            raise PubKeyError('Identity `%s` already has a public key' % intent.id)
        else:
            self._keys[intent.id] = intent.key

    @do
    def perform_pubkey_get(self, intent):
        try:
            key = self._keys[intent.id]
            return key if intent.raw else load_public_key(key)
        except KeyError:
            raise PubKeyNotFound('No public key for identity `%s`' % intent.id)

    def get_dispatcher(self):
        return TypeDispatcher({
            EPubkeyGet: self.perform_pubkey_get,
            EPubkeyAdd: self.perform_pubkey_add
        })