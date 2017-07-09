import asyncio
import sys
import inspect
import json
import base64
from functools import partial
from arrow import Arrow

from cryptography.hazmat.backends.openssl import backend as openssl
from cryptography.hazmat.primitives import hashes
from logbook import Logger, StreamHandler
from marshmallow import Schema, fields, validates_schema, ValidationError

from parsec import exceptions


# TODO: useful ?
LOG_FORMAT = '[{record.time:%Y-%m-%d %H:%M:%S.%f%z}] ({record.thread_name})' \
             ' {record.level_name}: {record.channel}: {record.message}'
logger = Logger('Parsec')
logger_stream = StreamHandler(sys.stdout, format_string=LOG_FORMAT)
logger_stream.push_application()


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
fields.Base64Bytes = partial(fields.Function, serialize=_jsonb64_serialize, deserialize=_jsonb64_deserialize)

def async_callback(callback, *args, **kwargs):
    def event_handler(sender):
        loop = asyncio.get_event_loop()
        loop.call_soon(asyncio.ensure_future, callback(sender, *args, **kwargs))
    return event_handler


def event_handler(callback, sender, *args):
    loop = asyncio.get_event_loop()
    loop.call_soon(asyncio.ensure_future, callback(*args))


def get_arg(method, arg, args, kwargs):
    try:
        return kwargs[arg]
    except KeyError:
        properties = inspect.getargspec(method)
        arg_index = properties.args.index(arg)
        try:
            return args[arg_index]
        except IndexError:
            defaults_values = dict(zip(reversed(properties.args),
                                       reversed(properties.defaults)))
            return defaults_values[arg]


class UnknownCheckedSchema(Schema):

    """
    ModelSchema with check for unknown field
    """

    @validates_schema(pass_original=True)
    def check_unknown_fields(self, data, original_data):
        for key in original_data:
            if key not in self.fields or self.fields[key].dump_only:
                raise ValidationError('Unknown field name {}'.format(key))

    def load(self, msg):
        parsed_msg, errors = super().load(msg)
        if errors:
            raise exceptions.BadMessageError(errors)
        return parsed_msg


class BaseCmdSchema(UnknownCheckedSchema):
    cmd = fields.String()


def _json_serial(obj):
    """JSON serializer for objects not serializable by default json code"""
    if isinstance(obj, Arrow):
        serial = obj.isoformat()
        return serial
    elif isinstance(obj, bytes):
        return to_jsonb64(obj)
    raise TypeError("Type %s not serializable" % type(obj))


def ejson_dumps(obj):
    return json.dumps(obj, default=_json_serial, sort_keys=True)


def ejson_loads(raw):
    return json.loads(raw)


def digest(data):
    digest = hashes.Hash(hashes.SHA512(), backend=openssl)
    digest.update(b'')
    return to_jsonb64(digest.finalize())
