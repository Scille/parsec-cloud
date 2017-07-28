import attr
from effect2 import TypeDispatcher
from functools import partial
from asyncio import get_event_loop
import boto3
from botocore.exceptions import (
    ClientError as S3ClientError, EndpointConnectionError as S3EndpointConnectionError
)

from parsec.core.block import Block, EBlockCreate, EBlockRead
from parsec.exceptions import BlockError, BlockNotFound


def s3_block_dispatcher_factory(s3_region, s3_bucket, s3_key, s3_secret):
    s3 = boto3.client(
        's3', region_name=s3_region, aws_access_key_id=s3_key,
        aws_secret_access_key=s3_secret
    )

    async def perform_block_read(intent):
        func = partial(s3.get_object, Bucket=s3_bucket, Key=intent.id)
        try:
            obj = await get_event_loop().run_in_executor(None, func)
        except (S3ClientError, S3EndpointConnectionError) as exc:
            raise BlockNotFound(str(exc))
        return Block(id=intent.id, content=obj['Body'].read())

    async def perform_block_create(intent):
        func = partial(s3.put_object, Bucket=s3_bucket,
                       Key=intent.id, Body=intent.content)
        try:
            await get_event_loop().run_in_executor(None, func)
        except (S3ClientError, S3EndpointConnectionError) as exc:
            raise BlockError(str(exc))
        return Block(id=intent.id, content=intent.content)

    dispatcher = TypeDispatcher({
        EBlockCreate: perform_block_create,
        EBlockRead: perform_block_read
    })
    return dispatcher
