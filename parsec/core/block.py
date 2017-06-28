import attr
from effect import TypeDispatcher
from aioeffect import performer as asyncio_performer
from functools import partial
from asyncio import get_event_loop
import boto3
from botocore.exceptions import (
    ClientError as S3ClientError, EndpointConnectionError as S3EndpointConnectionError
)

from parsec.exceptions import BlockError, BlockNotFound


@attr.s
class EBlockCreate:
    id = attr.ib()
    content = attr.ib()


@attr.s
class EBlockRead:
    id = attr.ib()


@attr.s
class Block:
    id = attr.ib()
    content = attr.ib()


def s3_block_dispatcher_factory(s3_region, s3_bucket, s3_key, s3_secret):
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
        EBlockCreate: perform_block_create,
        EBlockRead: perform_block_read
    })
    return dispatcher


def in_memory_block_dispatcher_factory():
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
        EBlockCreate: perform_block_create,
        EBlockRead: perform_block_read
    })
    return dispatcher
