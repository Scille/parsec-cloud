from marshmallow import fields
from effect2 import Effect, do

from parsec.core.privkey import EPrivKeyExport, EPrivKeyLoad
from parsec.tools import UnknownCheckedSchema


class cmd_PRIVKEY_EXPORT_Schema(UnknownCheckedSchema):
    password = fields.String(required=True)


class cmd_PRIVKEY_LOAD_Schema(UnknownCheckedSchema):
    id = fields.String(required=True)
    password = fields.String(required=True)


@do
def api_privkey_export(msg):
    msg = cmd_PRIVKEY_EXPORT_Schema().load(msg)
    yield Effect(EPrivKeyExport(**msg))
    return {'status': 'ok'}


@do
def api_privkey_load(msg):
    msg = cmd_PRIVKEY_LOAD_Schema().load(msg)
    yield Effect(EPrivKeyLoad(**msg))
    return {'status': 'ok'}
