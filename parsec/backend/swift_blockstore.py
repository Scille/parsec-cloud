# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import trio
from unittest.mock import Mock
import pbr.version
from functools import partial

original_version_info = pbr.version.VersionInfo


def side_effect(key):
    if key == "python-swiftclient":
        return "3.5.0"

    else:
        return original_version_info(key)


pbr.version.VersionInfo = Mock(side_effect=side_effect)

import swiftclient
from swiftclient.exceptions import ClientException
from structlog import get_logger

from parsec.api.protocol import OrganizationID, BlockID
from parsec.backend.block import BlockStoreError
from parsec.backend.blockstore import BaseBlockStoreComponent


logger = get_logger()


def build_swift_slug(organization_id: OrganizationID, id: BlockID):
    # The slug uses the UUID canonical textual representation (eg.
    # `CoolOrg/3b917792-35ac-409f-9af1-fe6de8d2b905`) where `BlockID.__str__`
    # uses the short textual representation (eg. `3b91779235ac409f9af1fe6de8d2b905`)
    return f"{organization_id}/{id.uuid}"


class SwiftBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, auth_url, tenant, container, user, password):
        self.swift_client = swiftclient.Connection(
            authurl=auth_url, user=":".join([user, tenant]), key=password
        )
        self._container = container
        self.swift_client.head_container(container)
        self._logger = logger.bind(blockstore_type="Swift", authurl=auth_url)

    async def read(self, organization_id: OrganizationID, id: BlockID) -> bytes:
        slug = build_swift_slug(organization_id=organization_id, id=id)
        try:
            _, obj = await trio.to_thread.run_sync(
                self.swift_client.get_object, self._container, slug
            )

        except ClientException as exc:
            self._logger.warning(
                "Block read error",
                organization_id=str(organization_id),
                block_id=str(id),
                exc_info=exc,
            )
            raise BlockStoreError(exc) from exc

        return obj

    async def create(self, organization_id: OrganizationID, id: BlockID, block: bytes) -> None:
        slug = build_swift_slug(organization_id=organization_id, id=id)
        try:
            await trio.to_thread.run_sync(
                partial(self.swift_client.put_object, self._container, slug, block)
            )

        except ClientException as exc:
            self._logger.warning(
                "Block create error",
                organization_id=str(organization_id),
                block_id=str(id),
                exc_info=exc,
            )
            raise BlockStoreError(exc) from exc
