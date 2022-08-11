# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import trio
import boto3
from botocore.exceptions import BotoCoreError, ClientError
from structlog import get_logger
from functools import partial

from parsec.api.protocol import OrganizationID, BlockID
from parsec.backend.block import BlockStoreError
from parsec.backend.blockstore import BaseBlockStoreComponent


logger = get_logger()


def build_s3_slug(organization_id: OrganizationID, id: BlockID):
    # The slug uses the UUID canonical textual representation (eg.
    # `CoolOrg/3b917792-35ac-409f-9af1-fe6de8d2b905`) where `BlockID.__str__`
    # uses the short textual representation (eg. `3b91779235ac409f9af1fe6de8d2b905`)
    return f"{organization_id}/{id.uuid}"


class S3BlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, s3_region, s3_bucket, s3_key, s3_secret, s3_endpoint_url=None):
        self._s3 = None
        self._s3_bucket = None
        self._s3 = boto3.client(
            "s3",
            region_name=s3_region,
            aws_access_key_id=s3_key,
            aws_secret_access_key=s3_secret,
            endpoint_url=s3_endpoint_url,
        )
        self._s3_bucket = s3_bucket
        self._s3.head_bucket(Bucket=s3_bucket)
        self._logger = logger.bind(blockstore_type="S3", s3_region=s3_region, s3_bucket=s3_bucket)

    async def read(self, organization_id: OrganizationID, id: BlockID) -> bytes:
        slug = build_s3_slug(organization_id=organization_id, id=id)
        try:
            obj = self._s3.get_object(Bucket=self._s3_bucket, Key=slug)
        except (BotoCoreError, ClientError) as exc:
            self._logger.warning(
                "Block read error",
                organization_id=str(organization_id),
                block_id=str(id),
                exc_info=exc,
            )
            raise BlockStoreError(exc) from exc

        return obj["Body"].read()

    async def create(self, organization_id: OrganizationID, id: BlockID, block: bytes) -> None:
        slug = build_s3_slug(organization_id=organization_id, id=id)
        try:
            await trio.to_thread.run_sync(
                partial(self._s3.put_object, Bucket=self._s3_bucket, Key=slug, Body=block)
            )
        except (BotoCoreError, ClientError) as exc:
            self._logger.warning(
                "Block create error",
                organization_id=str(organization_id),
                block_id=str(id),
                exc_info=exc,
            )
            raise BlockStoreError(exc) from exc
