import attr
import base64
import json
import trio
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
        await self.sockstream.send_all(json.dumps(msg).encode() + b'\n')

    async def recv(self):
        raw = b''
        # TODO: handle message longer than BUFFSIZE...
        raw = await self.sockstream.receive_some(BUFFSIZE)
        if not raw:
            # Empty body should normally never occurs, though it is sent
            # when peer closes connection
            raise trio.BrokenStreamError('Peer has closed connection')
        while raw[-1] != ord(b'\n'):
            raw += await self.sockstream.receive_some(BUFFSIZE)
        return json.loads(raw[:-1].decode())


def to_jsonb64(raw: bytes):
    return base64.encodebytes(raw).decode().replace('\\n', '')


def from_jsonb64(msg: str):
    return base64.decodebytes(msg.encode())


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

    def __init__(self, reason=None, **kwargs):
        if reason:
            self.kwargs = {**kwargs, 'reason': reason}
        else:
            self.kwargs = kwargs

    def to_dict(self):
        return {'status': self.status, **self.kwargs}


def abort(status='bad_message', **kwargs):
    error = ParsecError(**kwargs)
    error.status = status
    raise error
