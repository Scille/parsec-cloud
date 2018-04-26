import swiftclient
from swiftclient.exceptions import ClientException

from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.exceptions import AlreadyExistsError, NotFoundError


class OpenStackBlockStoreComponent(BaseBlockStoreComponent):

    def __init__(
        self, signal_ns, openstack_url, openstack_container, openstack_username, openstack_api_key
    ):
        super().__init__(signal_ns)
        self.swift_client = swiftclient.Connection(
            user=openstack_username, preauthtoken=openstack_api_key, preauthurl=openstack_url
        )
        self._openstack_container = openstack_container

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
