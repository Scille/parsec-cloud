import attr
import json
from marshmallow import fields
from effect2 import TypeDispatcher, Effect, do, asyncio_perform
from aiohttp import web

from parsec.exceptions import PrivKeyHashCollision, PrivKeyNotFound
from parsec.tools import UnknownCheckedSchema, ejson_dumps, ejson_loads
from parsec.exceptions import ParsecError


@attr.s
class EPrivKeyGet:
    hash = attr.ib()


@attr.s
class EPrivKeyAdd:
    hash = attr.ib()
    cipherkey = attr.ib()


class cmd_PRIVKEY_GET_Schema(UnknownCheckedSchema):
    hash = fields.String(required=True)


class cmd_PRIVKEY_ADD_Schema(UnknownCheckedSchema):
    hash = fields.String(required=True)
    cipherkey = fields.Base64Bytes(required=True)


@do
def api_privkey_get(msg):
    msg = cmd_PRIVKEY_GET_Schema().load(msg)
    cipherkey = yield Effect(EPrivKeyGet(**msg))
    return {'status': 'ok', 'hash': msg['hash'], 'cipherkey': cipherkey}


@do
def api_privkey_add(msg):
    msg = cmd_PRIVKEY_ADD_Schema().load(msg)
    yield Effect(EPrivKeyAdd(**msg))
    return {'status': 'ok'}


@attr.s
class MockedPrivKeyComponent:
    _keys = attr.ib(default=attr.Factory(dict))

    @do
    def perform_privkey_add(self, intent):
        # TODO: should check for authorization token to avoid impersonation
        assert isinstance(intent.cipherkey, (bytes, bytearray))
        if intent.hash in self._keys:
            raise PrivKeyHashCollision('Hash collision, change your password and retry.')
        else:
            self._keys[intent.hash] = intent.cipherkey

    @do
    def perform_privkey_get(self, intent):
        try:
            return self._keys[intent.hash]
        except KeyError:
            raise PrivKeyNotFound('No entry with this hash')

    def get_dispatcher(self):
        return TypeDispatcher({
            EPrivKeyGet: self.perform_privkey_get,
            EPrivKeyAdd: self.perform_privkey_add
        })


async def _load_json_body(request):
    try:
        return await request.json(loads=ejson_loads)
    except json.decoder.JSONDecodeError:
        raise web.HTTPBadRequest(
            body=ejson_dumps({'status': 'bad_msg', 'label': 'Message is not a valid JSON.'}))


def HTTPBadRequestJson(jsonbody):
    return web.HTTPBadRequest(
        body=ejson_dumps(jsonbody).encode('utf-8'),
        headers={'Content-type': 'application/json'})


def HTTPNotFoundJson(jsonbody):
    return web.HTTPNotFound(
        body=ejson_dumps(jsonbody).encode('utf-8'),
        headers={'Content-type': 'application/json'})


def register_privkey_api(app, components, route='/privkeys'):
    dispatcher = components.get_dispatcher()

    async def api_get_privkey(request):
        reqjson = await _load_json_body(request)
        try:
            retjson = await asyncio_perform(dispatcher, api_privkey_get(reqjson))
        except PrivKeyNotFound as exc:
            raise HTTPNotFoundJson(exc.to_dict())
        except ParsecError as exc:
            raise HTTPBadRequestJson(exc.to_dict())
        return web.json_response(retjson, dumps=ejson_dumps)

    async def api_add_privkey(request):
        reqjson = await _load_json_body(request)
        try:
            retjson = await asyncio_perform(dispatcher, api_privkey_add(reqjson))
        except ParsecError as exc:
            raise HTTPBadRequestJson(exc.to_dict())
        return web.json_response(retjson, dumps=ejson_dumps)

    app.router.add_get(route, api_get_privkey)
    app.router.add_post(route, api_add_privkey)
