from marshmallow import fields
from effect2 import Effect, do

from parsec.core.privkey import EPrivkeyAdd, EPrivkeyGet, EPrivkeyLoad
from parsec.tools import UnknownCheckedSchema
from parsec.exceptions import BadMessageError


class cmd_PRIVKEY_ADD_Schema(UnknownCheckedSchema):
    id = fields.String(required=True)
    key = fields.Base64Bytes(required=True)
    password = fields.String(required=True)


class cmd_PRIVKEY_GET_Schema(UnknownCheckedSchema):
    id = fields.String(required=True)
    password = fields.String(required=True)


class cmd_PRIVKEY_LOAD_Schema(UnknownCheckedSchema):
    id = fields.String(required=True)
    password = fields.String(required=True)


@do
def api_privkey_add(msg):
    msg, errors = cmd_PRIVKEY_ADD_Schema().load(msg)
    if errors:
        raise BadMessageError(errors)
    yield Effect(EPrivkeyAdd(**msg))
    return {'status': 'ok'}


@do
def api_privkey_get(msg):
    msg, errors = cmd_PRIVKEY_GET_Schema().load(msg)
    if errors:
        raise BadMessageError(errors)
    key = yield Effect(EPrivkeyGet(**msg))
    return {'status': 'ok', 'key': key.decode()}


@do
def api_privkey_load(msg):
    msg, errors = cmd_PRIVKEY_LOAD_Schema().load(msg)
    if errors:
        raise BadMessageError(errors)
    yield Effect(EPrivkeyLoad(**msg))
    return {'status': 'ok'}
