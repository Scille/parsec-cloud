import attr
import base64
import json
import trio
from functools import partial
from marshmallow import Schema, fields, validates_schema, ValidationError
from pendulum import Pendulum
from nacl.public import PrivateKey
from nacl.secret import SecretBox
from nacl.signing import SigningKey
import nacl.utils


BUFFSIZE = 4049


def generate_sym_key():
    return nacl.utils.random(SecretBox.KEY_SIZE)


@attr.s
class CookedSocket:
    sockstream = attr.ib()

    async def aclose(self):
        await self.sockstream.aclose()

    async def send(self, msg):
        print('[%s]==>%s' % (self.sockstream, msg))
        await self.sockstream.send_all(json.dumps(msg).encode() + b'\n')

    async def recv(self):
        raw = b''
        # TODO: handle message longer than BUFFSIZE...
        raw = await self.sockstream.receive_some(BUFFSIZE)
        print('[%s]<==%s' % (self.sockstream, raw))
        if not raw:
            # Empty body should normally never occurs, though it is sent
            # when peer closes connection
            raise trio.BrokenStreamError('Peer has closed connection')
        while raw[-1] != ord(b'\n'):
            raw += self.sockstream.receive_some(BUFFSIZE)
        return json.loads(raw[:-1].decode())


def to_jsonb64(raw: bytes):
    return base64.encodebytes(raw).decode().replace('\\n', '')


def from_jsonb64(msg: str):
    return base64.decodebytes(msg.encode())


# TODO: monkeypatch is ugly but I'm in a hurry...
def _jsonb64_serialize(obj):
    try:
        return to_jsonb64(obj)
    except:
        raise ValidationError('Invalid bytes')


def _jsonb64_deserialize(value):
    try:
        return from_jsonb64(value)
    except:
        raise ValidationError('Invalid base64 encoded data')


fields.Base64Bytes = partial(fields.Function,
                             serialize=_jsonb64_serialize,
                             deserialize=_jsonb64_deserialize)


class UnknownCheckedSchema(Schema):

    """
    ModelSchema with check for unknown field
    """

    @validates_schema(pass_original=True)
    def check_unknown_fields(self, data, original_data):
        for key in original_data:
            if key not in self.fields or self.fields[key].dump_only:
                raise ValidationError('Unknown field name {}'.format(key))

    def load(self, msg, abort_on_error=True):
        parsed_msg, errors = super().load(msg)
        if not abort_on_error:
            return parsed_msg, errors
        if errors:
            raise abort(errors=errors)
        else:
            return parsed_msg


class BaseCmdSchema(UnknownCheckedSchema):
    cmd = fields.String()


def _json_serial(obj):
    """JSON serializer for objects not serializable by default json code"""
    if isinstance(obj, Pendulum):
        serial = obj.isoformat()
        return serial
    elif isinstance(obj, bytes):
        return to_jsonb64(obj)
    raise TypeError("Type %s not serializable" % type(obj))


def ejson_dumps(obj):
    return json.dumps(obj, default=_json_serial, sort_keys=True)


def ejson_loads(raw):
    return json.loads(raw)


class ParsecError(Exception):
    status = 'error'
    reason = 'reason'

    def __init__(self, reason=None, **kwargs):
        if reason is None:
            reason = self.__class__.reason

        self.kwargs = {**kwargs, 'reason': reason}

    def to_dict(self):
        return {'status': self.status, **self.kwargs}


def abort(status='bad_message', **kwargs):
    error = ParsecError(**kwargs)
    error.status = status
    raise error


@attr.s(init=False)
class User:
    id = attr.ib()

    def __init__(self, id, privkey, signkey):
        self.id = id
        self.privkey = PrivateKey(privkey)
        self.signkey = SigningKey(signkey)

    @property
    def pubkey(self):
        return self.privkey.public_key

    @property
    def verifykey(self):
        return self.signkey.verify_key
