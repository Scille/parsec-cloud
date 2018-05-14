from unittest.mock import Mock
import pbr.version

from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.exceptions import AlreadyExistsError, NotFoundError

original_version_info = pbr.version.VersionInfo


def side_effect(key):
    if key == "python-swiftclient":
        return "3.5.0"
    else:
        return original_version_info(key)


pbr.version.VersionInfo = Mock(side_effect=side_effect)

import swiftclient  # noqa
from swiftclient.exceptions import ClientException  # noqa


class OpenStackBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(
        self,
        signal_ns,
        openstack_auth_url,
        openstack_container,
        openstack_user,
        openstack_tenant,
        openstack_password,
    ):
        super().__init__(signal_ns)
        self.swift_client = swiftclient.Connection(
            authurl=openstack_auth_url,
            user=":".join([openstack_user, openstack_tenant]),
            key=openstack_password,
        )
        self._openstack_container = openstack_container
        self.swift_client.head_container(openstack_container)

    async def get(self, id):
        try:
            _, obj = self.swift_client.get_object(self._openstack_container, id)
        except ClientException as exc:
            if exc.http_status == 404:
                raise NotFoundError("Unknown block id.")
            else:
                raise exc
        return obj

    async def post(self, id, block):
        # TODO find a more efficient way to check if block already exists
        try:
            _, obj = self.swift_client.get_object(self._openstack_container, id)
        except ClientException as exc:
            if exc.http_status == 404:
                self.swift_client.put_object(self._openstack_container, id, block)
            else:
                raise exc
        else:
            raise AlreadyExistsError("A block already exists with id `%s`." % id)

        return id
