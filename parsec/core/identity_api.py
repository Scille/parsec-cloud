from marshmallow import fields
from effect import Effect
from effect.do import do

from parsec.core.identity import EIdentityLoad, EIdentityUnload, EIdentityGet
from parsec.tools import UnknownCheckedSchema


class cmd_IDENTITY_LOAD_Schema(UnknownCheckedSchema):
    id = fields.String(required=True)
    key = fields.Base64Bytes(required=True)
    password = fields.String(missing=None)


@do
def api_identity_load(msg):
    msg = cmd_IDENTITY_LOAD_Schema().load(msg)
    yield Effect(EIdentityLoad(**msg))
    return {'status': 'ok'}


@do
def api_identity_unload(msg):
    UnknownCheckedSchema().load(msg)
    yield Effect(EIdentityUnload())
    return {'status': 'ok'}


@do
def api_identity_info(msg):
    UnknownCheckedSchema().load(msg)
    identity = yield Effect(EIdentityGet())
    if identity:
        return {'status': 'ok', 'loaded': True, 'id': identity.id}
    else:
        return {'status': 'ok', 'loaded': False}
