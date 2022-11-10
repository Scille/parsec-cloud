# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from unittest.mock import Mock
from unittest import mock
from swiftclient.exceptions import ClientException
import pytest

from parsec.api.protocol import OrganizationID, BlockID
from parsec.backend.block import BlockStoreError
from parsec.backend.swift_blockstore import SwiftBlockStoreComponent


@pytest.mark.trio
async def test_swift_get(caplog):
    org_id = OrganizationID("org42")
    block_id = BlockID.from_str("0694a21176354e8295e28a543e5887f9")

    def _assert_log():
        log = caplog.assert_occured_once("[warning  ] Block read error")
        assert f"organization_id={org_id.str}" in log
        assert f"block_id={block_id.str}" in log
        assert len(caplog.messages) == 1
        caplog.clear()

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
        assert not caplog.messages

        # Not found
        connection_mock().get_object.side_effect = ClientException(http_status=404, msg="")
        with pytest.raises(BlockStoreError):
            assert await blockstore.read(org_id, block_id)
        _assert_log()

        # Other exception
        connection_mock().get_object.side_effect = ClientException(http_status=500, msg="")
        with pytest.raises(BlockStoreError):
            assert await blockstore.read(org_id, block_id)
        _assert_log()


@pytest.mark.trio
async def test_swift_create(caplog):
    org_id = OrganizationID("org42")
    block_id = BlockID.from_str("0694a21176354e8295e28a543e5887f9")

    def _assert_log():
        log = caplog.assert_occured_once("[warning  ] Block create error")
        assert f"organization_id={org_id.str}" in log
        assert f"block_id={block_id.str}" in log
        assert len(caplog.messages) == 1
        caplog.clear()

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
        assert not caplog.messages

        # Connection error at PUT
        connection_mock().get_object.side_effect = ClientException(msg="Connection error")
        connection_mock().put_object.side_effect = ClientException(msg="Connection error")
        with pytest.raises(BlockStoreError):
            await blockstore.create(org_id, block_id, "content")
        _assert_log()

        # Unknown exception at PUT
        connection_mock().put_object.side_effect = ClientException(http_status=500, msg="")
        with pytest.raises(BlockStoreError):
            await blockstore.create(org_id, block_id, "content")
        _assert_log()
