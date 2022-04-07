# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from unittest.mock import Mock
from unittest import mock

from botocore.exceptions import (
    ClientError as S3ClientError,
    EndpointConnectionError as S3EndpointConnectionError,
)
import pytest

from parsec.api.protocol import OrganizationID, BlockID
from parsec.backend.s3_blockstore import S3BlockStoreComponent
from parsec.backend.block import BlockAlreadyExistsError, BlockNotFoundError, BlockTimeoutError


@pytest.mark.trio
async def test_s3_read():
    org_id = OrganizationID("org42")
    block_id = BlockID.from_hex("0694a21176354e8295e28a543e5887f9")

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
        # Not found
        client_mock().get_object.side_effect = S3ClientError(
            error_response={"Error": {"Code": "404"}}, operation_name="GET"
        )
        with pytest.raises(BlockNotFoundError):
            assert await blockstore.read(org_id, block_id)
        # Connection error
        client_mock().get_object.side_effect = S3EndpointConnectionError(endpoint_url="url")
        with pytest.raises(BlockTimeoutError):
            assert await blockstore.read(org_id, block_id)
        # Unknown exception
        client_mock().get_object.side_effect = S3ClientError(
            error_response={"Error": {"Code": "401"}}, operation_name="GET"
        )
        with pytest.raises(BlockTimeoutError):
            assert await blockstore.read(org_id, block_id)


@pytest.mark.trio
async def test_s3_create():
    org_id = OrganizationID("org42")
    block_id = BlockID.from_hex("0694a21176354e8295e28a543e5887f9")

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
        # Already exist
        client_mock().head_object.side_effect = None
        with pytest.raises(BlockAlreadyExistsError):
            await blockstore.create(org_id, block_id, "content")
        client_mock().put_object.assert_not_called()
        # Connection error at HEAD
        client_mock().head_object.side_effect = S3EndpointConnectionError(endpoint_url="url")
        with pytest.raises(BlockTimeoutError):
            await blockstore.create(org_id, block_id, "content")
        client_mock().put_object.assert_not_called()
        # Unknown exception at HEAD
        client_mock().head_object.side_effect = S3ClientError(
            error_response={"Error": {"Code": "401"}}, operation_name="HEAD"
        )
        with pytest.raises(BlockTimeoutError):
            await blockstore.create(org_id, block_id, "content")
        # Connection error at PUT
        client_mock().head_object.side_effect = S3ClientError(
            error_response={"Error": {"Code": "404"}}, operation_name="HEAD"
        )
        client_mock().put_object.side_effect = S3EndpointConnectionError(endpoint_url="url")
        with pytest.raises(BlockTimeoutError):
            await blockstore.create(org_id, block_id, "content")
        # Unknown exception at PUT
        client_mock().put_object.side_effect = S3ClientError(
            error_response={"Error": {"Code": "401"}}, operation_name="PUT"
        )
        with pytest.raises(BlockTimeoutError):
            await blockstore.create(org_id, block_id, "content")
