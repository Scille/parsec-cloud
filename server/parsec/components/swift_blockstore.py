# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from functools import partial
from typing import override
from unittest.mock import Mock

import anyio
import pbr.version

# TODO: is this still needed ? for what purpose ?
original_version_info = pbr.version.VersionInfo


def side_effect(key: str) -> str:
    if key == "python-swiftclient":
        return "3.5.0"

    else:
        return original_version_info(key)  # type: ignore


pbr.version.VersionInfo = Mock(side_effect=side_effect)

import swiftclient
from swiftclient.exceptions import ClientException

from parsec._parsec import BlockID, OrganizationID
from parsec.components.blockstore import (
    BaseBlockStoreComponent,
    BlockStoreCreateBadOutcome,
    BlockStoreReadBadOutcome,
)
from parsec.logging import get_logger

logger = get_logger()


def build_swift_slug(organization_id: OrganizationID, id: BlockID) -> str:
    # The slug uses the UUID canonical textual representation (eg.
    # `CoolOrg/3b917792-35ac-409f-9af1-fe6de8d2b905`)
    return f"{organization_id.str}/{id.hyphenated}"


class SwiftBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(
        self, auth_url: str, tenant: str, container: str, user: str, password: str
    ) -> None:
        self.swift_client = swiftclient.Connection(
            authurl=auth_url, user=":".join([user, tenant]), key=password
        )
        self._container = container
        self.swift_client.head_container(container)
        self._logger = logger.bind(blockstore_type="Swift", authurl=auth_url)

    @override
    async def read(
        self, organization_id: OrganizationID, block_id: BlockID
    ) -> bytes | BlockStoreReadBadOutcome:
        slug = build_swift_slug(organization_id=organization_id, id=block_id)
        try:
            _, obj = await anyio.to_thread.run_sync(
                self.swift_client.get_object, self._container, slug
            )
            # TODO: further tests are needed to ensure this is okay
            assert isinstance(obj, bytes)

        except ClientException as exc:
            self._logger.warning(
                "Block read error",
                organization_id=organization_id.str,
                block_id=block_id.hex,
                exc_info=exc,
            )
            return BlockStoreReadBadOutcome.STORE_UNAVAILABLE

        return obj

    @override
    async def create(
        self, organization_id: OrganizationID, block_id: BlockID, block: bytes
    ) -> None | BlockStoreCreateBadOutcome:
        slug = build_swift_slug(organization_id=organization_id, id=block_id)
        try:
            await anyio.to_thread.run_sync(
                partial(self.swift_client.put_object, self._container, slug, block)
            )

        except ClientException as exc:
            self._logger.warning(
                "Block create error",
                organization_id=organization_id.str,
                block_id=block_id.hex,
                exc_info=exc,
            )
            return BlockStoreCreateBadOutcome.STORE_UNAVAILABLE
