import json
from marshmallow import Schema, fields
from effect2 import Effect, do

from parsec.core import fs_api, identity_api, privkey_api
from parsec.tools import ejson_dumps, ejson_loads
from parsec.core.client_connection import EClientSubscribeEvent, EClientUnsubscribeEvent
from parsec.core.backend import EBackendStatus
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


@do
def api_backend_status(msg):
    yield Effect(EBackendStatus())
    return {'status': 'ok'}


API_CMDS_ROUTER = {
    'subscribe_event': api_subscribe_event,
    'unsubscribe_event': api_unsubscribe_event,
    'backend_status': api_backend_status,
    'identity_load': identity_api.api_identity_load,
    'identity_unload': identity_api.api_identity_unload,
    'identity_info': identity_api.api_identity_info,
    'privkey_add': privkey_api.api_privkey_add,
    'privkey_get': privkey_api.api_privkey_get,
    'privkey_load': privkey_api.api_privkey_load,
    'synchronize': fs_api.api_synchronize,
    'group_create': fs_api.api_group_create,
    'dustbin_show': fs_api.api_dustbin_show,
    'history': fs_api.api_manifest_history,  # TODO Integrate api_file_history
    'restore': fs_api.api_manifest_restore,  # TODO Integrate api_file_restore
    'file_create': fs_api.api_file_create,
    'file_read': fs_api.api_file_read,
    'file_write': fs_api.api_file_write,
    'file_truncate': fs_api.api_file_truncate,
    'folder_create': fs_api.api_folder_create,
    'stat': fs_api.api_stat,
    'move': fs_api.api_move,
    'delete': fs_api.api_delete,
    'undelete': fs_api.api_undelete,
}
