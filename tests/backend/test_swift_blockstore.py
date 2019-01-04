from unittest.mock import Mock
from unittest import mock
import swiftclient
from swiftclient.exceptions import ClientException
import pytest

from parsec.backend.blockstore import (
    BlockstoreAlreadyExistsError,
    BlockstoreNotFoundError,
    BlockstoreTimeoutError,
)
from parsec.backend.swift_blockstore import SwiftBlockstoreComponent

import swiftclient  # noqa
from swiftclient.exceptions import ClientException  # noqa


@pytest.mark.trio
async def test_swift_get():
    with mock.patch("swiftclient.Connection") as connection_mock:
        connection_mock.return_value = Mock()
        connection_mock().head_container.return_value = True
        blockstore = SwiftBlockstoreComponent("http://url", "scille", "parsec", "john", "secret")
        # Ok
        connection_mock().get_object.return_value = True, "content"
        assert await blockstore.read(123) == "content"
        # Not found
        connection_mock().get_object.side_effect = ClientException(http_status=404, msg="")
        with pytest.raises(BlockstoreNotFoundError):
            assert await blockstore.read(123)
        # Other exception
        connection_mock().get_object.side_effect = ClientException(http_status=500, msg="")
        with pytest.raises(BlockstoreTimeoutError):
            assert await blockstore.read(123)


@pytest.mark.trio
async def test_swift_post():
    with mock.patch("swiftclient.Connection") as connection_mock:
        connection_mock.return_value = Mock()
        connection_mock().head_container.return_value = True
        blockstore = SwiftBlockstoreComponent("http://url", "scille", "parsec", "john", "secret")
        # Ok
        connection_mock().get_object.side_effect = ClientException(http_status=404, msg="")
        await blockstore.create(123, "content", "device_id")
        connection_mock().put_object.assert_called_with(
            "parsec", "123", "content", headers={"x-object-meta-author": "device_id"}
        )
        # Already exists
        connection_mock().get_object.side_effect = None
        connection_mock().get_object.return_value = True, "content"
        with pytest.raises(BlockstoreAlreadyExistsError):
            await blockstore.create(123, "content", "device_id")
        # Other exception
        connection_mock().get_object.side_effect = ClientException(http_status=500, msg="")
        with pytest.raises(BlockstoreTimeoutError):
            await blockstore.create(123, "content", "device_id")
