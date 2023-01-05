# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from unittest import mock
from unittest.mock import Mock

import pytest
from botocore.exceptions import ClientError as S3ClientError
from botocore.exceptions import EndpointConnectionError as S3EndpointConnectionError

from parsec.api.protocol import BlockID, OrganizationID
from parsec.backend.block import BlockStoreError
from parsec.backend.s3_blockstore import S3BlockStoreComponent


@pytest.mark.trio
async def test_s3_read(caplog):
    org_id = OrganizationID("org42")
    block_id = BlockID.from_hex("0694a21176354e8295e28a543e5887f9")

    def _assert_log():
        log = caplog.assert_occurred_once("[warning  ] Block read error")
        assert f"organization_id={org_id.str}" in log
        assert f"block_id={block_id.hex}" in log
        assert len(caplog.messages) == 1
        caplog.clear()

    with mock.patch("boto3.client") as client_mock:
        client_mock.return_value = Mock()
        client_mock().head_bucket.return_value = True
        blockstore = S3BlockStoreComponent("europe", "parsec", "john", "secret")

        # Ok
        response_mock = Mock()
        response_mock.read.return_value = "content"
        client_mock().get_object.return_value = {"Body": response_mock}
        assert await blockstore.read(org_id, block_id) == "content"
        client_mock().get_object.assert_called_once_with(
            Bucket="parsec", Key="org42/0694a211-7635-4e82-95e2-8a543e5887f9"
        )
        client_mock().get_object.reset_mock()
        assert not caplog.messages

        # Not found
        client_mock().get_object.side_effect = S3ClientError(
            error_response={"Error": {"Code": "404"}}, operation_name="GET"
        )
        with pytest.raises(BlockStoreError):
            assert await blockstore.read(org_id, block_id)
        _assert_log()

        # Connection error
        client_mock().get_object.side_effect = S3EndpointConnectionError(endpoint_url="url")
        with pytest.raises(BlockStoreError):
            assert await blockstore.read(org_id, block_id)
        _assert_log()

        # Unknown exception
        client_mock().get_object.side_effect = S3ClientError(
            error_response={"Error": {"Code": "401"}}, operation_name="GET"
        )
        with pytest.raises(BlockStoreError):
            assert await blockstore.read(org_id, block_id)
        _assert_log()


# This test has been detected as flaky.
# Using re-runs is a valid temporary solutions but the problem should be investigated in the future.
@pytest.mark.trio
@pytest.mark.flaky(reruns=3)
async def test_s3_create(caplog):
    org_id = OrganizationID("org42")
    block_id = BlockID.from_hex("0694a21176354e8295e28a543e5887f9")

    def _assert_log():
        log = caplog.assert_occurred_once("[warning  ] Block create error")
        assert f"organization_id={org_id.str}" in log
        assert f"block_id={block_id.hex}" in log
        assert len(caplog.messages) == 1
        caplog.clear()

    with mock.patch("boto3.client") as client_mock:
        client_mock.return_value = Mock()
        client_mock().head_container.return_value = True
        blockstore = S3BlockStoreComponent("europe", "parsec", "john", "secret")

        # Ok
        client_mock().head_object.side_effect = S3ClientError(
            error_response={"Error": {"Code": "404"}}, operation_name="HEAD"
        )
        await blockstore.create(org_id, block_id, "content")
        client_mock().put_object.assert_called_with(
            Bucket="parsec", Key="org42/0694a211-7635-4e82-95e2-8a543e5887f9", Body="content"
        )
        client_mock().put_object.reset_mock()
        assert not caplog.messages

        # Connection error at PUT
        client_mock().head_object.side_effect = S3ClientError(
            error_response={"Error": {"Code": "404"}}, operation_name="HEAD"
        )
        client_mock().put_object.side_effect = S3EndpointConnectionError(endpoint_url="url")
        with pytest.raises(BlockStoreError):
            await blockstore.create(org_id, block_id, "content")
        _assert_log()

        # Unknown exception at PUT
        client_mock().put_object.side_effect = S3ClientError(
            error_response={"Error": {"Code": "401"}}, operation_name="PUT"
        )
        with pytest.raises(BlockStoreError):
            await blockstore.create(org_id, block_id, "content")
        _assert_log()
