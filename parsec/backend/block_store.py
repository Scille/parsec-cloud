import attr
from effect2 import TypeDispatcher
from aiohttp import web
from logbook import Logger


logger = Logger('block_store')


# TODO test perform_blockstore_get_url
@attr.s
class BlockStoreGetURL:
    pass


@attr.s
class BlockStoreInfoComponent:
    url = attr.ib()

    def perform_blockstore_get_url(self, intent):
        return self.url

    def get_dispatcher(self):
        return TypeDispatcher({
            BlockStoreGetURL: self.perform_blockstore_get_url
        })


def register_in_memory_block_store_api(app, prefix='/blockstore'):
    # Really, really simple stuff ;-)
    blocks = {}

    async def api_block_get(request):
        id = request.match_info['id']
        try:
            return web.Response(body=blocks[id], content_type='application/octet-stream')
        except KeyError:
            raise web.HTTPNotFound()

    async def api_block_post(request):
        id = request.match_info['id']
        if id in blocks:
            raise web.HTTPConflict()
        blocks[id] = await request.read()
        logger.debug('Create new block: %s' % id)
        return web.Response()

    app.router.add_get(prefix + '/{id}', api_block_get)
    app.router.add_post(prefix + '/{id}', api_block_post)
