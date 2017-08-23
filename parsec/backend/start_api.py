import attr
from effect2 import Effect, asyncio_perform
from aiohttp import web
from logbook import Logger

from parsec.backend.privkey import EPrivKeyGet, EPrivKeyAdd
from parsec.backend.pubkey import EPubKeyAdd
from parsec.exceptions import PrivKeyHashCollision, PrivKeyNotFound
from parsec.exceptions import ParsecError


logger = Logger('start_api')


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
        hash = request.match_info['hash']
        eff = Effect(EPrivKeyAdd(hash, cipherkey))
        try:
            await asyncio_perform(dispatcher, eff)
        except PrivKeyHashCollision:
            raise web.HTTPConflict(text='This hash already exists...')
        logger.info('New cipherkey `%s` registered' % hash)
        return web.Response()

    async def api_pubkey_post(request):
        pubkey = await request.read()
        # TODO: should provide a token to avoid impersonation
        identity = request.match_info['identity']
        eff = Effect(EPubKeyAdd(identity, pubkey))
        try:
            await asyncio_perform(dispatcher, eff)
        except ParsecError as exc:
            return web.HTTPConflict(text=exc.label)
        logger.info('New identity `%s` registered' % identity)
        return web.Response()

    app.router.add_get(route + '/cipherkey/{hash}', api_cipherkey_get)
    app.router.add_post(route + '/cipherkey/{hash}', api_cipherkey_post)
    app.router.add_post(route + '/pubkey/{identity}', api_pubkey_post)
