import json
from marshmallow import Schema, fields
from effect2 import Effect, do

from parsec.backend import vlob, user_vlob, group, message, pubkey
from parsec.backend.client_connection import EClientSubscribeEvent, EClientUnsubscribeEvent
from parsec.tools import ejson_dumps, ejson_loads
from parsec.exceptions import ParsecError, BadMessageError


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

    'vlob_create': vlob.api_vlob_create,
    'vlob_read': vlob.api_vlob_read,
    'vlob_update': vlob.api_vlob_update,

    'user_vlob_read': user_vlob.api_user_vlob_read,
    'user_vlob_update': user_vlob.api_user_vlob_update,

    'group_read': group.api_group_read,
    'group_create': group.api_group_create,
    'group_add_identities': group.api_group_add_identities,
    'group_remove_identities': group.api_group_remove_identities,

    'message_get': message.api_message_get,
    'message_new': message.api_message_new,

    'pubkey_get': pubkey.api_pubkey_get,
}
