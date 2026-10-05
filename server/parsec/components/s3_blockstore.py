# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from functools import partial
from typing import override

import anyio

# Required because the top-level module of anyio does not correctly load the submodule to_thread
# see https://github.com/microsoft/pyright/issues/10912
import anyio.to_thread
import boto3
from botocore.client import Config
from botocore.exceptions import BotoCoreError, ClientError

from parsec._parsec import BlockID, OrganizationID
from parsec.components.blockstore import (
    BaseBlockStoreComponent,
    BlockStoreCreateBadOutcome,
    BlockStoreReadBadOutcome,
)
from parsec.logging import get_logger

logger = get_logger()


def build_s3_slug(organization_id: OrganizationID, block_id: BlockID) -> str:
    # The slug uses the UUID canonical textual representation (eg.
    # `CoolOrg/3b917792-35ac-409f-9af1-fe6de8d2b905`)
    return f"{organization_id.str}/{block_id.hyphenated}"


class S3BlockStoreComponent(BaseBlockStoreComponent):
    def __init__(
        self,
        s3_region: str,
        s3_bucket: str,
        s3_key: str,
        s3_secret: str,
        s3_endpoint_url: str | None = None,
    ):
        # Create a session object, so that we can simplify creating resources or clients using a similar "configuration"
        self._s3_session = boto3.Session(
            region_name=s3_region,
            aws_access_key_id=s3_key,
            aws_secret_access_key=s3_secret,
        )
        self._s3_config = Config()

        # Verify that bucket exists
        client = self._s3_session.client("s3", config=self._s3_config, endpoint_url=s3_endpoint_url)
        client.head_bucket(Bucket=s3_bucket)

        # Create s3 resource & bucket
        self._s3 = self._s3_session.resource(
            "s3", config=self._s3_config, endpoint_url=s3_endpoint_url
        )
        self._s3_bucket = self._s3.Bucket(s3_bucket)

        # Configure logger
        self._logger = logger.bind(blockstore_type="S3", s3_region=s3_region, s3_bucket=s3_bucket)

    @override
    async def read(
        self, organization_id: OrganizationID, block_id: BlockID
    ) -> bytes | BlockStoreReadBadOutcome:
        slug = build_s3_slug(organization_id=organization_id, block_id=block_id)
        try:
            obj = await anyio.to_thread.run_sync(self._s3_bucket.Object(slug).get)
        except (BotoCoreError, ClientError) as exc:
            self._logger.warning(
                "Block read error",
                organization_id=organization_id.str,
                block_id=block_id.hex,
                exc_info=exc,
            )
            return BlockStoreReadBadOutcome.STORE_UNAVAILABLE

        return obj["Body"].read()

    @override
    async def create(
        self, organization_id: OrganizationID, block_id: BlockID, block: bytes
    ) -> BlockStoreCreateBadOutcome | None:
        slug = build_s3_slug(organization_id=organization_id, block_id=block_id)
        try:
            await anyio.to_thread.run_sync(partial(self._s3_bucket.Object(slug).put, Body=block))
        except (BotoCoreError, ClientError) as exc:
            self._logger.warning(
                "Block create error",
                organization_id=organization_id.str,
                block_id=block_id.hex,
                exc_info=exc,
            )
            return BlockStoreCreateBadOutcome.STORE_UNAVAILABLE

    async def wipe_organization_data(self, organization_id: OrganizationID) -> None:
        await anyio.to_thread.run_sync(
            partial(self.sync_wipe_organization_data, organization_id=organization_id)
        )

    def sync_wipe_organization_data(self, organization_id: OrganizationID) -> None:
        # Each object uploaded to the bucket are prefixed by the org id
        prefix = f"{organization_id}/"

        # Delete every objects and versions having the same prefix
        # https://docs.aws.amazon.com/boto3/latest/reference/services/s3/bucket/object_versions.html
        self._s3_bucket.object_versions.filter(Prefix=prefix).delete()
