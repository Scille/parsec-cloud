import attr
import json
from marshmallow import Schema, fields
from effect2 import Effect, do

from parsec.tools import ejson_dumps, ejson_loads
from parsec.core import privkey_api
from parsec.core.client_connection import EClientSubscribeEvent, EClientUnsubscribeEvent
from parsec.core.identity import EIdentityLoad, EIdentityUnload, EIdentityGet
from parsec.exceptions import ParsecError, BadMessageError


def parse_cmd(raw_cmd: bytes):
    try:
        return ejson_loads(raw_cmd.decode('utf-8'))
    except (json.decoder.JSONDecodeError, UnicodeDecodeError):
        pass


@do
def execute_raw_cmd(raw_cmd):
    params = parse_cmd(raw_cmd)
    if not params:
        ret = {'status': 'bad_message', 'label': 'Message is not a valid JSON.'}
    else:
        cmd_type = params.pop('cmd', None)
        if not isinstance(cmd_type, str):
            ret = {'status': 'bad_message', 'label': '`cmd` string field is mandatory.'}
        else:
            ret = yield execute_cmd(cmd_type, params)
    return ejson_dumps(ret).encode('utf-8')


@do
def execute_cmd(cmd, params):
    try:
        resp = yield API_CMDS_ROUTER[cmd](params)
    except KeyError:
        resp = {'status': 'bad_message', 'label': 'Unknown command `%s`' % cmd}
    except ParsecError as exc:
        resp = exc.to_dict()
    return resp


class cmd_IDENTITY_LOAD_Schema(Schema):
    id = fields.String(required=True)
    key = fields.Base64Bytes(required=True)
    password = fields.String(missing=None)


@do
def api_identity_load(msg):
    msg, errors = cmd_IDENTITY_LOAD_Schema().load(msg)
    if errors:
        raise BadMessageError(errors)
    yield Effect(EIdentityLoad(**msg))
    return {'status': 'ok'}


@do
def api_identity_unload(msg):
    yield Effect(EIdentityUnload())
    return {'status': 'ok'}


@do
def api_identity_info(msg):
    identity = yield Effect(EIdentityGet())
    if identity:
        return {'status': 'ok', 'loaded': True, 'id': identity.id}
    else:
        return {'status': 'ok', 'loaded': False}


class cmd_EVENT_Schema(Schema):
    event = fields.String(required=True)
    sender = fields.Base64Bytes(required=True)


@do
def api_subscribe_event(msg):
    msg, errors = cmd_EVENT_Schema().load(msg)
    if errors:
        raise BadMessageError(errors)
    yield Effect(EClientSubscribeEvent(**msg))
    return {'status': 'ok'}


@do
def api_unsubscribe_event(msg):
    msg, errors = cmd_EVENT_Schema().load(msg)
    if errors:
        raise BadMessageError(errors)
    yield Effect(EClientUnsubscribeEvent(**msg))
    return {'status': 'ok'}


API_CMDS_ROUTER = {
    'subscribe_event': api_subscribe_event,
    'unsubscribe_event': api_unsubscribe_event,
    'identity_load': api_identity_load,
    'identity_unload': api_identity_unload,
    'identity_info': api_identity_info,
    'privkey_add': privkey_api.api_privkey_add,
    'privkey_get': privkey_api.api_privkey_get,
    'privkey_load': privkey_api.api_privkey_load,
}
