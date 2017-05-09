import boto3
from botocore.exceptions import ClientError as S3ClientError, EndpointConnectionError as S3EndpointConnectionError
from asyncio import get_event_loop
from uuid import uuid4
from functools import partial
from datetime import datetime
from marshmallow import fields

from parsec.service import cmd
from parsec.core.block_service import BaseBlockService, BlockNotFound, BlockError
from parsec.tools import BaseCmdSchema


class cmd_INIT_Schema(BaseCmdSchema):
    s3_region = fields.String(required=True)
    s3_bucket = fields.String(required=True)
    s3_key = fields.String(required=True)
    s3_secret = fields.String(required=True)


class S3BlockService(BaseBlockService):

    def __init__(self):
        super().__init__()
        self._s3 = None
        self._s3_bucket = None

    @cmd('block_init')
    async def _cmd_INIT(self, session, msg):
        msg = cmd_INIT_Schema().load(msg)
        await self.init(**msg)
        return {'status': 'ok'}

    async def init(self, s3_region, s3_bucket, s3_key, s3_secret):
        self._s3 = boto3.client('s3', region_name=s3_region, aws_access_key_id=s3_key, aws_secret_access_key=s3_secret)
        self._s3_bucket = s3_bucket

    async def create(self, content, id=None):
        if not self._s3:
            raise BlockError('S3 block service is not initialized')
        id = id if id else uuid4().hex
        created = datetime.utcnow().timestamp()
        func = partial(self._s3.put_object, Bucket=self._s3_bucket, Key=id, Body=content, Metadata={'created': created})
        try:
            await get_event_loop().run_in_executor(None, func)
        except (S3ClientError, S3EndpointConnectionError) as exc:
            raise BlockError(str(exc))
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
