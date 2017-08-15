import attr
import aiohttp
from effect2 import TypeDispatcher, do, Effect, AsyncFunc

from parsec.core.backend import EBackendBlockStoreGetURL
from parsec.exceptions import BlockError, BlockNotFound, BlockConnectionError


# TODO: id shouldn't be allowed to be decided by user
@attr.s
class EBlockCreate:
    id = attr.ib()
    content = attr.ib()


@attr.s
class EBlockRead:
    id = attr.ib()


@attr.s
class EBlockReset:
    pass


@attr.s
class Block:
    id = attr.ib()
    content = attr.ib()


@attr.s
class RESTBlockConnection:
    url = attr.ib()

    async def close_connection(self):
        pass

    async def read(self, id: str):
        route = '%s/%s' % (self.url, id)
        async with aiohttp.ClientSession() as session:
            async with session.get(route) as resp:
                if resp.status == 200:
                    return await resp.read()
                elif resp.status == 404:
                    raise BlockNotFound('Block %s not found' % id)
                else:
                    raise BlockError(await resp.text())

    async def create(self, id: str, content: bytes):
        route = '%s/%s' % (self.url, id)
        async with aiohttp.ClientSession() as session:
            async with session.post(route, data=content) as resp:
                if resp.status != 200:
                    if resp.status == 409:
                        raise BlockError('Block %s already exists' % id)
                    else:
                        raise BlockError(await resp.text())


def block_connection_factory(url):
    if url.startswith('s3:'):
        try:
            from parsec.core.block_s3 import S3BlockConnection
            _, region, bucket, key_id, key_secret = url.split(':')
        except ImportError as exc:
            raise SystemExit('Parsec needs boto3 to support S3 block storage (error: %s).' %
                             exc)
        except ValueError:
            raise SystemExit('Invalid s3 block store '
                             ' (should be `s3:<region>:<bucket>:<id>:<secret>`.')
        return S3BlockConnection(region, bucket, key_id, key_secret)
    elif url.startswith('http://'):
        return RESTBlockConnection(url)
    else:
        raise SystemExit('Unknown block store `%s`.' % url)


@attr.s
class BlockComponent:
    url = attr.ib(default=None)
    connection = attr.ib(default=None)

    async def shutdown(self, app=None):
        await self.perform_block_reset()

    def performer_with_connection_factory(self, async_performer):
        @do
        def performer_with_connection(intent):
            if not self.connection:
                url = yield Effect(EBackendBlockStoreGetURL())
                self.connection = block_connection_factory(url)
            return (yield AsyncFunc(async_performer(intent)))

        return performer_with_connection

    async def close_connection(self):
        if self.connection:
            await self.connection.close_connection()
            self.connection = None

    async def perform_block_reset(self, intent=None):
        if self.connection:
            await self.connection.close_connection()
            self.connection = None

    async def perform_block_read(self, intent):
        content = await self.connection.read(intent.id)
        return Block(id=intent.id, content=content)

    async def perform_block_create(self, intent):
        await self.connection.create(intent.id, intent.content)
        return Block(id=intent.id, content=intent.content)

    def get_dispatcher(self):
        return TypeDispatcher({
            EBlockReset: self.perform_block_reset,
            EBlockCreate: self.performer_with_connection_factory(
                self.perform_block_create),
            EBlockRead: self.performer_with_connection_factory(
                self.perform_block_read)
        })
