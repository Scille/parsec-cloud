import boto3
from botocore.exceptions import (
    ClientError as S3ClientError,
    EndpointConnectionError as S3EndpointConnectionError,
)

from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.exceptions import AlreadyExistsError, NotFoundError


class S3BlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, s3_region, s3_bucket, s3_key, s3_secret):
        self._s3 = None
        self._s3_bucket = None
        self._s3 = boto3.client(
            "s3", region_name=s3_region, aws_access_key_id=s3_key, aws_secret_access_key=s3_secret
        )
        self._s3_bucket = s3_bucket
        self._s3.head_bucket(Bucket=s3_bucket)

    async def get(self, id):
        try:
            obj = self._s3.get_object(Bucket=self._s3_bucket, Key=id)
        except (S3ClientError, S3EndpointConnectionError) as exc:
            raise NotFoundError("Unknown block id.")

        return obj["Body"].read()

    async def post(self, id, block):
        try:
            self._s3.put_object(Bucket=self._s3_bucket, Key=id, Body=block)
        except (S3ClientError, S3EndpointConnectionError) as exc:
            raise AlreadyExistsError("A block already exists with id `%s`." % id)

        return id
