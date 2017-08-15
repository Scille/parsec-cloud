from functools import partial
from asyncio import get_event_loop
import boto3
from botocore.exceptions import (
    ClientError as S3ClientError, EndpointConnectionError as S3EndpointConnectionError
)

from parsec.exceptions import BlockError, BlockNotFound


class S3BlockConnection:
    def __init__(self, s3_region, s3_bucket, s3_key, s3_secret):
        self.s3 = boto3.client(
            's3', region_name=s3_region, aws_access_key_id=s3_key,
            aws_secret_access_key=s3_secret
        )
        self.s3_bucket = s3_bucket

    async def close_connection(self):
        pass

    async def read(self, id: str):
        func = partial(self.s3.get_object, Bucket=self.s3_bucket, Key=id)
        try:
            obj = await get_event_loop().run_in_executor(None, func)
        except (S3ClientError, S3EndpointConnectionError) as exc:
            raise BlockNotFound(str(exc))
        return obj['Body'].read()

    async def create(self, id: str, content: bytes):
        func = partial(self.s3.put_object, Bucket=self.s3_bucket,
                       Key=id, Body=content)
        try:
            await get_event_loop().run_in_executor(None, func)
        except (S3ClientError, S3EndpointConnectionError) as exc:
            raise BlockError(str(exc))
