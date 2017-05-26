import boto3
from botocore.exceptions import (
    ClientError as S3ClientError, EndpointConnectionError as S3EndpointConnectionError
)
from asyncio import get_event_loop
from uuid import uuid4
from functools import partial
from datetime import datetime
from marshmallow import fields

from parsec.service import cmd
from parsec.core2.block_service import BaseBlockService, BlockNotFound, BlockError
from parsec.tools import BaseCmdSchema


class cmd_INIT_Schema(BaseCmdSchema):
    s3_region = fields.String(required=True)
    s3_bucket = fields.String(required=True)
    s3_key = fields.String(required=True)
    s3_secret = fields.String(required=True)


class S3BlockService(BaseBlockService):

    def __init__(self, s3_region, s3_bucket, s3_key, s3_secret):
        super().__init__()
        self._s3 = None
        self._s3_region = s3_region
        self._s3_bucket = s3_bucket
        self._s3_key = s3_key
        self._s3_secret = s3_secret

    async def bootstrap(self):
        assert not self._s3, 'Service already bootstrapped'
        self._s3 = boto3.client(
            's3', region_name=self._s3_region, aws_access_key_id=self._s3_key,
            aws_secret_access_key=self._s3_secret
        )

    async def create(self, content, id=None):
        if not self._s3:
            raise BlockError('S3 block service is not initialized')
        id = id if id else uuid4().hex
        created = datetime.utcnow().timestamp()
        func = partial(self._s3.put_object,
                       Bucket=self._s3_bucket,
                       Key=id,
                       Body=content,
                       Metadata={'created': str(created)})
        try:
            await get_event_loop().run_in_executor(None, func)
        except (S3ClientError, S3EndpointConnectionError) as exc:
            raise BlockError(str(exc))
        stat = await self.stat(id)
        timestamp = stat['creation_timestamp']
        return id

    async def read(self, id):
        if not self._s3:
            raise BlockError('S3 block service is not initialized')
        func = partial(self._s3.get_object, Bucket=self._s3_bucket, Key=id)
        try:
            obj = await get_event_loop().run_in_executor(None, func)
        except (S3ClientError, S3EndpointConnectionError) as exc:
            raise BlockNotFound(str(exc))
        return {
            'content': obj['Body'].read().decode(),
            'creation_timestamp': float(obj['Metadata']['created'])
        }

    async def stat(self, id):
        if not self._s3:
            raise BlockError('S3 block service is not initialized')
        func = partial(self._s3.get_object, Bucket=self._s3_bucket, Key=id)
        try:
            obj = await get_event_loop().run_in_executor(None, func)
        except (S3ClientError, S3EndpointConnectionError) as exc:
            raise BlockNotFound(str(exc))
        return {'creation_timestamp': float(obj['Metadata']['created'])}
