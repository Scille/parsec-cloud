import attr
from uuid import uuid4
from marshmallow import fields
from effect import Effect, TypeDispatcher
from effect.do import do
from aioeffect import perform as asyncio_perform, performer as asyncio_performer
from functools import partial
from asyncio import get_event_loop
import boto3
from botocore.exceptions import (
    ClientError as S3ClientError, EndpointConnectionError as S3EndpointConnectionError
)

from parsec.service import BaseService, cmd
from parsec.exceptions import BlockError, BlockNotFound
from parsec.tools import BaseCmdSchema


@attr.s
class BlockCreate:
    id = attr.ib()
    content = attr.ib()


@attr.s
class BlockRead:
    id = attr.ib()


@attr.s
class Block:
    id = attr.ib()
    content = attr.ib()


class cmd_CREATE_Schema(BaseCmdSchema):
    content = fields.String(required=True)
    id = fields.String(missing=lambda: uuid4().hex)


@do
def api_block_create(msg):
    msg = cmd_CREATE_Schema().load(msg)
    block = yield Effect(BlockCreate(msg['id'], msg['content']))
    return {'status': 'ok', 'id': block.id}


class cmd_READ_Schema(BaseCmdSchema):
    id = fields.String(required=True)


@do
def api_block_read(msg):
    msg = cmd_READ_Schema().load(msg)
    block = yield Effect(BlockRead(msg['id']))
    return {'status': 'ok', 'id': block.id, 'content': block.content}


class BlockService(BaseService):

    name = 'BlockService'

    @cmd('block_create')
    async def _cmd_CREATE(self, session, msg):
        return await asyncio_perform(self._dispatcher, api_block_create(msg))

    @cmd('block_read')
    async def _cmd_READ(self, session, msg):
        return await asyncio_perform(self._dispatcher, api_block_read(msg))

    async def read(self, id):
        return await asyncio_perform(self.dispatcher, Effect(BlockRead(id)))

    async def create(self, content, id=None):
        id = id or uuid4().hex
        return await asyncio_perform(self.dispatcher, Effect(BlockCreate(id, content)))


def s3_block_service_factory(s3_region, s3_bucket, s3_key, s3_secret):
    s3 = boto3.client(
        's3', region_name=s3_region, aws_access_key_id=s3_key,
        aws_secret_access_key=s3_secret
    )

    @asyncio_performer
    async def perform_block_read(dispatcher, block_read):
        func = partial(s3.get_object, Bucket=s3_bucket, Key=block_read.id)
        try:
            obj = await get_event_loop().run_in_executor(None, func)
        except (S3ClientError, S3EndpointConnectionError) as exc:
            raise BlockNotFound(str(exc))
        return Block(id=block_read.id, content=obj['Body'].read())

    @asyncio_performer
    async def perform_block_create(dispatcher, block_create):
        func = partial(s3.put_object, Bucket=s3_bucket,
                       Key=block_create.id, Body=block_create.content)
        try:
            await get_event_loop().run_in_executor(None, func)
        except (S3ClientError, S3EndpointConnectionError) as exc:
            raise BlockError(str(exc))
        return Block(id=block_create.id, content=block_create.content)

    dispatcher = TypeDispatcher({
        BlockCreate: perform_block_create,
        BlockRead: perform_block_read
    })
    return BlockService(dispatcher=dispatcher)


def in_memory_block_service_factory():
    blocks = {}

    @asyncio_performer
    async def perform_block_read(dispatcher, block_read):
        try:
            return Block(id=block_read.id, content=blocks[block_read.id])
        except KeyError:
            raise BlockNotFound('Block %s not found' % block_read.id)

    @asyncio_performer
    async def perform_block_create(dispatcher, block_create):
        if block_create.id in blocks:
            raise BlockError('Block %s already exists' % block_create.id)
        blocks[block_create.id] = block_create.content
        return Block(id=block_create.id, content=block_create.content)

    dispatcher = TypeDispatcher({
        BlockCreate: perform_block_create,
        BlockRead: perform_block_read
    })
    return BlockService(dispatcher=dispatcher)
