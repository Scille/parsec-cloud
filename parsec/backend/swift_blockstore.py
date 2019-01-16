import trio
from unittest.mock import Mock
import pbr.version
from uuid import UUID
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

from parsec.types import DeviceID, OrganizationID
from parsec.backend.blockstore import (
    BaseBlockstoreComponent,
    BlockstoreAlreadyExistsError,
    BlockstoreNotFoundError,
    BlockstoreTimeoutError,
)


# Swift custom headers are case insensitive, but `get_object`
# return them in lower case in a case-sensitive dict...
AUTHOR_META_HEADER = "X-Object-Meta-AUTHOR".lower()


class SwiftBlockstoreComponent(BaseBlockstoreComponent):
    def __init__(self, auth_url, tenant, container, user, password):
        self.swift_client = swiftclient.Connection(
            authurl=auth_url, user=":".join([user, tenant]), key=password
        )
        self._container = container
        self.swift_client.head_container(container)

    async def read(self, organization_id: OrganizationID, id: UUID) -> bytes:
        slug = f"{organization_id}/{id}"
        try:
            headers, obj = await trio.run_sync_in_worker_thread(
                self.swift_client.get_object, self._container, slug
            )

        except ClientException as exc:
            if exc.http_status == 404:
                raise BlockstoreNotFoundError() from exc

            else:
                raise BlockstoreTimeoutError() from exc

        # Remember, to retreive the author: DeviceID(headers[AUTHOR_META_HEADER])
        return obj

    async def create(
        self, organization_id: OrganizationID, id: UUID, block: bytes, author: DeviceID
    ) -> None:
        slug = f"{organization_id}/{id}"
        try:
            _, obj = await trio.run_sync_in_worker_thread(
                self.swift_client.get_object, self._container, slug
            )

        except ClientException as exc:
            if exc.http_status == 404:
                await trio.run_sync_in_worker_thread(
                    partial(
                        self.swift_client.put_object,
                        self._container,
                        slug,
                        block,
                        headers={AUTHOR_META_HEADER: author},
                    )
                )
            else:
                raise BlockstoreTimeoutError() from exc

        else:
            raise BlockstoreAlreadyExistsError()
