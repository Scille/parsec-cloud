# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from unittest.mock import Mock
from unittest import mock
from swiftclient.exceptions import ClientException
import pytest

from parsec.api.protocol import OrganizationID, BlockID
from parsec.backend.block import BlockAlreadyExistsError, BlockNotFoundError, BlockTimeoutError
from parsec.backend.swift_blockstore import SwiftBlockStoreComponent


@pytest.mark.trio
async def test_swift_get():
    org_id = OrganizationID("org42")
    block_id = BlockID.from_hex("0694a21176354e8295e28a543e5887f9")

    with mock.patch("swiftclient.Connection") as connection_mock:
        connection_mock.return_value = Mock()
        connection_mock().head_container.return_value = True
        blockstore = SwiftBlockStoreComponent("http://url", "scille", "parsec", "john", "secret")
        # Ok
        connection_mock().get_object.return_value = True, "content"
        assert await blockstore.read(org_id, block_id) == "content"
        connection_mock().get_object.assert_called_once_with(
            "parsec", "org42/0694a211-7635-4e82-95e2-8a543e5887f9"
        )
        connection_mock().get_object.reset_mock()
        # Not found
        connection_mock().get_object.side_effect = ClientException(http_status=404, msg="")
        with pytest.raises(BlockNotFoundError):
            assert await blockstore.read(org_id, block_id)
        # Other exception
        connection_mock().get_object.side_effect = ClientException(http_status=500, msg="")
        with pytest.raises(BlockTimeoutError):
            assert await blockstore.read(org_id, block_id)


@pytest.mark.trio
async def test_swift_create():
    org_id = OrganizationID("org42")
    block_id = BlockID.from_hex("0694a21176354e8295e28a543e5887f9")

    with mock.patch("swiftclient.Connection") as connection_mock:
        connection_mock.return_value = Mock()
        connection_mock().head_container.return_value = True
        blockstore = SwiftBlockStoreComponent("http://url", "scille", "parsec", "john", "secret")
        # Ok
        connection_mock().get_object.side_effect = ClientException(http_status=404, msg="")
        await blockstore.create(org_id, block_id, "content")
        connection_mock().put_object.assert_called_with(
            "parsec", "org42/0694a211-7635-4e82-95e2-8a543e5887f9", "content"
        )
        connection_mock().put_object.reset_mock()
        # Already exist
        connection_mock().get_object.side_effect = None
        with pytest.raises(BlockAlreadyExistsError):
            await blockstore.create(org_id, block_id, "content")
        connection_mock().put_object.assert_not_called()
        # Connection error at HEAD
        connection_mock().get_object.side_effect = ClientException(msg="Connection error")
        with pytest.raises(BlockTimeoutError):
            await blockstore.create(org_id, block_id, "content")
        connection_mock().put_object.assert_not_called()
        # Unknown exception at HEAD
        connection_mock().get_object.side_effect = ClientException(http_status=500, msg="")
        with pytest.raises(BlockTimeoutError):
            await blockstore.create(org_id, block_id, "content")
        # Connection error at PUT
        connection_mock().get_object.side_effect = ClientException(msg="Connection error")
        connection_mock().put_object.side_effect = ClientException(msg="Connection error")
        with pytest.raises(BlockTimeoutError):
            await blockstore.create(org_id, block_id, "content")
        # Unknown exception at PUT
        connection_mock().put_object.side_effect = ClientException(http_status=500, msg="")
        with pytest.raises(BlockTimeoutError):
            await blockstore.create(org_id, block_id, "content")
