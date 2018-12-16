import trio
import boto3
from botocore.exceptions import (
    ClientError as S3ClientError,
    EndpointConnectionError as S3EndpointConnectionError,
)
from uuid import UUID
from functools import partial

from parsec.types import DeviceID
from parsec.backend.blockstore import (
    BaseblockStoreComponent,
    BlockstoreAlreadyExistsError,
    BlockstoreNotFoundError,
    BlockstoreTimeoutError,
)


class S3BlockstoreComponent(BaseblockStoreComponent):
    def __init__(self, s3_region, s3_bucket, s3_key, s3_secret):
        self._s3 = None
        self._s3_bucket = None
        self._s3 = boto3.client(
            "s3", region_name=s3_region, aws_access_key_id=s3_key, aws_secret_access_key=s3_secret
        )
        self._s3_bucket = s3_bucket
        self._s3.head_bucket(Bucket=s3_bucket)

    async def read(self, id: UUID) -> bytes:
        try:
            obj = self._s3.get_object(Bucket=self._s3_bucket, Key=id)

        except S3ClientError as exc:
            if exc.response["Error"]["Code"] == "404":
                raise BlockstoreNotFoundError() from exc

            else:
                raise BlockstoreTimeoutError() from exc

        except S3EndpointConnectionError as exc:
            raise BlockstoreTimeoutError() from exc

        # Remember, to retreive the author: DeviceID(obj["Metadata"]["author"])
        return obj["Body"].read()

    async def create(self, id: UUID, block: bytes, author: DeviceID) -> None:
        try:
            await trio.run_sync_in_worker_thread(
                partial(
                    self._s3.put_object,
                    Bucket=self._s3_bucket,
                    Key=id,
                    Body=block,
                    Metadata={"author": author},
                )
            )
        except (S3ClientError, S3EndpointConnectionError) as exc:
            raise BlockstoreTimeoutError() from exc
        # TODO: Handle AlreadyExistsError exception
