import attr
from effect2 import Effect, asyncio_perform
from aiohttp import web

from parsec.backend.privkey import EPrivKeyGet, EPrivKeyAdd
from parsec.backend.pubkey import EPubKeyAdd
from parsec.exceptions import PrivKeyHashCollision, PrivKeyNotFound
from parsec.exceptions import ParsecError


def register_start_api(app, dispatcher, route='/start'):

    async def api_cipherkey_get(request):
        eff = Effect(EPrivKeyGet(request.match_info['hash']))
        try:
            key = await asyncio_perform(dispatcher, eff)
        except PrivKeyNotFound:
            raise web.HTTPNotFound(text='Unknown hash')
        return web.Response(body=key, content_type='application/octet-stream')

    async def api_cipherkey_post(request):
        cipherkey = await request.read()
        eff = Effect(EPrivKeyAdd(request.match_info['hash'], cipherkey))
        try:
            await asyncio_perform(dispatcher, eff)
        except PrivKeyHashCollision:
            raise web.HTTPConflict(text='This hash already exists...')
        return web.Response()

    async def api_pubkey_post(request):
        pubkey = await request.read()
        # TODO: should provide a token to avoid impersonation
        eff = Effect(EPubKeyAdd(request.match_info['identity'], pubkey))
        try:
            await asyncio_perform(dispatcher, eff)
        except ParsecError as exc:
            return web.HTTPConflict(text=exc.label)
        return web.Response()

    app.router.add_get(route + '/cipherkey/{hash}', api_cipherkey_get)
    app.router.add_post(route + '/cipherkey/{hash}', api_cipherkey_post)
    app.router.add_post(route + '/pubkey/{identity}', api_pubkey_post)
