from marshmallow import fields, validate
from effect2 import Effect, do

from parsec.core.identity import (
    EIdentityLoad, EIdentityUnload, EIdentityGet, EIdentityLogin, EIdentitySignup)
from parsec.exceptions import IdentityNotLoadedError
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
    try:
        identity = yield Effect(EIdentityGet())
        return {'status': 'ok', 'loaded': True, 'id': identity.id}
    except IdentityNotLoadedError:
        return {'status': 'ok', 'loaded': False, 'id': None}


class cmd_SIGNUP_Schema(UnknownCheckedSchema):
    id = fields.String(required=True)
    password = fields.String(required=True)
    key_size = fields.Int(default=2048, validate=validate.Range(min=1023, max=4097))


class cmd_LOGIN_Schema(UnknownCheckedSchema):
    id = fields.String(required=True)
    password = fields.String(required=True)


@do
def api_identity_signup(msg):
    msg = cmd_SIGNUP_Schema().load(msg)
    yield Effect(EIdentitySignup(**msg))
    return {'status': 'ok'}


@do
def api_identity_login(msg):
    msg = cmd_LOGIN_Schema().load(msg)
    yield Effect(EIdentityLogin(**msg))
    return {'status': 'ok'}