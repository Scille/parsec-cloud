import json
from marshmallow import Schema, fields
from effect2 import Effect, do

from parsec.backend import vlob
# from parsec.core.client_connection import EClientSubscribeEvent, EClientUnsubscribeEvent
from parsec.tools import ejson_dumps, ejson_loads
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


# class cmd_EVENT_Schema(Schema):
#     event = fields.String(required=True)
#     sender = fields.Base64Bytes(required=True)


# @do
# def api_subscribe_event(msg):
#     msg, errors = cmd_EVENT_Schema().load(msg)
#     if errors:
#         raise BadMessageError(errors)
#     yield Effect(EClientSubscribeEvent(**msg))
#     return {'status': 'ok'}


# @do
# def api_unsubscribe_event(msg):
#     msg, errors = cmd_EVENT_Schema().load(msg)
#     if errors:
#         raise BadMessageError(errors)
#     yield Effect(EClientUnsubscribeEvent(**msg))
#     return {'status': 'ok'}


API_CMDS_ROUTER = {
    # 'subscribe_event': api_subscribe_event,
    # 'unsubscribe_event': api_unsubscribe_event,
    'vlob_create': vlob.api_vlob_create,
    'vlob_read': vlob.api_vlob_read,
    'vlob_update': vlob.api_vlob_update,
}
